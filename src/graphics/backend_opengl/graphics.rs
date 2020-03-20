use super::gl;
use crate::graphics::{Color, GraphicsCommand, Texture};
use crate::{Context, GameOptions};

#[derive(Default)]
pub struct GraphicsContext {
    textures: Vec<gl::Texture>,
    dimensions: Vec<(u32, u32)>,
}

impl GraphicsContext {
    pub fn create_texture(&mut self, width: u32, height: u32, pixels: &[Color]) -> Texture {
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

        self.textures.push(tex);
        self.dimensions.push((width, height));
        Texture(self.textures.len() - 1)
    }

    pub fn texture_dimensions(&self, tex: Texture) -> (u32, u32) {
        self.dimensions[tex.0]
    }

    pub fn get_texture(&self, index: usize) -> gl::Texture {
        self.textures[index]
    }
}

pub struct Renderer {
    vbo: gl::VBO,
    default_program: gl::Program,
    blank_texture: gl::Texture,
}

#[derive(Debug)]
struct Command {
    verts: Vec<f32>,
    depth: f32,
    texture: Option<Texture>,
}

struct Batch {
    vert_count: i32,
    texture: Option<Texture>,
}

impl Renderer {
    pub fn new(options: &GameOptions) -> Self {
        let vbo;
        let default_program;
        let blank_texture;

        let vertex_shader_source = r#"
            #version 330
            
            layout(location = 0) in vec2 aPos;
            layout(location = 1) in vec2 aTexCoord;
            layout(location = 2) in vec4 aColor;

            uniform vec2 uViewport;
        
            out vec2 vTexCoord;
            out vec4 vColor;
    
            void main() {
                gl_Position = vec4(aPos.x * 2 / uViewport.x - 1, 1 - aPos.y * 2 / uViewport.y, 0.0, 1.0);
                vTexCoord = aTexCoord;
                vColor = aColor;
            }
        "#;
        let fragment_shader_source = r#"
            #version 330
            
            in vec2 vTexCoord;
            in vec4 vColor;

            uniform sampler2D uTexture;

            out vec4 FragColor;

            void main() {
                FragColor = texture(uTexture, vTexCoord) * vColor;
            }
        "#;

        vbo = gl::VBO::new();
        default_program = gl::Program::from_source(vertex_shader_source, fragment_shader_source)
            .unwrap_or_else(|s| panic!("{}", s));

        blank_texture = gl::Texture::new();
        gl::bind_texture(gl::TextureKind::Texture2D, Some(blank_texture));
        gl::tex_image_2d(
            gl::TextureKind::Texture2D,
            0,
            gl::TextureFormat::RGBA,
            1,
            1,
            &[255, 255, 255, 255],
        );

        gl::use_program(Some(default_program));
        gl::enable_vertex_attrib_array(0);
        gl::enable_vertex_attrib_array(1);
        gl::enable_vertex_attrib_array(2);

        let uniform_viewport = default_program.get_uniform_location("uViewport");
        gl::viewport(0, 0, options.size.0 as i32, options.size.1 as i32);
        gl::uniform_2(
            uniform_viewport,
            options.size.0 as f32,
            options.size.1 as f32,
        );

        gl::enable(gl::Capability::Blend);
        gl::blend_func(gl::BlendFactor::SrcAlpha, gl::BlendFactor::OneMinusSrcAlpha);

        Renderer {
            vbo,
            default_program,
            blank_texture,
        }
    }

    fn push_rect(
        output: &mut Vec<f32>,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
        color: Color,
    ) {
        let rgba = [
            color.r_normalized(),
            color.g_normalized(),
            color.b_normalized(),
            color.a_normalized(),
        ];
        let verts = [
            [x, y, sx, sy],
            [x, y + h, sx, sy + sh],
            [x + w, y, sx + sw, sy],
            [x, y + h, sx, sy + sh],
            [x + w, y + h, sx + sw, sy + sh],
            [x + w, y, sx + sw, sy],
        ];
        for vert in &verts {
            output.extend_from_slice(vert);
            output.extend_from_slice(&rgba);
        }
    }

    #[rustfmt::skip]
    pub(crate) fn flush(
        &mut self,
        ctx: &mut Context,
        commands: &mut Vec<GraphicsCommand>,
        clear_color: Option<Color>,
    ) {
        let mut verts: Vec<f32> = Vec::default();
        let mut batches: Vec<Batch> = Vec::default();

        let mut last_batch;
        commands.sort_by(|a, b| a.depth().partial_cmp(&b.depth()).unwrap());

        let mut texture = commands.first().and_then(|x| x.texture());
        batches.push(Batch {
            vert_count: 0,
            texture,
        });

        for cmd in commands.iter() {
            last_batch = batches.last_mut().unwrap();
            if cmd.texture() != texture {
                texture = cmd.texture();
                let new_batch = Batch {
                    vert_count: 0,
                    texture,
                };
                batches.push(new_batch);
                last_batch = batches.last_mut().unwrap();
            }

            match *cmd {
                GraphicsCommand::FillRect { color, x, y, w, h, .. } => {
                    Self::push_rect(&mut verts, x, y, w, h, 0.0, 0.0, 0.0, 0.0, color);
                    last_batch.vert_count += 48;
                },
                GraphicsCommand::DrawImage { color, texture, x, y, .. } => {
                    let (tw, th) = texture.dimensions(ctx);
                    let tw = tw as f32;
                    let th = th as f32;
                    Self::push_rect(&mut verts, x, y, tw, th, 0.0, 0.0, 1.0, 1.0, color);
                    last_batch.vert_count += 48;
                },
                GraphicsCommand::DrawSubimage { color, texture, x, y, sx, sy, sw, sh, .. } => {
                    let (tw, th) = texture.dimensions(ctx);
                    let tw = tw as f32;
                    let th = th as f32;
                    Self::push_rect(&mut verts, x, y, sw, sh, sx / tw, sy / th, sw / tw, sh / th, color);
                    last_batch.vert_count += 48;
                },
                GraphicsCommand::DrawImageScaled { color, x, y, w, h, .. } => {
                    Self::push_rect(&mut verts, x, y, w, h, 0.0, 0.0, 1.0, 1.0, color);
                    last_batch.vert_count += 48;
                },
                GraphicsCommand::DrawSubimageScaled { color, texture, x, y, w, h, sx, sy, sw, sh, .. } => {
                    let (tw, th) = texture.dimensions(ctx);
                    let tw = tw as f32;
                    let th = th as f32;
                    Self::push_rect(&mut verts, x, y, w, h, sx / tw, sy / th, sw / tw, sh / th, color);
                    last_batch.vert_count += 48;
                },
            }
        }

        commands.clear();
        if verts.len() == 0 {
            return;
        }

        gl::use_program(Some(self.default_program));

        gl::bind_buffer(gl::BufferKind::Array, Some(self.vbo));
        gl::buffer_data(gl::BufferKind::Array, &verts, gl::UsageMode::Stream);
        gl::vertex_attrib_pointer::<f32>(0, 2, false, 8, 0);
        gl::vertex_attrib_pointer::<f32>(1, 2, false, 8, 2);
        gl::vertex_attrib_pointer::<f32>(2, 4, false, 8, 4);

        if let Some(color) = clear_color {
            gl::clear_color(
                color.r_normalized(),
                color.g_normalized(),
                color.b_normalized(),
                color.a_normalized(),
            );
            gl::clear(gl::BufferBit::Color);
        }

        let mut index = 0;
        for batch in batches {
            gl::bind_texture(
                gl::TextureKind::Texture2D,
                batch
                    .texture
                    .and_then(|x| Some(ctx.graphics_context.get_texture(x.0)))
                    .or(Some(self.blank_texture)),
            );
            gl::draw_arrays(gl::DrawMode::Triangles, index, index + batch.vert_count);
            index += batch.vert_count;
        }
    }
}
