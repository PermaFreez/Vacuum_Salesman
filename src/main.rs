use bevy::{
    prelude::*,
    core::FixedTimestep,
    window::WindowMode
};

pub mod targets;
use crate::targets::target_handling;

pub mod player;
use crate::player::player_handling;

pub mod misc;
use crate::misc::qol;

pub mod scores;
use crate::scores::score_board;

pub mod solids;
#[allow(unused_imports)]
use crate::solids::walls;

pub mod constants {
    use bevy::prelude::*;

    pub const TIME_STEP: f32 = 1.0 / 60.0;
    pub const HIGH_TIME_STEP: f32 = 1.0 / 240.0;
    pub const WINDOW_NAME: &str = "Vacuum salesman";

    pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.7, 0.6);

    // Map variables
    pub const MAP_SIZE_X: f32 = 600.0;
    pub const MAP_SIZE_Y: f32 = 300.0;
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            mode: WindowMode::Windowed,
            //width: 1920.0,
            //height: 1080.0,
            resizable: true,
            title: constants::WINDOW_NAME.to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(constants::BACKGROUND_COLOR))
        .insert_resource(score_board::ScoreTimer(Timer::from_seconds(score_board::SCORE_DIFFERENCE, true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(solids::walls::create_walls)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(constants::TIME_STEP as f64))
                .with_system(target_handling::move_targets),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(constants::HIGH_TIME_STEP as f64))
                .with_system(player_handling::move_player)
                .with_system(target_handling::check_eating)
                .with_system(target_handling::change_direction),
        )
        .add_system(target_handling::create_targets)
        .add_system(score_board::handle_scores)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(qol::exit_on_q)
        .add_system(qol::set_fullscreen)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawning the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Scoreboard setup
    let font = asset_server.load("fonts/Azonix.otf");
    let text_style = TextStyle {
        font,
        font_size: score_board::SCORE_FONT_SIZE,
        color: score_board::SCORE_FONT_COLOR,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands
        .spawn_bundle(Text2dBundle {
        text: Text::with_section("0", text_style.clone(), text_alignment),
        transform: Transform {
            translation: Vec3::new(0.0, constants::MAP_SIZE_Y + score_board::SCOREBOARD_Y_OFFSET, 0.0),
            ..Default::default()
        },
        ..Default::default()
        })
        .insert(score_board::ScoreBoard);
    
    // Spawning the player sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: player_handling::PLAYER_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(player_handling::PLAYER_SIZE, player_handling::PLAYER_SIZE, 0.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(player_handling::Player);
}