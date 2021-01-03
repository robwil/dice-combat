use crate::shared::ClientGameState;
use crate::shared::ClientCombatant;
use crate::shared::ClientPhase;
use crate::combat_state::CombatPhase;
use crate::combat_state::CombatState;
use crate::components::DicePool;
use crate::components::Health;
use crate::components::Named;
use crate::log::CombatLog;
use specs::ReadExpect;
use specs::ReadStorage;

use specs::System;
use specs::WriteExpect;

pub struct MaterializeSystem;

impl<'a> System<'a> for MaterializeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, DicePool>,
        ReadExpect<'a, CombatLog>,
        WriteExpect<'a, CombatState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            healths,
            dice_pools,
            combat_log,
            mut combat_state,
        ) = data;

        let current_entity = combat_state.combatants[combat_state.current_character];

        let client_phase = match &combat_state.current_phase {
            CombatPhase::Drafting => {
                let dice_pool = dice_pools.get(current_entity).unwrap();
                ClientPhase::DraftDice(dice_pool.available.clone(), dice_pool.max_draft_amount)
            }
            // TODO: implement other phases
            _ => ClientPhase::Waiting,
        };

        combat_state.materialized_state = ClientGameState{
            client_phase,
            combatants: combat_state.combatants.iter().flat_map(|combatant| {
                if let (Some(named), Some(health)) = (names.get(*combatant), healths.get(*combatant)) {
                    Some(ClientCombatant { name: named.name.clone(), hp: health.hp })
                } else {
                    None
                }
            }).collect(),
            combat_log: combat_log.logs.clone(),
        };
    }
}