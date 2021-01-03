use specs::Entity;

pub enum CombatPhase {
    Drafting,
    Roll,
    SelectAction(Vec<(String, CombatAction)>),
    Action(CombatAction),
}

#[derive(Copy, Clone)]
pub enum CombatAction {
    LightAttack(Option<Entity>),
    PrepHeavyAttack,
    HeavyAttack(Option<Entity>),
    Defend,
}

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
