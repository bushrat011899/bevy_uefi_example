use bevy_app::{First, Plugin};
use bevy_ecs::{
    event::{Event, EventWriter},
    system::NonSendMut,
};
use uefi::{
    boot::{self, ScopedProtocol},
    proto::console::text::{Input, Key},
};

pub struct KeyInputPlugin;

impl Plugin for KeyInputPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let text_input_handle = boot::get_handle_for_protocol::<Input>().unwrap();
        let text_input = boot::open_protocol_exclusive::<Input>(text_input_handle).unwrap();

        app.add_event::<KeyEvent>()
            .insert_non_send_resource(text_input)
            .add_systems(First, update);
    }
}

fn update(mut input: NonSendMut<ScopedProtocol<Input>>, mut writer: EventWriter<KeyEvent>) {
    while let Ok(Some(key)) = input.read_key() {
        writer.send(KeyEvent { key });
    }
}

#[derive(Event, Debug)]
pub struct KeyEvent {
    pub key: Key,
}
