use seed::{prelude::*, *};
use std::rc::Rc;
use shared::*;
use std::collections::HashSet;

mod shared;

const WS_URL: &str = "ws://127.0.0.1:9000/ws";

// ------ ------
//     Model
// ------ ------

pub struct Model {
    // WebSocket handling
    web_socket: WebSocket,
    web_socket_reconnector: Option<StreamHandle>,
    // UI
    game_state: ClientGameState,
    drafted_dice: HashSet<usize>,
}

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        game_state: ClientGameState{
            // Will be replaced once connected to server for first time
            client_phase: ClientPhase::Waiting,
            combatants: vec![],
            combat_log: vec![],
        },
        drafted_dice: HashSet::new(),
        web_socket: create_websocket(orders),
        web_socket_reconnector: None,
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    // WebSocket handling
    WebSocketOpened,
    TextMessageReceived(shared::ServerMessage),
    CloseWebSocket,
    WebSocketClosed(CloseEvent),
    WebSocketFailed,
    ReconnectWebSocket(usize),
    SendMessage(shared::ClientMessage),
    // UI handling
    DraftDie(usize),
    FinishDrafting,
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // WebSocket handling
        Msg::WebSocketOpened => {
            model.web_socket_reconnector = None;
            log!("WebSocket connection is open now");
        }
        Msg::CloseWebSocket => {
            model.web_socket_reconnector = None;
            model
                .web_socket
                .close(None, Some("user clicked Close button"))
                .unwrap();
        }
        Msg::WebSocketClosed(close_event) => {
            log!("==================");
            log!("WebSocket connection was closed:");
            log!("Clean:", close_event.was_clean());
            log!("Code:", close_event.code());
            log!("Reason:", close_event.reason());
            log!("==================");

            // Chrome doesn't invoke `on_error` when the connection is lost.
            if !close_event.was_clean() && model.web_socket_reconnector.is_none() {
                model.web_socket_reconnector = Some(
                    orders.stream_with_handle(streams::backoff(None, Msg::ReconnectWebSocket)),
                );
            }
        }
        Msg::WebSocketFailed => {
            log!("WebSocket failed");
            if model.web_socket_reconnector.is_none() {
                model.web_socket_reconnector = Some(
                    orders.stream_with_handle(streams::backoff(None, Msg::ReconnectWebSocket)),
                );
            }
        }
        Msg::ReconnectWebSocket(retries) => {
            log!("Reconnect attempt:", retries);
            model.web_socket = create_websocket(orders);
        }
        Msg::SendMessage(msg) => {
            model.web_socket.send_json(&msg).unwrap();
        }
        Msg::TextMessageReceived(message) => {
            log!("got message from server and decoded it successfully");
            match message {
                ServerMessage::NewState(client_state) => {
                    model.game_state = client_state;
                    log!("new game state set");
                }
            }
        }
        // UI handling
        Msg::DraftDie(selected) => {
            if model.drafted_dice.contains(&selected) {
                model.drafted_dice.remove(&selected);
            } else {
                model.drafted_dice.insert(selected);
            }
        },
        Msg::FinishDrafting => {
            model.web_socket.send_json(&ClientMessage::FinishDrafting(
                model.drafted_dice.iter().map(|selected| *selected).collect()
            )).unwrap()
        }
    }
}

fn create_websocket(orders: &impl Orders<Msg>) -> WebSocket {
    let msg_sender = orders.msg_sender();

    WebSocket::builder(WS_URL, orders)
        .on_open(|| Msg::WebSocketOpened)
        .on_message(move |msg| decode_message(msg, msg_sender))
        .on_close(Msg::WebSocketClosed)
        .on_error(|| Msg::WebSocketFailed)
        .build_and_open()
        .unwrap()
}

fn decode_message(message: WebSocketMessage, msg_sender: Rc<dyn Fn(Option<Msg>)>) {
    if message.contains_text() {
        let msg = message
            .json::<shared::ServerMessage>()
            .expect("Failed to decode WebSocket text message");

        msg_sender(Some(Msg::TextMessageReceived(msg)));
    } else {
        log!("Unexpected empty WebSocket message without any text");
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![
            C!["container"],
            nav![
                C!["navbar", "navbar-dark", "bg-dark"],
                div![C!["container-fluid"], span![C!["navbar-brand mb-3 h1"], "Dice Combat"]]
            ],
            if model.web_socket.state() == web_socket::State::Open {
                div![
                    // Top status bar
                    div![
                        C!["row"],
                        div![
                            C!["card text-dark bg-light"],
                            div![
                                C!["card-body"],
                                h5![C!["card-title"], "Status"],
                                p![C!["card-text"], format!("Connected to server: {}", WS_URL)],
                            ]
                        ]
                    ],
                    // Main interaction area
                    div![
                        C!["row"],
                        div![
                            C!["col-sm-6"],
                            div![
                                C!["card text-white bg-dark"],
                                div![
                                    C!["card-body"],
                                    match &model.game_state.client_phase {
                                        ClientPhase::Waiting => p!["Waiting for server..."],
                                        ClientPhase::DraftDice(dice, max) => div![
                                            h5![C!["card-title"], format!("Draft Dice ({} / {})", model.drafted_dice.len(), max)],
                                            div![
                                                C!["d-flex"],
                                                dice.iter().enumerate().map(|(i, die)| {
                                                    let maxed = model.drafted_dice.len() >= *max;
                                                    let selected = model.drafted_dice.contains(&i);
                                                    let event = match (maxed, selected) {
                                                        // if maxed out draft already and this is not already selected, there is no event on click
                                                        (true, false) => None,
                                                        _ => Some(Msg::DraftDie(i)),
                                                    };
                                                    render_die(die, model.drafted_dice.contains(&i), event)
                                                })
                                            ],
                                            button![
                                                C!["btn btn-primary"],
                                                ev(Ev::Click, |_| Msg::FinishDrafting),
                                                "Finish Drafting",
                                            ]
                                        ],
                                        ClientPhase::SelectAction(dice, actions) => div![
                                            h5![C!["card-title"], "Choose Action"],
                                            div![
                                                C!["d-flex"],
                                                dice.iter().map(|die| render_die(die, true, None))
                                            ],
                                            actions.iter().map(|action| {
                                                p![format!("{:?}", action)]
                                            })
                                        ],
                                        ClientPhase::SelectTarget(targets) => div![
                                            h5![C!["card-title"], "Choose Target"],
                                            targets.iter().map(|target| {
                                                p![format!("{:?}", target)]
                                            })
                                        ],
                                    }
                                ] // end actions card-body
                            ] // end actions card
                        ], // end first column
                        div![
                            C!["col-sm-6"],
                            // Combatants information
                            table![
                                C!["table table-dark table-striped"],
                                thead![tr![th!["Name"], th!["HP"]]],
                                tbody![model.game_state.combatants.iter().map(|combatant| {
                                    // combatant.
                                    tr![
                                        td![&combatant.name],
                                        td![combatant.hp],
                                    ]
                                })] // end combatants tbody
                            ], // end combatants table
                        ] // end second column
                    ],
                    button![
                        C!["btn btn-danger"],
                        ev(Ev::Click, |_| Msg::CloseWebSocket),
                        "Close websocket connection"
                    ],
                ]
            } else {
                div![
                    C!["row"],
                    div![
                        C!["card text-dark bg-light"],
                        div![
                            C!["card-body"],
                            h5![C!["card-title"], "Status"],
                            p![C!["card-text"], format!("Disconnected from server: {}", WS_URL)],
                        ]
                    ]
                ]
            }
        ]
    ]
}

fn render_die(die: &Die, selected: bool, msg: Option<Msg>) -> Node<Msg> {
    let num = match die.rolled_value {
        Some(value) => value,
        None => die.sides,
    };
    let mut die_classes = vec!["face", match die.color {
        Color::Colorless => "face-colorless",
        Color::Red => "face-red",
        Color::Green => "face-green",
        Color::Blue => "face-blue",
        Color::Yellow => "face-yellow",
    }];
    if selected {
        die_classes.push("face-selected");
    }
    div![
        C![die_classes],
        ev(Ev::Click, |_| msg),
        (0 .. num).map(|_| span![C!["pip"]])
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
