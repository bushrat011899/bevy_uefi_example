use core::time::Duration;

use bevy_app::{Last, Plugin};
use bevy_ecs::system::{ResMut, Resource};
use bevy_utils::Instant;

pub struct DiagnosticPlugin;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(Diagnostics {
            frames: 0,
            last_diagnostic: Instant::now(),
        })
        .add_systems(Last, |mut diagnostics: ResMut<Diagnostics>| {
            diagnostics.frames += 1;
            let diagnostic_period = diagnostics.last_diagnostic.elapsed();

            if diagnostic_period > Duration::from_secs(1) {
                let frametime = diagnostic_period.as_secs_f32() / (diagnostics.frames as f32);
                let fps = 1. / frametime;

                log::info!("FPS: {fps}; Frame Time: {}ms", frametime * 1_000.);

                diagnostics.frames = 0;
                diagnostics.last_diagnostic = Instant::now();
            }
        });
    }
}

#[derive(Resource)]
struct Diagnostics {
    frames: usize,
    last_diagnostic: Instant,
}
