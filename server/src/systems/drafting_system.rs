use crate::combat_state::CombatPhase;
use crate::combat_state::CombatState;
use crate::components::Defender;
use crate::components::DicePool;
use crate::components::Named;
use crate::events::Event;
use crate::log::CombatLog;
use crate::EventQueue;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::WriteExpect;
use specs::WriteStorage;

use specs::System;

pub struct DraftingSystem;

impl<'a> System<'a> for DraftingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, EventQueue>,
        ReadStorage<'a, Named>,
        WriteStorage<'a, Defender>,
        WriteStorage<'a, DicePool>,
        ReadExpect<'a, CombatState>,
        WriteExpect<'a, CombatLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, names, mut defenders, mut dice_pools, combat_state, mut combat_log) =
            data;
        let current_entity = combat_state.combatants[combat_state.current_character];

        // At start of Drafting phase, clean up leftover state from last turn by moving all rolled dice back to available dice.
        if let CombatPhase::Drafting = combat_state.current_phase {
            if let Some(dice_pool) = dice_pools.get_mut(current_entity) {
                if !dice_pool.rolled.is_empty() {
                    for die in dice_pool.rolled.iter_mut() {
                        die.rolled_value = None;
                    }
                    dice_pool.available.append(&mut dice_pool.rolled);
                }
                if let Some(defender) = defenders.get_mut(current_entity) {
                    if !defender.prepped_defense.is_empty() {
                        for die in defender.prepped_defense.iter_mut() {
                            die.rolled_value = None;
                        }
                        dice_pool.available.append(&mut defender.prepped_defense);
                    }
                }
            }
        }

        for event in event_queue.events.iter() {
            // TODO: this is warning only because we have just one event. in the future we should have more events, so going to leave this as is for now.
            if let Event::DraftDie(n) = event {
                if let Some(dice_pool) = dice_pools.get_mut(current_entity) {
                    if dice_pool.drafted.len() < dice_pool.max_draft_amount {
                        let die = dice_pool.available.remove(*n);
                        dice_pool.drafted.push(die);
                        combat_log.add(format!(
                            "{} drafted {}",
                            names.get(current_entity).unwrap().name,
                            die
                        ));
                    }
                }
            }
        }
    }
}
