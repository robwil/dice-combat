use crate::log::CombatLog;
use specs::ReadExpect;
use crate::combat_state::CombatPhase;
use crate::combat_state::CombatState;
use crate::components::Defender;
use crate::components::DicePool;
use crate::components::Health;
use crate::components::HeavyAttacker;
use crate::components::Named;
use crate::events::Event;
use crate::EventQueue;
use specs::ReadStorage;

use specs::System;
use specs::WriteExpect;

use crate::megaui::widgets::Button;
use crate::megaui::widgets::Group;
use crate::megaui::Vector2;
use macroquad::prelude::*;
use megaui_macroquad::draw_window;
use megaui_macroquad::megaui::hash;
use megaui_macroquad::WindowParams;

pub struct UiSystem;

impl<'a> System<'a> for UiSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, HeavyAttacker>,
        ReadStorage<'a, Defender>,
        ReadStorage<'a, DicePool>,
        ReadExpect<'a, CombatLog>,
        WriteExpect<'a, CombatState>,
        WriteExpect<'a, EventQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            healths,
            _heavy_attackers,
            _defenders,
            dice_pools,
            combat_log,
            mut combat_state,
            mut event_queue,
        ) = data;
        let current_entity = combat_state.combatants[combat_state.current_character];

        draw_window(
            hash!(),
            vec2(10., 10.),
            vec2(780., 40.),
            WindowParams {
                titlebar: false,
                close_button: false,
                movable: false,
                ..Default::default()
            },
            |ui| {
                ui.label(
                    Vector2::new(5., 0.),
                    &format!("Current Turn: {}", names.get(current_entity).unwrap().name),
                );
            },
        );

        draw_window(
            hash!(),
            vec2(10., 60.),
            vec2(390., 420.),
            WindowParams {
                label: "Dice Area".to_string(),
                close_button: false,
                movable: false,
                ..Default::default()
            },
            |ui| {
                Group::new(hash!(), Vector2::new(180., 380.)).ui(ui, |ui| {
                    match combat_state.current_phase {
                        CombatPhase::Drafting => {
                            ui.label(
                                Vector2::new(5., 10.),
                                &format!(
                                    "Draft {} dice to roll",
                                    dice_pools.get(current_entity).unwrap().max_draft_amount
                                ),
                            );
                            if Button::new("Finish drafting")
                                .position(Vector2::new(5., 50.))
                                .size(Vector2::new(160., 30.))
                                .ui(ui)
                            {
                                combat_state.current_phase = CombatPhase::Roll;
                            }
                        }
                        CombatPhase::Roll => {
                            if Button::new("Roll")
                                .position(Vector2::new(5., 10.))
                                .size(Vector2::new(50., 30.))
                                .ui(ui)
                            {
                                event_queue.new_events.push(Event::RollDice);
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
                    }
                });
                Group::new(hash!(), Vector2::new(176., 380.)).ui(ui, |ui| {
                    match combat_state.current_phase {
                        CombatPhase::Drafting => {
                            let available_dice = &dice_pools.get(current_entity).unwrap().available;
                            for (n, die) in available_dice.iter().enumerate() {
                                if Button::new(&format!("{}", &die))
                                    .size(Vector2::new(50., 50.))
                                    .ui(ui)
                                {
                                    event_queue.new_events.push(Event::DraftDie(n))
                                }
                            }
                        }
                        CombatPhase::Roll => {
                            let drafted_dice = &dice_pools.get(current_entity).unwrap().drafted;
                            for (n, die) in drafted_dice.iter().enumerate() {
                                Group::new(hash!("dice", n), Vector2::new(50., 50.)).ui(ui, |ui| {
                                    ui.label(Vector2::new(5., 10.), &format!("{}", &die));
                                });
                            }
                        }
                        _ => {
                            let rolled_dice = &dice_pools.get(current_entity).unwrap().rolled;
                            for (n, die) in rolled_dice.iter().enumerate() {
                                Group::new(hash!("dice", n), Vector2::new(50., 50.)).ui(ui, |ui| {
                                    ui.label(Vector2::new(5., 10.), &format!("{}", &die));
                                });
                            }
                        }
                    }
                });
            },
        );
        draw_window(
            hash!(),
            vec2(410., 60.),
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
                            ui.label(
                                Vector2::new(5., 10.),
                                &names.get(*character).unwrap().name.to_string(),
                            );
                            // }
                            ui.label(
                                Vector2::new(105., 10.),
                                &format!("{}", &healths.get(*character).unwrap().hp),
                            );
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
        draw_window(
            hash!(),
            vec2(10., 500.),
            vec2(780., 280.),
            WindowParams {
                label: "Combat Log".to_string(),
                close_button: false,
                movable: false,
                ..Default::default()
            },
            |ui| {
                for (i, log) in combat_log.logs.iter().rev().enumerate() {
                    ui.label(Vector2::new(10., (i * 30) as f32), log);
                }
            }
        );
    }
}
