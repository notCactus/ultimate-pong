use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    ecs::{Read, ReadExpect},
    renderer::SpriteRender,
};

use crate::audio::{play_bounce_sound, Sounds};
use crate::pong::{Ball, Paddle, Side, ARENA_HEIGHT, BALL_SPEEDUP};
use std::ops::Deref;

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (mut balls, paddles, transforms, storage, sounds, audio_output, mut sprite_renderer): Self::SystemData,
    ) {
        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent multiple collisions
        // from occurring.
        for (ball, transform, sprite_renderer) in (&mut balls, &transforms, &mut sprite_renderer).join() {
            
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Bounce at the top or the bottom of the arena.
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            // Bounce at the paddles.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {
                        ball.velocity[0] = -ball.velocity[0];

                        ball.consecutive_hits = ball.consecutive_hits + 1;

                        println!("Consecutive hits: {}", ball.consecutive_hits);

                        // Changes the ball's sprite based off consecutive hits
                        if ball.consecutive_hits%4 == 3 {
                            if sprite_renderer.sprite_number < 4 {
                                sprite_renderer.sprite_number = sprite_renderer.sprite_number + 1;
                            }
                        }
                        
                        // Velocity is increased everytime someone hits the ball.
                        ball.velocity[0] = ball.velocity[0]*BALL_SPEEDUP;

                        play_bounce_sound(
                            &*sounds,
                            &storage,
                            audio_output.as_ref().map(|o| o.deref()),
                        );
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
