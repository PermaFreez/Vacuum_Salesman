use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Block Snake".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_system(animate_translation)
        .run();
}

#[derive(Component)]
struct AnimateTranslation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Inital text setup
    let font = asset_server.load("fonts/Azonix.otf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    
    // Sprite size
    //
    let sprite_size = Vec2::new(60.0, 60.0);

    // Spawning the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawning the player sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,            
            custom_size: Some(sprite_size),
            ..Default::default()
        },
        ..Default::default()
    });
/*
    commands
        .spawn_bundle(Text2dBundle {
        text: Text::with_section("0", text_style.clone(), text_alignment),
        ..Default::default()
        })
        .insert(AnimateTranslation);
        */
}

