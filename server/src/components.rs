use crate::shared::Die;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Named {
    pub name: String,
}

#[derive(Component)]
pub struct Health {
    pub hp: usize,
}

#[derive(Component, Default)]
pub struct DicePool {
    pub available: Vec<Die>,
    pub max_draft_amount: usize,
    pub drafted: Vec<Die>,
    pub rolled: Vec<Die>,
}

#[derive(Component)]
pub struct LightAttacker;

#[derive(Component, Default)]
pub struct HeavyAttacker {
    pub prepped_attack: Vec<Die>,
}

#[derive(Component, Default)]
pub struct Defender {
    pub prepped_defense: Vec<Die>,
}
