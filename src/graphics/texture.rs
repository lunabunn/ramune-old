use super::backend_opengl::GLTextureManager;
use super::Color;
use crate::Context;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Texture {
    pub(crate) id: u32,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(ctx: &mut Context, width: u32, height: u32, pixels: &[Color]) -> Self {
        ctx.texture_manager.create_texture(width, height, pixels)
    }
}

pub(crate) trait TextureManager {
    fn create_texture(&mut self, width: u32, height: u32, pixels: &[Color]) -> Texture;
}

pub(crate) enum TextureManagerHandle {
    GL(GLTextureManager),
}

impl TextureManager for TextureManagerHandle {
    fn create_texture(&mut self, width: u32, height: u32, pixels: &[Color]) -> Texture {
        match self {
            Self::GL(tm) => tm.create_texture(width, height, pixels),
        }
    }
}
