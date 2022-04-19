pub mod menu {
    pub use bevy::prelude::*;

    pub const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.5, 0.5);
    pub const CLICKED_BUTTON: Color = Color::rgb(0.0, 0.3, 0.4);
    pub const TEXT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

    pub fn draw_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn_bundle(UiCameraBundle::default());
        commands.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "stuff",
                    TextStyle {
                        font: asset_server.load("fonts/Azonix.otf"),
                        font_size: 40.0,
                        color: TEXT_COLOR.into(),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
    }

    pub fn buttons(mut interaction_query: Query<
        (&Interaction, &mut UiColor), With<Button>>) {
        for (interaction, mut color) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Clicked => *color = CLICKED_BUTTON.into(),
                Interaction::Hovered => *color = HOVERED_BUTTON.into(),
                Interaction::None => *color = NORMAL_BUTTON.into(),
            }
        }
    }
}