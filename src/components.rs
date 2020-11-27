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

pub enum Color {
    Blue,
    Red,
    Yellow,
}

pub struct Die {
    pub color: Color,
    pub number: usize,
}

impl Die {
    pub fn blue(n: usize) -> Self {
        Die {
            color: Color::Blue,
            number: n,
        }
    }
    pub fn red(n: usize) -> Self {
        Die {
            color: Color::Red,
            number: n,
        }
    }
    pub fn yellow(n: usize) -> Self {
        Die {
            color: Color::Yellow,
            number: n,
        }
    }
}

#[derive(Component, Default)]
pub struct DicePool {
    pub available: Vec<Die>,
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
