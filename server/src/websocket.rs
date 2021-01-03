use ws::Handshake;
use specs::DispatcherBuilder;
use crate::components::*;
use crate::combatant::*;
use crate::combat_state::*;
use crate::events::*;
use crate::log::*;
use crate::shared::*;
use crate::systems::*;
use specs::World;
use specs::WorldExt;
use ws::{CloseCode, Handler, Message, Request, Response, Result, Sender};


pub struct Server<'a, 'b> {
    pub out: Sender,
    pub dispatcher: specs::Dispatcher<'a, 'b>,
    pub world: specs::World,
    // TODO: when we have multi-player, this would be a HashMap<Player, ClientGameState> or something like that
    pub materialized_state: ClientGameState,
}

impl Handler for Server<'_, '_> {
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
        // send initial state to client
        let combat_state = self.world.read_resource::<CombatState>();
        println!("Server state: {:?}", *combat_state);
        println!("Sending initial state to client: {:?}", self.materialized_state.clone());
        let server_msg: Message = serde_json::to_string(&ServerMessage::NewState(self.materialized_state.clone()))
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

        let combat_state = self.world.read_resource::<CombatState>();
        println!("Server state: {:?}", *combat_state);
        println!("Sending new state to client: {:?}", self.materialized_state.clone());
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

impl Server<'_, '_> {
    pub fn new(out: Sender) -> Self {
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
            .with(RollingSystem, "rolling", &["drafting"])
            .with(ActionSystem, "action", &[])
            .with(MaterializeSystem, "materialize", &["drafting", "rolling", "action"])
            .build();
        dispatcher.setup(&mut world);

        let initial_state = get_materialized_state(&mut world);
        let mut server = Server{
            out,
            dispatcher,
            world,
            materialized_state: initial_state,
        };

        server.game_loop();

        server
    }
    fn handle_text_message(&mut self, client_id: usize, msg: Message) -> Message {
        let txt = &msg.into_text().unwrap();
        let client_msg: ClientMessage =
            serde_json::from_str(txt).unwrap();
    
        println!(
            "Server received text message\ntext: '{}'\nfrom: '{}'\n",
            txt, client_id
        );
        
        // dispatch event/etc. based on incoming message
        match client_msg {
            ClientMessage::FinishDrafting(draft_choices) => {
                // User is finished drafting. Send all their draft choices into our Drafting system.
                {
                    let mut event_queue = self.world.write_resource::<EventQueue>();
                    for choice in draft_choices {
                        event_queue.new_events.push(Event::DraftDie(choice))
                    }
                }
                // Let game loop process the new drafted dice before transitioning to next phase.
                self.game_loop();
                // And then transition the combat phase to Rolling
                let mut combat_state = self.world.write_resource::<CombatState>();
                combat_state.current_phase = CombatPhase::Roll
            }
        }

        self.game_loop();
        
        let server_msg: Message = serde_json::to_string(&ServerMessage::NewState(self.materialized_state.clone()))
            .unwrap()
            .into();

        server_msg
    }
    fn game_loop(&mut self) {
        // since some game loop iterations create events that get handled on next iteration,
        // we keep looping until materialized state does not change
        loop {
            // run ECS systems
            self.dispatcher.dispatch(&self.world);
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

fn get_materialized_state(world: &mut specs::World) -> ClientGameState {
    let combat_state = world.read_resource::<CombatState>();
    return combat_state.materialized_state.clone();
}
