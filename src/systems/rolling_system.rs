use crate::components::Named;
use specs::ReadStorage;
use crate::log::CombatLog;
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
        ReadStorage<'a, Named>,
        WriteStorage<'a, DicePool>,
        WriteExpect<'a, CombatState>,
        WriteExpect<'a, CombatLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (event_queue, names, mut dice_pools, mut combat_state, mut combat_log) = data;
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
                combat_log.add(format!("{} rolled [{}]",
                    names.get(current_entity).unwrap().name,
                    dice_pool.rolled.iter().map(|die| { die.to_string() }).collect::<Vec<String>>().join(",")
                ));
            }
        }
    }
}
