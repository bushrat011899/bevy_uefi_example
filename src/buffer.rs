use alloc::vec;
use alloc::vec::Vec;
use bevy_app::{Last, Plugin};
use bevy_ecs::system::{NonSendMut, Res, ResMut, Resource};
use uefi::{
    boot::{self, ScopedProtocol},
    proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
        let gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

        let (width, height) = gop.current_mode_info().resolution();

        app.insert_resource(Buffer::new(width, height))
            .insert_non_send_resource(gop)
            .add_systems(
                Last,
                |buffer: Res<Buffer>, mut gop: NonSendMut<ScopedProtocol<GraphicsOutput>>| {
                    if buffer.blit(&mut gop).is_err() {
                        log::warn!("Failed to update graphics output");
                    }
                },
            );
    }
}

#[derive(Resource)]
pub struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    /// Create a new [`Buffer`].
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    /// Get the `width` and `height` of this [`Buffer`]
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Get a single pixel.
    pub fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }

    /// Blit the buffer to the framebuffer.
    pub fn blit(&self, gop: &mut GraphicsOutput) -> uefi::Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: self.size(),
        })
    }

    /// Clear this [`Buffer`]
    pub fn clear(mut buffer: ResMut<Buffer>) {
        let (width, height) = buffer.size();

        for y in 0..height {
            for x in 0..width {
                let pixel = buffer.pixel(x, y).unwrap();
                pixel.red = 0;
                pixel.green = 0;
                pixel.blue = 0;
            }
        }
    }
}
