use amethyst::{
    core::Float,
    core::Time,
    ecs::{Join, Read, ReadExpect, System, WriteStorage},
};

use crate::{
    components::{Monster, WorldPosition},
    data_resources::MonsterDefinitions,
};

pub struct MonsterMovementSystem;

impl<'s> System<'s> for MonsterMovementSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadExpect<'s, MonsterDefinitions>,
        WriteStorage<'s, Monster>,
        WriteStorage<'s, WorldPosition>,
    );

    fn run(
        &mut self,
        (time, monster_definitions, monsters, mut world_positions): Self::SystemData,
    ) {
        for (monster, world_position) in (&monsters, &mut world_positions).join() {
            let monster_definition = monster_definitions.0.get(&monster.name).unwrap();

            let monster_position = &mut **world_position;
            let monster_speed = monster_definition.base_speed;
            let time = time.delta_real_seconds();
            let travel_distance_squared = monster_speed * monster_speed * time * time;

            let displacement = monster.destination - *monster_position;
            *monster_position = if displacement.norm_squared() - travel_distance_squared.into()
                < 0.01.into()
            {
                monster.destination
            } else {
                *monster_position + displacement.normalize() * Float::from_f32(monster_speed * time)
            };
        }
    }
}
