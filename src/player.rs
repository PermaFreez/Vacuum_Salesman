pub mod player_handling {
    use bevy::{
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
    };

    // Player variables
    pub const PLAYER_SIZE: f32 = 47.0;
    pub const PLAYER_COLOR: Color = Color::rgb(0.0, 0.3, 0.4);
    pub const PLAYER_SPEED: f32 = 600.0;

    #[derive(Component)]
    pub struct Player;

    pub fn move_player(keyboard_input: Res<Input<KeyCode>>, 
        mut queries: QuerySet<(
            QueryState<&mut Transform, With<Player>>,
            QueryState<&Transform, With<crate::solids::walls::Collider>>)>) {
    
    let mut x_change: f32 = 0.0;
    let mut y_change: f32 = 0.0;
    
    let mut colliders: Vec<Transform> = Vec::new();
    
    for collider in queries.q1().iter() {
    colliders.push(collider.clone());
    }
    
    let mut left = true;
    let mut right = true;
    let mut up = true;
    let mut down = true;
    
    for collider in colliders {
    let collision = collide(
    collider.translation,
    collider.scale.truncate(),
    queries.q0().single().translation,
    queries.q0().single().scale.truncate(),
    );
    
    match collision {
    Some(Collision::Left) => {left = false},
    Some(Collision::Right) => {right = false},
    Some(Collision::Top) => {up = false},
    Some(Collision::Bottom) => {down = false},
    None => {},
    }
    }
    
    if (keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)) && left {
    x_change -= PLAYER_SPEED * crate::constants::HIGH_TIME_STEP;
    }
    if (keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)) && right {
    x_change += PLAYER_SPEED * crate::constants::HIGH_TIME_STEP;
    }
    if (keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W)) && up {
    y_change += PLAYER_SPEED * crate::constants::HIGH_TIME_STEP;
    }
    if (keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S)) && down {
    y_change -= PLAYER_SPEED * crate::constants::HIGH_TIME_STEP;
    }
    
    queries.q0().single_mut().translation.x += x_change;
    queries.q0().single_mut().translation.y += y_change;
    }
}