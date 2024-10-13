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
    system::{Commands, Query, ResMut},
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
        .add_systems(Update, (move_player, Buffer::clear, render_sprites).chain());
    }
}

fn setup(mut commands: Commands) {
    const SPRITE_BYTES: &[u8] = include_bytes!("../assets/bevy_bird_dark.png");

    commands.spawn((Player, Position::default(), Sprite::from_png(SPRITE_BYTES)));
}

fn render_sprites(mut buffer: ResMut<Buffer>, query: Query<(&Position, &Sprite)>) {
    for (pos, sprite) in query.iter() {
        buffer.draw_sprite(sprite, (pos.0, pos.1));
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
struct Player;
