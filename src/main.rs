use bevy::prelude::*;
use bevy::time::FixedTimestep;

use hello::structs::score_board::*;
use hello::structs::collision_event::*;

mod functions;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(functions::setup::setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(functions::collision::check_for_collisions)
                .with_system(
                    functions::paddle::move_paddle.before(
                        functions::collision::check_for_collisions
                    )
                )
                .with_system(
                    functions::velocity::apply_velocity.before(
                        functions::collision::check_for_collisions
                    )
                )
                .with_system(
                    functions::sound::play_collision_sound.after(
                        functions::collision::check_for_collisions
                    )
                ),
        )
        .add_system(functions::score::update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}
