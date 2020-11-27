use crate::components::Named;
use crate::components::Health;
use crate::components::HeavyAttacker;
use crate::components::Defender;
use specs::ReadStorage;
use crate::combat_state::CombatState;
use crate::combat_state::CombatPhase;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::KeyCode;

use specs::System;
use specs::WriteExpect;

use crate::megaui::widgets::Button;
use crate::megaui::widgets::Group;
use crate::megaui::Style;
use crate::megaui::Vector2;
use macroquad::prelude::*;
use megaui::Color;
use megaui::FontAtlas;
use megaui_macroquad::draw_window;
use megaui_macroquad::set_ui_style;
use megaui_macroquad::WindowParams;
use megaui_macroquad::{
    draw_megaui,
    megaui::{self, hash},
    set_font_atlas,
};

pub struct UiSystem;

impl<'a> System<'a> for UiSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, HeavyAttacker>,
        ReadStorage<'a, Defender>,
        WriteExpect<'a, CombatState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (names, healths, heavy_attackers, defenders, mut combat_state,) = data;

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
                ui.label(
                    Vector2::new(5., 0.),
                    &format!("Current Turn: {}", names.get(combat_state.combatants[combat_state.current_character]).unwrap().name),
                );
            },
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
                Group::new(hash!(), Vector2::new(180., 380.)).ui(ui, |ui| match combat_state.current_phase {
                    CombatPhase::Roll => {
                        if Button::new("Roll")
                            .position(Vector2::new(5., 10.))
                            .size(Vector2::new(50., 30.))
                            .ui(ui)
                        {
                            // dice.clear();
                            // for die in characters[current_character].dice.iter() {
                            //     dice.push(qrand::gen_range(1, die + 1));
                            // }
                            // current_phase = CombatPhase::SelectAction;
                        }
                    }
                    CombatPhase::SelectAction => {
                        if Button::new("Light Attack")
                            .position(Vector2::new(5., 10.))
                            .size(Vector2::new(160., 30.))
                            .ui(ui)
                        {
                            // current_phase =
                            //     CombatPhase::Action(CombatAction::LightAttack(DiceRoll {
                            //         dice: dice.clone(),
                            //     }));
                        }
                        if Button::new("Heavy Attack")
                            .position(Vector2::new(5., 45.))
                            .size(Vector2::new(160., 30.))
                            .ui(ui)
                        {
                            // characters[current_character].heavy_attack +=
                            //     dice.iter().sum::<usize>();
                            // current_phase = CombatPhase::Roll;
                            // dice = vec![];
                            // current_character += 1;
                        }
                        if Button::new("Defend")
                            .position(Vector2::new(5., 80.))
                            .size(Vector2::new(160., 30.))
                            .ui(ui)
                        {
                            // characters[current_character].defend += dice.iter().sum::<usize>();
                            // current_phase = CombatPhase::Roll;
                            // dice = vec![];
                            // current_character += 1;
                        }
                    }
                    _ => {}
                });
                Group::new(hash!(), Vector2::new(176., 380.)).ui(ui, |ui| {
                    // TODO: how to get dice
                    // for (n, item) in dice.iter().enumerate() {
                    //     Group::new(hash!("dice", n), Vector2::new(50., 50.)).ui(ui, |ui| {
                    //         ui.label(Vector2::new(5., 10.), &format!("  {}", &item));
                    //     });
                    // }
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
                Group::new(hash!("character_status_header"), Vector2::new(350., 50.)).ui(
                    ui,
                    |ui| {
                        ui.label(Vector2::new(5., 10.), "Name");
                        ui.label(Vector2::new(105., 10.), "HP");
                        ui.label(Vector2::new(205., 10.), "H Atk");
                        ui.label(Vector2::new(305., 10.), "Def");
                    },
                );
                for (n, character) in combat_state.combatants.iter_mut().enumerate() {
                    Group::new(hash!("character_status", n), Vector2::new(350., 50.)).ui(
                        ui,
                        |ui| {
                            // if let CombatPhase::Action(combat_action) = &current_phase {
                            //     if n == current_character {
                            //         ui.label(
                            //             Vector2::new(5., 10.),
                            //             &format!("{}", &character.name),
                            //         );
                            //     } else if Button::new(&format!("{}", &character.name))
                            //         .position(Vector2::new(5., 10.))
                            //         .size(Vector2::new(100., 30.))
                            //         .ui(ui)
                            //     {
                            //         if let CombatAction::LightAttack(dice_roll) = combat_action {
                            //             character.hp -= dice_roll.dice.iter().sum::<usize>();
                            //             current_phase = CombatPhase::Roll;
                            //             dice = vec![];
                            //             current_character += 1;
                            //         }
                            //     }
                            // } else {
                            ui.label(Vector2::new(5., 10.), &format!("{}", &names.get(*character).unwrap().name));
                            // }
                            ui.label(Vector2::new(105., 10.), &format!("{}", &healths.get(*character).unwrap().hp));
                            // TODO: deal with these optionals
                            // ui.label(
                            //     Vector2::new(205., 10.),
                            //     &format!("{}", &heavy_attackers.get(*character).unwrap_or_default().heavy_attack),
                            // );
                            // ui.label(Vector2::new(305., 10.), &format!("{}", &character.defend));
                        },
                    );
                }
            },
        );
    }
}
