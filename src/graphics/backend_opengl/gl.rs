#![allow(dead_code)]

use super::bindings::gl32 as gl;

macro_rules! newtype {
    ($x:ident: $y:ty) => {
        #[derive(Clone, Copy)]
        pub struct $x($y);
        impl From<$x> for $y {
            fn from(x: $x) -> Self {
                x.0
            }
        }
    };

    ($x:ident: $y:ty, $($a:ident: $b:ty),+) => {
        newtype!($x: $y);
        newtype!($($a: $b),+);
    };

    ($($x:ident: $y:ty),+,) => {
        newtype!($($x: $y),+);
    };
}
newtype!(
    Shader: u32,
    Program: u32,
    VBO: u32,
    UniformLocation: i32,
    BufferBit: u32,
    Texture: u32,
);

impl Shader {
    pub fn new(kind: ShaderKind) -> Self {
        unsafe { Self(gl::CreateShader(kind as u32)) }
    }

    pub fn delete(self) {
        unsafe {
            gl::DeleteShader(self.into());
        }
    }

    pub fn source(self, source: &str) {
        unsafe {
            let c_str = std::ffi::CString::new(source).unwrap();
            gl::ShaderSource(self.into(), 1, &c_str.as_ptr(), std::ptr::null());
        }
    }

    pub fn compile(self) {
        unsafe {
            gl::CompileShader(self.into());
        }
    }

    pub fn get_compile_status(self) -> bool {
        unsafe {
            let mut status = gl::GL_FALSE as i32;
            gl::GetShaderiv(self.into(), gl::GL_COMPILE_STATUS, &mut status);

            status == gl::GL_TRUE as i32
        }
    }

    pub fn get_info_log(self) -> String {
        unsafe {
            let mut len = 0;
            gl::GetShaderiv(self.into(), gl::GL_INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                self.into(),
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut gl::GLchar,
            );

            String::from_utf8(buf)
                .ok()
                .expect("glGetShaderInfoLog didn't return valid UTF-8!")
        }
    }

    pub fn from_source(kind: ShaderKind, source: &str) -> Result<Self, String> {
        let shader = Self::new(kind);
        shader.source(source);
        shader.compile();
        if shader.get_compile_status() {
            Ok(shader)
        } else {
            Err(shader.get_info_log())
        }
    }
}

impl Program {
    pub fn new() -> Self {
        unsafe { Self(gl::CreateProgram() as u32) }
    }

    pub fn attach_shader(self, shader: Shader) {
        unsafe {
            gl::AttachShader(self.into(), shader.into());
        }
    }

    pub fn link(self) {
        unsafe {
            gl::LinkProgram(self.into());
        }
    }

    pub fn get_uniform_location(self, name: &str) -> Option<UniformLocation> {
        unsafe {
            let c_str = std::ffi::CString::new(name).unwrap();
            let uniform = gl::GetUniformLocation(self.into(), c_str.as_ptr());
            if uniform != -1 {
                Some(UniformLocation(uniform))
            } else {
                None
            }
        }
    }

    pub fn get_link_status(self) -> bool {
        unsafe {
            let mut status = gl::GL_FALSE as i32;
            gl::GetProgramiv(self.into(), gl::GL_LINK_STATUS, &mut status);

            status == gl::GL_TRUE as i32
        }
    }

    pub fn get_info_log(self) -> String {
        unsafe {
            let mut len = 0;
            gl::GetProgramiv(self.into(), gl::GL_INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                self.into(),
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut gl::GLchar,
            );

            String::from_utf8(buf)
                .ok()
                .expect("glGetProgramInfoLog didn't return valid UTF-8!")
        }
    }

    pub fn from_source(source_vertex: &str, source_fragment: &str) -> Result<Self, String> {
        let vertex_shader = Shader::from_source(ShaderKind::Vertex, source_vertex)?;
        let fragment_shader = Shader::from_source(ShaderKind::Fragment, source_fragment)?;

        let program = Self::new();
        program.attach_shader(vertex_shader);
        program.attach_shader(fragment_shader);
        program.link();
        if program.get_link_status() {
            Ok(program)
        } else {
            Err(program.get_info_log())
        }
    }
}

impl VBO {
    pub fn new() -> Self {
        unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);

            Self(vbo)
        }
    }
}

impl BufferBit {
    pub const Color: Self = Self(gl::GL_COLOR_BUFFER_BIT);
    pub const Depth: Self = Self(gl::GL_DEPTH_BUFFER_BIT);
    pub const Stencil: Self = Self(gl::GL_STENCIL_BUFFER_BIT);
}
impl std::ops::BitOr for BufferBit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl Texture {
    pub fn new() -> Self {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);

            Self(texture)
        }
    }
}

#[derive(Clone, Copy)]
pub enum ShaderKind {
    Fragment = gl::GL_FRAGMENT_SHADER as isize,
    Vertex = gl::GL_VERTEX_SHADER as isize,
}

#[derive(Clone, Copy)]
pub enum BufferKind {
    Array = gl::GL_ARRAY_BUFFER as isize,
    ElementArray = gl::GL_ELEMENT_ARRAY_BUFFER as isize,
}

#[derive(Clone, Copy)]
pub enum TextureKind {
    Texture2D = gl::GL_TEXTURE_2D as isize,
    CubeMap = gl::GL_TEXTURE_CUBE_MAP as isize,
}

#[derive(Clone, Copy)]
pub enum UsageMode {
    Stream = gl::GL_STREAM_DRAW as isize,
    Static = gl::GL_STATIC_DRAW as isize,
    Dynamic = gl::GL_DYNAMIC_DRAW as isize,
}

#[derive(Clone, Copy)]
pub enum DrawMode {
    Points = gl::GL_POINTS as isize,
    LineStrip = gl::GL_LINE_STRIP as isize,
    LineLoop = gl::GL_LINE_LOOP as isize,
    Lines = gl::GL_LINES as isize,
    TriangleStrip = gl::GL_TRIANGLE_STRIP as isize,
    TriangleFan = gl::GL_TRIANGLE_FAN as isize,
    Triangles = gl::GL_TRIANGLES as isize,
}

#[derive(Clone, Copy)]
pub enum TextureParameter {
    MinFilter = gl::GL_TEXTURE_MIN_FILTER as isize,
    MagFilter = gl::GL_TEXTURE_MAG_FILTER as isize,
    WrapS = gl::GL_TEXTURE_WRAP_S as isize,
    WrapT = gl::GL_TEXTURE_WRAP_T as isize,
}

#[derive(Clone, Copy)]
pub enum TextureFormat {
    Alpha = gl::GL_ALPHA as isize,
    RGB = gl::GL_RGB as isize,
    RGBA = gl::GL_RGBA as isize,
}

#[derive(Clone, Copy)]
pub enum NumberEnum {
    Byte = gl::GL_BYTE as isize,
    UnsignedByte = gl::GL_UNSIGNED_BYTE as isize,
    Short = gl::GL_SHORT as isize,
    UnsignedShort = gl::GL_UNSIGNED_SHORT as isize,
    Int = gl::GL_INT as isize,
    UnsignedInt = gl::GL_UNSIGNED_INT as isize,
    Float = gl::GL_FLOAT as isize,
    Double = gl::GL_DOUBLE as isize,
}

pub trait SetUniform {
    fn uniform_1(uniform: Option<UniformLocation>, x: Self);
    fn uniform_2(uniform: Option<UniformLocation>, x: Self, y: Self);
    fn uniform_3(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self);
    fn uniform_4(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self, w: Self);
}
impl SetUniform for i32 {
    fn uniform_1(uniform: Option<UniformLocation>, x: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform1i(uniform.into(), x);
            }
        }
    }
    fn uniform_2(uniform: Option<UniformLocation>, x: Self, y: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform2i(uniform.into(), x, y);
            }
        }
    }
    fn uniform_3(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform3i(uniform.into(), x, y, z);
            }
        }
    }
    fn uniform_4(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self, w: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform4i(uniform.into(), x, y, z, w);
            }
        }
    }
}
impl SetUniform for u32 {
    fn uniform_1(uniform: Option<UniformLocation>, x: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform1ui(uniform.into(), x);
            }
        }
    }
    fn uniform_2(uniform: Option<UniformLocation>, x: Self, y: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform2ui(uniform.into(), x, y);
            }
        }
    }
    fn uniform_3(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform3ui(uniform.into(), x, y, z);
            }
        }
    }
    fn uniform_4(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self, w: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform4ui(uniform.into(), x, y, z, w);
            }
        }
    }
}
impl SetUniform for f32 {
    fn uniform_1(uniform: Option<UniformLocation>, x: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform1f(uniform.into(), x);
            }
        }
    }
    fn uniform_2(uniform: Option<UniformLocation>, x: Self, y: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform2f(uniform.into(), x, y);
            }
        }
    }
    fn uniform_3(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform3f(uniform.into(), x, y, z);
            }
        }
    }
    fn uniform_4(uniform: Option<UniformLocation>, x: Self, y: Self, z: Self, w: Self) {
        unsafe {
            if let Some(uniform) = uniform {
                gl::Uniform4f(uniform.into(), x, y, z, w);
            }
        }
    }
}

pub trait SetTextureParameter {
    fn tex_parameter(target: TextureKind, pname: TextureParameter, param: Self);
}
impl SetTextureParameter for i32 {
    fn tex_parameter(target: TextureKind, pname: TextureParameter, param: Self) {
        unsafe {
            gl::TexParameteri(target as u32, pname as u32, param);
        }
    }
}
impl SetTextureParameter for f32 {
    fn tex_parameter(target: TextureKind, pname: TextureParameter, param: Self) {
        unsafe {
            gl::TexParameterf(target as u32, pname as u32, param);
        }
    }
}

pub trait HasNumberEnum {
    fn get() -> NumberEnum;
}
impl HasNumberEnum for i8 {
    fn get() -> NumberEnum {
        NumberEnum::Byte
    }
}
impl HasNumberEnum for u8 {
    fn get() -> NumberEnum {
        NumberEnum::UnsignedByte
    }
}
impl HasNumberEnum for i16 {
    fn get() -> NumberEnum {
        NumberEnum::Short
    }
}
impl HasNumberEnum for u16 {
    fn get() -> NumberEnum {
        NumberEnum::UnsignedShort
    }
}
impl HasNumberEnum for i32 {
    fn get() -> NumberEnum {
        NumberEnum::Int
    }
}
impl HasNumberEnum for u32 {
    fn get() -> NumberEnum {
        NumberEnum::UnsignedInt
    }
}
impl HasNumberEnum for f32 {
    fn get() -> NumberEnum {
        NumberEnum::Float
    }
}
impl HasNumberEnum for f64 {
    fn get() -> NumberEnum {
        NumberEnum::Double
    }
}

/// Wrapper struct around an OpenGL context.
/// 
/// This struct is currently a huge lie, because glad2 Rust does not yet support multicontext. (see: https://github.com/Dav1dde/glad/issues/254)
/// 
/// TODO - This should be a newtype over `GladGLContext`.
pub struct Gl;

impl Gl {
    pub fn load(loadfn: impl FnMut(&'static str) -> *const std::ffi::c_void) -> Self {
        gl::load(loadfn);

        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        Self
    }

    pub fn bind_buffer(&self, target: BufferKind, buffer: Option<VBO>) {
        unsafe {
            gl::BindBuffer(
                target as u32,
                if let Some(buffer) = buffer {
                    buffer.into()
                } else {
                    0
                },
            );
        }
    }

    pub fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }

    pub fn buffer_data<T: Sized>(&self, target: BufferKind, data: &[T], usage: UsageMode) {
        unsafe {
            gl::BufferData(
                target as u32,
                (std::mem::size_of::<f32>() * data.len()) as isize,
                data as *const _ as *const std::ffi::c_void,
                usage as u32,
            );
        }
    }

    pub fn vertex_attrib_pointer<T: HasNumberEnum>(
        &self,
        index: u32,
        size: u32,
        normalized: bool,
        stride: u32,
        offset: u32,
    ) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                size as i32,
                T::get() as u32,
                normalized as u8,
                stride as i32 * std::mem::size_of::<T>() as i32,
                (offset as usize * std::mem::size_of::<T>()) as *const std::ffi::c_void,
            );
        }
    }

    pub fn use_program(&self, program: Option<Program>) {
        unsafe {
            gl::UseProgram(if let Some(program) = program {
                program.into()
            } else {
                0
            });
        }
    }

    pub fn uniform_1<T: SetUniform>(&self, uniform: Option<UniformLocation>, x: T) {
        T::uniform_1(uniform, x);
    }

    pub fn uniform_2<T: SetUniform>(&self, uniform: Option<UniformLocation>, x: T, y: T) {
        T::uniform_2(uniform, x, y);
    }

    pub fn uniform_3<T: SetUniform>(&self, uniform: Option<UniformLocation>, x: T, y: T, z: T) {
        T::uniform_3(uniform, x, y, z);
    }

    pub fn uniform_4<T: SetUniform>(
        &self,
        uniform: Option<UniformLocation>,
        x: T,
        y: T,
        z: T,
        w: T,
    ) {
        T::uniform_4(uniform, x, y, z, w);
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear(&self, mask: BufferBit) {
        unsafe {
            gl::Clear(mask.into());
        }
    }

    pub fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(mode as u32, first, count);
        }
    }

    pub fn bind_texture(&self, kind: TextureKind, texture: Option<Texture>) {
        unsafe {
            gl::BindTexture(
                kind as u32,
                if let Some(texture) = texture {
                    texture.into()
                } else {
                    0
                },
            );
        }
    }

    pub fn tex_parameter<T: SetTextureParameter>(
        &self,
        target: TextureKind,
        pname: TextureParameter,
        param: T,
    ) {
        T::tex_parameter(target, pname, param);
    }

    pub fn tex_image_2d(
        &self,
        target: TextureKind,
        level: u32,
        format: TextureFormat,
        width: u32,
        height: u32,
        data: &[u8],
    ) {
        unsafe {
            gl::TexImage2D(
                target as u32,
                level as i32,
                format as i32,
                width as i32,
                height as i32,
                0,
                format as u32,
                gl::GL_UNSIGNED_BYTE,
                data as *const _ as *const std::ffi::c_void,
            );
        }
    }

    pub fn generate_mipmap(&self, target: TextureKind) {
        unsafe {
            gl::GenerateMipmap(target as u32);
        }
    }
}
