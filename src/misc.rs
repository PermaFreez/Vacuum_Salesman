pub mod qol {
    use bevy::{
        prelude::*,
        ecs::prelude::EventWriter,
        app::AppExit,
        window::WindowMode,
    };

    pub fn set_fullscreen(keyboard_input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
        let window = windows.get_primary_mut().unwrap();
        if keyboard_input.just_pressed(KeyCode::F11) {
            match window.mode() {
                WindowMode::Fullscreen => window.set_mode(WindowMode::Windowed),
                WindowMode::Windowed => window.set_mode(WindowMode::Fullscreen),
                _ => ()
            }
        }
    }
    
    pub fn exit_on_q(keyboard_input: Res<Input<KeyCode>>,
                 mut app_exit_events: EventWriter<AppExit>) {
        if keyboard_input.just_pressed(KeyCode::Q) {
            app_exit_events.send(AppExit);
        }
    }
}