pub mod walls {
    use bevy::{
        prelude::*,
    };

    // Wall variables
    pub const WALL_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
    pub const WALL_THICKNESS: f32 = crate::target_handling::TARGET_SIZE;
    pub const WALL_HEIGHT: f32 = (crate::constants::MAP_SIZE_Y + WALL_THICKNESS * 1.5) * 2.0;
    pub const WALL_LENGTH: f32 = (crate::constants::MAP_SIZE_X + WALL_THICKNESS * 1.5) * 2.0;

    #[derive(Component)]
    pub struct Collider;

    pub fn create_walls(mut commands: Commands) {
        let mut walls_trans = Vec::new();
        let mut walls_scale = Vec::new();
        
        // Left
        walls_trans.push(Vec3::new(-crate::constants::MAP_SIZE_X - WALL_THICKNESS, 0.0, 0.0));
        walls_scale.push(Vec3::new(WALL_THICKNESS, WALL_HEIGHT, 0.0),);
        
        // Right
        walls_trans.push(Vec3::new(crate::constants::MAP_SIZE_X + WALL_THICKNESS, 0.0, 0.0));
        walls_scale.push(Vec3::new(WALL_THICKNESS, WALL_HEIGHT, 0.0));

        // Up
        walls_trans.push(Vec3::new(0.0, crate::constants::MAP_SIZE_Y + WALL_THICKNESS, 0.0));
        walls_scale.push(Vec3::new(WALL_LENGTH, WALL_THICKNESS, 0.0));

        // Down
        walls_trans.push(Vec3::new(0.0, -crate::constants::MAP_SIZE_Y - WALL_THICKNESS, 0.0));
        walls_scale.push(Vec3::new(WALL_LENGTH, WALL_THICKNESS, 0.0));
        // Spawning wall

        for i in 0..walls_trans.len() {
            commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: walls_trans[i],
                    scale: walls_scale[i],
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider);
        }
    }
}