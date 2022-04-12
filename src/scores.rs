pub mod score_board {
    use bevy::{
        prelude::*,
    };

    // Score and scoreboard vairables
    pub const SCORE_STEP: i32 = 1;
    pub const SCORE_DIFFERENCE: f32 = 0.01;
    pub const SCORE_FONT_SIZE: f32 = 60.0;
    pub const SCORE_FONT_COLOR: Color = Color::rgb(0.4, 0.5, 0.5);
    pub const SCOREBOARD_Y_OFFSET: f32 = 100.0;
    pub struct ScoreTimer(pub Timer);

    #[derive(Component)]
    pub struct ScoreBoard;

    pub fn handle_scores(time: Res<Time>, mut score_timer: ResMut<ScoreTimer>, mut query: Query<&mut Text, With<ScoreBoard>>) {
        let mut scoreboard = query.single_mut();
        if score_timer.0.tick(time.delta()).just_finished() {
            scoreboard.sections[0].value = (scoreboard.sections[0].value.parse::<i32>()
                                            .expect("Code error - Shouldn't make score into a strig") + SCORE_STEP).to_string();
        }
    }
}