use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use home_core::config::Config;
use home_core::icecast::fetch_now_playing;
use home_core::news::fetch_top_stories;
use russh::keys::key::PublicKey;
use russh::server::{Auth, Handle, Msg, Server, Session};
use russh::{Channel, ChannelId, CryptoVec};
use russh_keys::key::KeyPair;
use tokio::sync::mpsc;

use crate::app::App;
use crate::state::SharedState;
use uuid::Uuid;

/// Write adapter that buffers output and flushes to the SSH handle.
#[derive(Clone)]
pub struct SshWriter {
    pub handle: Handle,
    pub channel_id: ChannelId,
    pub sink: Vec<u8>,
}

impl std::io::Write for SshWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let handle = self.handle.clone();
        let channel_id = self.channel_id;
        let data: CryptoVec = self.sink.clone().into();
        futures::executor::block_on(async move {
            let _ = handle.data(channel_id, data).await;
        });
        self.sink.clear();
        Ok(())
    }
}

pub async fn run_server(pool: Pool, config: Config) -> Result<()> {
    let state = SharedState::new(pool, config.clone());

    // Background: poll Icecast every 10s
    let np_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            if let Ok(Some(np)) = fetch_now_playing(&np_state.config.icecast_url).await {
                let mut guard = np_state.now_playing.lock().await;
                *guard = Some(np);
            }
        }
    });

    // Background: poll HN every 5 minutes, and once on startup
    {
        let hn_state = state.clone();
        tokio::spawn(async move {
            if let Ok(stories) = fetch_top_stories(20).await {
                let mut guard = hn_state.hn_stories.lock().await;
                *guard = stories;
            }
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300));
            interval.tick().await; // skip first immediate tick
            loop {
                interval.tick().await;
                if let Ok(stories) = fetch_top_stories(20).await {
                    let mut guard = hn_state.hn_stories.lock().await;
                    *guard = stories;
                }
            }
        });
    }

    let port = config.ssh_port;
    let key = KeyPair::generate_ed25519();

    let ssh_config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_millis(100),
        auth_rejection_time_initial: Some(std::time::Duration::from_millis(0)),
        keys: vec![key],
        ..Default::default()
    };

    let mut server = HomeServer {
        state: state.clone(),
    };

    server
        .run_on_address(Arc::new(ssh_config), format!("0.0.0.0:{}", port))
        .await
        .context("SSH server failed")?;

    Ok(())
}

struct HomeServer {
    state: Arc<SharedState>,
}

impl Server for HomeServer {
    type Handler = ClientHandler;

    fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> ClientHandler {
        tracing::info!("New client from {:?}", peer_addr);
        ClientHandler {
            state: self.state.clone(),
            user_id: None,
            username: String::new(),
            fingerprint: String::new(),
            input_tx: None,
            resize_tx: None,
            session_id: Uuid::new_v4(),
        }
    }
}

pub struct ClientHandler {
    state: Arc<SharedState>,
    user_id: Option<Uuid>,
    username: String,
    fingerprint: String,
    input_tx: Option<mpsc::Sender<Vec<u8>>>,
    resize_tx: Option<mpsc::Sender<(u16, u16)>>,
    session_id: Uuid,
}

#[async_trait]
impl russh::server::Handler for ClientHandler {
    type Error = anyhow::Error;

    async fn auth_publickey(
        &mut self,
        user: &str,
        public_key: &PublicKey,
    ) -> Result<Auth> {
        self.username = user.to_string();
        self.fingerprint = public_key.fingerprint();
        Ok(Auth::Accept)
    }

    async fn auth_password(&mut self, user: &str, _password: &str) -> Result<Auth> {
        self.username = user.to_string();
        // Use username as fingerprint for password auth
        self.fingerprint = format!("pw:{}", user);
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool> {
        let (input_tx, input_rx) = mpsc::channel::<Vec<u8>>(256);
        let (resize_tx, resize_rx) = mpsc::channel::<(u16, u16)>(16);

        self.input_tx = Some(input_tx);
        self.resize_tx = Some(resize_tx);

        // Get or create user
        let user = match self.state
            .user_svc
            .get_or_create_user(&self.fingerprint, &self.username)
            .await
        {
            Ok(u) => u,
            Err(e) => {
                tracing::error!("Failed to get/create user: {}", e);
                return Ok(false);
            }
        };

        let user_id = user.id;
        self.user_id = Some(user_id);

        // Register active user
        {
            let mut active = self.state.active_users.lock().await;
            active.insert(user_id, user.username.clone());
        }

        let state = self.state.clone();
        let session_id = self.session_id;
        let handle = session.handle();
        let channel_id = channel.id();

        tokio::spawn(async move {
            let writer = SshWriter {
                handle,
                channel_id,
                sink: Vec::new(),
            };
            let mut app = App::new(state.clone(), user, writer, input_rx, resize_rx);
            if let Err(e) = app.run().await {
                tracing::error!("App error: {}", e);
            }
            // Deregister user
            let mut active = state.active_users.lock().await;
            active.remove(&user_id);
            tracing::info!("Session {} ended", session_id);
        });

        Ok(true)
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<()> {
        if let Some(tx) = &self.input_tx {
            let _ = tx.send(data.to_vec()).await;
        }
        Ok(())
    }

    async fn pty_request(
        &mut self,
        _channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(russh::Pty, u32)],
        _session: &mut Session,
    ) -> Result<()> {
        if let Some(tx) = &self.resize_tx {
            let _ = tx.send((col_width as u16, row_height as u16)).await;
        }
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        _channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _session: &mut Session,
    ) -> Result<()> {
        if let Some(tx) = &self.resize_tx {
            let _ = tx.send((col_width as u16, row_height as u16)).await;
        }
        Ok(())
    }

    async fn channel_close(
        &mut self,
        _channel: ChannelId,
        _session: &mut Session,
    ) -> Result<()> {
        if let Some(uid) = self.user_id {
            let mut active = self.state.active_users.lock().await;
            active.remove(&uid);
        }
        Ok(())
    }
}
