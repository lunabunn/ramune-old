use super::{Color, GraphicsContext};
use crate::Context;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Texture(pub usize);

impl Texture {
    pub fn new(ctx: &mut Context, width: u32, height: u32, pixels: &[Color]) -> Self {
        ctx.graphics_context.create_texture(width, height, pixels)
    }

    pub fn dimensions(self, ctx: &Context) -> (u32, u32) {
        ctx.graphics_context.texture_dimensions(self)
    }
}
