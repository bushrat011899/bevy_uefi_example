#![no_std]

extern crate alloc;

mod buffer;
mod diagnostics;
mod keyboard;
mod pointer;
mod time;

use bevy_ecs::{
    component::Component,
    event::EventReader,
    query::With,
    system::{Commands, Query, ResMut},
};
use buffer::Buffer;
use buffer::GraphicsPlugin;
use diagnostics::DiagnosticPlugin;
use keyboard::KeyEvent;
use keyboard::KeyInputPlugin;
use pointer::PointerPlugin;
use time::TimePlugin;
use uefi::proto::console::{gop::BltPixel, text::ScanCode};

use bevy_app::prelude::*;

pub struct BevyUefiExample;

impl Plugin for BevyUefiExample {
    fn build(&self, app: &mut App) {
        uefi::helpers::init().unwrap();

        app.add_plugins((
            GraphicsPlugin,
            TimePlugin,
            KeyInputPlugin,
            PointerPlugin,
            DiagnosticPlugin,
        ))
        .set_runner(|mut app| loop {
            app.update();
        })
        .add_systems(Startup, (Buffer::clear, setup))
        .add_systems(Update, (move_player, render_points));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        Position::default(),
        Point {
            color: BltPixel::new(0, 128, 0),
        },
    ));
}

fn render_points(mut buffer: ResMut<Buffer>, query: Query<(&Position, &Point)>) {
    for (pos, point) in query.iter() {
        if let Some(pixel) = buffer.pixel(pos.0, pos.1) {
            *pixel = point.color;
        }
    }
}

fn move_player(
    mut key_events: EventReader<KeyEvent>,
    mut query: Query<&mut Position, With<Player>>,
) {
    use uefi::proto::console::text::Key::Special;

    let (mut dy, mut dx): (isize, isize) = (0, 0);

    for event in key_events.read() {
        match event.key {
            Special(ScanCode::UP) => {
                dy -= 1;
            }
            Special(ScanCode::DOWN) => {
                dy += 1;
            }
            Special(ScanCode::LEFT) => {
                dx -= 1;
            }
            Special(ScanCode::RIGHT) => {
                dx += 1;
            }
            _ => {}
        }
    }

    for mut player in query.iter_mut() {
        player.0 = if dx > 0 {
            player.0.saturating_add(dx as usize)
        } else {
            player.0.saturating_sub(-dx as usize)
        };

        player.1 = if dy > 0 {
            player.1.saturating_add(dy as usize)
        } else {
            player.1.saturating_sub(-dy as usize)
        };
    }
}

#[derive(Component, Default)]
struct Position(usize, usize);

#[derive(Component)]
struct Point {
    color: BltPixel,
}

#[derive(Component)]
struct Player;
