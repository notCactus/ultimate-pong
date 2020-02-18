use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, Write, WriteStorage},
    ecs::Read,
    ui::UiText,
    renderer::SpriteRender,
};

use crate::audio::{play_score_sound, Sounds};
use std::ops::Deref;

use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH, BALL_VELOCITY_X};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text,
        storage,
        sounds,
        audio_output,
        mut sprite_renderer,
    ): Self::SystemData,
    ) {
        for (ball, transform, sprite_renderer) in (&mut balls, &mut locals, &mut sprite_renderer).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                
                // The consecutive hits counter is reset.
                ball.consecutive_hits = 0;
                
                // The ball sprite gets reset
                sprite_renderer.sprite_number = 1;

                // The velocity is reset.
                ball.velocity[0] = BALL_VELOCITY_X;

                // Right player scored on the left side.
                // Score is topped at 999 in order to avoid overlap.
                scores.score_right = (scores.score_right + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // The consecutive hits counter is reset.
                ball.consecutive_hits = 0;

                // The ball sprite gets reset
                sprite_renderer.sprite_number = 1;

                // The velocity is reset.
                ball.velocity[0] = BALL_VELOCITY_X;

                // Left player scored on the right side.
                // Score is topped at 999 in order to avoid overlap.
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));

                // Print the scoreboard.
                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
            }
        }
    }
}
