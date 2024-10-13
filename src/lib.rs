#![no_std]

extern crate alloc;

mod buffer;
mod diagnostics;
mod keyboard;
mod pointer;
mod time;

use bevy_ecs::{
    change_detection::DetectChanges,
    event::EventReader,
    system::{Res, ResMut, Resource},
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
        .init_resource::<BackgroundColor>()
        .add_systems(Startup, Buffer::clear)
        .add_systems(Update, (update_background, set_background));
    }
}

fn set_background(mut buffer: ResMut<Buffer>, background_color: Res<BackgroundColor>) {
    let (width, height) = buffer.size();

    if background_color.is_changed() {
        let &BackgroundColor(color) = background_color.as_ref();

        for y in 0..height {
            for x in 0..width {
                let pixel = buffer.pixel(x, y).unwrap();
                *pixel = color;
            }
        }
    }
}

fn update_background(
    mut key_events: EventReader<KeyEvent>,
    mut background_color: ResMut<BackgroundColor>,
) {
    use uefi::proto::console::text::Key::{Printable, Special};

    for event in key_events.read() {
        match event.key {
            Printable(char16) => {
                if char16 == '1' {
                    background_color.0 = BltPixel::new(0, 0, 0);
                } else if char16 == '2' {
                    background_color.0 = BltPixel::new(255, 255, 255);
                } else if char16 == '3' {
                    background_color.0 = BltPixel::new(128, 0, 0);
                } else if char16 == '4' {
                    background_color.0 = BltPixel::new(0, 128, 0);
                } else if char16 == '5' {
                    background_color.0 = BltPixel::new(0, 0, 128);
                }
            }
            Special(ScanCode::UP) => {
                background_color.0.red = background_color.0.red.saturating_add(1);
                background_color.0.green = background_color.0.green.saturating_add(1);
                background_color.0.blue = background_color.0.blue.saturating_add(1);
            }
            Special(ScanCode::DOWN) => {
                background_color.0.red = background_color.0.red.saturating_sub(1);
                background_color.0.green = background_color.0.green.saturating_sub(1);
                background_color.0.blue = background_color.0.blue.saturating_sub(1);
            }
            _ => {}
        }
    }
}

#[derive(Resource, Clone, Copy)]
struct BackgroundColor(BltPixel);

impl Default for BackgroundColor {
    fn default() -> Self {
        Self::new()
    }
}

impl BackgroundColor {
    const fn new() -> Self {
        Self(BltPixel::new(0, 0, 0))
    }
}
