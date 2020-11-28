use crate::components::Named;
use specs::ReadStorage;
use crate::log::CombatLog;
use specs::WriteExpect;
use crate::combat_state::CombatState;
use crate::components::DicePool;
use crate::events::Event;
use crate::EventQueue;
use specs::ReadExpect;
use specs::WriteStorage;

use specs::System;

pub struct DraftingSystem;

impl<'a> System<'a> for DraftingSystem {
    type SystemData = (
        ReadExpect<'a, EventQueue>,
        ReadStorage<'a, Named>,
        WriteStorage<'a, DicePool>,
        ReadExpect<'a, CombatState>,
        WriteExpect<'a, CombatLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, names, mut dice_pools, combat_state, mut combat_log) = data;
        let current_entity = combat_state.combatants[combat_state.current_character];

        for event in event_queue.events.iter() {
            if let Event::DraftDie(n) = event {
                let dice_pool = dice_pools.get_mut(current_entity).unwrap();
                if dice_pool.drafted.len() < dice_pool.max_draft_amount {
                    let die = dice_pool.available.remove(*n);
                    dice_pool.drafted.push(die);
                    combat_log.add(format!("{} drafted {}", names.get(current_entity).unwrap().name, die));
                }
            }
        }
    }
}
