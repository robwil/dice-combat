use crate::combat_state::CombatAction;
use crate::combat_state::CombatPhase;
use crate::combat_state::CombatState;
use crate::components::Defender;
use crate::components::DicePool;
use crate::components::Health;
use crate::components::HeavyAttacker;
use crate::components::LightAttacker;
use crate::components::Named;
use crate::log::CombatLog;
use specs::ReadStorage;
use specs::WriteExpect;
use specs::WriteStorage;

use specs::System;

pub struct ActionSystem;

impl<'a> System<'a> for ActionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, LightAttacker>,
        WriteStorage<'a, HeavyAttacker>,
        WriteStorage<'a, Defender>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, DicePool>,
        WriteExpect<'a, CombatState>,
        WriteExpect<'a, CombatLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            light_attackers,
            mut heavy_attackers,
            mut defenders,
            mut healths,
            mut dice_pools,
            mut combat_state,
            mut combat_log,
        ) = data;
        let current_entity = combat_state.combatants[combat_state.current_character];

        // SelectAction phase: populate possible actions if they are not already populated
        if let CombatPhase::SelectAction(possible_actions) = &combat_state.current_phase {
            if possible_actions.is_empty() {
                let mut possible_actions = vec![];
                if light_attackers.get(current_entity).is_some() {
                    possible_actions
                        .push(("Light Attack".to_owned(), CombatAction::LightAttack(None)))
                }
                if let Some(heavy_attacker) = heavy_attackers.get(current_entity) {
                    if heavy_attacker.prepped_attack.is_empty() {
                        possible_actions
                            .push(("Prep Heavy Atk".to_owned(), CombatAction::PrepHeavyAttack))
                    } else {
                        possible_actions
                            .push(("Heavy Attack".to_owned(), CombatAction::HeavyAttack(None)))
                    }
                }
                if defenders.get(current_entity).is_some() {
                    possible_actions.push(("Defend".to_owned(), CombatAction::Defend))
                }
                combat_state.current_phase = CombatPhase::SelectAction(possible_actions);
            }
        }

        // Action Phase: resolve the action, but only if a target has been chosen (for targeted actions)
        let mut did_action = false;
        if let CombatPhase::Action(action) = &combat_state.current_phase {
            match action {
                CombatAction::LightAttack(Some(target)) => {
                    if let Some(health) = healths.get_mut(*target) {
                        if let Some(dice_pool) = dice_pools.get(current_entity) {
                            let damage = dice_pool
                                .rolled
                                .iter()
                                .map(|die| die.rolled_value.unwrap())
                                .sum::<usize>();
                            // TODO: this damage dealing needs to consider defense
                            health.hp -= damage;
                            combat_log.add(format!(
                                "{} light attack did {} damage to {}",
                                names.get(current_entity).unwrap().name,
                                damage,
                                names.get(*target).unwrap().name,
                            ));
                        }
                    }
                    did_action = true;
                }
                CombatAction::PrepHeavyAttack => {
                    if let Some(heavy_attack) = heavy_attackers.get_mut(current_entity) {
                        if let Some(dice_pool) = dice_pools.get_mut(current_entity) {
                            heavy_attack.prepped_attack.append(&mut dice_pool.rolled);
                            combat_log.add(format!(
                                "{} prepped for heavy attack",
                                names.get(current_entity).unwrap().name,
                            ));
                        }
                    }
                    did_action = true;
                }
                CombatAction::HeavyAttack(Some(target)) => {
                    if let Some(heavy_attack) = heavy_attackers.get_mut(current_entity) {
                        if let Some(health) = healths.get_mut(*target) {
                            if let Some(dice_pool) = dice_pools.get_mut(current_entity) {
                                // TODO: here would be something more complicated, to calculate bonus damage, but for now we just sum all dice values
                                dice_pool.rolled.append(&mut heavy_attack.prepped_attack);
                                let damage = dice_pool
                                    .rolled
                                    .iter()
                                    .map(|die| die.rolled_value.unwrap())
                                    .sum::<usize>();
                                // TODO: this damage dealing needs to consider defense
                                health.hp -= damage;
                                combat_log.add(format!(
                                    "{} heavy attack did {} damage to {}",
                                    names.get(current_entity).unwrap().name,
                                    damage,
                                    names.get(*target).unwrap().name,
                                ));
                            }
                        }
                    }
                    did_action = true;
                }
                CombatAction::Defend => {
                    if let Some(defender) = defenders.get_mut(current_entity) {
                        if let Some(dice_pool) = dice_pools.get_mut(current_entity) {
                            defender.prepped_defense.append(&mut dice_pool.rolled);
                            combat_log.add(format!(
                                "{} prepped for defense",
                                names.get(current_entity).unwrap().name,
                            ));
                        }
                    }
                    did_action = true;
                }
                _ => {}
            }
        }
        if did_action {
            // move to next character's turn, starting with Drafting phase
            combat_state.current_phase = CombatPhase::Drafting;
            combat_state.current_character += 1;
            if combat_state.current_character >= combat_state.combatants.len() {
                combat_state.current_character = 0;
            }
        }
    }
}
