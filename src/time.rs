use core::time::Duration;

use bevy_app::{First, Plugin};
use bevy_ecs::system::{Res, Resource};
use bevy_utils::Instant;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(Time {
            start_ticks: timer_tick(),
        })
        .add_systems(First, update_time);
    }
}

#[derive(Resource)]
struct Time {
    start_ticks: u64,
}

fn update_time(time: Res<Time>) {
    let ticks = timer_tick();
    let runtime = Duration::from_nanos(ticks - time.start_ticks);

    // SAFETY: Probably.
    unsafe {
        Instant::update(runtime);
    }
}

fn timer_tick() -> u64 {
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::x86::_rdtsc()
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut ticks: u64;
        core::arch::asm!("mrs {}, cntvct_el0", out(reg) ticks);
        ticks
    }
}
