#![no_main]
#![no_std]

extern crate alloc;

use bevy_app::App;
use bevy_uefi_example::BevyUefiExample;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    use bevy_app::AppExit::{Error, Success};

    let result = App::new().add_plugins(BevyUefiExample).run();

    match result {
        Success => Status::SUCCESS,
        Error(code) => {
            log::error!("App Error: {}", code);
            Status::ABORTED
        }
    }
}
