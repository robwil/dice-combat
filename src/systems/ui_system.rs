use crate::combat_state::CombatAction;
use crate::combat_state::CombatPhase;
use crate::combat_state::CombatState;
use crate::components::Defender;
use crate::components::DicePool;
use crate::components::Die;
use crate::components::Health;
use crate::components::HeavyAttacker;
use crate::components::Named;
use crate::events::Event;
use crate::log::CombatLog;
use crate::EventQueue;
use specs::ReadExpect;
use specs::ReadStorage;

use specs::System;
use specs::WriteExpect;

use crate::megaui::widgets::Button;
use crate::megaui::widgets::Group;
use crate::megaui::Layout;
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
            heavy_attackers,
            defenders,
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
                    match &combat_state.current_phase {
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
                        CombatPhase::SelectAction(possible_actions) => {
                            let mut new_phase = None;
                            for (i, (action_name, action)) in possible_actions.iter().enumerate() {
                                if Button::new(action_name)
                                    .position(Vector2::new(5., 10. + 35. * i as f32))
                                    .size(Vector2::new(160., 30.))
                                    .ui(ui)
                                {
                                    new_phase = Some(CombatPhase::Action(*action));
                                }
                            }
                            if let Some(new_phase) = new_phase {
                                combat_state.current_phase = new_phase;
                            }
                        }
                        _ => {}
                    }
                });
                Group::new(hash!(), Vector2::new(176., 380.)).ui(ui, |ui| {
                    // TODO: don't use unwrap() so much here, we can use if let Some instead.
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
                let mut new_phase = None;
                for (n, character) in combat_state.combatants.iter().enumerate() {
                    Group::new(hash!("character_status", n), Vector2::new(350., 50.)).ui(
                        ui,
                        |ui| {
                            // If a combat action has been chosen, the character names shift to buttons used for targeting.
                            if let CombatPhase::Action(combat_action) = &combat_state.current_phase
                            {
                                if let Some(named) = names.get(*character) {
                                    if n == combat_state.current_character {
                                        ui.label(Vector2::new(5., 10.), &named.name);
                                    } else if Button::new(&format!("{}", &named.name))
                                        .position(Vector2::new(5., 10.))
                                        .size(Vector2::new(100., 30.))
                                        .ui(ui)
                                    {
                                        new_phase = match combat_action {
                                            CombatAction::LightAttack(None) => {
                                                Some(CombatAction::LightAttack(Some(*character)))
                                            }
                                            CombatAction::HeavyAttack(None) => {
                                                Some(CombatAction::HeavyAttack(Some(*character)))
                                            }
                                            _ => None,
                                        };
                                    }
                                }
                            } else {
                                if let Some(named) = names.get(*character) {
                                    ui.label(Vector2::new(5., 10.), &named.name);
                                }
                            }

                            // Status display of current health, prepped heavy attack, prepped defense for each combatant.
                            if let Some(health) = healths.get(*character) {
                                ui.label(Vector2::new(105., 10.), &format!("{}", health.hp));
                            }
                            if let Some(heavy_attacker) = heavy_attackers.get(*character) {
                                draw_dice_label(
                                    ui,
                                    &heavy_attacker.prepped_attack,
                                    Vector2::new(205., 10.),
                                );
                            }
                            if let Some(defender) = defenders.get(*character) {
                                draw_dice_label(
                                    ui,
                                    &defender.prepped_defense,
                                    Vector2::new(305., 10.),
                                );
                            }
                        },
                    );
                }
                if let Some(new_phase) = new_phase {
                    combat_state.current_phase = CombatPhase::Action(new_phase);
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
            },
        );
    }
}

fn draw_dice_label(ui: &mut megaui_macroquad::megaui::Ui, dice: &Vec<Die>, starting_pos: Vector2) {
    let context = ui.get_active_window_context();

    let mut total_width = 0.;
    for die in dice {
        if let Some(rolled_value) = die.rolled_value {
            let mut size = context
                .window
                .draw_commands
                .label_size(&format!("{}", rolled_value), None);

            let pos = context.window.cursor.fit(
                size,
                Some(starting_pos + Vector2::new(total_width, 0.))
                    .map_or(Layout::Vertical, Layout::Free),
            ) + Vector2::new(0., context.global_style.margin);

            size.y += context.global_style.margin * 2.;

            if let Some(advance) = context.window.draw_commands.draw_character(
                // TODO: this only supports single digit dice values at the moment. we should use draw_label if we ever want to support multiple digits
                (48 + rolled_value) as u8 as char,
                pos,
                die.color.into(),
            ) {
                total_width += advance;
            }
        }
    }
}
