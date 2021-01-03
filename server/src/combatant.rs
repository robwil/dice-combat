use crate::shared::Die;
use crate::components::*;
use specs::{Builder, Entity, World, WorldExt};

pub fn create_combatants(world: &mut World) -> Vec<Entity> {
    let player = world
        .create_entity()
        .with(Named {
            name: "Player".to_owned(),
        })
        .with(Health { hp: 100 })
        .with(LightAttacker)
        .with(HeavyAttacker {
            ..Default::default()
        })
        .with(Defender {
            ..Default::default()
        })
        .with(DicePool {
            available: vec![Die::blue(6), Die::red(6), Die::yellow(6), Die::red(6)],
            max_draft_amount: 2,
            ..Default::default()
        })
        .build();

    let red_goblin = world
        .create_entity()
        .with(Named {
            name: "Red Goblin".to_owned(),
        })
        .with(Health { hp: 50 })
        .with(LightAttacker)
        .with(Defender {
            ..Default::default()
        })
        .with(DicePool {
            available: vec![Die::red(4), Die::red(4)],
            max_draft_amount: 2,
            ..Default::default()
        })
        .build();

    let blue_goblin = world
        .create_entity()
        .with(Named {
            name: "Blue Goblin".to_owned(),
        })
        .with(Health { hp: 50 })
        .with(LightAttacker)
        .with(Defender {
            ..Default::default()
        })
        .with(DicePool {
            available: vec![Die::blue(4), Die::blue(4)],
            max_draft_amount: 2,
            ..Default::default()
        })
        .build();

    vec![player, red_goblin, blue_goblin]
}
