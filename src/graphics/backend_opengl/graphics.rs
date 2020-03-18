use super::gl;
use crate::graphics::{Color, Graphics, Texture};
use crate::GameOptions;

pub struct GLGraphics {
    vbo_a: gl::VBO,
    vbo_b: gl::VBO,
    default_program: gl::Program,

    blank_texture: gl::Texture,

    uniform_viewport: Option<gl::UniformLocation>,

    vbo_switch: bool,
    batches_prev: Option<Vec<Batch>>,

    curr_color: Color,
    curr_depth: f32,
    commands: Vec<Command>,
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

impl GLGraphics {
    pub fn new(options: &GameOptions) -> Self {
        let vbo_a;
        let vbo_b;
        let default_program;

        let blank_texture;

        let uniform_viewport;

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

        vbo_a = gl::VBO::new();
        vbo_b = gl::VBO::new();
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

        uniform_viewport = default_program.get_uniform_location("uViewport");
        gl::viewport(0, 0, options.size.0 as i32, options.size.1 as i32);
        gl::uniform_2(
            uniform_viewport,
            options.size.0 as f32,
            options.size.1 as f32,
        );

        gl::enable(gl::Capability::Blend);
        gl::blend_func(gl::BlendFactor::SrcAlpha, gl::BlendFactor::OneMinusSrcAlpha);

        let curr_color = Color::rgb(255, 255, 255);
        let curr_depth = 0.0;
        let commands = Vec::default();
        GLGraphics {
            vbo_a,
            vbo_b,
            default_program,

            blank_texture,

            uniform_viewport,

            vbo_switch: false,
            batches_prev: None,

            curr_color,
            curr_depth,
            commands,
        }
    }

    pub fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        gl::use_program(Some(self.default_program));
        gl::viewport(x, y, width, height);
        gl::uniform_2(self.uniform_viewport, width as f32, height as f32);
    }
}

impl Graphics for GLGraphics {
    fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let verts = [
            x,
            y,
            0.0,
            0.0,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x,
            y + h,
            0.0,
            0.0,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x + w,
            y,
            0.0,
            0.0,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x,
            y + h,
            0.0,
            0.0,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x + w,
            y + h,
            0.0,
            0.0,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x + w,
            y,
            0.0,
            0.0,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
        ]
        .to_vec();
        let depth = self.curr_depth;
        self.commands.push(Command {
            verts,
            depth,
            texture: None,
        });
    }

    fn draw_subimage_scaled_normalized(
        &mut self,
        texture: Texture,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        sx: f32,
        sy: f32,
        sw: f32,
        sh: f32,
    ) {
        let verts = [
            x,
            y,
            sx,
            sy,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x,
            y + h,
            sx,
            sy + sh,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x + w,
            y,
            sx + sw,
            sy,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x,
            y + h,
            sx,
            sy + sh,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x + w,
            y + h,
            sx + sw,
            sy + sh,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
            x + w,
            y,
            sx + sw,
            sy,
            self.curr_color.r_normalized(),
            self.curr_color.g_normalized(),
            self.curr_color.b_normalized(),
            self.curr_color.a_normalized(),
        ]
        .to_vec();
        let depth = self.curr_depth;
        self.commands.push(Command {
            verts,
            depth,
            texture: Some(texture),
        });
    }

    fn flush(&mut self, clear_color: Option<Color>) {
        let mut verts: Vec<f32> = Vec::default();
        let mut batches: Vec<Batch> = Vec::default();

        let mut last_batch;
        self.commands
            .sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap());

        let mut texture = self.commands.first().and_then(|x| x.texture);
        batches.push(Batch {
            vert_count: 0,
            texture,
        });
        for cmd in self.commands.iter() {
            last_batch = batches.last_mut().unwrap();
            if cmd.texture != texture {
                texture = cmd.texture;
                let new_batch = Batch {
                    vert_count: 0,
                    texture,
                };
                batches.push(new_batch);
                last_batch = batches.last_mut().unwrap();
            };
            for vert in &cmd.verts {
                verts.push(*vert);
                last_batch.vert_count += 1;
            }
        }

        self.commands.clear();
        if verts.len() == 0 {
            return;
        }

        let (vbo_write, vbo_render) = match self.vbo_switch {
            true => (self.vbo_a, self.vbo_b),
            false => (self.vbo_b, self.vbo_a),
        };
        self.vbo_switch = !self.vbo_switch;

        gl::use_program(Some(self.default_program));

        gl::bind_buffer(gl::BufferKind::Array, Some(vbo_write));
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

        if let Some(batches_prev) = &self.batches_prev {
            gl::bind_buffer(gl::BufferKind::Array, Some(vbo_render));
            let mut index = 0;
            for batch in batches_prev {
                gl::bind_texture(
                    gl::TextureKind::Texture2D,
                    batch
                        .texture
                        .and_then(|x| Some(x.id.into()))
                        .or(Some(self.blank_texture)),
                );
                gl::draw_arrays(gl::DrawMode::Triangles, index, index + batch.vert_count);
                index += batch.vert_count;
            }
        }

        self.batches_prev = Some(batches);
    }
}
