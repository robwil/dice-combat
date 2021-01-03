use crate::events::EventQueue;
use quad_rand as qrand;
use std::time::SystemTime;
use ws::listen;

mod combat_state;
mod combatant;
mod components;
mod constants;
mod events;
mod log;
mod shared;
mod systems;
mod websocket;

fn main() {
    // seed random to current timestamp
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    qrand::srand(time.as_secs());

    listen("127.0.0.1:9000", |out| websocket::Server::new(out)).unwrap()
}