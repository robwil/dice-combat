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
        WriteStorage<'a, DicePool>,
        ReadExpect<'a, CombatState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, mut dice_pools, combat_state) = data;
        let current_entity = combat_state.combatants[combat_state.current_character];

        for event in event_queue.events.iter() {
            if let Event::DraftDie(n) = event {
                let dice_pool = dice_pools.get_mut(current_entity).unwrap();
                if dice_pool.drafted.len() < dice_pool.max_draft_amount {
                    let die = dice_pool.available.remove(*n);
                    dice_pool.drafted.push(die);
                }
            }
        }
    }
}
