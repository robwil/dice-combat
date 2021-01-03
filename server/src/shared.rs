// TODO: eventually extract to another crate lib, but for now copy-paste between server and client

use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt;
use serde::{Deserialize, Serialize};

/// Message from the server to the client is very simple. It just gives entirely new state.
#[derive(Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    NewState(ClientGameState),
}

/// Message from the client to the server.
#[derive(Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    FinishDrafting(Vec<usize>),
}

// Representation of game state on client side
// Server side representation is the entire Specs world, most of which doesn't need to be known to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientGameState {
    pub client_phase: ClientPhase,
    pub combatants: Vec<ClientCombatant>,
    pub combat_log: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ClientPhase {
    Waiting,
    DraftDice(Vec<Die>, usize), // server gives us available dice to pick from
    SelectAction(Vec<Die>, Vec<ClientAction>), // server gives us rolled dice and available actions to pick from
    // TODO: target probably needs to be a (String, Uuid) or something, unless we can easily translate Name -> Entity on server side
    SelectTarget(Vec<String>), // server gives us available targets to pick from
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCombatant {
    pub name: String,
    pub hp: usize,
    // TODO: attack and defend dice
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ClientAction {
    LightAttack,
    PrepHeavyAttack,
    HeavyAttack,
    Defend,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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
            Color::Green => "grn",
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
    pub fn colorless(n: usize) -> Self {
        Die {
            color: Color::Colorless,
            sides: n,
            rolled_value: None,
        }
    }
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
    pub fn green(n: usize) -> Self {
        Die {
            color: Color::Green,
            sides: n,
            rolled_value: None,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Color {
    Colorless,
    Blue,
    Red,
    Yellow,
    Green,
}