use crate::combat_state::CombatState;
use crate::combatant::create_combatants;
use crate::components::*;
use crate::events::EventQueue;
use crate::log::CombatLog;
use crate::systems::ActionSystem;
use crate::systems::DraftingSystem;
use crate::systems::RollingSystem;
use quad_rand as qrand;
use specs::DispatcherBuilder;
use specs::{World, WorldExt};
use std::time::SystemTime;
use ws::{listen, CloseCode, Handler, Message, Request, Response, Result, Sender};

mod combat_state;
mod combatant;
mod components;
mod constants;
mod events;
mod log;
mod shared;
mod systems;

fn main() {
    // seed random to current timestamp
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    qrand::srand(time.as_secs());

    // Setup specs world
    let mut world = World::new();
    world.register::<Named>();
    world.register::<Health>();
    world.register::<LightAttacker>();
    world.register::<HeavyAttacker>();
    world.register::<Defender>();
    world.register::<DicePool>();
    let combatants = create_combatants(&mut world);

    let combat_state = CombatState::new(combatants);

    // Insert global resources
    world.insert(combat_state);
    world.insert(EventQueue {
        ..Default::default()
    });
    world.insert(CombatLog {
        ..Default::default()
    });

    // Dispatcher setup will register all systems and do other setup
    let mut dispatcher = DispatcherBuilder::new()
        .with(DraftingSystem, "drafting", &[])
        .with(RollingSystem, "rolling", &[])
        .with(ActionSystem, "action", &[])
        .build();
    dispatcher.setup(&mut world);

    game_loop(&mut dispatcher, &mut world);

    listen("127.0.0.1:9000", |out| Server { out }).unwrap()
}

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            _ => Ok(Response::new(
                200,
                "OK",
                b"Websocket server is running".to_vec(),
            )),
        }
    }

    // Handle messages recieved in the websocket (in this case, only on `/ws`).
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let client_id: usize = self.out.token().into();

        let server_msg = if msg.is_text() {
            Some(handle_text_message(client_id, msg))
        } else if msg.is_binary() {
            Some(handle_binary_message(client_id, msg))
        } else {
            None
        };

        // Broadcast to all connections.
        server_msg.map_or(Ok(()), |msg| self.out.broadcast(msg))
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        let client_id: usize = self.out.token().into();
        let code_number: u16 = code.into();
        println!(
            "WebSocket closing - client: {}, code: {} {:?}, reason: {}",
            client_id, code_number, code, reason
        );
    }
}

fn handle_text_message(client_id: usize, msg: Message) -> Message {
    let client_msg: shared::ClientMessage =
        serde_json::from_str(&msg.into_text().unwrap()).unwrap();

    println!(
        "Server received text message\ntext: '{}'\nfrom: '{}'\n",
        client_msg.text, client_id
    );

    let server_msg: Message = serde_json::to_string(&shared::ServerMessage {
        id: client_id,
        text: client_msg.text,
    })
    .unwrap()
    .into();

    server_msg
}

fn handle_binary_message(client_id: usize, msg: Message) -> Message {
    let binary_msg: shared::ClientMessage = rmp_serde::from_slice(&msg.into_data()).unwrap();

    println!(
        "Server received binary message\ntext: '{}'\nfrom: '{}'\n",
        binary_msg.text, client_id
    );

    let server_msg: Message = rmp_serde::to_vec(&shared::ServerMessage {
        id: client_id,
        text: binary_msg.text,
    })
    .unwrap()
    .into();

    server_msg
}

fn game_loop(dispatcher: &mut specs::Dispatcher, world: &mut specs::World) {
    // run ECS systems
    dispatcher.dispatch(&world);
    world.maintain();

    // handle events
    let mut event_queue = world.write_resource::<EventQueue>();
    if !event_queue.events.is_empty() {
        println!("current events: {:?}", event_queue.events);
    }
    if !event_queue.new_events.is_empty() {
        println!("new events: {:?}", event_queue.new_events);
    }
    event_queue.events = (*event_queue.new_events).to_vec();
    event_queue.new_events.clear();
}