use crate::megaui::widgets::Button;
use crate::megaui::Vector2;
use crate::megaui::widgets::Group;
use megaui_macroquad::WindowParams;
use megaui_macroquad::draw_window;
use crate::megaui::Style;
use crate::constants::FONT_SIZE;
use macroquad::prelude::*;
use megaui::Color;
use megaui::FontAtlas;
use megaui_macroquad::set_ui_style;
use megaui_macroquad::{
    draw_megaui,
    megaui::{self, hash},
    set_font_atlas,
};
use specs::DispatcherBuilder;
use specs::{World, WorldExt};
use quad_rand as qrand;
use std::time::{SystemTime};

mod constants;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dice Combat".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

struct Character {
    name: String,
    hp: u32,
    heavy_attack: u32,
    defend: u32,
}

#[macroquad::main(window_conf)]
async fn main() {
    // seed random to current timestamp
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    qrand::srand(time.as_secs());

    // setup UI style
    let font_bytes = &include_bytes!("../assets/fonts/Roboto-Bold.ttf")[..];
    let font_atlas =
        FontAtlas::new(font_bytes, FONT_SIZE, FontAtlas::ascii_character_list()).unwrap();
    set_font_atlas(font_atlas);
    set_ui_style(Style {
        title_height: 32.,
        margin: 5.,
        // this style basically sets up no difference between Focused and Inactive, since we have a multi-window interface
        window_background_focused: Color::from_rgb(0, 0, 150),
        window_background_inactive: Color::from_rgb(0, 0, 150),
        focused_title: Color::from_rgb(255, 255, 255),
        focused_text: Color::from_rgb(255, 255, 255),
        inactive_title: Color::from_rgb(255, 255, 255),
        inactive_text: Color::from_rgb(255, 255, 255),
        group_border_focused: Color::from_rgba(255, 255, 255, 68),
        group_border_inactive: Color::from_rgba(255, 255, 255, 68),
        button_background_focused: Color::from_rgba(104, 104, 104, 235),
        button_background_inactive: Color::from_rgba(104, 104, 104, 235),
        button_background_focused_hovered: Color::from_rgba(170, 170, 170, 235),
        button_background_focused_clicked: Color::from_rgba(187, 187, 187, 255),
        ..Default::default()
    });
    // need to recreate font_atlas that got moved above, so we can use it below
    let font_atlas =
        FontAtlas::new(font_bytes, FONT_SIZE, FontAtlas::ascii_character_list()).unwrap();

    // Setup specs world
    let mut world = World::new();

    // Insert global resources
    // world.insert(EventQueue {
    //     ..Default::default()
    // });
    // world.insert(UiState {
    //     font_atlas,
    //     dialog_box: None,
    // });

    // Dispatcher setup will register all systems and do other setup
    let mut dispatcher = DispatcherBuilder::new()
        // .with(InputSystem, "input", &[])
        // .with(ActionSystem, "action", &[])
        // .with(
        //     RenderingSystem {
        //         ..Default::default()
        //     },
        //     "rendering",
        //     &[],
        // )
        // .with(UiSystem, "ui", &["rendering"])
        .build();
    dispatcher.setup(&mut world);

    let mut dice: Vec<String> = vec![];
    let mut characters: Vec<Character> = vec![
        Character{name: "Player".to_owned(), hp: 100, heavy_attack: 0, defend: 0},
        Character{name: "Goblin 1".to_owned(), hp: 20, heavy_attack: 0, defend: 0},
        Character{name: "Goblin 2".to_owned(), hp: 20, heavy_attack: 0, defend: 0},
    ];
    let mut current_character = 0;

    loop {
        clear_background(BLACK);

        // run ECS systems
        dispatcher.dispatch(&world);
        world.maintain();

        // handle events
        // let mut event_queue = world.write_resource::<EventQueue>();
        // if !event_queue.events.is_empty() {
        //     println!("current events: {:?}", event_queue.events);
        // }
        // if !event_queue.new_events.is_empty() {
        //     println!("new events: {:?}", event_queue.new_events);
        // }
        // event_queue.events = (*event_queue.new_events).to_vec();
        // event_queue.new_events.clear();

        draw_window(
            hash!(),
            vec2(10., 10.),
            vec2(600., 40.),
            WindowParams {
                titlebar: false,
                close_button: false,
                movable: false,
                ..Default::default()
            },
            |ui| {
                ui.label(Vector2::new(5., 0.), &format!("Current Turn: {}", characters[current_character].name));
            }
        );

        draw_window(
            hash!(),
            vec2(10., 60.),
            vec2(380., 420.),
            WindowParams {
                label: "Dice Area".to_string(),
                close_button: false,
                movable: false,
                ..Default::default()
            },
            |ui| {
                Group::new(hash!(), Vector2::new(180., 380.)).ui(ui, |ui| {
                    // TODO: show/hide buttons based on combat phase
                    if Button::new("Roll").position(Vector2::new(5., 10.)).size(Vector2::new(50., 30.)).ui(ui) {
                        dice.clear();
                        // 2d6
                        dice.push(format!("{}", qrand::gen_range(1, 7)));
                        dice.push(format!("{}", qrand::gen_range(1, 7)));
                        dice.push(format!("{}", qrand::gen_range(1, 7)));
                    }
                    if Button::new("Light Attack").position(Vector2::new(5., 45.)).size(Vector2::new(160., 30.)).ui(ui) {
                    }
                    if Button::new("Heavy Attack").position(Vector2::new(5., 80.)).size(Vector2::new(160., 30.)).ui(ui) {
                    }
                    if Button::new("Defend").position(Vector2::new(5., 115.)).size(Vector2::new(160., 30.)).ui(ui) {
                    }
                });
                Group::new(hash!(), Vector2::new(176., 380.)).ui(ui, |ui| {
                    for (n, item) in dice.iter().enumerate() {
                        Group::new(hash!("inventory", n), Vector2::new(50., 50.))
                            .ui(ui, |ui| {
                                ui.label(Vector2::new(5., 10.), &format!("  {}", &item));
                            });
                    }
                });
            },
        );
        draw_window(
            hash!(),
            vec2(400., 60.),
            vec2(380., 420.),
            WindowParams {
                label: "Status".to_string(),
                close_button: false,
                movable: false,
                ..Default::default()
            },
            |ui| {
                Group::new(hash!("character_status_header"), Vector2::new(350., 50.))
                .ui(ui, |ui| {
                    ui.label(Vector2::new(5., 10.), "Name");
                    ui.label(Vector2::new(105., 10.), "HP");
                    ui.label(Vector2::new(205., 10.), "H Atk");
                    ui.label(Vector2::new(305., 10.), "Def");
                });
                for (n, character) in characters.iter().enumerate() {
                    Group::new(hash!("character_status", n), Vector2::new(350., 50.))
                        .ui(ui, |ui| {
                            ui.label(Vector2::new(5., 10.), &format!("{}", &character.name));
                            ui.label(Vector2::new(5., 10.), &format!("{}", &character.name));
                            ui.label(Vector2::new(105., 10.), &format!("{}", &character.hp));
                            ui.label(Vector2::new(205., 10.), &format!("{}", &character.heavy_attack));
                            ui.label(Vector2::new(305., 10.), &format!("{}", &character.defend));
                        });
                }
            },
        );

        draw_megaui();

        next_frame().await;
    }
}
