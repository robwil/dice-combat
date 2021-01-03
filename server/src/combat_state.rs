use crate::shared::*;
use specs::Entity;

#[derive(Debug, Clone)]
pub enum CombatPhase {
    Drafting,
    Roll,
    SelectAction(Vec<(String, CombatAction)>),
    Action(CombatAction),
}

#[derive(Copy, Clone, Debug)]
pub enum CombatAction {
    LightAttack(Option<Entity>),
    PrepHeavyAttack,
    HeavyAttack(Option<Entity>),
    Defend,
}

#[derive(Debug, Clone)]
pub struct CombatState {
    pub current_character: usize,
    pub combatants: Vec<Entity>,
    pub current_phase: CombatPhase,
    // Client-side version of the current state and world, materialized by MaterializeSystem
    pub materialized_state: ClientGameState,
}

impl CombatState {
    pub fn new(combatants: Vec<Entity>) -> Self {
        CombatState {
            current_character: 0,
            combatants,
            current_phase: CombatPhase::Drafting,
            materialized_state: ClientGameState {
                // These values don't matter, they will get immediately replaced by MaterializeSystem
                client_phase: ClientPhase::Waiting,
                combatants: vec![],
                combat_log: vec![],
            },
        }
    }
}
