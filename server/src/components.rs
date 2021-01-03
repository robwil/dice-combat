use specs::{Component, DenseVecStorage};
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Colorless,
    Blue,
    Red,
    Yellow,
}
impl Default for Color {
    fn default() -> Self {
        Color::Colorless
    }
}
impl Into<megaui_macroquad::megaui::Color> for Color {
    fn into(self) -> megaui_macroquad::megaui::Color {
        match self {
            Color::Red => megaui_macroquad::megaui::Color::from_rgb(255, 0, 0),
            Color::Yellow => megaui_macroquad::megaui::Color::from_rgb(255, 255, 0),
            Color::Blue => megaui_macroquad::megaui::Color::from_rgb(100, 100, 255),
            Color::Colorless => megaui_macroquad::megaui::Color::from_rgb(255, 255, 255),
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Die {
    pub color: Color,
    pub sides: usize,
    pub rolled_value: Option<usize>,
}
impl Display for Die {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let color = match self.color {
            Color::Red => "red",
            Color::Blue => "blu",
            Color::Yellow => "yel",
            Color::Colorless => "non",
        };
        write!(f, "{}", color)?;
        if let Some(rolled) = self.rolled_value {
            write!(f, "{} ({})", rolled, self.sides)?;
        } else {
            write!(f, "{}", self.sides)?;
        }
        Ok(())
    }
}

impl Die {
    pub fn blue(n: usize) -> Self {
        Die {
            color: Color::Blue,
            sides: n,
            rolled_value: None,
        }
    }
    pub fn red(n: usize) -> Self {
        Die {
            color: Color::Red,
            sides: n,
            rolled_value: None,
        }
    }
    pub fn yellow(n: usize) -> Self {
        Die {
            color: Color::Yellow,
            sides: n,
            rolled_value: None,
        }
    }
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
