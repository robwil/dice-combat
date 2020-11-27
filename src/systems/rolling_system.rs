use crate::combat_state::CombatPhase;
use crate::combat_state::CombatState;
use crate::components::DicePool;
use crate::events::Event;
use crate::EventQueue;
use quad_rand as qrand;
use specs::ReadExpect;
use specs::WriteExpect;
use specs::WriteStorage;

use specs::System;

pub struct RollingSystem;

impl<'a> System<'a> for RollingSystem {
    type SystemData = (
        ReadExpect<'a, EventQueue>,
        WriteStorage<'a, DicePool>,
        WriteExpect<'a, CombatState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, mut dice_pools, mut combat_state) = data;
        let current_entity = combat_state.combatants[combat_state.current_character];

        for event in event_queue.events.iter() {
            if let Event::RollDice = event {
                let dice_pool = dice_pools.get_mut(current_entity).unwrap();
                for die in dice_pool.drafted.iter() {
                    let mut rolled_die = *die;
                    rolled_die.rolled_value = Some(qrand::gen_range(1, die.sides + 1));
                    dice_pool.rolled.push(rolled_die);
                }
                dice_pool.drafted.clear();
                combat_state.current_phase = CombatPhase::SelectAction;
            }
        }
    }
}
