# Contributing to home.sh

Thanks for taking a look. Read this before you open a PR — it covers what
home.sh is, how it's laid out, and the bar a change has to clear.

## What this project actually is

home.sh is a proof of concept. It is a Frankenstein — a place to prototype
ideas and push the good ones upstream to [late.sh](https://github.com/mpiorowski/late-sh),
which is the real, maintained project. It is not built for production or for
day-to-day personal use. If you want something polished, go use late.sh.

That framing matters for contributions: a feature here earns its keep by being
worth porting to late.sh. So changes should be written so they can move
upstream cleanly, following late.sh's conventions even though this repo is
looser.

## Layout

A two-crate Cargo workspace:

- **`home-core`** — config, database, migrations, models, services, the
  Icecast/news plumbing. No TUI.
- **`home-ssh`** — the SSH server and the ratatui TUI. The app lives under
  `home-ssh/src/app/`:
  - `app_state.rs` — in-memory UI state.
  - `input.rs` — key routing.
  - `tick.rs` — per-frame update; the heartbeat of the app.
  - `render.rs` — draws the frame.
  - `screens/` — full-screen surfaces (dashboard, chat, arcade, snake,
    wordle, profile).
  - `widgets/` — sidebar/inline pieces (cat, clock, now playing, visualizer).

The render loop is sync. Anything that touches the database or the network is
an async service in `home-core/src/services`; the UI drains results on the
next tick rather than awaiting in the draw path. Keep that boundary — never
`.await` in render or input handling.

## Conventions

- **Match late.sh's domain pattern** for anything you intend to send upstream:
  a self-contained module with its own state, service, and UI, no cross-module
  re-exports. Look at how the cat widget is isolated and copy that shape.
- **Tests.** late.sh requires tests for every change; hold to the same here.
  Pure logic gets inline `#[cfg(test)]` unit tests. Anything touching the DB
  gets an integration test against a real test database, not a mock.
- **Run the build and the tests before you push.** Don't make CI catch what
  `cargo fmt`, `cargo clippy`, and `cargo test` would have caught locally.
- **Sign off your commits** (`git commit -s`) if the change is headed upstream
  — late.sh requires DCO sign-off on every commit.
- **No AI attribution.** Don't leave `Co-Authored-By` AI trailers, "generated
  with" lines, or tool credits in commits, PR descriptions, or code comments.
  Write commit messages and docs in plain language.
- **Keep changes focused.** One idea per PR. Preserve the MIT licence notice.

## Recent changes

- **The cat never dies.** Neglect only makes it unhappy — there is no death
  state. Feed (`f`), water (`w`), groom (`g`) to bring its mood back up.
- **Cat and goldfish companions, ported upstream.** Both were built as
  isolated modules following the late.sh bonsai pattern and contributed to
  late.sh in [PR #205](https://github.com/mpiorowski/late-sh/pull/205). The
  sidebar shows one pet at a time — `k` opens the cat, `g` opens the goldfish,
  and whichever was opened last is the one displayed. Neither pet dies; the
  worst mood either reaches is "sad". The goldfish also takes rocks and
  plants, lights, water changes, and up to five friends.

## Licence

MIT. By contributing you agree your work is contributed under that licence,
and you certify you have the right to contribute it.
