use crate::megaui::Style;
use crate::constants::FONT_SIZE;
use macroquad::prelude::*;
use megaui::Color;
use megaui::FontAtlas;
use megaui_macroquad::set_ui_style;
use megaui_macroquad::{
    draw_megaui,
    megaui::{self},
    set_font_atlas,
};
use specs::DispatcherBuilder;
use specs::{World, WorldExt};

mod constants;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dice Combat".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // setup UI style
    let font_bytes = &include_bytes!("../assets/fonts/Roboto-Bold.ttf")[..];
    let font_atlas =
        FontAtlas::new(font_bytes, FONT_SIZE, FontAtlas::ascii_character_list()).unwrap();
    set_font_atlas(font_atlas);
    set_ui_style(Style {
        title_height: 32.,
        margin: 5.,
        window_background_focused: Color::from_rgb(0, 0, 150),
        focused_title: Color::from_rgb(255, 255, 255),
        focused_text: Color::from_rgb(255, 255, 255),
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

        draw_megaui();

        next_frame().await;
    }
}
