use specs::Entity;

pub enum CombatPhase {
    Drafting,
    Roll,
    SelectAction,
    Action(/*CombatAction*/),
}

// enum CombatAction {
//     LightAttack(DiceRoll),
//     HeavyAttack(DiceRoll),
//     Defend(DiceRoll),
// }

pub struct CombatState {
    pub current_character: usize,
    pub combatants: Vec<Entity>,
    pub current_phase: CombatPhase,
}

impl CombatState {
    pub fn new(combatants: Vec<Entity>) -> Self {
        CombatState {
            current_character: 0,
            combatants,
            current_phase: CombatPhase::Drafting,
        }
    }
}
