use super::gl;
use crate::graphics::{Color, Graphics};
use crate::GameOptions;

pub struct GLGraphics {
    vbo_a: gl::VBO,
    vbo_b: gl::VBO,
    default_program: gl::Program,

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
}

struct Batch {
    vert_count: i32,
}

impl GLGraphics {
    pub fn new(options: &GameOptions) -> Self {
        let vbo_a: gl::VBO;
        let vbo_b: gl::VBO;
        let default_program: gl::Program;

        let uniform_viewport: Option<gl::UniformLocation>;

        let vertex_shader_source = r#"
            #version 330
            
            layout(location = 0) in vec2 aPos;
            layout(location = 2) in vec4 aColor;

            uniform vec2 uViewport;
    
            out vec4 vColor;
    
            void main() {
                gl_Position = vec4(aPos.x * 2 / uViewport.x - 1, 1 - aPos.y * 2 / uViewport.y, 0.0, 1.0);
                vColor = aColor;
            }
        "#;
        let fragment_shader_source = r#"
            #version 330
            
            in vec4 vColor;

            void main() {
                gl_FragColor = vColor;
            }
        "#;

        vbo_a = gl::VBO::new();
        vbo_b = gl::VBO::new();
        default_program = gl::Program::from_source(vertex_shader_source, fragment_shader_source)
            .unwrap_or_else(|s| panic!("{}", s));

        gl::enable_vertex_attrib_array(0);
        gl::enable_vertex_attrib_array(1);

        gl::use_program(Some(default_program));
        uniform_viewport = default_program.get_uniform_location("uViewport");
        gl::viewport(0, 0, options.size.0 as i32, options.size.1 as i32);
        gl::uniform_2(
            uniform_viewport,
            options.size.0 as f32,
            options.size.1 as f32,
        );
        gl::use_program(None);

        let curr_color = Color::rgb(1.0, 1.0, 1.0);
        let curr_depth = 0.0;
        let commands = Vec::default();
        GLGraphics {
            vbo_a,
            vbo_b,
            default_program,

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
        gl::use_program(None);
    }
}

impl Graphics for GLGraphics {
    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let verts = [
            x,
            y,
            self.curr_color.r,
            self.curr_color.g,
            self.curr_color.b,
            self.curr_color.a,
            x,
            y + height,
            self.curr_color.r,
            self.curr_color.g,
            self.curr_color.b,
            self.curr_color.a,
            x + width,
            y,
            self.curr_color.r,
            self.curr_color.g,
            self.curr_color.b,
            self.curr_color.a,
            x,
            y + height,
            self.curr_color.r,
            self.curr_color.g,
            self.curr_color.b,
            self.curr_color.a,
            x + width,
            y + height,
            self.curr_color.r,
            self.curr_color.g,
            self.curr_color.b,
            self.curr_color.a,
            x + width,
            y,
            self.curr_color.r,
            self.curr_color.g,
            self.curr_color.b,
            self.curr_color.a,
        ]
        .to_vec();
        let depth = self.curr_depth;
        self.commands.push(Command { verts, depth });
    }

    fn flush(&mut self, clear_color: Option<Color>) {
        let mut verts: Vec<f32> = Vec::default();
        let mut batches: Vec<Batch> = Vec::default();

        let mut last_batch;
        self.commands
            .sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap());
        for cmd in self.commands.iter() {
            if batches.len() == 0 {
                batches.push(Batch { vert_count: 0 });
            }
            last_batch = batches.last_mut().unwrap();
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
        gl::vertex_attrib_pointer::<f32>(0, 2, false, 6, 0);
        gl::vertex_attrib_pointer::<f32>(1, 4, false, 6, 2);

        if let Some(color) = clear_color {
            gl::clear_color(color.r, color.g, color.b, color.a);
            gl::clear(gl::BufferBit::Color);
        }

        if let Some(batches_prev) = &self.batches_prev {
            gl::bind_buffer(gl::BufferKind::Array, Some(vbo_render));
            let mut index = 0;
            for batch in batches_prev {
                gl::draw_arrays(gl::DrawMode::Triangles, index, index + batch.vert_count);
                index += batch.vert_count;
            }
        }

        self.batches_prev = Some(batches);

        gl::bind_buffer(gl::BufferKind::Array, None);
        gl::use_program(None);
    }
}
