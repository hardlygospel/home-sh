```
 ██╗  ██╗ ██████╗ ███╗   ███╗███████╗   ███████╗██╗  ██╗
 ██║  ██║██╔═══██╗████╗ ████║██╔════╝   ██╔════╝██║  ██║
 ███████║██║   ██║██╔████╔██║█████╗     ███████╗███████║
 ██╔══██║██║   ██║██║╚██╔╝██║██╔══╝     ╚════██║██╔══██║
 ██║  ██║╚██████╔╝██║ ╚═╝ ██║███████╗██╗███████║██║  ██║
 ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝╚═╝╚══════╝╚═╝  ╚═╝
```

<div align="center">

> **Proof of concept.** Early-stage experiment — expect rough edges and breaking changes. Not production-ready.

**A cozy terminal home. Lo-fi beats, chat, games, and a cat. All over SSH.**

![Status](https://img.shields.io/badge/status-proof%20of%20concept-orange?style=flat-square)
![Rust](https://img.shields.io/badge/rust-1.82%2B-orange?style=flat-square&logo=rust)
![PostgreSQL](https://img.shields.io/badge/postgresql-16-336791?style=flat-square&logo=postgresql&logoColor=white)
![Icecast](https://img.shields.io/badge/icecast-streaming-orange?style=flat-square)
![Docker](https://img.shields.io/badge/docker-compose-2496ED?style=flat-square&logo=docker&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)

```
ssh -p 2222 yourserver
```

</div>

---

Inspired by [late.sh](https://github.com/mpiorowski/late-sh). Same idea, Rust stack, and a cat instead of a bonsai.

## Features

- **Lo-fi radio** — Icecast stream plays the moment you connect. Now-playing widget shows track, artist, and progress.
- **Multi-room chat** — `#general`, `#tech`, `#music`, `#random`. @mention support. Real-time across all sessions.
- **Virtual cat** — grows from kitten to majestic cat over 7 stages. Feed, water, and groom daily or it gets sad. Neglect it for a week and it dies.
- **Hacker News** — top 20 stories, scrollable, refreshed every 5 minutes.
- **World clocks** — UTC, New York, London, Tokyo in the sidebar.
- **63 themes** — Late, Catppuccin (Mocha/Macchiato/Frappe/Latte), Gruvbox, One Dark, Rose Pine, Tokyo Night, Kanagawa, Dracula, CRT series (Amber/Green/Cyan/C64/Blood), Cyberpunk, Monokai, and more.
- **Games** — Snake and Wordle built in.
- **Profile** — set username, bio, timezone, theme.

## Layout

```
┌─ home.sh ────────────────────────────────────────────────────────────────┐
│ home.sh  [d]ash [c]hat [a]rcade [p]rofile [?]help        ● 3 online     │
├──────────────────┬───────────────────────────────────┬───────────────────┤
│  ── Rooms ──     │                                   │  ── Clock ──      │
│  > #general      │   (active screen content)         │  UTC  14:35       │
│    #tech         │                                   │  NY   10:35       │
│    #music        │                                   │  ── Now Playing ──│
│    #random       │                                   │  SomaFM           │
│  ── Online ──    │                                   │  Groove Salad     │
│  ● yourname      │                                   │  ██████░░  2:14   │
│                  │                                   │  ── Cat ──────────│
│                  │                                   │   /\_/\           │
│                  │                                   │  ( ^.^ )  happy   │
│                  │                                   │  grow: ████░░ s3  │
│                  │                                   │  [f]eed [w]ater   │
├──────────────────┴───────────────────────────────────┴───────────────────┤
│ [i]nput [h/l]rooms [j/k]scroll [q]quit                thanks for hanging out│
└──────────────────────────────────────────────────────────────────────────┘
```

## Quick start

Requires Docker.

```bash
git clone https://github.com/hardlygospel/home-sh
cd home-sh
docker compose up -d
ssh -p 2222 localhost
```

On first connect, pick any username. Your session persists.

## Configuration

Edit `docker-compose.yml` or set environment variables:

| Variable | Default | Description |
|---|---|---|
| `HOME_SSH_PORT` | `2222` | SSH listen port |
| `HOME_DB_HOST` | `postgres` | PostgreSQL host |
| `HOME_DB_USER` | `homeuser` | PostgreSQL user |
| `HOME_DB_PASSWORD` | `homepass` | PostgreSQL password — **change this** |
| `HOME_DB_NAME` | `homedb` | Database name |
| `HOME_ICECAST_URL` | `http://icecast:8000` | Icecast base URL |
| `RUST_LOG` | `info` | Log level |

## Radio

Liquidsoap relays [SomaFM Groove Salad](https://somafm.com/groovesalad/) through Icecast automatically. The now-playing widget updates every 10 seconds. Listen from outside the TUI:

```bash
mpv http://yourserver:8000/stream
```

Override the stream source in `infra/liquidsoap/radio.liq`.

## Key bindings

| Key | Action |
|---|---|
| `d` | Dashboard |
| `c` | Chat |
| `a` | Arcade |
| `p` | Profile |
| `?` | Help |
| `q` | Quit |
| `f` | Feed cat |
| `w` | Water cat |
| `g` | Groom cat |
| `i` | Chat input |
| `h` / `l` | Switch chat rooms |
| `j` / `k` | Scroll |

## Stack

| Layer | Technology |
|---|---|
| SSH server | [russh](https://github.com/Eugeny/russh) |
| TUI | [ratatui](https://ratatui.rs/) + crossterm |
| Database | PostgreSQL 16 via deadpool-postgres |
| Audio | Icecast + Liquidsoap |
| News | HN Firebase API |
| Weather | wttr.in |
| Async | tokio |

---

<div align="center">

Inspired by [late.sh](https://github.com/mpiorowski/late-sh) — go use that if you want something polished.

</div>
