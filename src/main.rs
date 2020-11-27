use crate::combat_state::CombatState;
use crate::combatant::create_combatants;
use crate::components::*;
use crate::constants::FONT_SIZE;
use crate::events::EventQueue;
use crate::megaui::Style;
use crate::systems::DraftingSystem;
use crate::systems::RollingSystem;
use crate::systems::UiSystem;
use macroquad::prelude::*;
use megaui::Color;
use megaui::FontAtlas;
use megaui_macroquad::set_ui_style;
use megaui_macroquad::{
    draw_megaui,
    megaui::{self},
    set_font_atlas,
};
use quad_rand as qrand;
use specs::DispatcherBuilder;
use specs::{World, WorldExt};
use std::time::SystemTime;

mod combat_state;
mod combatant;
mod components;
mod constants;
mod events;
mod systems;

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
    // seed random to current timestamp
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
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
    // let font_atlas =
    //     FontAtlas::new(font_bytes, FONT_SIZE, FontAtlas::ascii_character_list()).unwrap();

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
    // world.insert(UiState {
    //     font_atlas,
    //     dialog_box: None,
    // });

    // Dispatcher setup will register all systems and do other setup
    let mut dispatcher = DispatcherBuilder::new()
        .with(UiSystem, "ui", &[])
        .with(DraftingSystem, "drafting", &[])
        .with(RollingSystem, "rolling", &[])
        .build();
    dispatcher.setup(&mut world);

    loop {
        clear_background(BLACK);

        // TODO: find a better place for this to live
        // if combat_state.current_character >= combat_state.combatants.len() {
        //     combat_state.current_character = 0;
        // }

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

        draw_megaui();

        next_frame().await;
    }
}
