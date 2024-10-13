#![no_main]
#![no_std]

extern crate alloc;

#[uefi::entry]
fn main() -> uefi::Status {
    use bevy_app::AppExit::{Error, Success};
    
    uefi::helpers::init().unwrap();

    let result = bevy_app::App::new()
        .add_plugins(bevy_uefi_example::BevyUefiExample)
        .run();

    match result {
        Success => uefi::Status::SUCCESS,
        Error(code) => {
            log::error!("App Error: {}", code);
            uefi::Status::ABORTED
        }
    }
}
