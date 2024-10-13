use bevy_app::{First, Plugin};
use bevy_ecs::system::{NonSendMut, ResMut, Resource};
use uefi::boot::{self, ScopedProtocol};
use uefi::proto::console::pointer::Pointer as UefiPointer;

pub struct PointerPlugin;

impl Plugin for PointerPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let pointer_handle = boot::get_handle_for_protocol::<UefiPointer>().unwrap();
        let pointer = boot::open_protocol_exclusive::<UefiPointer>(pointer_handle).unwrap();

        app.init_resource::<Pointer>()
            .insert_non_send_resource(pointer)
            .add_systems(
                First,
                |mut this: ResMut<Pointer>,
                 mut pointer: NonSendMut<ScopedProtocol<UefiPointer>>| {
                    let pointer = pointer.get_mut().unwrap();
                    let Some(state) = pointer.read_state().unwrap() else {
                        return;
                    };

                    let [dx, dy, dz] = state.relative_movement;
                    let [left_click, right_click] = state.button;

                    this.x = this.x.saturating_add(dx as isize);
                    this.y = this.y.saturating_add(dy as isize);
                    this.z = this.z.saturating_add(dz as isize);

                    this.left_click = left_click;
                    this.right_click = right_click;
                },
            );
    }
}

#[derive(Resource, Default, Debug)]
pub struct Pointer {
    x: isize,
    y: isize,
    z: isize,
    left_click: bool,
    right_click: bool,
}

impl Pointer {
    pub fn is_left_pressed(&self) -> bool {
        self.left_click
    }

    pub fn is_right_pressed(&self) -> bool {
        self.right_click
    }

    pub fn get_position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn get_scroll(&self) -> isize {
        self.z
    }
}
