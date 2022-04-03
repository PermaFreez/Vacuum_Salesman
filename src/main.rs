use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.7, 0.6);
const PLAYER_SIZE: f32 = 47.0;
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.3, 0.4);
const PLAYER_SPEED: f32 = 5.0;
const SCORE_FONT_SIZE: f32 = 60.0;
const SCORE_FONT_COLOR: Color = Color::rgb(0.4, 0.5, 0.5);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Block Snake".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Component)]
struct ScoreBoard;

#[derive(Component)]
struct Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawning the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Scoreboard setup
    let font = asset_server.load("fonts/Azonix.otf");
    let text_style = TextStyle {
        font,
        font_size: SCORE_FONT_SIZE,
        color: SCORE_FONT_COLOR,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands
        .spawn_bundle(Text2dBundle {
        text: Text::with_section("0", text_style.clone(), text_alignment),
        ..Default::default()
        })
        .insert(ScoreBoard);

    // Spawning the player sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: PLAYER_COLOR,
            custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
//            scale: PLAYER_SIZE
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player);
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, 
               mut query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = query.single_mut();

    let mut x_change: f32 = 0.0;
    let mut y_change: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        x_change -= PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        x_change += PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        y_change += PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        y_change -= PLAYER_SPEED;
    }

    player_transform.translation.x += x_change;
    player_transform.translation.y += y_change;
}
    

