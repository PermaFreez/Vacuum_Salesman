use bevy::{
    prelude::*,
    core::FixedTimestep,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.7, 0.6);
// Player variables
const PLAYER_SIZE: f32 = 47.0;
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.3, 0.4);
const PLAYER_SPEED: f32 = 600.0;
// Target variables
const TARGET_SIZE: f32 = 25.0;
const TARGET_COLOR: Color = Color::rgb(0.8, 0.2, 0.2);
const MAX_TARGETS: u8 = 100;
// Score and scoreboard vairables
const SCORE_STEP: i32 = 1;
const SCORE_DIFFERENCE: f32 = 0.03;
const SCORE_FONT_SIZE: f32 = 60.0;
const SCORE_FONT_COLOR: Color = Color::rgb(0.4, 0.5, 0.5);
const SCOREBOARD_Y_OFFSET: f32 = 50.0;
// Map variables
const MAP_SIZE_X: f32 = 600.0;
const MAP_SIZE_Y: f32 = 300.0;
//const WALL_THICKNES: u8 = 30;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            mode: bevy::window::WindowMode::SizedFullscreen,
            width: 1920.0,
            height: 1080.0,
            title: "Block Snake".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(ScoreTimer(Timer::from_seconds(SCORE_DIFFERENCE, true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_collisions),
        )
        .add_system(move_player)
        .add_system(handle_targets)
        .add_system(handle_scores)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

struct ScoreTimer(Timer);

#[derive(Component)]
struct ScoreBoard;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Collider;

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
        transform: Transform {
            translation: Vec3::new(0.0, MAP_SIZE_Y + SCOREBOARD_Y_OFFSET, 0.0),
            ..Default::default()
        },
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
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player)
    .insert(Collider);
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, 
               mut query: Query<&mut Transform, With<Player>>) {
    let mut player_transform = query.single_mut();

    let mut x_change: f32 = 0.0;
    let mut y_change: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)  {
        x_change -= PLAYER_SPEED * TIME_STEP;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)  {
        x_change += PLAYER_SPEED * TIME_STEP;
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        y_change += PLAYER_SPEED * TIME_STEP;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        y_change -= PLAYER_SPEED * TIME_STEP;
    }

    player_transform.translation.x += x_change.floor();
    player_transform.translation.y += y_change.floor();
}
    
fn handle_targets(mut commands: Commands, query: Query<&mut Transform, With<Target>>) {
    let mut remaining_targets: u8 = 0;
    
    for _target in query.iter() {
        remaining_targets += 1
    }

    if remaining_targets == 0 {
        for _i in 0..MAX_TARGETS {
            let x: f32 = rand::thread_rng().gen_range(-MAP_SIZE_X,MAP_SIZE_X + 1.0);
            let y: f32 = rand::thread_rng().gen_range(-MAP_SIZE_Y,MAP_SIZE_Y + 1.0);
            
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: TARGET_COLOR,
                    custom_size: Some(Vec2::new(TARGET_SIZE, TARGET_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x.floor(), y.floor(), 0.0),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(Target)
            .insert(Collider);
        }
    }
}

fn handle_scores(time: Res<Time>, mut score_timer: ResMut<ScoreTimer>, mut query: Query<&mut Text, With<ScoreBoard>>) {
    let mut scoreboard = query.single_mut();
    if score_timer.0.tick(time.delta()).just_finished() {
        scoreboard.sections[0].value = (scoreboard.sections[0].value.parse::<i32>()
                                        .expect("Code error - Shouldn't make score into a strig") + SCORE_STEP).to_string();
    }
}

fn check_collisions(mut commands: Commands,
                    mut player: Query<&Transform, With<Player>>,
                    targets: Query<(Entity, &Transform), With<Target>>) {    
    let player_transform = player.single_mut();
    for target in targets.iter() {
        if collide(
            target.1.translation,
            Vec2::new(TARGET_SIZE, TARGET_SIZE),
            player_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        ).is_some() {
            commands.entity(target.0).despawn();
        }
    }
}
