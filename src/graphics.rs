use alloc::vec;
use alloc::vec::Vec;
use bevy_app::{Last, Plugin};
use bevy_ecs::{
    component::Component,
    system::{NonSendMut, Res, ResMut, Resource},
};
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

#[derive(Component)]
pub struct Sprite {
    width: usize,
    height: usize,
    pixels: Vec<Option<BltPixel>>,
}

impl Sprite {
    pub fn from_png(bytes: &[u8]) -> Self {
        let (header, data) = png_decoder::decode(bytes).unwrap();

        let width = header.width as usize;
        let height = header.height as usize;

        let mut pixels = Vec::with_capacity(width * height);

        match (header.color_type, header.bit_depth) {
            (png_decoder::ColorType::RgbAlpha, png_decoder::BitDepth::Eight) => {
                let reds = data.iter().step_by(4);
                let greens = data.iter().skip(1).step_by(4);
                let blues = data.iter().skip(2).step_by(4);
                let alphas = data.iter().skip(3).step_by(4);

                for (((&r, &g), &b), &a) in reds.zip(greens).zip(blues).zip(alphas) {
                    if a == 0 {
                        pixels.push(None);
                    } else {
                        pixels.push(Some(BltPixel::new(r, g, b)));
                    }
                }
            }
            _ => unimplemented!(),
        }

        Self {
            width,
            height,
            pixels,
        }
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
        if x > self.width || y > self.height {
            return None;
        }

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

    pub fn draw_sprite(&mut self, sprite: &Sprite, pos: (usize, usize)) {
        let (width, height) = (sprite.width, sprite.height);

        for y in 0..height {
            for x in 0..width {
                let &pixel = sprite.pixels.get(y * sprite.width + x).unwrap();
                let Some(color) = pixel else { continue };
                let Some(pixel) = self.pixel(x + pos.0, y + pos.1) else {
                    continue;
                };

                *pixel = color;
            }
        }
    }
}
