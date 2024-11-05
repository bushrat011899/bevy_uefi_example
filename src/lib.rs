#![no_std]

extern crate alloc;

mod diagnostics;
mod graphics;
mod keyboard;
mod pointer;
mod time;

use bevy_app::prelude::*;
use bevy_ecs::{
    component::Component,
    event::EventReader,
    query::With,
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, ResMut},
};
use diagnostics::DiagnosticPlugin;
use graphics::{Buffer, GraphicsPlugin, Sprite};
use keyboard::{KeyEvent, KeyInputPlugin};
use pointer::PointerPlugin;
use time::TimePlugin;
use uefi::proto::console::text::ScanCode;

pub struct BevyUefiExample;

impl Plugin for BevyUefiExample {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GraphicsPlugin,
            TimePlugin,
            KeyInputPlugin,
            PointerPlugin,
            DiagnosticPlugin,
        ))
        .set_runner(|mut app| loop {
            app.update();

            if let Some(exit) = app.should_exit() {
                return exit;
            }
        })
        .add_systems(Startup, (Buffer::clear, setup))
        .add_systems(Update, (move_player, bounce_sprites, apply_velocity, Buffer::clear, render_sprites).chain());
    }
}

fn setup(mut commands: Commands) {
    const SPRITE_BYTES: &[u8] = include_bytes!("../assets/bevy_bird_dark.png");

    commands.spawn((Player, Position::default(), Velocity(2, 1), Sprite::from_png(SPRITE_BYTES)));
}

fn render_sprites(mut buffer: ResMut<Buffer>, query: Query<(&Position, &Sprite)>) {
    for (pos, sprite) in query.iter() {
        buffer.draw_sprite(sprite, (pos.0, pos.1));
    }
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 = if velocity.0 > 0 {
            position.0.saturating_add(velocity.0 as usize)
        } else {
            position.0.saturating_sub(-velocity.0 as usize)
        };

        position.1 = if velocity.1 > 0 {
            position.1.saturating_add(velocity.1 as usize)
        } else {
            position.1.saturating_sub(-velocity.1 as usize)
        };
    }
}

fn bounce_sprites(mut query: Query<(&Position, &mut Velocity, &Sprite)>, buffer: Res<Buffer>) {
    let (max_x, max_y) = buffer.size();

    for (position, mut velocity, sprite) in query.iter_mut() {
        if position.0 + sprite.width() > max_x && velocity.0 > 0 {
            velocity.0 = -velocity.0;
        }

        if position.0 == 0 && velocity.0 < 0 {
            velocity.0 = -velocity.0;
        }

        if position.1 + sprite.height() > max_y && velocity.1 > 0 {
            velocity.1 = -velocity.1;
        }

        if position.1 == 0 && velocity.1 < 0 {
            velocity.1 = -velocity.1;
        }
    }
}

fn move_player(
    mut key_events: EventReader<KeyEvent>,
    mut query: Query<&mut Velocity, With<Player>>,
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

    for mut velocity in query.iter_mut() {
        velocity.0 += dx;
        velocity.1 += dy;
    }
}

#[derive(Component, Default)]
struct Position(usize, usize);

#[derive(Component, Default)]
struct Velocity(isize, isize);

#[derive(Component)]
struct Player;
