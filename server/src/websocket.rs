use std::cmp::Reverse;
use crate::combat_state::*;
use crate::combatant::*;
use crate::components::*;
use crate::events::*;
use crate::log::*;
use crate::shared::*;
use crate::systems::*;
use specs::RunNow;
use specs::World;
use specs::WorldExt;
use std::sync::Arc;
use std::sync::Mutex;
use ws::Handshake;
use ws::{CloseCode, Handler, Message, Request, Response, Result, Sender};

pub struct Server {
    pub world: specs::World,
    // TODO: when we have multi-player, this would be a HashMap<Player, ClientGameState> or something like that
    pub materialized_state: ClientGameState,
}

pub struct Connection {
    pub out: Sender,
    pub server: Arc<Mutex<Server>>,
}

impl Handler for Connection {
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

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let server = self.server.lock().unwrap();
        // send initial state to client
        let combat_state = server.world.read_resource::<CombatState>();
        println!("Server state: {:?}", *combat_state);
        println!(
            "Sending initial state to client: {:?}",
            server.materialized_state.clone()
        );
        let server_msg: Message =
            serde_json::to_string(&ServerMessage::NewState(server.materialized_state.clone()))
                .unwrap()
                .into();
        self.out.send(server_msg)
    }

    // Handle messages recieved in the websocket (in this case, only on `/ws`).
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let client_id: usize = self.out.token().into();

        let server_msg = if msg.is_text() {
            Some(self.handle_text_message(client_id, msg))
        } else {
            None
        };

        // Broadcast to all connections.
        // TODO: need to change this to be specific to each client (when multiplayer)

        println!("Handled client message");

        let server = self.server.lock().unwrap();
        let combat_state = server.world.read_resource::<CombatState>();
        println!("Server state: {:?}", *combat_state);
        println!(
            "Sending new state to client: {:?}",
            server.materialized_state.clone()
        );
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

impl Server {
    pub fn new() -> Self {
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

        let initial_state = get_materialized_state(&mut world);
        let mut server = Server {
            world,
            materialized_state: initial_state,
        };

        println!("CREATED INITIAL GAME WORLD!!!");

        server.game_loop();

        server
    }

    fn game_loop(&mut self) {
        // since some game loop iterations create events that get handled on next iteration,
        // we keep looping until materialized state does not change
        loop {
            // run ECS systems
            let mut drafting_system = DraftingSystem {};
            let mut rolling_system = RollingSystem {};
            let mut action_system = ActionSystem {};
            let mut materialize_system = MaterializeSystem {};
            drafting_system.run_now(&self.world);
            rolling_system.run_now(&self.world);
            action_system.run_now(&self.world);
            materialize_system.run_now(&self.world);
            self.world.maintain();

            // handle events
            {
                let mut event_queue = self.world.write_resource::<EventQueue>();
                if !event_queue.events.is_empty() {
                    println!("current events: {:?}", event_queue.events);
                }
                if !event_queue.new_events.is_empty() {
                    println!("new events: {:?}", event_queue.new_events);
                }
                event_queue.events = (*event_queue.new_events).to_vec();
                event_queue.new_events.clear();
            }

            // check if it's time to end this current game loop
            let old_state: String = serde_json::to_string(&self.materialized_state).unwrap();
            self.materialized_state = get_materialized_state(&mut self.world);
            if serde_json::to_string(&self.materialized_state).unwrap() == old_state {
                println!("Game state stabilized, ending game loop for now.");
                break;
            }
        }
    }
}

impl Connection {
    fn handle_text_message(&mut self, client_id: usize, msg: Message) -> Message {
        let txt = &msg.into_text().unwrap();
        let client_msg: ClientMessage = serde_json::from_str(txt).unwrap();

        println!(
            "Server received text message\ntext: '{}'\nfrom: '{}'\n",
            txt, client_id
        );

        let mut server = self.server.lock().unwrap();

        // dispatch event/etc. based on incoming message
        match client_msg {
            ClientMessage::FinishDrafting(draft_choices) => {
                // User is finished drafting. Send all their draft choices into our Drafting system.
                {
                    let mut event_queue = server.world.write_resource::<EventQueue>();
                    // process draft choices in reverse index order so we don't invalidate the indexes
                    let mut sorted_choices = draft_choices;
                    sorted_choices.sort_by_key(|&b| Reverse(b));
                    for choice in sorted_choices {
                        event_queue.new_events.push(Event::DraftDie(choice));
                    }
                }
                // Let game loop process the new drafted dice before transitioning to next phase.
                server.game_loop();
                // And then transition the combat phase to Rolling
                let mut combat_state = server.world.write_resource::<CombatState>();
                combat_state.current_phase = CombatPhase::Roll
            }
        }

        server.game_loop();

        let server_msg: Message =
            serde_json::to_string(&ServerMessage::NewState(server.materialized_state.clone()))
                .unwrap()
                .into();

        server_msg
    }
}

fn get_materialized_state(world: &mut specs::World) -> ClientGameState {
    let combat_state = world.read_resource::<CombatState>();
    combat_state.materialized_state.clone()
}
