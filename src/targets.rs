pub mod target_handling {
    use bevy::{
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
    };

    use rand::Rng;

    // Target variables
    pub const TARGET_SIZE: f32 = 25.0;
    pub const TARGET_COLOR: Color = Color::rgb(0.8, 0.2, 0.2);
    pub const TARGET_SPEED: f32 = 200.0;
    pub const MAX_TARGETS: u8 = 100;

    #[derive(Component)]
    pub struct Target;

    #[derive(Component)]
    pub struct MoveDirection(Option<Direction>);

    pub fn create_targets(mut commands: Commands, query: Query<&mut Transform, With<Target>>) {
        let mut remaining_targets: u8 = 0;
        
        for _target in query.iter() {
            remaining_targets += 1
        }
    
        if remaining_targets == 0 {
            for _i in 0..MAX_TARGETS {
                let x: f32 = rand::thread_rng().gen_range(-crate::constants::MAP_SIZE_X + TARGET_SIZE * 0.5,crate::constants::MAP_SIZE_X - TARGET_SIZE * 0.5);
                let y: f32 = rand::thread_rng().gen_range(-crate::constants::MAP_SIZE_Y + TARGET_SIZE * 0.5,crate::constants::MAP_SIZE_Y - TARGET_SIZE * 0.5);
                let direction: u8 = rand::thread_rng().gen_range(0, 4);
                let mut direct_enum: Option<Direction> = None;
    
                match direction {
                    0 => direct_enum = Some(Direction::Lu),
                    1 => direct_enum = Some(Direction::Ru),
                    2 => direct_enum = Some(Direction::Ld),
                    3 => direct_enum = Some(Direction::Rd),
                    _ => (),
                }
                      
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: TARGET_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(x.floor(), y.floor(), 0.0),
                        scale: Vec3::new(TARGET_SIZE, TARGET_SIZE, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Target)
                .insert(MoveDirection(direct_enum))
                .insert(crate::solids::walls::Collider);
            }
        }
    }

    pub fn move_targets(mut query: Query<(&mut Transform, &MoveDirection), With<Target>>) {
        for mut target in query.iter_mut() {
            match target.1.0 {
                Some(Direction::Lu) => {target.0.translation.x -= TARGET_SPEED * crate::constants::TIME_STEP; target.0.translation.y += TARGET_SPEED * crate::constants::TIME_STEP},
                Some(Direction::Ru) => {target.0.translation.x += TARGET_SPEED * crate::constants::TIME_STEP; target.0.translation.y += TARGET_SPEED * crate::constants::TIME_STEP},
                Some(Direction::Ld) => {target.0.translation.x -= TARGET_SPEED * crate::constants::TIME_STEP; target.0.translation.y -= TARGET_SPEED * crate::constants::TIME_STEP},
                Some(Direction::Rd) => {target.0.translation.x += TARGET_SPEED * crate::constants::TIME_STEP; target.0.translation.y -= TARGET_SPEED * crate::constants::TIME_STEP},
                None => (),
            }
        }
    }
    
    pub fn change_direction(mut queries: QuerySet<(
                            QueryState<(&mut Transform, &mut MoveDirection), With<Target>>,
                            QueryState<&Transform, With<crate::solids::walls::Collider>>)>) { 
        
    
        let mut colliders: Vec<Transform> = Vec::new();
        
        for collider in queries.q1().iter() {
            colliders.push(collider.clone());
        }
        
        for collider in colliders {
            for mut target in queries.q0().iter_mut() {
                let collision = collide(
                    collider.translation,
                    collider.scale.truncate(),
                    target.0.translation,
                    target.0.scale.truncate(),
                );
                
                match collision {
                    Some(Collision::Left) => match target.1.0 {
                            Some(Direction::Lu) => target.1.0 = Some(Direction::Ru),
                            Some(Direction::Ld) => target.1.0 = Some(Direction::Rd),
                            _ => ()
                        },
                    Some(Collision::Right) => match target.1.0 {
                            Some(Direction::Ru) => target.1.0 = Some(Direction::Lu),
                            Some(Direction::Rd) => target.1.0 = Some(Direction::Ld),
                            _ => ()
                        },
                    Some(Collision::Top) => match target.1.0 {
                            Some(Direction::Lu) => target.1.0 = Some(Direction::Ld),
                            Some(Direction::Ru) => target.1.0 = Some(Direction::Rd),
                            _ => ()
                        },
                    Some(Collision::Bottom) => match target.1.0 {
                            Some(Direction::Ld) => target.1.0 = Some(Direction::Lu),
                            Some(Direction::Rd) => target.1.0 = Some(Direction::Ru),
                            _ => ()
                        },
                    None => {},
                }
            }
        }
    }

    pub fn check_eating(mut commands: Commands,
                    mut player: Query<&Transform, With<crate::player::player_handling::Player>>,
                    targets: Query<(Entity, &Transform), With<Target>>) {
        //let player_transform = player.single_mut();
        for player_transform in player.iter_mut() {
            for target in targets.iter() {
                if collide(
                    target.1.translation,
                    target.1.scale.truncate(),
                    player_transform.translation,
                    player_transform.scale.truncate(),
                ).is_some() {
                    commands.entity(target.0).despawn();
                }
            }
        }
    }
    pub enum Direction {
        Lu,
        Ru,
        Ld,
        Rd,
    }
}