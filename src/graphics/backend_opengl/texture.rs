use super::gl;
use crate::graphics::{Color, Texture, TextureManager};

pub struct GLTextureManager;

impl TextureManager for GLTextureManager {
    fn create_texture(&mut self, width: u32, height: u32, pixels: &[Color]) -> Texture {
        let tex = gl::Texture::new();
        gl::bind_texture(gl::TextureKind::Texture2D, Some(tex));
        let mut data = Vec::new();
        for pixel in pixels {
            data.push(pixel.r);
            data.push(pixel.g);
            data.push(pixel.b);
            data.push(pixel.a);
        }
        gl::tex_parameter(
            gl::TextureKind::Texture2D,
            gl::TextureParameter::WrapS,
            gl::WrapMode::Repeat,
        );
        gl::tex_parameter(
            gl::TextureKind::Texture2D,
            gl::TextureParameter::WrapT,
            gl::WrapMode::Repeat,
        );
        gl::tex_parameter(
            gl::TextureKind::Texture2D,
            gl::TextureParameter::MinFilter,
            gl::FilterMode::Nearest,
        );
        gl::tex_parameter(
            gl::TextureKind::Texture2D,
            gl::TextureParameter::MagFilter,
            gl::FilterMode::Nearest,
        );
        gl::tex_image_2d(
            gl::TextureKind::Texture2D,
            0,
            gl::TextureFormat::RGBA,
            width,
            height,
            &data,
        );

        Texture {
            id: tex.into(),
            width,
            height,
        }
    }
}
