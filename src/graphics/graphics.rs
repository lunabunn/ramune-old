use super::backend_opengl::graphics::GLGraphics;

pub trait Graphics {
    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn flush(&mut self, clear_color: Option<Color>);
}

pub enum GraphicsBackends {
    GL(GLGraphics),
}

impl Graphics for GraphicsBackends {
    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        match self {
            GraphicsBackends::GL(g) => g.fill_rect(x, y, width, height),
        }
    }
    fn flush(&mut self, clear_color: Option<Color>) {
        match self {
            GraphicsBackends::GL(g) => g.flush(clear_color),
        }
    }
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    #[inline(always)]
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
