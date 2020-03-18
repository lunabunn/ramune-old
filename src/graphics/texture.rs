use super::{Color, GraphicsContext};
use crate::Context;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Texture {
    pub(crate) id: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(ctx: &mut Context, width: u32, height: u32, pixels: &[Color]) -> Self {
        ctx.graphics_context.create_texture(width, height, pixels)
    }
}
