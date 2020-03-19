use super::backend_opengl::{GLGraphics, GLGraphicsContext};
use super::Texture;
use crate::Context;

pub(crate) trait GraphicsContext {
    fn create_texture(&mut self, width: u32, height: u32, pixels: &[Color]) -> Texture;
    fn texture_dimensions(&self, tex: Texture) -> (u32, u32);
}

pub(crate) enum GraphicsContextHandle {
    GL(GLGraphicsContext),
}

impl GraphicsContextHandle {
    pub fn as_gl(&self) -> Option<&GLGraphicsContext> {
        if let Self::GL(ctx) = self {
            Some(ctx)
        } else {
            None
        }
    }
}

impl GraphicsContext for GraphicsContextHandle {
    fn create_texture(&mut self, width: u32, height: u32, pixels: &[Color]) -> Texture {
        match self {
            Self::GL(tm) => tm.create_texture(width, height, pixels),
        }
    }
    fn texture_dimensions(&self, tex: Texture) -> (u32, u32) {
        match self {
            Self::GL(tm) => tm.texture_dimensions(tex),
        }
    }
}

#[rustfmt::skip]
pub(crate) enum GraphicsCommand {
    FillRect { color: Color, depth: f32, x: f32, y: f32, w: f32, h: f32 },
    DrawImage { color: Color, depth: f32, texture: Texture, x: f32, y: f32 },
    DrawSubimage { color: Color, depth: f32, texture: Texture, x: f32, y: f32, sx: f32, sy: f32, sw: f32, sh: f32 },
    DrawImageScaled { color: Color, depth: f32, texture: Texture, x: f32, y: f32, w: f32, h: f32 },
    DrawSubimageScaled { color: Color, depth: f32, texture: Texture, x: f32, y: f32, w: f32, h: f32, sx: f32, sy: f32, sw: f32, sh: f32 },
}

impl GraphicsCommand {
    pub fn depth(&self) -> f32 {
        match self {
            Self::FillRect { depth, .. }
            | Self::DrawImage { depth, .. }
            | Self::DrawSubimage { depth, .. }
            | Self::DrawImageScaled { depth, .. }
            | Self::DrawSubimageScaled { depth, .. } => *depth,
        }
    }

    pub fn texture(&self) -> Option<Texture> {
        match self {
            Self::FillRect { .. } => None,
            Self::DrawImage { texture, .. }
            | Self::DrawSubimage { texture, .. }
            | Self::DrawImageScaled { texture, .. }
            | Self::DrawSubimageScaled { texture, .. } => Some(*texture),
        }
    }
}

pub struct Graphics {
    backend: GraphicsBackend,
    commands: Vec<GraphicsCommand>,
    pub color: Color,
    pub depth: f32,
}

pub(crate) enum GraphicsBackend {
    GL(GLGraphics),
}

#[rustfmt::skip]
impl Graphics {
    pub(crate) fn from_backend(backend: GraphicsBackend) -> Self {
        Self {
            backend,
            commands: Vec::new(),
            color: Color::white(),
            depth: 0.0,
        }
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.commands.push(GraphicsCommand::FillRect { color: self.color, depth: self.depth, x, y, w, h });
    }

    pub fn draw_image(&mut self, texture: Texture, x: f32, y: f32) {
        self.commands.push(GraphicsCommand::DrawImage { color: self.color, depth: self.depth, texture, x, y });
    }

    pub fn draw_subimage(&mut self, texture: Texture, x: f32, y: f32, sx: f32, sy: f32, sw: f32, sh: f32) {
        self.commands.push(GraphicsCommand::DrawSubimage { color: self.color, depth: self.depth, texture, x, y, sx, sy, sw, sh });
    }

    pub fn draw_image_scaled(&mut self, texture: Texture, x: f32, y: f32, w: f32, h: f32) {
        self.commands.push(GraphicsCommand::DrawImageScaled { color: self.color, depth: self.depth, texture, x, y, w, h });
    }

    pub fn draw_subimage_scaled(&mut self, texture: Texture, x: f32, y: f32, w: f32, h: f32, sx: f32, sy: f32, sw: f32, sh: f32) {
        self.commands.push(GraphicsCommand::DrawSubimageScaled { color: self.color, depth: self.depth, texture, x, y, w, h, sx, sy, sw, sh });
    }

    pub fn flush(&mut self, ctx: &mut Context, clear_color: Option<Color>) {
        match &mut self.backend {
            GraphicsBackend::GL(g) => {
                g.flush(ctx, &mut self.commands, clear_color);
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[rustfmt::skip]
    const BYTE_FLOAT_LOOKUP_TABLE: [f32; 256] = [0.0, 0.003921569, 0.007843138, 0.011764707, 0.015686275, 0.019607844, 0.023529414, 0.027450982, 0.03137255, 0.03529412, 0.039215688, 0.043137256, 0.04705883, 0.050980397, 0.054901965, 0.058823533, 0.0627451, 0.06666667, 0.07058824, 0.07450981, 0.078431375, 0.08235294, 0.08627451, 0.09019608, 0.09411766, 0.098039225, 0.10196079, 0.10588236, 0.10980393, 0.1137255, 0.11764707, 0.121568635, 0.1254902, 0.12941177, 0.13333334, 0.13725491, 0.14117648, 0.14509805, 0.14901961, 0.15294118, 0.15686275, 0.16078432, 0.16470589, 0.16862746, 0.17254902, 0.1764706, 0.18039216, 0.18431373, 0.18823531, 0.19215688, 0.19607845, 0.20000002, 0.20392159, 0.20784315, 0.21176472, 0.21568629, 0.21960786, 0.22352943, 0.227451, 0.23137257, 0.23529413, 0.2392157, 0.24313727, 0.24705884, 0.2509804, 0.25490198, 0.25882354, 0.2627451, 0.26666668, 0.27058825, 0.27450982, 0.2784314, 0.28235295, 0.28627452, 0.2901961, 0.29411766, 0.29803923, 0.3019608, 0.30588236, 0.30980393, 0.3137255, 0.31764707, 0.32156864, 0.3254902, 0.32941177, 0.33333334, 0.3372549, 0.34117648, 0.34509805, 0.34901962, 0.3529412, 0.35686275, 0.36078432, 0.3647059, 0.36862746, 0.37254903, 0.37647063, 0.3803922, 0.38431376, 0.38823533, 0.3921569, 0.39607847, 0.40000004, 0.4039216, 0.40784317, 0.41176474, 0.4156863, 0.41960788, 0.42352945, 0.427451, 0.43137258, 0.43529415, 0.43921572, 0.4431373, 0.44705886, 0.45098042, 0.454902, 0.45882356, 0.46274513, 0.4666667, 0.47058827, 0.47450984, 0.4784314, 0.48235297, 0.48627454, 0.4901961, 0.49411768, 0.49803925, 0.5019608, 0.5058824, 0.50980395, 0.5137255, 0.5176471, 0.52156866, 0.5254902, 0.5294118, 0.53333336, 0.5372549, 0.5411765, 0.54509807, 0.54901963, 0.5529412, 0.5568628, 0.56078434, 0.5647059, 0.5686275, 0.57254905, 0.5764706, 0.5803922, 0.58431375, 0.5882353, 0.5921569, 0.59607846, 0.6, 0.6039216, 0.60784316, 0.6117647, 0.6156863, 0.61960787, 0.62352943, 0.627451, 0.6313726, 0.63529414, 0.6392157, 0.6431373, 0.64705884, 0.6509804, 0.654902, 0.65882355, 0.6627451, 0.6666667, 0.67058825, 0.6745098, 0.6784314, 0.68235296, 0.6862745, 0.6901961, 0.69411767, 0.69803923, 0.7019608, 0.7058824, 0.70980394, 0.7137255, 0.7176471, 0.72156864, 0.7254902, 0.7294118, 0.73333335, 0.7372549, 0.7411765, 0.74509805, 0.7490196, 0.75294125, 0.7568628, 0.7607844, 0.76470596, 0.7686275, 0.7725491, 0.77647066, 0.7803922, 0.7843138, 0.78823537, 0.79215693, 0.7960785, 0.8000001, 0.80392164, 0.8078432, 0.8117648, 0.81568635, 0.8196079, 0.8235295, 0.82745105, 0.8313726, 0.8352942, 0.83921576, 0.8431373, 0.8470589, 0.85098046, 0.854902, 0.8588236, 0.86274517, 0.86666673, 0.8705883, 0.8745099, 0.87843144, 0.882353, 0.8862746, 0.89019614, 0.8941177, 0.8980393, 0.90196085, 0.9058824, 0.909804, 0.91372555, 0.9176471, 0.9215687, 0.92549026, 0.9294118, 0.9333334, 0.93725497, 0.94117653, 0.9450981, 0.9490197, 0.95294124, 0.9568628, 0.9607844, 0.96470594, 0.9686275, 0.9725491, 0.97647065, 0.9803922, 0.9843138, 0.98823535, 0.9921569, 0.9960785, 1.0];

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn r_normalized(&self) -> f32 {
        Self::BYTE_FLOAT_LOOKUP_TABLE[self.r as usize]
    }

    pub fn g_normalized(&self) -> f32 {
        Self::BYTE_FLOAT_LOOKUP_TABLE[self.g as usize]
    }

    pub fn b_normalized(&self) -> f32 {
        Self::BYTE_FLOAT_LOOKUP_TABLE[self.b as usize]
    }

    pub fn a_normalized(&self) -> f32 {
        Self::BYTE_FLOAT_LOOKUP_TABLE[self.a as usize]
    }

    // COLORS

    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }
    pub fn silver() -> Self {
        Self::rgb(192, 192, 192)
    }
    pub fn gray() -> Self {
        Self::rgb(128, 128, 128)
    }
    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }
    pub fn red() -> Self {
        Self::rgb(255, 0, 0)
    }
    pub fn maroon() -> Self {
        Self::rgb(128, 0, 0)
    }
    pub fn yellow() -> Self {
        Self::rgb(255, 255, 0)
    }
    pub fn olive() -> Self {
        Self::rgb(128, 128, 0)
    }
    pub fn lime() -> Self {
        Self::rgb(0, 255, 0)
    }
    pub fn green() -> Self {
        Self::rgb(0, 128, 0)
    }
    pub fn aqua() -> Self {
        Self::rgb(0, 255, 255)
    }
    pub fn teal() -> Self {
        Self::rgb(0, 128, 128)
    }
    pub fn blue() -> Self {
        Self::rgb(0, 0, 255)
    }
    pub fn navy() -> Self {
        Self::rgb(0, 0, 128)
    }
    pub fn fuchsia() -> Self {
        Self::rgb(255, 0, 255)
    }
    pub fn purple() -> Self {
        Self::rgb(128, 0, 128)
    }
}
