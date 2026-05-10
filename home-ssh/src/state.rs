use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

use home_core::{
    config::Config,
    models::article::Article,
    icecast::NowPlaying,
    services::{ChatService, CatService, UserService},
};
use home_core::models::chat::ChatMessage;

pub type Db = deadpool_postgres::Pool;

#[derive(Debug, Clone)]
pub struct ActiveUser {
    pub id: Uuid,
    pub username: String,
}

pub struct SharedState {
    pub db: Db,
    pub config: Config,
    pub user_svc: UserService,
    pub chat_svc: ChatService,
    pub cat_svc: CatService,
    pub now_playing: Arc<Mutex<Option<NowPlaying>>>,
    pub hn_stories: Arc<Mutex<Vec<Article>>>,
    pub active_users: Arc<Mutex<HashMap<Uuid, String>>>,
    pub chat_broadcast: broadcast::Sender<ChatMessage>,
}

impl SharedState {
    pub fn new(db: Db, config: Config) -> Arc<Self> {
        let (chat_broadcast, _) = broadcast::channel(512);
        Arc::new(SharedState {
            user_svc: UserService::new(db.clone()),
            chat_svc: ChatService::new(db.clone()),
            cat_svc: CatService::new(db.clone()),
            db,
            config,
            now_playing: Arc::new(Mutex::new(None)),
            hn_stories: Arc::new(Mutex::new(Vec::new())),
            active_users: Arc::new(Mutex::new(HashMap::new())),
            chat_broadcast,
        })
    }
}
