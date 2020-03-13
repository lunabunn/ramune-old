//! A module containing OpenGL 3.2 Core Profile bindings.
//! These bindings were automatically generated using Glad 2 (Dav1dde/glad).

#![allow(dead_code)]

pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw;

pub struct FnPtr {
    ptr: *const raw::c_void,
    is_loaded: bool
}

impl FnPtr {
    pub fn empty() -> FnPtr {
        FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false }
    }

    pub fn load<F>(&mut self, loadfn: &mut F, name: &'static str) where F: FnMut(&'static str) -> *const raw::c_void {
        let loaded = loadfn(name);
        if !loaded.is_null() {
            self.ptr = loaded;
            self.is_loaded = true;
        } else {
            self.ptr = FnPtr::not_initialized as *const raw::c_void;
            self.is_loaded = false;
        };
    }

    pub fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            self.ptr = other.ptr;
            self.is_loaded = other.is_loaded;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! { panic!("gl: function not initialized") }
}

pub mod types {
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use std::os::raw;

pub type GLvoid = raw::c_void;

pub type GLbyte = raw::c_char;
pub type GLubyte = raw::c_uchar;
pub type GLchar = raw::c_char;
pub type GLboolean = raw::c_uchar;

pub type GLshort = raw::c_short;
pub type GLushort = raw::c_ushort;

pub type GLint = raw::c_int;
pub type GLuint = raw::c_uint;
pub type GLint64 = i64;
pub type GLuint64 = u64;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub type GLsizei = GLint;
pub type GLclampx = raw::c_int;
pub type GLfixed = GLint;
pub type GLhalf = raw::c_ushort;
pub type GLhalfNV = raw::c_ushort;
pub type GLhalfARB = raw::c_ushort;

pub type GLenum = raw::c_uint;
pub type GLbitfield = raw::c_uint;

pub type GLfloat = raw::c_float;
pub type GLdouble = raw::c_double;
pub type GLclampf = raw::c_float;
pub type GLclampd = raw::c_double;

pub type GLcharARB = raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = raw::c_uint;

pub enum __GLsync {}

pub type GLsync = *const __GLsync;

pub enum _cl_context {}

pub enum _cl_event {}

pub type GLvdpauSurfaceNV = GLintptr;
pub type GLeglClientBufferEXT = *const raw::c_void;
pub type GLeglImageOES = *const raw::c_void;


pub type GLDEBUGPROC = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLDEBUGPROCAMD = extern "system" fn (
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLVULKANPROCNV = extern "system" fn ();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std;
    use super::types::*;

    pub const GL_ACTIVE_ATTRIBUTES: std::os::raw::c_uint = 0x8B89;
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: std::os::raw::c_uint = 0x8B8A;
    pub const GL_ACTIVE_TEXTURE: std::os::raw::c_uint = 0x84E0;
    pub const GL_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8B86;
    pub const GL_ACTIVE_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A36;
    pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: std::os::raw::c_uint = 0x8A35;
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: std::os::raw::c_uint = 0x8B87;
    pub const GL_ALIASED_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x846E;
    pub const GL_ALPHA: std::os::raw::c_uint = 0x1906;
    pub const GL_ALREADY_SIGNALED: std::os::raw::c_uint = 0x911A;
    pub const GL_ALWAYS: std::os::raw::c_uint = 0x0207;
    pub const GL_AND: std::os::raw::c_uint = 0x1501;
    pub const GL_AND_INVERTED: std::os::raw::c_uint = 0x1504;
    pub const GL_AND_REVERSE: std::os::raw::c_uint = 0x1502;
    pub const GL_ARRAY_BUFFER: std::os::raw::c_uint = 0x8892;
    pub const GL_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8894;
    pub const GL_ATTACHED_SHADERS: std::os::raw::c_uint = 0x8B85;
    pub const GL_BACK: std::os::raw::c_uint = 0x0405;
    pub const GL_BACK_LEFT: std::os::raw::c_uint = 0x0402;
    pub const GL_BACK_RIGHT: std::os::raw::c_uint = 0x0403;
    pub const GL_BGR: std::os::raw::c_uint = 0x80E0;
    pub const GL_BGRA: std::os::raw::c_uint = 0x80E1;
    pub const GL_BGRA_INTEGER: std::os::raw::c_uint = 0x8D9B;
    pub const GL_BGR_INTEGER: std::os::raw::c_uint = 0x8D9A;
    pub const GL_BLEND: std::os::raw::c_uint = 0x0BE2;
    pub const GL_BLEND_COLOR: std::os::raw::c_uint = 0x8005;
    pub const GL_BLEND_DST: std::os::raw::c_uint = 0x0BE0;
    pub const GL_BLEND_DST_ALPHA: std::os::raw::c_uint = 0x80CA;
    pub const GL_BLEND_DST_RGB: std::os::raw::c_uint = 0x80C8;
    pub const GL_BLEND_EQUATION: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_EQUATION_ALPHA: std::os::raw::c_uint = 0x883D;
    pub const GL_BLEND_EQUATION_RGB: std::os::raw::c_uint = 0x8009;
    pub const GL_BLEND_SRC: std::os::raw::c_uint = 0x0BE1;
    pub const GL_BLEND_SRC_ALPHA: std::os::raw::c_uint = 0x80CB;
    pub const GL_BLEND_SRC_RGB: std::os::raw::c_uint = 0x80C9;
    pub const GL_BLUE: std::os::raw::c_uint = 0x1905;
    pub const GL_BLUE_INTEGER: std::os::raw::c_uint = 0x8D96;
    pub const GL_BOOL: std::os::raw::c_uint = 0x8B56;
    pub const GL_BOOL_VEC2: std::os::raw::c_uint = 0x8B57;
    pub const GL_BOOL_VEC3: std::os::raw::c_uint = 0x8B58;
    pub const GL_BOOL_VEC4: std::os::raw::c_uint = 0x8B59;
    pub const GL_BUFFER_ACCESS: std::os::raw::c_uint = 0x88BB;
    pub const GL_BUFFER_ACCESS_FLAGS: std::os::raw::c_uint = 0x911F;
    pub const GL_BUFFER_MAPPED: std::os::raw::c_uint = 0x88BC;
    pub const GL_BUFFER_MAP_LENGTH: std::os::raw::c_uint = 0x9120;
    pub const GL_BUFFER_MAP_OFFSET: std::os::raw::c_uint = 0x9121;
    pub const GL_BUFFER_MAP_POINTER: std::os::raw::c_uint = 0x88BD;
    pub const GL_BUFFER_SIZE: std::os::raw::c_uint = 0x8764;
    pub const GL_BUFFER_USAGE: std::os::raw::c_uint = 0x8765;
    pub const GL_BYTE: std::os::raw::c_uint = 0x1400;
    pub const GL_CCW: std::os::raw::c_uint = 0x0901;
    pub const GL_CLAMP_READ_COLOR: std::os::raw::c_uint = 0x891C;
    pub const GL_CLAMP_TO_BORDER: std::os::raw::c_uint = 0x812D;
    pub const GL_CLAMP_TO_EDGE: std::os::raw::c_uint = 0x812F;
    pub const GL_CLEAR: std::os::raw::c_uint = 0x1500;
    pub const GL_CLIP_DISTANCE0: std::os::raw::c_uint = 0x3000;
    pub const GL_CLIP_DISTANCE1: std::os::raw::c_uint = 0x3001;
    pub const GL_CLIP_DISTANCE2: std::os::raw::c_uint = 0x3002;
    pub const GL_CLIP_DISTANCE3: std::os::raw::c_uint = 0x3003;
    pub const GL_CLIP_DISTANCE4: std::os::raw::c_uint = 0x3004;
    pub const GL_CLIP_DISTANCE5: std::os::raw::c_uint = 0x3005;
    pub const GL_CLIP_DISTANCE6: std::os::raw::c_uint = 0x3006;
    pub const GL_CLIP_DISTANCE7: std::os::raw::c_uint = 0x3007;
    pub const GL_COLOR: std::os::raw::c_uint = 0x1800;
    pub const GL_COLOR_ATTACHMENT0: std::os::raw::c_uint = 0x8CE0;
    pub const GL_COLOR_ATTACHMENT1: std::os::raw::c_uint = 0x8CE1;
    pub const GL_COLOR_ATTACHMENT10: std::os::raw::c_uint = 0x8CEA;
    pub const GL_COLOR_ATTACHMENT11: std::os::raw::c_uint = 0x8CEB;
    pub const GL_COLOR_ATTACHMENT12: std::os::raw::c_uint = 0x8CEC;
    pub const GL_COLOR_ATTACHMENT13: std::os::raw::c_uint = 0x8CED;
    pub const GL_COLOR_ATTACHMENT14: std::os::raw::c_uint = 0x8CEE;
    pub const GL_COLOR_ATTACHMENT15: std::os::raw::c_uint = 0x8CEF;
    pub const GL_COLOR_ATTACHMENT16: std::os::raw::c_uint = 0x8CF0;
    pub const GL_COLOR_ATTACHMENT17: std::os::raw::c_uint = 0x8CF1;
    pub const GL_COLOR_ATTACHMENT18: std::os::raw::c_uint = 0x8CF2;
    pub const GL_COLOR_ATTACHMENT19: std::os::raw::c_uint = 0x8CF3;
    pub const GL_COLOR_ATTACHMENT2: std::os::raw::c_uint = 0x8CE2;
    pub const GL_COLOR_ATTACHMENT20: std::os::raw::c_uint = 0x8CF4;
    pub const GL_COLOR_ATTACHMENT21: std::os::raw::c_uint = 0x8CF5;
    pub const GL_COLOR_ATTACHMENT22: std::os::raw::c_uint = 0x8CF6;
    pub const GL_COLOR_ATTACHMENT23: std::os::raw::c_uint = 0x8CF7;
    pub const GL_COLOR_ATTACHMENT24: std::os::raw::c_uint = 0x8CF8;
    pub const GL_COLOR_ATTACHMENT25: std::os::raw::c_uint = 0x8CF9;
    pub const GL_COLOR_ATTACHMENT26: std::os::raw::c_uint = 0x8CFA;
    pub const GL_COLOR_ATTACHMENT27: std::os::raw::c_uint = 0x8CFB;
    pub const GL_COLOR_ATTACHMENT28: std::os::raw::c_uint = 0x8CFC;
    pub const GL_COLOR_ATTACHMENT29: std::os::raw::c_uint = 0x8CFD;
    pub const GL_COLOR_ATTACHMENT3: std::os::raw::c_uint = 0x8CE3;
    pub const GL_COLOR_ATTACHMENT30: std::os::raw::c_uint = 0x8CFE;
    pub const GL_COLOR_ATTACHMENT31: std::os::raw::c_uint = 0x8CFF;
    pub const GL_COLOR_ATTACHMENT4: std::os::raw::c_uint = 0x8CE4;
    pub const GL_COLOR_ATTACHMENT5: std::os::raw::c_uint = 0x8CE5;
    pub const GL_COLOR_ATTACHMENT6: std::os::raw::c_uint = 0x8CE6;
    pub const GL_COLOR_ATTACHMENT7: std::os::raw::c_uint = 0x8CE7;
    pub const GL_COLOR_ATTACHMENT8: std::os::raw::c_uint = 0x8CE8;
    pub const GL_COLOR_ATTACHMENT9: std::os::raw::c_uint = 0x8CE9;
    pub const GL_COLOR_BUFFER_BIT: std::os::raw::c_uint = 0x00004000;
    pub const GL_COLOR_CLEAR_VALUE: std::os::raw::c_uint = 0x0C22;
    pub const GL_COLOR_LOGIC_OP: std::os::raw::c_uint = 0x0BF2;
    pub const GL_COLOR_WRITEMASK: std::os::raw::c_uint = 0x0C23;
    pub const GL_COMPARE_REF_TO_TEXTURE: std::os::raw::c_uint = 0x884E;
    pub const GL_COMPILE_STATUS: std::os::raw::c_uint = 0x8B81;
    pub const GL_COMPRESSED_RED: std::os::raw::c_uint = 0x8225;
    pub const GL_COMPRESSED_RED_RGTC1: std::os::raw::c_uint = 0x8DBB;
    pub const GL_COMPRESSED_RG: std::os::raw::c_uint = 0x8226;
    pub const GL_COMPRESSED_RGB: std::os::raw::c_uint = 0x84ED;
    pub const GL_COMPRESSED_RGBA: std::os::raw::c_uint = 0x84EE;
    pub const GL_COMPRESSED_RG_RGTC2: std::os::raw::c_uint = 0x8DBD;
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1: std::os::raw::c_uint = 0x8DBC;
    pub const GL_COMPRESSED_SIGNED_RG_RGTC2: std::os::raw::c_uint = 0x8DBE;
    pub const GL_COMPRESSED_SRGB: std::os::raw::c_uint = 0x8C48;
    pub const GL_COMPRESSED_SRGB_ALPHA: std::os::raw::c_uint = 0x8C49;
    pub const GL_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A3;
    pub const GL_CONDITION_SATISFIED: std::os::raw::c_uint = 0x911C;
    pub const GL_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8003;
    pub const GL_CONSTANT_COLOR: std::os::raw::c_uint = 0x8001;
    pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_CONTEXT_CORE_PROFILE_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_CONTEXT_FLAGS: std::os::raw::c_uint = 0x821E;
    pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_CONTEXT_PROFILE_MASK: std::os::raw::c_uint = 0x9126;
    pub const GL_COPY: std::os::raw::c_uint = 0x1503;
    pub const GL_COPY_INVERTED: std::os::raw::c_uint = 0x150C;
    pub const GL_COPY_READ_BUFFER: std::os::raw::c_uint = 0x8F36;
    pub const GL_COPY_WRITE_BUFFER: std::os::raw::c_uint = 0x8F37;
    pub const GL_CULL_FACE: std::os::raw::c_uint = 0x0B44;
    pub const GL_CULL_FACE_MODE: std::os::raw::c_uint = 0x0B45;
    pub const GL_CURRENT_PROGRAM: std::os::raw::c_uint = 0x8B8D;
    pub const GL_CURRENT_QUERY: std::os::raw::c_uint = 0x8865;
    pub const GL_CURRENT_VERTEX_ATTRIB: std::os::raw::c_uint = 0x8626;
    pub const GL_CW: std::os::raw::c_uint = 0x0900;
    pub const GL_DECR: std::os::raw::c_uint = 0x1E03;
    pub const GL_DECR_WRAP: std::os::raw::c_uint = 0x8508;
    pub const GL_DELETE_STATUS: std::os::raw::c_uint = 0x8B80;
    pub const GL_DEPTH: std::os::raw::c_uint = 0x1801;
    pub const GL_DEPTH24_STENCIL8: std::os::raw::c_uint = 0x88F0;
    pub const GL_DEPTH32F_STENCIL8: std::os::raw::c_uint = 0x8CAD;
    pub const GL_DEPTH_ATTACHMENT: std::os::raw::c_uint = 0x8D00;
    pub const GL_DEPTH_BUFFER_BIT: std::os::raw::c_uint = 0x00000100;
    pub const GL_DEPTH_CLAMP: std::os::raw::c_uint = 0x864F;
    pub const GL_DEPTH_CLEAR_VALUE: std::os::raw::c_uint = 0x0B73;
    pub const GL_DEPTH_COMPONENT: std::os::raw::c_uint = 0x1902;
    pub const GL_DEPTH_COMPONENT16: std::os::raw::c_uint = 0x81A5;
    pub const GL_DEPTH_COMPONENT24: std::os::raw::c_uint = 0x81A6;
    pub const GL_DEPTH_COMPONENT32: std::os::raw::c_uint = 0x81A7;
    pub const GL_DEPTH_COMPONENT32F: std::os::raw::c_uint = 0x8CAC;
    pub const GL_DEPTH_FUNC: std::os::raw::c_uint = 0x0B74;
    pub const GL_DEPTH_RANGE: std::os::raw::c_uint = 0x0B70;
    pub const GL_DEPTH_STENCIL: std::os::raw::c_uint = 0x84F9;
    pub const GL_DEPTH_STENCIL_ATTACHMENT: std::os::raw::c_uint = 0x821A;
    pub const GL_DEPTH_TEST: std::os::raw::c_uint = 0x0B71;
    pub const GL_DEPTH_WRITEMASK: std::os::raw::c_uint = 0x0B72;
    pub const GL_DITHER: std::os::raw::c_uint = 0x0BD0;
    pub const GL_DONT_CARE: std::os::raw::c_uint = 0x1100;
    pub const GL_DOUBLE: std::os::raw::c_uint = 0x140A;
    pub const GL_DOUBLEBUFFER: std::os::raw::c_uint = 0x0C32;
    pub const GL_DRAW_BUFFER: std::os::raw::c_uint = 0x0C01;
    pub const GL_DRAW_BUFFER0: std::os::raw::c_uint = 0x8825;
    pub const GL_DRAW_BUFFER1: std::os::raw::c_uint = 0x8826;
    pub const GL_DRAW_BUFFER10: std::os::raw::c_uint = 0x882F;
    pub const GL_DRAW_BUFFER11: std::os::raw::c_uint = 0x8830;
    pub const GL_DRAW_BUFFER12: std::os::raw::c_uint = 0x8831;
    pub const GL_DRAW_BUFFER13: std::os::raw::c_uint = 0x8832;
    pub const GL_DRAW_BUFFER14: std::os::raw::c_uint = 0x8833;
    pub const GL_DRAW_BUFFER15: std::os::raw::c_uint = 0x8834;
    pub const GL_DRAW_BUFFER2: std::os::raw::c_uint = 0x8827;
    pub const GL_DRAW_BUFFER3: std::os::raw::c_uint = 0x8828;
    pub const GL_DRAW_BUFFER4: std::os::raw::c_uint = 0x8829;
    pub const GL_DRAW_BUFFER5: std::os::raw::c_uint = 0x882A;
    pub const GL_DRAW_BUFFER6: std::os::raw::c_uint = 0x882B;
    pub const GL_DRAW_BUFFER7: std::os::raw::c_uint = 0x882C;
    pub const GL_DRAW_BUFFER8: std::os::raw::c_uint = 0x882D;
    pub const GL_DRAW_BUFFER9: std::os::raw::c_uint = 0x882E;
    pub const GL_DRAW_FRAMEBUFFER: std::os::raw::c_uint = 0x8CA9;
    pub const GL_DRAW_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CA6;
    pub const GL_DST_ALPHA: std::os::raw::c_uint = 0x0304;
    pub const GL_DST_COLOR: std::os::raw::c_uint = 0x0306;
    pub const GL_DYNAMIC_COPY: std::os::raw::c_uint = 0x88EA;
    pub const GL_DYNAMIC_DRAW: std::os::raw::c_uint = 0x88E8;
    pub const GL_DYNAMIC_READ: std::os::raw::c_uint = 0x88E9;
    pub const GL_ELEMENT_ARRAY_BUFFER: std::os::raw::c_uint = 0x8893;
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x8895;
    pub const GL_EQUAL: std::os::raw::c_uint = 0x0202;
    pub const GL_EQUIV: std::os::raw::c_uint = 0x1509;
    pub const GL_EXTENSIONS: std::os::raw::c_uint = 0x1F03;
    pub const GL_FALSE: std::os::raw::c_uchar = 0;
    pub const GL_FASTEST: std::os::raw::c_uint = 0x1101;
    pub const GL_FILL: std::os::raw::c_uint = 0x1B02;
    pub const GL_FIRST_VERTEX_CONVENTION: std::os::raw::c_uint = 0x8E4D;
    pub const GL_FIXED_ONLY: std::os::raw::c_uint = 0x891D;
    pub const GL_FLOAT: std::os::raw::c_uint = 0x1406;
    pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: std::os::raw::c_uint = 0x8DAD;
    pub const GL_FLOAT_MAT2: std::os::raw::c_uint = 0x8B5A;
    pub const GL_FLOAT_MAT2x3: std::os::raw::c_uint = 0x8B65;
    pub const GL_FLOAT_MAT2x4: std::os::raw::c_uint = 0x8B66;
    pub const GL_FLOAT_MAT3: std::os::raw::c_uint = 0x8B5B;
    pub const GL_FLOAT_MAT3x2: std::os::raw::c_uint = 0x8B67;
    pub const GL_FLOAT_MAT3x4: std::os::raw::c_uint = 0x8B68;
    pub const GL_FLOAT_MAT4: std::os::raw::c_uint = 0x8B5C;
    pub const GL_FLOAT_MAT4x2: std::os::raw::c_uint = 0x8B69;
    pub const GL_FLOAT_MAT4x3: std::os::raw::c_uint = 0x8B6A;
    pub const GL_FLOAT_VEC2: std::os::raw::c_uint = 0x8B50;
    pub const GL_FLOAT_VEC3: std::os::raw::c_uint = 0x8B51;
    pub const GL_FLOAT_VEC4: std::os::raw::c_uint = 0x8B52;
    pub const GL_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8B30;
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: std::os::raw::c_uint = 0x8B8B;
    pub const GL_FRAMEBUFFER: std::os::raw::c_uint = 0x8D40;
    pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: std::os::raw::c_uint = 0x8215;
    pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: std::os::raw::c_uint = 0x8214;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: std::os::raw::c_uint = 0x8210;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: std::os::raw::c_uint = 0x8211;
    pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: std::os::raw::c_uint = 0x8216;
    pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: std::os::raw::c_uint = 0x8213;
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: std::os::raw::c_uint = 0x8DA7;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: std::os::raw::c_uint = 0x8CD1;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: std::os::raw::c_uint = 0x8CD0;
    pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: std::os::raw::c_uint = 0x8212;
    pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: std::os::raw::c_uint = 0x8217;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: std::os::raw::c_uint = 0x8CD3;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: std::os::raw::c_uint = 0x8CD4;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: std::os::raw::c_uint = 0x8CD2;
    pub const GL_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CA6;
    pub const GL_FRAMEBUFFER_COMPLETE: std::os::raw::c_uint = 0x8CD5;
    pub const GL_FRAMEBUFFER_DEFAULT: std::os::raw::c_uint = 0x8218;
    pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: std::os::raw::c_uint = 0x8CD6;
    pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: std::os::raw::c_uint = 0x8CDB;
    pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: std::os::raw::c_uint = 0x8DA8;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: std::os::raw::c_uint = 0x8CD7;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: std::os::raw::c_uint = 0x8D56;
    pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: std::os::raw::c_uint = 0x8CDC;
    pub const GL_FRAMEBUFFER_SRGB: std::os::raw::c_uint = 0x8DB9;
    pub const GL_FRAMEBUFFER_UNDEFINED: std::os::raw::c_uint = 0x8219;
    pub const GL_FRAMEBUFFER_UNSUPPORTED: std::os::raw::c_uint = 0x8CDD;
    pub const GL_FRONT: std::os::raw::c_uint = 0x0404;
    pub const GL_FRONT_AND_BACK: std::os::raw::c_uint = 0x0408;
    pub const GL_FRONT_FACE: std::os::raw::c_uint = 0x0B46;
    pub const GL_FRONT_LEFT: std::os::raw::c_uint = 0x0400;
    pub const GL_FRONT_RIGHT: std::os::raw::c_uint = 0x0401;
    pub const GL_FUNC_ADD: std::os::raw::c_uint = 0x8006;
    pub const GL_FUNC_REVERSE_SUBTRACT: std::os::raw::c_uint = 0x800B;
    pub const GL_FUNC_SUBTRACT: std::os::raw::c_uint = 0x800A;
    pub const GL_GEOMETRY_INPUT_TYPE: std::os::raw::c_uint = 0x8917;
    pub const GL_GEOMETRY_OUTPUT_TYPE: std::os::raw::c_uint = 0x8918;
    pub const GL_GEOMETRY_SHADER: std::os::raw::c_uint = 0x8DD9;
    pub const GL_GEOMETRY_VERTICES_OUT: std::os::raw::c_uint = 0x8916;
    pub const GL_GEQUAL: std::os::raw::c_uint = 0x0206;
    pub const GL_GREATER: std::os::raw::c_uint = 0x0204;
    pub const GL_GREEN: std::os::raw::c_uint = 0x1904;
    pub const GL_GREEN_INTEGER: std::os::raw::c_uint = 0x8D95;
    pub const GL_HALF_FLOAT: std::os::raw::c_uint = 0x140B;
    pub const GL_INCR: std::os::raw::c_uint = 0x1E02;
    pub const GL_INCR_WRAP: std::os::raw::c_uint = 0x8507;
    pub const GL_INFO_LOG_LENGTH: std::os::raw::c_uint = 0x8B84;
    pub const GL_INT: std::os::raw::c_uint = 0x1404;
    pub const GL_INTERLEAVED_ATTRIBS: std::os::raw::c_uint = 0x8C8C;
    pub const GL_INT_SAMPLER_1D: std::os::raw::c_uint = 0x8DC9;
    pub const GL_INT_SAMPLER_1D_ARRAY: std::os::raw::c_uint = 0x8DCE;
    pub const GL_INT_SAMPLER_2D: std::os::raw::c_uint = 0x8DCA;
    pub const GL_INT_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DCF;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9109;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x910C;
    pub const GL_INT_SAMPLER_2D_RECT: std::os::raw::c_uint = 0x8DCD;
    pub const GL_INT_SAMPLER_3D: std::os::raw::c_uint = 0x8DCB;
    pub const GL_INT_SAMPLER_BUFFER: std::os::raw::c_uint = 0x8DD0;
    pub const GL_INT_SAMPLER_CUBE: std::os::raw::c_uint = 0x8DCC;
    pub const GL_INT_VEC2: std::os::raw::c_uint = 0x8B53;
    pub const GL_INT_VEC3: std::os::raw::c_uint = 0x8B54;
    pub const GL_INT_VEC4: std::os::raw::c_uint = 0x8B55;
    pub const GL_INVALID_ENUM: std::os::raw::c_uint = 0x0500;
    pub const GL_INVALID_FRAMEBUFFER_OPERATION: std::os::raw::c_uint = 0x0506;
    pub const GL_INVALID_INDEX: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_INVALID_OPERATION: std::os::raw::c_uint = 0x0502;
    pub const GL_INVALID_VALUE: std::os::raw::c_uint = 0x0501;
    pub const GL_INVERT: std::os::raw::c_uint = 0x150A;
    pub const GL_KEEP: std::os::raw::c_uint = 0x1E00;
    pub const GL_LAST_VERTEX_CONVENTION: std::os::raw::c_uint = 0x8E4E;
    pub const GL_LEFT: std::os::raw::c_uint = 0x0406;
    pub const GL_LEQUAL: std::os::raw::c_uint = 0x0203;
    pub const GL_LESS: std::os::raw::c_uint = 0x0201;
    pub const GL_LINE: std::os::raw::c_uint = 0x1B01;
    pub const GL_LINEAR: std::os::raw::c_uint = 0x2601;
    pub const GL_LINEAR_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2703;
    pub const GL_LINEAR_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2701;
    pub const GL_LINES: std::os::raw::c_uint = 0x0001;
    pub const GL_LINES_ADJACENCY: std::os::raw::c_uint = 0x000A;
    pub const GL_LINE_LOOP: std::os::raw::c_uint = 0x0002;
    pub const GL_LINE_SMOOTH: std::os::raw::c_uint = 0x0B20;
    pub const GL_LINE_SMOOTH_HINT: std::os::raw::c_uint = 0x0C52;
    pub const GL_LINE_STRIP: std::os::raw::c_uint = 0x0003;
    pub const GL_LINE_STRIP_ADJACENCY: std::os::raw::c_uint = 0x000B;
    pub const GL_LINE_WIDTH: std::os::raw::c_uint = 0x0B21;
    pub const GL_LINE_WIDTH_GRANULARITY: std::os::raw::c_uint = 0x0B23;
    pub const GL_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x0B22;
    pub const GL_LINK_STATUS: std::os::raw::c_uint = 0x8B82;
    pub const GL_LOGIC_OP_MODE: std::os::raw::c_uint = 0x0BF0;
    pub const GL_LOWER_LEFT: std::os::raw::c_uint = 0x8CA1;
    pub const GL_MAJOR_VERSION: std::os::raw::c_uint = 0x821B;
    pub const GL_MAP_FLUSH_EXPLICIT_BIT: std::os::raw::c_uint = 0x0010;
    pub const GL_MAP_INVALIDATE_BUFFER_BIT: std::os::raw::c_uint = 0x0008;
    pub const GL_MAP_INVALIDATE_RANGE_BIT: std::os::raw::c_uint = 0x0004;
    pub const GL_MAP_READ_BIT: std::os::raw::c_uint = 0x0001;
    pub const GL_MAP_UNSYNCHRONIZED_BIT: std::os::raw::c_uint = 0x0020;
    pub const GL_MAP_WRITE_BIT: std::os::raw::c_uint = 0x0002;
    pub const GL_MAX: std::os::raw::c_uint = 0x8008;
    pub const GL_MAX_3D_TEXTURE_SIZE: std::os::raw::c_uint = 0x8073;
    pub const GL_MAX_ARRAY_TEXTURE_LAYERS: std::os::raw::c_uint = 0x88FF;
    pub const GL_MAX_CLIP_DISTANCES: std::os::raw::c_uint = 0x0D32;
    pub const GL_MAX_COLOR_ATTACHMENTS: std::os::raw::c_uint = 0x8CDF;
    pub const GL_MAX_COLOR_TEXTURE_SAMPLES: std::os::raw::c_uint = 0x910E;
    pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A33;
    pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A32;
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4D;
    pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2E;
    pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8A31;
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: std::os::raw::c_uint = 0x851C;
    pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: std::os::raw::c_uint = 0x910F;
    pub const GL_MAX_DRAW_BUFFERS: std::os::raw::c_uint = 0x8824;
    pub const GL_MAX_ELEMENTS_INDICES: std::os::raw::c_uint = 0x80E9;
    pub const GL_MAX_ELEMENTS_VERTICES: std::os::raw::c_uint = 0x80E8;
    pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: std::os::raw::c_uint = 0x9125;
    pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2D;
    pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B49;
    pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: std::os::raw::c_uint = 0x9123;
    pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x9124;
    pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: std::os::raw::c_uint = 0x8DE0;
    pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8C29;
    pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x8DE1;
    pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2C;
    pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8DDF;
    pub const GL_MAX_INTEGER_SAMPLES: std::os::raw::c_uint = 0x9110;
    pub const GL_MAX_PROGRAM_TEXEL_OFFSET: std::os::raw::c_uint = 0x8905;
    pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: std::os::raw::c_uint = 0x84F8;
    pub const GL_MAX_RENDERBUFFER_SIZE: std::os::raw::c_uint = 0x84E8;
    pub const GL_MAX_SAMPLES: std::os::raw::c_uint = 0x8D57;
    pub const GL_MAX_SAMPLE_MASK_WORDS: std::os::raw::c_uint = 0x8E59;
    pub const GL_MAX_SERVER_WAIT_TIMEOUT: std::os::raw::c_uint = 0x9111;
    pub const GL_MAX_TEXTURE_BUFFER_SIZE: std::os::raw::c_uint = 0x8C2B;
    pub const GL_MAX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8872;
    pub const GL_MAX_TEXTURE_LOD_BIAS: std::os::raw::c_uint = 0x84FD;
    pub const GL_MAX_TEXTURE_SIZE: std::os::raw::c_uint = 0x0D33;
    pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: std::os::raw::c_uint = 0x8C8A;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: std::os::raw::c_uint = 0x8C8B;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: std::os::raw::c_uint = 0x8C80;
    pub const GL_MAX_UNIFORM_BLOCK_SIZE: std::os::raw::c_uint = 0x8A30;
    pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: std::os::raw::c_uint = 0x8A2F;
    pub const GL_MAX_VARYING_COMPONENTS: std::os::raw::c_uint = 0x8B4B;
    pub const GL_MAX_VARYING_FLOATS: std::os::raw::c_uint = 0x8B4B;
    pub const GL_MAX_VERTEX_ATTRIBS: std::os::raw::c_uint = 0x8869;
    pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: std::os::raw::c_uint = 0x9122;
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: std::os::raw::c_uint = 0x8B4C;
    pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: std::os::raw::c_uint = 0x8A2B;
    pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: std::os::raw::c_uint = 0x8B4A;
    pub const GL_MAX_VIEWPORT_DIMS: std::os::raw::c_uint = 0x0D3A;
    pub const GL_MIN: std::os::raw::c_uint = 0x8007;
    pub const GL_MINOR_VERSION: std::os::raw::c_uint = 0x821C;
    pub const GL_MIN_PROGRAM_TEXEL_OFFSET: std::os::raw::c_uint = 0x8904;
    pub const GL_MIRRORED_REPEAT: std::os::raw::c_uint = 0x8370;
    pub const GL_MULTISAMPLE: std::os::raw::c_uint = 0x809D;
    pub const GL_NAND: std::os::raw::c_uint = 0x150E;
    pub const GL_NEAREST: std::os::raw::c_uint = 0x2600;
    pub const GL_NEAREST_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2702;
    pub const GL_NEAREST_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2700;
    pub const GL_NEVER: std::os::raw::c_uint = 0x0200;
    pub const GL_NICEST: std::os::raw::c_uint = 0x1102;
    pub const GL_NONE: std::os::raw::c_uint = 0;
    pub const GL_NOOP: std::os::raw::c_uint = 0x1505;
    pub const GL_NOR: std::os::raw::c_uint = 0x1508;
    pub const GL_NOTEQUAL: std::os::raw::c_uint = 0x0205;
    pub const GL_NO_ERROR: std::os::raw::c_uint = 0;
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: std::os::raw::c_uint = 0x86A2;
    pub const GL_NUM_EXTENSIONS: std::os::raw::c_uint = 0x821D;
    pub const GL_OBJECT_TYPE: std::os::raw::c_uint = 0x9112;
    pub const GL_ONE: std::os::raw::c_uint = 1;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: std::os::raw::c_uint = 0x8004;
    pub const GL_ONE_MINUS_CONSTANT_COLOR: std::os::raw::c_uint = 0x8002;
    pub const GL_ONE_MINUS_DST_ALPHA: std::os::raw::c_uint = 0x0305;
    pub const GL_ONE_MINUS_DST_COLOR: std::os::raw::c_uint = 0x0307;
    pub const GL_ONE_MINUS_SRC_ALPHA: std::os::raw::c_uint = 0x0303;
    pub const GL_ONE_MINUS_SRC_COLOR: std::os::raw::c_uint = 0x0301;
    pub const GL_OR: std::os::raw::c_uint = 0x1507;
    pub const GL_OR_INVERTED: std::os::raw::c_uint = 0x150D;
    pub const GL_OR_REVERSE: std::os::raw::c_uint = 0x150B;
    pub const GL_OUT_OF_MEMORY: std::os::raw::c_uint = 0x0505;
    pub const GL_PACK_ALIGNMENT: std::os::raw::c_uint = 0x0D05;
    pub const GL_PACK_IMAGE_HEIGHT: std::os::raw::c_uint = 0x806C;
    pub const GL_PACK_LSB_FIRST: std::os::raw::c_uint = 0x0D01;
    pub const GL_PACK_ROW_LENGTH: std::os::raw::c_uint = 0x0D02;
    pub const GL_PACK_SKIP_IMAGES: std::os::raw::c_uint = 0x806B;
    pub const GL_PACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0D04;
    pub const GL_PACK_SKIP_ROWS: std::os::raw::c_uint = 0x0D03;
    pub const GL_PACK_SWAP_BYTES: std::os::raw::c_uint = 0x0D00;
    pub const GL_PIXEL_PACK_BUFFER: std::os::raw::c_uint = 0x88EB;
    pub const GL_PIXEL_PACK_BUFFER_BINDING: std::os::raw::c_uint = 0x88ED;
    pub const GL_PIXEL_UNPACK_BUFFER: std::os::raw::c_uint = 0x88EC;
    pub const GL_PIXEL_UNPACK_BUFFER_BINDING: std::os::raw::c_uint = 0x88EF;
    pub const GL_POINT: std::os::raw::c_uint = 0x1B00;
    pub const GL_POINTS: std::os::raw::c_uint = 0x0000;
    pub const GL_POINT_FADE_THRESHOLD_SIZE: std::os::raw::c_uint = 0x8128;
    pub const GL_POINT_SIZE: std::os::raw::c_uint = 0x0B11;
    pub const GL_POINT_SIZE_GRANULARITY: std::os::raw::c_uint = 0x0B13;
    pub const GL_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x0B12;
    pub const GL_POINT_SPRITE_COORD_ORIGIN: std::os::raw::c_uint = 0x8CA0;
    pub const GL_POLYGON_MODE: std::os::raw::c_uint = 0x0B40;
    pub const GL_POLYGON_OFFSET_FACTOR: std::os::raw::c_uint = 0x8038;
    pub const GL_POLYGON_OFFSET_FILL: std::os::raw::c_uint = 0x8037;
    pub const GL_POLYGON_OFFSET_LINE: std::os::raw::c_uint = 0x2A02;
    pub const GL_POLYGON_OFFSET_POINT: std::os::raw::c_uint = 0x2A01;
    pub const GL_POLYGON_OFFSET_UNITS: std::os::raw::c_uint = 0x2A00;
    pub const GL_POLYGON_SMOOTH: std::os::raw::c_uint = 0x0B41;
    pub const GL_POLYGON_SMOOTH_HINT: std::os::raw::c_uint = 0x0C53;
    pub const GL_PRIMITIVES_GENERATED: std::os::raw::c_uint = 0x8C87;
    pub const GL_PRIMITIVE_RESTART: std::os::raw::c_uint = 0x8F9D;
    pub const GL_PRIMITIVE_RESTART_INDEX: std::os::raw::c_uint = 0x8F9E;
    pub const GL_PROGRAM_POINT_SIZE: std::os::raw::c_uint = 0x8642;
    pub const GL_PROVOKING_VERTEX: std::os::raw::c_uint = 0x8E4F;
    pub const GL_PROXY_TEXTURE_1D: std::os::raw::c_uint = 0x8063;
    pub const GL_PROXY_TEXTURE_1D_ARRAY: std::os::raw::c_uint = 0x8C19;
    pub const GL_PROXY_TEXTURE_2D: std::os::raw::c_uint = 0x8064;
    pub const GL_PROXY_TEXTURE_2D_ARRAY: std::os::raw::c_uint = 0x8C1B;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9101;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9103;
    pub const GL_PROXY_TEXTURE_3D: std::os::raw::c_uint = 0x8070;
    pub const GL_PROXY_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x851B;
    pub const GL_PROXY_TEXTURE_RECTANGLE: std::os::raw::c_uint = 0x84F7;
    pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: std::os::raw::c_uint = 0x8E4C;
    pub const GL_QUERY_BY_REGION_NO_WAIT: std::os::raw::c_uint = 0x8E16;
    pub const GL_QUERY_BY_REGION_WAIT: std::os::raw::c_uint = 0x8E15;
    pub const GL_QUERY_COUNTER_BITS: std::os::raw::c_uint = 0x8864;
    pub const GL_QUERY_NO_WAIT: std::os::raw::c_uint = 0x8E14;
    pub const GL_QUERY_RESULT: std::os::raw::c_uint = 0x8866;
    pub const GL_QUERY_RESULT_AVAILABLE: std::os::raw::c_uint = 0x8867;
    pub const GL_QUERY_WAIT: std::os::raw::c_uint = 0x8E13;
    pub const GL_R11F_G11F_B10F: std::os::raw::c_uint = 0x8C3A;
    pub const GL_R16: std::os::raw::c_uint = 0x822A;
    pub const GL_R16F: std::os::raw::c_uint = 0x822D;
    pub const GL_R16I: std::os::raw::c_uint = 0x8233;
    pub const GL_R16UI: std::os::raw::c_uint = 0x8234;
    pub const GL_R16_SNORM: std::os::raw::c_uint = 0x8F98;
    pub const GL_R32F: std::os::raw::c_uint = 0x822E;
    pub const GL_R32I: std::os::raw::c_uint = 0x8235;
    pub const GL_R32UI: std::os::raw::c_uint = 0x8236;
    pub const GL_R3_G3_B2: std::os::raw::c_uint = 0x2A10;
    pub const GL_R8: std::os::raw::c_uint = 0x8229;
    pub const GL_R8I: std::os::raw::c_uint = 0x8231;
    pub const GL_R8UI: std::os::raw::c_uint = 0x8232;
    pub const GL_R8_SNORM: std::os::raw::c_uint = 0x8F94;
    pub const GL_RASTERIZER_DISCARD: std::os::raw::c_uint = 0x8C89;
    pub const GL_READ_BUFFER: std::os::raw::c_uint = 0x0C02;
    pub const GL_READ_FRAMEBUFFER: std::os::raw::c_uint = 0x8CA8;
    pub const GL_READ_FRAMEBUFFER_BINDING: std::os::raw::c_uint = 0x8CAA;
    pub const GL_READ_ONLY: std::os::raw::c_uint = 0x88B8;
    pub const GL_READ_WRITE: std::os::raw::c_uint = 0x88BA;
    pub const GL_RED: std::os::raw::c_uint = 0x1903;
    pub const GL_RED_INTEGER: std::os::raw::c_uint = 0x8D94;
    pub const GL_RENDERBUFFER: std::os::raw::c_uint = 0x8D41;
    pub const GL_RENDERBUFFER_ALPHA_SIZE: std::os::raw::c_uint = 0x8D53;
    pub const GL_RENDERBUFFER_BINDING: std::os::raw::c_uint = 0x8CA7;
    pub const GL_RENDERBUFFER_BLUE_SIZE: std::os::raw::c_uint = 0x8D52;
    pub const GL_RENDERBUFFER_DEPTH_SIZE: std::os::raw::c_uint = 0x8D54;
    pub const GL_RENDERBUFFER_GREEN_SIZE: std::os::raw::c_uint = 0x8D51;
    pub const GL_RENDERBUFFER_HEIGHT: std::os::raw::c_uint = 0x8D43;
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT: std::os::raw::c_uint = 0x8D44;
    pub const GL_RENDERBUFFER_RED_SIZE: std::os::raw::c_uint = 0x8D50;
    pub const GL_RENDERBUFFER_SAMPLES: std::os::raw::c_uint = 0x8CAB;
    pub const GL_RENDERBUFFER_STENCIL_SIZE: std::os::raw::c_uint = 0x8D55;
    pub const GL_RENDERBUFFER_WIDTH: std::os::raw::c_uint = 0x8D42;
    pub const GL_RENDERER: std::os::raw::c_uint = 0x1F01;
    pub const GL_REPEAT: std::os::raw::c_uint = 0x2901;
    pub const GL_REPLACE: std::os::raw::c_uint = 0x1E01;
    pub const GL_RG: std::os::raw::c_uint = 0x8227;
    pub const GL_RG16: std::os::raw::c_uint = 0x822C;
    pub const GL_RG16F: std::os::raw::c_uint = 0x822F;
    pub const GL_RG16I: std::os::raw::c_uint = 0x8239;
    pub const GL_RG16UI: std::os::raw::c_uint = 0x823A;
    pub const GL_RG16_SNORM: std::os::raw::c_uint = 0x8F99;
    pub const GL_RG32F: std::os::raw::c_uint = 0x8230;
    pub const GL_RG32I: std::os::raw::c_uint = 0x823B;
    pub const GL_RG32UI: std::os::raw::c_uint = 0x823C;
    pub const GL_RG8: std::os::raw::c_uint = 0x822B;
    pub const GL_RG8I: std::os::raw::c_uint = 0x8237;
    pub const GL_RG8UI: std::os::raw::c_uint = 0x8238;
    pub const GL_RG8_SNORM: std::os::raw::c_uint = 0x8F95;
    pub const GL_RGB: std::os::raw::c_uint = 0x1907;
    pub const GL_RGB10: std::os::raw::c_uint = 0x8052;
    pub const GL_RGB10_A2: std::os::raw::c_uint = 0x8059;
    pub const GL_RGB12: std::os::raw::c_uint = 0x8053;
    pub const GL_RGB16: std::os::raw::c_uint = 0x8054;
    pub const GL_RGB16F: std::os::raw::c_uint = 0x881B;
    pub const GL_RGB16I: std::os::raw::c_uint = 0x8D89;
    pub const GL_RGB16UI: std::os::raw::c_uint = 0x8D77;
    pub const GL_RGB16_SNORM: std::os::raw::c_uint = 0x8F9A;
    pub const GL_RGB32F: std::os::raw::c_uint = 0x8815;
    pub const GL_RGB32I: std::os::raw::c_uint = 0x8D83;
    pub const GL_RGB32UI: std::os::raw::c_uint = 0x8D71;
    pub const GL_RGB4: std::os::raw::c_uint = 0x804F;
    pub const GL_RGB5: std::os::raw::c_uint = 0x8050;
    pub const GL_RGB5_A1: std::os::raw::c_uint = 0x8057;
    pub const GL_RGB8: std::os::raw::c_uint = 0x8051;
    pub const GL_RGB8I: std::os::raw::c_uint = 0x8D8F;
    pub const GL_RGB8UI: std::os::raw::c_uint = 0x8D7D;
    pub const GL_RGB8_SNORM: std::os::raw::c_uint = 0x8F96;
    pub const GL_RGB9_E5: std::os::raw::c_uint = 0x8C3D;
    pub const GL_RGBA: std::os::raw::c_uint = 0x1908;
    pub const GL_RGBA12: std::os::raw::c_uint = 0x805A;
    pub const GL_RGBA16: std::os::raw::c_uint = 0x805B;
    pub const GL_RGBA16F: std::os::raw::c_uint = 0x881A;
    pub const GL_RGBA16I: std::os::raw::c_uint = 0x8D88;
    pub const GL_RGBA16UI: std::os::raw::c_uint = 0x8D76;
    pub const GL_RGBA16_SNORM: std::os::raw::c_uint = 0x8F9B;
    pub const GL_RGBA2: std::os::raw::c_uint = 0x8055;
    pub const GL_RGBA32F: std::os::raw::c_uint = 0x8814;
    pub const GL_RGBA32I: std::os::raw::c_uint = 0x8D82;
    pub const GL_RGBA32UI: std::os::raw::c_uint = 0x8D70;
    pub const GL_RGBA4: std::os::raw::c_uint = 0x8056;
    pub const GL_RGBA8: std::os::raw::c_uint = 0x8058;
    pub const GL_RGBA8I: std::os::raw::c_uint = 0x8D8E;
    pub const GL_RGBA8UI: std::os::raw::c_uint = 0x8D7C;
    pub const GL_RGBA8_SNORM: std::os::raw::c_uint = 0x8F97;
    pub const GL_RGBA_INTEGER: std::os::raw::c_uint = 0x8D99;
    pub const GL_RGB_INTEGER: std::os::raw::c_uint = 0x8D98;
    pub const GL_RG_INTEGER: std::os::raw::c_uint = 0x8228;
    pub const GL_RIGHT: std::os::raw::c_uint = 0x0407;
    pub const GL_SAMPLER_1D: std::os::raw::c_uint = 0x8B5D;
    pub const GL_SAMPLER_1D_ARRAY: std::os::raw::c_uint = 0x8DC0;
    pub const GL_SAMPLER_1D_ARRAY_SHADOW: std::os::raw::c_uint = 0x8DC3;
    pub const GL_SAMPLER_1D_SHADOW: std::os::raw::c_uint = 0x8B61;
    pub const GL_SAMPLER_2D: std::os::raw::c_uint = 0x8B5E;
    pub const GL_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DC1;
    pub const GL_SAMPLER_2D_ARRAY_SHADOW: std::os::raw::c_uint = 0x8DC4;
    pub const GL_SAMPLER_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9108;
    pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x910B;
    pub const GL_SAMPLER_2D_RECT: std::os::raw::c_uint = 0x8B63;
    pub const GL_SAMPLER_2D_RECT_SHADOW: std::os::raw::c_uint = 0x8B64;
    pub const GL_SAMPLER_2D_SHADOW: std::os::raw::c_uint = 0x8B62;
    pub const GL_SAMPLER_3D: std::os::raw::c_uint = 0x8B5F;
    pub const GL_SAMPLER_BUFFER: std::os::raw::c_uint = 0x8DC2;
    pub const GL_SAMPLER_CUBE: std::os::raw::c_uint = 0x8B60;
    pub const GL_SAMPLER_CUBE_SHADOW: std::os::raw::c_uint = 0x8DC5;
    pub const GL_SAMPLES: std::os::raw::c_uint = 0x80A9;
    pub const GL_SAMPLES_PASSED: std::os::raw::c_uint = 0x8914;
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE: std::os::raw::c_uint = 0x809E;
    pub const GL_SAMPLE_ALPHA_TO_ONE: std::os::raw::c_uint = 0x809F;
    pub const GL_SAMPLE_BUFFERS: std::os::raw::c_uint = 0x80A8;
    pub const GL_SAMPLE_COVERAGE: std::os::raw::c_uint = 0x80A0;
    pub const GL_SAMPLE_COVERAGE_INVERT: std::os::raw::c_uint = 0x80AB;
    pub const GL_SAMPLE_COVERAGE_VALUE: std::os::raw::c_uint = 0x80AA;
    pub const GL_SAMPLE_MASK: std::os::raw::c_uint = 0x8E51;
    pub const GL_SAMPLE_MASK_VALUE: std::os::raw::c_uint = 0x8E52;
    pub const GL_SAMPLE_POSITION: std::os::raw::c_uint = 0x8E50;
    pub const GL_SCISSOR_BOX: std::os::raw::c_uint = 0x0C10;
    pub const GL_SCISSOR_TEST: std::os::raw::c_uint = 0x0C11;
    pub const GL_SEPARATE_ATTRIBS: std::os::raw::c_uint = 0x8C8D;
    pub const GL_SET: std::os::raw::c_uint = 0x150F;
    pub const GL_SHADER_SOURCE_LENGTH: std::os::raw::c_uint = 0x8B88;
    pub const GL_SHADER_TYPE: std::os::raw::c_uint = 0x8B4F;
    pub const GL_SHADING_LANGUAGE_VERSION: std::os::raw::c_uint = 0x8B8C;
    pub const GL_SHORT: std::os::raw::c_uint = 0x1402;
    pub const GL_SIGNALED: std::os::raw::c_uint = 0x9119;
    pub const GL_SIGNED_NORMALIZED: std::os::raw::c_uint = 0x8F9C;
    pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: std::os::raw::c_uint = 0x0B23;
    pub const GL_SMOOTH_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x0B22;
    pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: std::os::raw::c_uint = 0x0B13;
    pub const GL_SMOOTH_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x0B12;
    pub const GL_SRC1_ALPHA: std::os::raw::c_uint = 0x8589;
    pub const GL_SRC_ALPHA: std::os::raw::c_uint = 0x0302;
    pub const GL_SRC_ALPHA_SATURATE: std::os::raw::c_uint = 0x0308;
    pub const GL_SRC_COLOR: std::os::raw::c_uint = 0x0300;
    pub const GL_SRGB: std::os::raw::c_uint = 0x8C40;
    pub const GL_SRGB8: std::os::raw::c_uint = 0x8C41;
    pub const GL_SRGB8_ALPHA8: std::os::raw::c_uint = 0x8C43;
    pub const GL_SRGB_ALPHA: std::os::raw::c_uint = 0x8C42;
    pub const GL_STATIC_COPY: std::os::raw::c_uint = 0x88E6;
    pub const GL_STATIC_DRAW: std::os::raw::c_uint = 0x88E4;
    pub const GL_STATIC_READ: std::os::raw::c_uint = 0x88E5;
    pub const GL_STENCIL: std::os::raw::c_uint = 0x1802;
    pub const GL_STENCIL_ATTACHMENT: std::os::raw::c_uint = 0x8D20;
    pub const GL_STENCIL_BACK_FAIL: std::os::raw::c_uint = 0x8801;
    pub const GL_STENCIL_BACK_FUNC: std::os::raw::c_uint = 0x8800;
    pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x8802;
    pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x8803;
    pub const GL_STENCIL_BACK_REF: std::os::raw::c_uint = 0x8CA3;
    pub const GL_STENCIL_BACK_VALUE_MASK: std::os::raw::c_uint = 0x8CA4;
    pub const GL_STENCIL_BACK_WRITEMASK: std::os::raw::c_uint = 0x8CA5;
    pub const GL_STENCIL_BUFFER_BIT: std::os::raw::c_uint = 0x00000400;
    pub const GL_STENCIL_CLEAR_VALUE: std::os::raw::c_uint = 0x0B91;
    pub const GL_STENCIL_FAIL: std::os::raw::c_uint = 0x0B94;
    pub const GL_STENCIL_FUNC: std::os::raw::c_uint = 0x0B92;
    pub const GL_STENCIL_INDEX: std::os::raw::c_uint = 0x1901;
    pub const GL_STENCIL_INDEX1: std::os::raw::c_uint = 0x8D46;
    pub const GL_STENCIL_INDEX16: std::os::raw::c_uint = 0x8D49;
    pub const GL_STENCIL_INDEX4: std::os::raw::c_uint = 0x8D47;
    pub const GL_STENCIL_INDEX8: std::os::raw::c_uint = 0x8D48;
    pub const GL_STENCIL_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x0B96;
    pub const GL_STENCIL_REF: std::os::raw::c_uint = 0x0B97;
    pub const GL_STENCIL_TEST: std::os::raw::c_uint = 0x0B90;
    pub const GL_STENCIL_VALUE_MASK: std::os::raw::c_uint = 0x0B93;
    pub const GL_STENCIL_WRITEMASK: std::os::raw::c_uint = 0x0B98;
    pub const GL_STEREO: std::os::raw::c_uint = 0x0C33;
    pub const GL_STREAM_COPY: std::os::raw::c_uint = 0x88E2;
    pub const GL_STREAM_DRAW: std::os::raw::c_uint = 0x88E0;
    pub const GL_STREAM_READ: std::os::raw::c_uint = 0x88E1;
    pub const GL_SUBPIXEL_BITS: std::os::raw::c_uint = 0x0D50;
    pub const GL_SYNC_CONDITION: std::os::raw::c_uint = 0x9113;
    pub const GL_SYNC_FENCE: std::os::raw::c_uint = 0x9116;
    pub const GL_SYNC_FLAGS: std::os::raw::c_uint = 0x9115;
    pub const GL_SYNC_FLUSH_COMMANDS_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_SYNC_GPU_COMMANDS_COMPLETE: std::os::raw::c_uint = 0x9117;
    pub const GL_SYNC_STATUS: std::os::raw::c_uint = 0x9114;
    pub const GL_TEXTURE: std::os::raw::c_uint = 0x1702;
    pub const GL_TEXTURE0: std::os::raw::c_uint = 0x84C0;
    pub const GL_TEXTURE1: std::os::raw::c_uint = 0x84C1;
    pub const GL_TEXTURE10: std::os::raw::c_uint = 0x84CA;
    pub const GL_TEXTURE11: std::os::raw::c_uint = 0x84CB;
    pub const GL_TEXTURE12: std::os::raw::c_uint = 0x84CC;
    pub const GL_TEXTURE13: std::os::raw::c_uint = 0x84CD;
    pub const GL_TEXTURE14: std::os::raw::c_uint = 0x84CE;
    pub const GL_TEXTURE15: std::os::raw::c_uint = 0x84CF;
    pub const GL_TEXTURE16: std::os::raw::c_uint = 0x84D0;
    pub const GL_TEXTURE17: std::os::raw::c_uint = 0x84D1;
    pub const GL_TEXTURE18: std::os::raw::c_uint = 0x84D2;
    pub const GL_TEXTURE19: std::os::raw::c_uint = 0x84D3;
    pub const GL_TEXTURE2: std::os::raw::c_uint = 0x84C2;
    pub const GL_TEXTURE20: std::os::raw::c_uint = 0x84D4;
    pub const GL_TEXTURE21: std::os::raw::c_uint = 0x84D5;
    pub const GL_TEXTURE22: std::os::raw::c_uint = 0x84D6;
    pub const GL_TEXTURE23: std::os::raw::c_uint = 0x84D7;
    pub const GL_TEXTURE24: std::os::raw::c_uint = 0x84D8;
    pub const GL_TEXTURE25: std::os::raw::c_uint = 0x84D9;
    pub const GL_TEXTURE26: std::os::raw::c_uint = 0x84DA;
    pub const GL_TEXTURE27: std::os::raw::c_uint = 0x84DB;
    pub const GL_TEXTURE28: std::os::raw::c_uint = 0x84DC;
    pub const GL_TEXTURE29: std::os::raw::c_uint = 0x84DD;
    pub const GL_TEXTURE3: std::os::raw::c_uint = 0x84C3;
    pub const GL_TEXTURE30: std::os::raw::c_uint = 0x84DE;
    pub const GL_TEXTURE31: std::os::raw::c_uint = 0x84DF;
    pub const GL_TEXTURE4: std::os::raw::c_uint = 0x84C4;
    pub const GL_TEXTURE5: std::os::raw::c_uint = 0x84C5;
    pub const GL_TEXTURE6: std::os::raw::c_uint = 0x84C6;
    pub const GL_TEXTURE7: std::os::raw::c_uint = 0x84C7;
    pub const GL_TEXTURE8: std::os::raw::c_uint = 0x84C8;
    pub const GL_TEXTURE9: std::os::raw::c_uint = 0x84C9;
    pub const GL_TEXTURE_1D: std::os::raw::c_uint = 0x0DE0;
    pub const GL_TEXTURE_1D_ARRAY: std::os::raw::c_uint = 0x8C18;
    pub const GL_TEXTURE_2D: std::os::raw::c_uint = 0x0DE1;
    pub const GL_TEXTURE_2D_ARRAY: std::os::raw::c_uint = 0x8C1A;
    pub const GL_TEXTURE_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9100;
    pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9102;
    pub const GL_TEXTURE_3D: std::os::raw::c_uint = 0x806F;
    pub const GL_TEXTURE_ALPHA_SIZE: std::os::raw::c_uint = 0x805F;
    pub const GL_TEXTURE_ALPHA_TYPE: std::os::raw::c_uint = 0x8C13;
    pub const GL_TEXTURE_BASE_LEVEL: std::os::raw::c_uint = 0x813C;
    pub const GL_TEXTURE_BINDING_1D: std::os::raw::c_uint = 0x8068;
    pub const GL_TEXTURE_BINDING_1D_ARRAY: std::os::raw::c_uint = 0x8C1C;
    pub const GL_TEXTURE_BINDING_2D: std::os::raw::c_uint = 0x8069;
    pub const GL_TEXTURE_BINDING_2D_ARRAY: std::os::raw::c_uint = 0x8C1D;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: std::os::raw::c_uint = 0x9104;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x9105;
    pub const GL_TEXTURE_BINDING_3D: std::os::raw::c_uint = 0x806A;
    pub const GL_TEXTURE_BINDING_BUFFER: std::os::raw::c_uint = 0x8C2C;
    pub const GL_TEXTURE_BINDING_CUBE_MAP: std::os::raw::c_uint = 0x8514;
    pub const GL_TEXTURE_BINDING_RECTANGLE: std::os::raw::c_uint = 0x84F6;
    pub const GL_TEXTURE_BLUE_SIZE: std::os::raw::c_uint = 0x805E;
    pub const GL_TEXTURE_BLUE_TYPE: std::os::raw::c_uint = 0x8C12;
    pub const GL_TEXTURE_BORDER_COLOR: std::os::raw::c_uint = 0x1004;
    pub const GL_TEXTURE_BUFFER: std::os::raw::c_uint = 0x8C2A;
    pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: std::os::raw::c_uint = 0x8C2D;
    pub const GL_TEXTURE_COMPARE_FUNC: std::os::raw::c_uint = 0x884D;
    pub const GL_TEXTURE_COMPARE_MODE: std::os::raw::c_uint = 0x884C;
    pub const GL_TEXTURE_COMPRESSED: std::os::raw::c_uint = 0x86A1;
    pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: std::os::raw::c_uint = 0x86A0;
    pub const GL_TEXTURE_COMPRESSION_HINT: std::os::raw::c_uint = 0x84EF;
    pub const GL_TEXTURE_CUBE_MAP: std::os::raw::c_uint = 0x8513;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: std::os::raw::c_uint = 0x8516;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: std::os::raw::c_uint = 0x8518;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: std::os::raw::c_uint = 0x851A;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: std::os::raw::c_uint = 0x8515;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: std::os::raw::c_uint = 0x8517;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: std::os::raw::c_uint = 0x8519;
    pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: std::os::raw::c_uint = 0x884F;
    pub const GL_TEXTURE_DEPTH: std::os::raw::c_uint = 0x8071;
    pub const GL_TEXTURE_DEPTH_SIZE: std::os::raw::c_uint = 0x884A;
    pub const GL_TEXTURE_DEPTH_TYPE: std::os::raw::c_uint = 0x8C16;
    pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: std::os::raw::c_uint = 0x9107;
    pub const GL_TEXTURE_GREEN_SIZE: std::os::raw::c_uint = 0x805D;
    pub const GL_TEXTURE_GREEN_TYPE: std::os::raw::c_uint = 0x8C11;
    pub const GL_TEXTURE_HEIGHT: std::os::raw::c_uint = 0x1001;
    pub const GL_TEXTURE_INTERNAL_FORMAT: std::os::raw::c_uint = 0x1003;
    pub const GL_TEXTURE_LOD_BIAS: std::os::raw::c_uint = 0x8501;
    pub const GL_TEXTURE_MAG_FILTER: std::os::raw::c_uint = 0x2800;
    pub const GL_TEXTURE_MAX_LEVEL: std::os::raw::c_uint = 0x813D;
    pub const GL_TEXTURE_MAX_LOD: std::os::raw::c_uint = 0x813B;
    pub const GL_TEXTURE_MIN_FILTER: std::os::raw::c_uint = 0x2801;
    pub const GL_TEXTURE_MIN_LOD: std::os::raw::c_uint = 0x813A;
    pub const GL_TEXTURE_RECTANGLE: std::os::raw::c_uint = 0x84F5;
    pub const GL_TEXTURE_RED_SIZE: std::os::raw::c_uint = 0x805C;
    pub const GL_TEXTURE_RED_TYPE: std::os::raw::c_uint = 0x8C10;
    pub const GL_TEXTURE_SAMPLES: std::os::raw::c_uint = 0x9106;
    pub const GL_TEXTURE_SHARED_SIZE: std::os::raw::c_uint = 0x8C3F;
    pub const GL_TEXTURE_STENCIL_SIZE: std::os::raw::c_uint = 0x88F1;
    pub const GL_TEXTURE_WIDTH: std::os::raw::c_uint = 0x1000;
    pub const GL_TEXTURE_WRAP_R: std::os::raw::c_uint = 0x8072;
    pub const GL_TEXTURE_WRAP_S: std::os::raw::c_uint = 0x2802;
    pub const GL_TEXTURE_WRAP_T: std::os::raw::c_uint = 0x2803;
    pub const GL_TIMEOUT_EXPIRED: std::os::raw::c_uint = 0x911B;
    pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: std::os::raw::c_uint = 0x8C8E;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: std::os::raw::c_uint = 0x8C8F;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: std::os::raw::c_uint = 0x8C7F;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: std::os::raw::c_uint = 0x8C85;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: std::os::raw::c_uint = 0x8C84;
    pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: std::os::raw::c_uint = 0x8C88;
    pub const GL_TRANSFORM_FEEDBACK_VARYINGS: std::os::raw::c_uint = 0x8C83;
    pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: std::os::raw::c_uint = 0x8C76;
    pub const GL_TRIANGLES: std::os::raw::c_uint = 0x0004;
    pub const GL_TRIANGLES_ADJACENCY: std::os::raw::c_uint = 0x000C;
    pub const GL_TRIANGLE_FAN: std::os::raw::c_uint = 0x0006;
    pub const GL_TRIANGLE_STRIP: std::os::raw::c_uint = 0x0005;
    pub const GL_TRIANGLE_STRIP_ADJACENCY: std::os::raw::c_uint = 0x000D;
    pub const GL_TRUE: std::os::raw::c_uchar = 1;
    pub const GL_UNIFORM_ARRAY_STRIDE: std::os::raw::c_uint = 0x8A3C;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: std::os::raw::c_uint = 0x8A42;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: std::os::raw::c_uint = 0x8A43;
    pub const GL_UNIFORM_BLOCK_BINDING: std::os::raw::c_uint = 0x8A3F;
    pub const GL_UNIFORM_BLOCK_DATA_SIZE: std::os::raw::c_uint = 0x8A40;
    pub const GL_UNIFORM_BLOCK_INDEX: std::os::raw::c_uint = 0x8A3A;
    pub const GL_UNIFORM_BLOCK_NAME_LENGTH: std::os::raw::c_uint = 0x8A41;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: std::os::raw::c_uint = 0x8A46;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: std::os::raw::c_uint = 0x8A45;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: std::os::raw::c_uint = 0x8A44;
    pub const GL_UNIFORM_BUFFER: std::os::raw::c_uint = 0x8A11;
    pub const GL_UNIFORM_BUFFER_BINDING: std::os::raw::c_uint = 0x8A28;
    pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: std::os::raw::c_uint = 0x8A34;
    pub const GL_UNIFORM_BUFFER_SIZE: std::os::raw::c_uint = 0x8A2A;
    pub const GL_UNIFORM_BUFFER_START: std::os::raw::c_uint = 0x8A29;
    pub const GL_UNIFORM_IS_ROW_MAJOR: std::os::raw::c_uint = 0x8A3E;
    pub const GL_UNIFORM_MATRIX_STRIDE: std::os::raw::c_uint = 0x8A3D;
    pub const GL_UNIFORM_NAME_LENGTH: std::os::raw::c_uint = 0x8A39;
    pub const GL_UNIFORM_OFFSET: std::os::raw::c_uint = 0x8A3B;
    pub const GL_UNIFORM_SIZE: std::os::raw::c_uint = 0x8A38;
    pub const GL_UNIFORM_TYPE: std::os::raw::c_uint = 0x8A37;
    pub const GL_UNPACK_ALIGNMENT: std::os::raw::c_uint = 0x0CF5;
    pub const GL_UNPACK_IMAGE_HEIGHT: std::os::raw::c_uint = 0x806E;
    pub const GL_UNPACK_LSB_FIRST: std::os::raw::c_uint = 0x0CF1;
    pub const GL_UNPACK_ROW_LENGTH: std::os::raw::c_uint = 0x0CF2;
    pub const GL_UNPACK_SKIP_IMAGES: std::os::raw::c_uint = 0x806D;
    pub const GL_UNPACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0CF4;
    pub const GL_UNPACK_SKIP_ROWS: std::os::raw::c_uint = 0x0CF3;
    pub const GL_UNPACK_SWAP_BYTES: std::os::raw::c_uint = 0x0CF0;
    pub const GL_UNSIGNALED: std::os::raw::c_uint = 0x9118;
    pub const GL_UNSIGNED_BYTE: std::os::raw::c_uint = 0x1401;
    pub const GL_UNSIGNED_BYTE_2_3_3_REV: std::os::raw::c_uint = 0x8362;
    pub const GL_UNSIGNED_BYTE_3_3_2: std::os::raw::c_uint = 0x8032;
    pub const GL_UNSIGNED_INT: std::os::raw::c_uint = 0x1405;
    pub const GL_UNSIGNED_INT_10F_11F_11F_REV: std::os::raw::c_uint = 0x8C3B;
    pub const GL_UNSIGNED_INT_10_10_10_2: std::os::raw::c_uint = 0x8036;
    pub const GL_UNSIGNED_INT_24_8: std::os::raw::c_uint = 0x84FA;
    pub const GL_UNSIGNED_INT_2_10_10_10_REV: std::os::raw::c_uint = 0x8368;
    pub const GL_UNSIGNED_INT_5_9_9_9_REV: std::os::raw::c_uint = 0x8C3E;
    pub const GL_UNSIGNED_INT_8_8_8_8: std::os::raw::c_uint = 0x8035;
    pub const GL_UNSIGNED_INT_8_8_8_8_REV: std::os::raw::c_uint = 0x8367;
    pub const GL_UNSIGNED_INT_SAMPLER_1D: std::os::raw::c_uint = 0x8DD1;
    pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: std::os::raw::c_uint = 0x8DD6;
    pub const GL_UNSIGNED_INT_SAMPLER_2D: std::os::raw::c_uint = 0x8DD2;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: std::os::raw::c_uint = 0x8DD7;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: std::os::raw::c_uint = 0x910A;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: std::os::raw::c_uint = 0x910D;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: std::os::raw::c_uint = 0x8DD5;
    pub const GL_UNSIGNED_INT_SAMPLER_3D: std::os::raw::c_uint = 0x8DD3;
    pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: std::os::raw::c_uint = 0x8DD8;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE: std::os::raw::c_uint = 0x8DD4;
    pub const GL_UNSIGNED_INT_VEC2: std::os::raw::c_uint = 0x8DC6;
    pub const GL_UNSIGNED_INT_VEC3: std::os::raw::c_uint = 0x8DC7;
    pub const GL_UNSIGNED_INT_VEC4: std::os::raw::c_uint = 0x8DC8;
    pub const GL_UNSIGNED_NORMALIZED: std::os::raw::c_uint = 0x8C17;
    pub const GL_UNSIGNED_SHORT: std::os::raw::c_uint = 0x1403;
    pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: std::os::raw::c_uint = 0x8366;
    pub const GL_UNSIGNED_SHORT_4_4_4_4: std::os::raw::c_uint = 0x8033;
    pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: std::os::raw::c_uint = 0x8365;
    pub const GL_UNSIGNED_SHORT_5_5_5_1: std::os::raw::c_uint = 0x8034;
    pub const GL_UNSIGNED_SHORT_5_6_5: std::os::raw::c_uint = 0x8363;
    pub const GL_UNSIGNED_SHORT_5_6_5_REV: std::os::raw::c_uint = 0x8364;
    pub const GL_UPPER_LEFT: std::os::raw::c_uint = 0x8CA2;
    pub const GL_VALIDATE_STATUS: std::os::raw::c_uint = 0x8B83;
    pub const GL_VENDOR: std::os::raw::c_uint = 0x1F00;
    pub const GL_VERSION: std::os::raw::c_uint = 0x1F02;
    pub const GL_VERTEX_ARRAY_BINDING: std::os::raw::c_uint = 0x85B5;
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: std::os::raw::c_uint = 0x889F;
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: std::os::raw::c_uint = 0x8622;
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: std::os::raw::c_uint = 0x88FD;
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: std::os::raw::c_uint = 0x886A;
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: std::os::raw::c_uint = 0x8645;
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: std::os::raw::c_uint = 0x8623;
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: std::os::raw::c_uint = 0x8624;
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: std::os::raw::c_uint = 0x8625;
    pub const GL_VERTEX_PROGRAM_POINT_SIZE: std::os::raw::c_uint = 0x8642;
    pub const GL_VERTEX_SHADER: std::os::raw::c_uint = 0x8B31;
    pub const GL_VIEWPORT: std::os::raw::c_uint = 0x0BA2;
    pub const GL_WAIT_FAILED: std::os::raw::c_uint = 0x911D;
    pub const GL_WRITE_ONLY: std::os::raw::c_uint = 0x88B9;
    pub const GL_XOR: std::os::raw::c_uint = 0x1506;
    pub const GL_ZERO: std::os::raw::c_uint = 0;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code)]

    use std;
    use std::mem;
    use super::storage;
    use super::types::*;

     #[inline] pub unsafe fn ActiveTexture(texture: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ActiveTexture.ptr)(texture) }
     #[inline] pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::AttachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn BeginConditionalRender(id: GLuint, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::BeginConditionalRender.ptr)(id, mode) }
     #[inline] pub unsafe fn BeginQuery(target: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BeginQuery.ptr)(target, id) }
     #[inline] pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BeginTransformFeedback.ptr)(primitiveMode) }
     #[inline] pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindAttribLocation.ptr)(program, index, name) }
     #[inline] pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindBuffer.ptr)(target, buffer) }
     #[inline] pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(storage::BindBufferBase.ptr)(target, index, buffer) }
     #[inline] pub unsafe fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::BindBufferRange.ptr)(target, index, buffer, offset, size) }
     #[inline] pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindFragDataLocation.ptr)(program, color, name) }
     #[inline] pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindFramebuffer.ptr)(target, framebuffer) }
     #[inline] pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindRenderbuffer.ptr)(target, renderbuffer) }
     #[inline] pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTexture.ptr)(target, texture) }
     #[inline] pub unsafe fn BindVertexArray(array: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::BindVertexArray.ptr)(array) }
     #[inline] pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::BlendColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn BlendEquation(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BlendEquation.ptr)(mode) }
     #[inline] pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendEquationSeparate.ptr)(modeRGB, modeAlpha) }
     #[inline] pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendFunc.ptr)(sfactor, dfactor) }
     #[inline] pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::BlendFuncSeparate.ptr)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
     #[inline] pub unsafe fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>(storage::BlitFramebuffer.ptr)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
     #[inline] pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const std::os::raw::c_void, usage: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const std::os::raw::c_void, GLenum) -> ()>(storage::BufferData.ptr)(target, size, data, usage) }
     #[inline] pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const std::os::raw::c_void) -> ()>(storage::BufferSubData.ptr)(target, offset, size, data) }
     #[inline] pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum { mem::transmute::<_, extern "system" fn(GLenum) -> GLenum>(storage::CheckFramebufferStatus.ptr)(target) }
     #[inline] pub unsafe fn ClampColor(target: GLenum, clamp: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::ClampColor.ptr)(target, clamp) }
     #[inline] pub unsafe fn Clear(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::Clear.ptr)(mask) }
     #[inline] pub unsafe fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>(storage::ClearBufferfi.ptr)(buffer, drawbuffer, depth, stencil) }
     #[inline] pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(storage::ClearBufferfv.ptr)(buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>(storage::ClearBufferiv.ptr)(buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>(storage::ClearBufferuiv.ptr)(buffer, drawbuffer, value) }
     #[inline] pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ClearDepth(depth: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::ClearDepth.ptr)(depth) }
     #[inline] pub unsafe fn ClearStencil(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ClearStencil.ptr)(s) }
     #[inline] pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum>(storage::ClientWaitSync.ptr)(sync, flags, timeout) }
     #[inline] pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMask.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMaski.ptr)(index, r, g, b, a) }
     #[inline] pub unsafe fn CompileShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CompileShader.ptr)(shader) }
     #[inline] pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage1D.ptr)(target, level, internalformat, width, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage2D.ptr)(target, level, internalformat, width, height, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexImage3D.ptr)(target, level, internalformat, width, height, depth, border, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage1D.ptr)(target, level, xoffset, width, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
     #[inline] pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::CompressedTexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
     #[inline] pub unsafe fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> ()>(storage::CopyBufferSubData.ptr)(readTarget, writeTarget, readOffset, writeOffset, size) }
     #[inline] pub unsafe fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint) -> ()>(storage::CopyTexImage1D.ptr)(target, level, internalformat, x, y, width, border) }
     #[inline] pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> ()>(storage::CopyTexImage2D.ptr)(target, level, internalformat, x, y, width, height, border) }
     #[inline] pub unsafe fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei) -> ()>(storage::CopyTexSubImage1D.ptr)(target, level, xoffset, x, y, width) }
     #[inline] pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage2D.ptr)(target, level, xoffset, yoffset, x, y, width, height) }
     #[inline] pub unsafe fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
     #[inline] pub unsafe fn CreateProgram() -> GLuint { mem::transmute::<_, extern "system" fn() -> GLuint>(storage::CreateProgram.ptr)() }
     #[inline] pub unsafe fn CreateShader(type_: GLenum) -> GLuint { mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(storage::CreateShader.ptr)(type_) }
     #[inline] pub unsafe fn CullFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::CullFace.ptr)(mode) }
     #[inline] pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteFramebuffers.ptr)(n, framebuffers) }
     #[inline] pub unsafe fn DeleteProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteProgram.ptr)(program) }
     #[inline] pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteQueries.ptr)(n, ids) }
     #[inline] pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteRenderbuffers.ptr)(n, renderbuffers) }
     #[inline] pub unsafe fn DeleteShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteShader.ptr)(shader) }
     #[inline] pub unsafe fn DeleteSync(sync: GLsync) -> () { mem::transmute::<_, extern "system" fn(GLsync) -> ()>(storage::DeleteSync.ptr)(sync) }
     #[inline] pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteVertexArrays.ptr)(n, arrays) }
     #[inline] pub unsafe fn DepthFunc(func: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DepthFunc.ptr)(func) }
     #[inline] pub unsafe fn DepthMask(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::DepthMask.ptr)(flag) }
     #[inline] pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::DepthRange.ptr)(n, f) }
     #[inline] pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DetachShader.ptr)(program, shader) }
     #[inline] pub unsafe fn Disable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Disable.ptr)(cap) }
     #[inline] pub unsafe fn DisableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DisableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn Disablei(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::Disablei.ptr)(target, index) }
     #[inline] pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(storage::DrawArrays.ptr)(mode, first, count) }
     #[inline] pub unsafe fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> ()>(storage::DrawArraysInstanced.ptr)(mode, first, count, instancecount) }
     #[inline] pub unsafe fn DrawBuffer(buf: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DrawBuffer.ptr)(buf) }
     #[inline] pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(storage::DrawBuffers.ptr)(n, bufs) }
     #[inline] pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawElements.ptr)(mode, count, type_, indices) }
     #[inline] pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLint) -> ()>(storage::DrawElementsBaseVertex.ptr)(mode, count, type_, indices, basevertex) }
     #[inline] pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLsizei) -> ()>(storage::DrawElementsInstanced.ptr)(mode, count, type_, indices, instancecount) }
     #[inline] pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, instancecount: GLsizei, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const std::os::raw::c_void, GLsizei, GLint) -> ()>(storage::DrawElementsInstancedBaseVertex.ptr)(mode, count, type_, indices, instancecount, basevertex) }
     #[inline] pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawRangeElements.ptr)(mode, start, end, count, type_, indices) }
     #[inline] pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const std::os::raw::c_void, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const std::os::raw::c_void, GLint) -> ()>(storage::DrawRangeElementsBaseVertex.ptr)(mode, start, end, count, type_, indices, basevertex) }
     #[inline] pub unsafe fn Enable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Enable.ptr)(cap) }
     #[inline] pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::EnableVertexAttribArray.ptr)(index) }
     #[inline] pub unsafe fn Enablei(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::Enablei.ptr)(target, index) }
     #[inline] pub unsafe fn EndConditionalRender() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndConditionalRender.ptr)() }
     #[inline] pub unsafe fn EndQuery(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::EndQuery.ptr)(target) }
     #[inline] pub unsafe fn EndTransformFeedback() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndTransformFeedback.ptr)() }
     #[inline] pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync { mem::transmute::<_, extern "system" fn(GLenum, GLbitfield) -> GLsync>(storage::FenceSync.ptr)(condition, flags) }
     #[inline] pub unsafe fn Finish() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.ptr)() }
     #[inline] pub unsafe fn Flush() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.ptr)() }
     #[inline] pub unsafe fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>(storage::FlushMappedBufferRange.ptr)(target, offset, length) }
     #[inline] pub unsafe fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> ()>(storage::FramebufferRenderbuffer.ptr)(target, attachment, renderbuffertarget, renderbuffer) }
     #[inline] pub unsafe fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture.ptr)(target, attachment, texture, level) }
     #[inline] pub unsafe fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture1D.ptr)(target, attachment, textarget, texture, level) }
     #[inline] pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture2D.ptr)(target, attachment, textarget, texture, level) }
     #[inline] pub unsafe fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint) -> ()>(storage::FramebufferTexture3D.ptr)(target, attachment, textarget, texture, level, zoffset) }
     #[inline] pub unsafe fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> ()>(storage::FramebufferTextureLayer.ptr)(target, attachment, texture, level, layer) }
     #[inline] pub unsafe fn FrontFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::FrontFace.ptr)(mode) }
     #[inline] pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenBuffers.ptr)(n, buffers) }
     #[inline] pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenFramebuffers.ptr)(n, framebuffers) }
     #[inline] pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenQueries.ptr)(n, ids) }
     #[inline] pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenRenderbuffers.ptr)(n, renderbuffers) }
     #[inline] pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenTextures.ptr)(n, textures) }
     #[inline] pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenVertexArrays.ptr)(n, arrays) }
     #[inline] pub unsafe fn GenerateMipmap(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::GenerateMipmap.ptr)(target) }
     #[inline] pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveAttrib.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveUniform.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveUniformBlockName.ptr)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
     #[inline] pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveUniformBlockiv.ptr)(program, uniformBlockIndex, pname, params) }
     #[inline] pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveUniformName.ptr)(program, uniformIndex, bufSize, length, uniformName) }
     #[inline] pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveUniformsiv.ptr)(program, uniformCount, uniformIndices, pname, params) }
     #[inline] pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>(storage::GetAttachedShaders.ptr)(program, maxCount, count, shaders) }
     #[inline] pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetAttribLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLboolean) -> ()>(storage::GetBooleani_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(storage::GetBooleanv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>(storage::GetBufferParameteri64v.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetBufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetBufferPointerv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut std::os::raw::c_void) -> ()>(storage::GetBufferSubData.ptr)(target, offset, size, data) }
     #[inline] pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut std::os::raw::c_void) -> ()>(storage::GetCompressedTexImage.ptr)(target, level, img) }
     #[inline] pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(storage::GetDoublev.ptr)(pname, data) }
     #[inline] pub unsafe fn GetError() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetError.ptr)() }
     #[inline] pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetFloatv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetFragDataLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) -> ()>(storage::GetFramebufferAttachmentParameteriv.ptr)(target, attachment, pname, params) }
     #[inline] pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>(storage::GetInteger64i_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>(storage::GetInteger64v.ptr)(pname, data) }
     #[inline] pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>(storage::GetIntegeri_v.ptr)(target, index, data) }
     #[inline] pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(storage::GetIntegerv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(storage::GetMultisamplefv.ptr)(pname, index, val) }
     #[inline] pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramInfoLog.ptr)(program, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetProgramiv.ptr)(program, pname, params) }
     #[inline] pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetQueryObjectiv.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetQueryObjectuiv.ptr)(id, pname, params) }
     #[inline] pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetQueryiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetRenderbufferParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderInfoLog.ptr)(shader, bufSize, length, infoLog) }
     #[inline] pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderSource.ptr)(shader, bufSize, length, source) }
     #[inline] pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetShaderiv.ptr)(shader, pname, params) }
     #[inline] pub unsafe fn GetString(name: GLenum) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(storage::GetString.ptr)(name) }
     #[inline] pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> *const GLubyte>(storage::GetStringi.ptr)(name, index) }
     #[inline] pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> ()>(storage::GetSynciv.ptr)(sync, pname, count, length, values) }
     #[inline] pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::GetTexImage.ptr)(target, level, format, type_, pixels) }
     #[inline] pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> ()>(storage::GetTexLevelParameterfv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(storage::GetTexLevelParameteriv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameterIiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>(storage::GetTexParameterIuiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar) -> ()>(storage::GetTransformFeedbackVarying.ptr)(program, index, bufSize, length, size, type_, name) }
     #[inline] pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLuint>(storage::GetUniformBlockIndex.ptr)(program, uniformBlockName) }
     #[inline] pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) -> ()>(storage::GetUniformIndices.ptr)(program, uniformCount, uniformNames, uniformIndices) }
     #[inline] pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetUniformLocation.ptr)(program, name) }
     #[inline] pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>(storage::GetUniformfv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>(storage::GetUniformiv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>(storage::GetUniformuiv.ptr)(program, location, params) }
     #[inline] pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribIiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetVertexAttribIuiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut *mut std::os::raw::c_void) -> ()>(storage::GetVertexAttribPointerv.ptr)(index, pname, pointer) }
     #[inline] pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(storage::GetVertexAttribdv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetVertexAttribfv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribiv.ptr)(index, pname, params) }
     #[inline] pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::Hint.ptr)(target, mode) }
     #[inline] pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsBuffer.ptr)(buffer) }
     #[inline] pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::IsEnabled.ptr)(cap) }
     #[inline] pub unsafe fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> GLboolean>(storage::IsEnabledi.ptr)(target, index) }
     #[inline] pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsFramebuffer.ptr)(framebuffer) }
     #[inline] pub unsafe fn IsProgram(program: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgram.ptr)(program) }
     #[inline] pub unsafe fn IsQuery(id: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsQuery.ptr)(id) }
     #[inline] pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsRenderbuffer.ptr)(renderbuffer) }
     #[inline] pub unsafe fn IsShader(shader: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsShader.ptr)(shader) }
     #[inline] pub unsafe fn IsSync(sync: GLsync) -> GLboolean { mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>(storage::IsSync.ptr)(sync) }
     #[inline] pub unsafe fn IsTexture(texture: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTexture.ptr)(texture) }
     #[inline] pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsVertexArray.ptr)(array) }
     #[inline] pub unsafe fn LineWidth(width: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::LineWidth.ptr)(width) }
     #[inline] pub unsafe fn LinkProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LinkProgram.ptr)(program) }
     #[inline] pub unsafe fn LogicOp(opcode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::LogicOp.ptr)(opcode) }
     #[inline] pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut std::os::raw::c_void { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> *mut std::os::raw::c_void>(storage::MapBuffer.ptr)(target, access) }
     #[inline] pub unsafe fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut std::os::raw::c_void { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut std::os::raw::c_void>(storage::MapBufferRange.ptr)(target, offset, length, access) }
     #[inline] pub unsafe fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei) -> ()>(storage::MultiDrawArrays.ptr)(mode, first, count, drawcount) }
     #[inline] pub unsafe fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const std::os::raw::c_void, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const std::os::raw::c_void, GLsizei) -> ()>(storage::MultiDrawElements.ptr)(mode, count, type_, indices, drawcount) }
     #[inline] pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const std::os::raw::c_void, drawcount: GLsizei, basevertex: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const std::os::raw::c_void, GLsizei, *const GLint) -> ()>(storage::MultiDrawElementsBaseVertex.ptr)(mode, count, type_, indices, drawcount, basevertex) }
     #[inline] pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PixelStoref.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelStorei.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PointParameterf.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::PointParameterfv.ptr)(pname, params) }
     #[inline] pub unsafe fn PointParameteri(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PointParameteri.ptr)(pname, param) }
     #[inline] pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::PointParameteriv.ptr)(pname, params) }
     #[inline] pub unsafe fn PointSize(size: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::PointSize.ptr)(size) }
     #[inline] pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::PolygonMode.ptr)(face, mode) }
     #[inline] pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PolygonOffset.ptr)(factor, units) }
     #[inline] pub unsafe fn PrimitiveRestartIndex(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::PrimitiveRestartIndex.ptr)(index) }
     #[inline] pub unsafe fn ProvokingVertex(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ProvokingVertex.ptr)(mode) }
     #[inline] pub unsafe fn ReadBuffer(src: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ReadBuffer.ptr)(src) }
     #[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::ReadPixels.ptr)(x, y, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) -> ()>(storage::RenderbufferStorage.ptr)(target, internalformat, width, height) }
     #[inline] pub unsafe fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::RenderbufferStorageMultisample.ptr)(target, samples, internalformat, width, height) }
     #[inline] pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>(storage::SampleCoverage.ptr)(value, invert) }
     #[inline] pub unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>(storage::SampleMaski.ptr)(maskNumber, mask) }
     #[inline] pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Scissor.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>(storage::ShaderSource.ptr)(shader, count, string, length) }
     #[inline] pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(storage::StencilFunc.ptr)(func, ref_, mask) }
     #[inline] pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>(storage::StencilFuncSeparate.ptr)(face, func, ref_, mask) }
     #[inline] pub unsafe fn StencilMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::StencilMask.ptr)(mask) }
     #[inline] pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::StencilMaskSeparate.ptr)(face, mask) }
     #[inline] pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(storage::StencilOp.ptr)(fail, zfail, zpass) }
     #[inline] pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::StencilOpSeparate.ptr)(face, sfail, dpfail, dppass) }
     #[inline] pub unsafe fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(storage::TexBuffer.ptr)(target, internalformat, buffer) }
     #[inline] pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage1D.ptr)(target, level, internalformat, width, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage2D.ptr)(target, level, internalformat, width, height, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexImage2DMultisample.ptr)(target, samples, internalformat, width, height, fixedsamplelocations) }
     #[inline] pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage3D.ptr)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexImage3DMultisample.ptr)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
     #[inline] pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameterIiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(storage::TexParameterIuiv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexParameterf.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexParameteri.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage1D.ptr)(target, level, xoffset, width, format, type_, pixels) }
     #[inline] pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage2D.ptr)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexSubImage3D.ptr)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
     #[inline] pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> ()>(storage::TransformFeedbackVaryings.ptr)(program, count, varyings, bufferMode) }
     #[inline] pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(storage::Uniform1f.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform1fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform1i(location: GLint, v0: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::Uniform1i.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform1iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>(storage::Uniform1ui.ptr)(location, v0) }
     #[inline] pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform1uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(storage::Uniform2f.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform2fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Uniform2i.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform2iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>(storage::Uniform2ui.ptr)(location, v0, v1) }
     #[inline] pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform2uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform3f.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform3fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Uniform3i.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform3iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>(storage::Uniform3ui.ptr)(location, v0, v1, v2) }
     #[inline] pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform3uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform4f.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform4fv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint) -> ()>(storage::Uniform4i.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform4iv.ptr)(location, count, value) }
     #[inline] pub unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::Uniform4ui.ptr)(location, v0, v1, v2, v3) }
     #[inline] pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform4uiv.ptr)(location, count, value) }
     #[inline] pub unsafe fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::UniformBlockBinding.ptr)(program, uniformBlockIndex, uniformBlockBinding) }
     #[inline] pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2x3fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2x4fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3x2fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3x4fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4x2fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4x3fv.ptr)(location, count, transpose, value) }
     #[inline] pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::UnmapBuffer.ptr)(target) }
     #[inline] pub unsafe fn UseProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::UseProgram.ptr)(program) }
     #[inline] pub unsafe fn ValidateProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgram.ptr)(program) }
     #[inline] pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(storage::VertexAttrib1d.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib1dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>(storage::VertexAttrib1f.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib1fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort) -> ()>(storage::VertexAttrib1s.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib1sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::VertexAttrib2d.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib2dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>(storage::VertexAttrib2f.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib2fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort) -> ()>(storage::VertexAttrib2s.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib2sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib3d.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib3dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib3f.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib3fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib3s.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib3sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4Nbv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4Niv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4Nsv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte) -> ()>(storage::VertexAttrib4Nub.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4Nubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4Nuiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4Nusv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4bv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib4d.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib4dv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib4f.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib4fv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib4s.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4ubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4usv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI1i(index: GLuint, x: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>(storage::VertexAttribI1i.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI1iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexAttribI1ui.ptr)(index, x) }
     #[inline] pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI1uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(storage::VertexAttribI2i.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttribI2iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI2iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI2ui.ptr)(index, x, y) }
     #[inline] pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI2uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(storage::VertexAttribI3i.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttribI3iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI3iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI3ui.ptr)(index, x, y, z) }
     #[inline] pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI3uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttribI4bv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(storage::VertexAttribI4i.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI4iv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttribI4sv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttribI4ubv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI4ui.ptr)(index, x, y, z, w) }
     #[inline] pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI4uiv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribI4usv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttribI4usv.ptr)(index, v) }
     #[inline] pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexAttribIPointer.ptr)(index, size, type_, stride, pointer) }
     #[inline] pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const std::os::raw::c_void) -> ()>(storage::VertexAttribPointer.ptr)(index, size, type_, normalized, stride, pointer) }
     #[inline] pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Viewport.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> () { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>(storage::WaitSync.ptr)(sync, flags, timeout) }
}

mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw;

     pub static mut ActiveTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut AttachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginConditionalRender: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BeginTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBufferBase: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindFragDataLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BindVertexArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendEquationSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlitFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CheckFramebufferStatus: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClampColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Clear: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferfi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearBufferuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearDepth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearStencil: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClientWaitSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMaski: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompileShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CompressedTexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyTexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CreateShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CullFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteFramebuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteQueries: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteRenderbuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteVertexArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DetachShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Disable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DisableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Disablei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawArraysInstanced: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsInstanced: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawElementsInstancedBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawRangeElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawRangeElementsBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Enable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EnableVertexAttribArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Enablei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndConditionalRender: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndTransformFeedback: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FenceSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Finish: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Flush: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FlushMappedBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTexture3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FramebufferTextureLayer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FrontFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenBuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenFramebuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenQueries: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenRenderbuffers: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenTextures: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenVertexArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenerateMipmap: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniform: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformBlockName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformBlockiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetActiveUniformsiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttachedShaders: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetAttribLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBooleani_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBooleanv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferParameteri64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBufferSubData: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetCompressedTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetDoublev: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetError: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFloatv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFragDataLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFramebufferAttachmentParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetInteger64i_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetInteger64v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetIntegeri_v: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetIntegerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMultisamplefv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetProgramiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjectiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryObjectuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetQueryiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetRenderbufferParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderInfoLog: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetShaderiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetString: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetStringi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetSynciv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTransformFeedbackVarying: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformBlockIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformIndices: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformLocation: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetUniformuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribPointerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetVertexAttribiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Hint: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsEnabled: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsEnabledi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsFramebuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsQuery: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsRenderbuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsShader: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsTexture: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsVertexArray: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LineWidth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LinkProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LogicOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapBufferRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawArrays: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawElements: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultiDrawElementsBaseVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStoref: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStorei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointSize: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonOffset: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PrimitiveRestartIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ProvokingVertex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RenderbufferStorage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RenderbufferStorageMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SampleCoverage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SampleMaski: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scissor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShaderSource: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFuncSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMaskSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOpSeparate: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage2DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage3DMultisample: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterIiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterIuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexSubImage3D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TransformFeedbackVaryings: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform1uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform2uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Uniform4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformBlockBinding: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2x3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix2x4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3x2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix3x4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4x2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UniformMatrix4x3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UnmapBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut UseProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ValidateProgram: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib1sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nbv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Niv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nsv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4Nusv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttrib4usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI1uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI2uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribI4usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribIPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut VertexAttribPointer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Viewport: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut WaitSync: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
}

pub fn load<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
    unsafe {
         storage::ActiveTexture.load(&mut loadfn, "glActiveTexture");
         storage::AttachShader.load(&mut loadfn, "glAttachShader");
         storage::BeginConditionalRender.load(&mut loadfn, "glBeginConditionalRender");
         storage::BeginQuery.load(&mut loadfn, "glBeginQuery");
         storage::BeginTransformFeedback.load(&mut loadfn, "glBeginTransformFeedback");
         storage::BindAttribLocation.load(&mut loadfn, "glBindAttribLocation");
         storage::BindBuffer.load(&mut loadfn, "glBindBuffer");
         storage::BindBufferBase.load(&mut loadfn, "glBindBufferBase");
         storage::BindBufferRange.load(&mut loadfn, "glBindBufferRange");
         storage::BindFragDataLocation.load(&mut loadfn, "glBindFragDataLocation");
         storage::BindFramebuffer.load(&mut loadfn, "glBindFramebuffer");
         storage::BindRenderbuffer.load(&mut loadfn, "glBindRenderbuffer");
         storage::BindTexture.load(&mut loadfn, "glBindTexture");
         storage::BindVertexArray.load(&mut loadfn, "glBindVertexArray");
         storage::BlendColor.load(&mut loadfn, "glBlendColor");
         storage::BlendEquation.load(&mut loadfn, "glBlendEquation");
         storage::BlendEquationSeparate.load(&mut loadfn, "glBlendEquationSeparate");
         storage::BlendFunc.load(&mut loadfn, "glBlendFunc");
         storage::BlendFuncSeparate.load(&mut loadfn, "glBlendFuncSeparate");
         storage::BlitFramebuffer.load(&mut loadfn, "glBlitFramebuffer");
         storage::BufferData.load(&mut loadfn, "glBufferData");
         storage::BufferSubData.load(&mut loadfn, "glBufferSubData");
         storage::CheckFramebufferStatus.load(&mut loadfn, "glCheckFramebufferStatus");
         storage::ClampColor.load(&mut loadfn, "glClampColor");
         storage::Clear.load(&mut loadfn, "glClear");
         storage::ClearBufferfi.load(&mut loadfn, "glClearBufferfi");
         storage::ClearBufferfv.load(&mut loadfn, "glClearBufferfv");
         storage::ClearBufferiv.load(&mut loadfn, "glClearBufferiv");
         storage::ClearBufferuiv.load(&mut loadfn, "glClearBufferuiv");
         storage::ClearColor.load(&mut loadfn, "glClearColor");
         storage::ClearDepth.load(&mut loadfn, "glClearDepth");
         storage::ClearStencil.load(&mut loadfn, "glClearStencil");
         storage::ClientWaitSync.load(&mut loadfn, "glClientWaitSync");
         storage::ColorMask.load(&mut loadfn, "glColorMask");
         storage::ColorMaski.load(&mut loadfn, "glColorMaski");
         storage::CompileShader.load(&mut loadfn, "glCompileShader");
         storage::CompressedTexImage1D.load(&mut loadfn, "glCompressedTexImage1D");
         storage::CompressedTexImage2D.load(&mut loadfn, "glCompressedTexImage2D");
         storage::CompressedTexImage3D.load(&mut loadfn, "glCompressedTexImage3D");
         storage::CompressedTexSubImage1D.load(&mut loadfn, "glCompressedTexSubImage1D");
         storage::CompressedTexSubImage2D.load(&mut loadfn, "glCompressedTexSubImage2D");
         storage::CompressedTexSubImage3D.load(&mut loadfn, "glCompressedTexSubImage3D");
         storage::CopyBufferSubData.load(&mut loadfn, "glCopyBufferSubData");
         storage::CopyTexImage1D.load(&mut loadfn, "glCopyTexImage1D");
         storage::CopyTexImage2D.load(&mut loadfn, "glCopyTexImage2D");
         storage::CopyTexSubImage1D.load(&mut loadfn, "glCopyTexSubImage1D");
         storage::CopyTexSubImage2D.load(&mut loadfn, "glCopyTexSubImage2D");
         storage::CopyTexSubImage3D.load(&mut loadfn, "glCopyTexSubImage3D");
         storage::CreateProgram.load(&mut loadfn, "glCreateProgram");
         storage::CreateShader.load(&mut loadfn, "glCreateShader");
         storage::CullFace.load(&mut loadfn, "glCullFace");
         storage::DeleteBuffers.load(&mut loadfn, "glDeleteBuffers");
         storage::DeleteFramebuffers.load(&mut loadfn, "glDeleteFramebuffers");
         storage::DeleteProgram.load(&mut loadfn, "glDeleteProgram");
         storage::DeleteQueries.load(&mut loadfn, "glDeleteQueries");
         storage::DeleteRenderbuffers.load(&mut loadfn, "glDeleteRenderbuffers");
         storage::DeleteShader.load(&mut loadfn, "glDeleteShader");
         storage::DeleteSync.load(&mut loadfn, "glDeleteSync");
         storage::DeleteTextures.load(&mut loadfn, "glDeleteTextures");
         storage::DeleteVertexArrays.load(&mut loadfn, "glDeleteVertexArrays");
         storage::DepthFunc.load(&mut loadfn, "glDepthFunc");
         storage::DepthMask.load(&mut loadfn, "glDepthMask");
         storage::DepthRange.load(&mut loadfn, "glDepthRange");
         storage::DetachShader.load(&mut loadfn, "glDetachShader");
         storage::Disable.load(&mut loadfn, "glDisable");
         storage::DisableVertexAttribArray.load(&mut loadfn, "glDisableVertexAttribArray");
         storage::Disablei.load(&mut loadfn, "glDisablei");
         storage::DrawArrays.load(&mut loadfn, "glDrawArrays");
         storage::DrawArraysInstanced.load(&mut loadfn, "glDrawArraysInstanced");
         storage::DrawBuffer.load(&mut loadfn, "glDrawBuffer");
         storage::DrawBuffers.load(&mut loadfn, "glDrawBuffers");
         storage::DrawElements.load(&mut loadfn, "glDrawElements");
         storage::DrawElementsBaseVertex.load(&mut loadfn, "glDrawElementsBaseVertex");
         storage::DrawElementsInstanced.load(&mut loadfn, "glDrawElementsInstanced");
         storage::DrawElementsInstancedBaseVertex.load(&mut loadfn, "glDrawElementsInstancedBaseVertex");
         storage::DrawRangeElements.load(&mut loadfn, "glDrawRangeElements");
         storage::DrawRangeElementsBaseVertex.load(&mut loadfn, "glDrawRangeElementsBaseVertex");
         storage::Enable.load(&mut loadfn, "glEnable");
         storage::EnableVertexAttribArray.load(&mut loadfn, "glEnableVertexAttribArray");
         storage::Enablei.load(&mut loadfn, "glEnablei");
         storage::EndConditionalRender.load(&mut loadfn, "glEndConditionalRender");
         storage::EndQuery.load(&mut loadfn, "glEndQuery");
         storage::EndTransformFeedback.load(&mut loadfn, "glEndTransformFeedback");
         storage::FenceSync.load(&mut loadfn, "glFenceSync");
         storage::Finish.load(&mut loadfn, "glFinish");
         storage::Flush.load(&mut loadfn, "glFlush");
         storage::FlushMappedBufferRange.load(&mut loadfn, "glFlushMappedBufferRange");
         storage::FramebufferRenderbuffer.load(&mut loadfn, "glFramebufferRenderbuffer");
         storage::FramebufferTexture.load(&mut loadfn, "glFramebufferTexture");
         storage::FramebufferTexture1D.load(&mut loadfn, "glFramebufferTexture1D");
         storage::FramebufferTexture2D.load(&mut loadfn, "glFramebufferTexture2D");
         storage::FramebufferTexture3D.load(&mut loadfn, "glFramebufferTexture3D");
         storage::FramebufferTextureLayer.load(&mut loadfn, "glFramebufferTextureLayer");
         storage::FrontFace.load(&mut loadfn, "glFrontFace");
         storage::GenBuffers.load(&mut loadfn, "glGenBuffers");
         storage::GenFramebuffers.load(&mut loadfn, "glGenFramebuffers");
         storage::GenQueries.load(&mut loadfn, "glGenQueries");
         storage::GenRenderbuffers.load(&mut loadfn, "glGenRenderbuffers");
         storage::GenTextures.load(&mut loadfn, "glGenTextures");
         storage::GenVertexArrays.load(&mut loadfn, "glGenVertexArrays");
         storage::GenerateMipmap.load(&mut loadfn, "glGenerateMipmap");
         storage::GetActiveAttrib.load(&mut loadfn, "glGetActiveAttrib");
         storage::GetActiveUniform.load(&mut loadfn, "glGetActiveUniform");
         storage::GetActiveUniformBlockName.load(&mut loadfn, "glGetActiveUniformBlockName");
         storage::GetActiveUniformBlockiv.load(&mut loadfn, "glGetActiveUniformBlockiv");
         storage::GetActiveUniformName.load(&mut loadfn, "glGetActiveUniformName");
         storage::GetActiveUniformsiv.load(&mut loadfn, "glGetActiveUniformsiv");
         storage::GetAttachedShaders.load(&mut loadfn, "glGetAttachedShaders");
         storage::GetAttribLocation.load(&mut loadfn, "glGetAttribLocation");
         storage::GetBooleani_v.load(&mut loadfn, "glGetBooleani_v");
         storage::GetBooleanv.load(&mut loadfn, "glGetBooleanv");
         storage::GetBufferParameteri64v.load(&mut loadfn, "glGetBufferParameteri64v");
         storage::GetBufferParameteriv.load(&mut loadfn, "glGetBufferParameteriv");
         storage::GetBufferPointerv.load(&mut loadfn, "glGetBufferPointerv");
         storage::GetBufferSubData.load(&mut loadfn, "glGetBufferSubData");
         storage::GetCompressedTexImage.load(&mut loadfn, "glGetCompressedTexImage");
         storage::GetDoublev.load(&mut loadfn, "glGetDoublev");
         storage::GetError.load(&mut loadfn, "glGetError");
         storage::GetFloatv.load(&mut loadfn, "glGetFloatv");
         storage::GetFragDataLocation.load(&mut loadfn, "glGetFragDataLocation");
         storage::GetFramebufferAttachmentParameteriv.load(&mut loadfn, "glGetFramebufferAttachmentParameteriv");
         storage::GetInteger64i_v.load(&mut loadfn, "glGetInteger64i_v");
         storage::GetInteger64v.load(&mut loadfn, "glGetInteger64v");
         storage::GetIntegeri_v.load(&mut loadfn, "glGetIntegeri_v");
         storage::GetIntegerv.load(&mut loadfn, "glGetIntegerv");
         storage::GetMultisamplefv.load(&mut loadfn, "glGetMultisamplefv");
         storage::GetProgramInfoLog.load(&mut loadfn, "glGetProgramInfoLog");
         storage::GetProgramiv.load(&mut loadfn, "glGetProgramiv");
         storage::GetQueryObjectiv.load(&mut loadfn, "glGetQueryObjectiv");
         storage::GetQueryObjectuiv.load(&mut loadfn, "glGetQueryObjectuiv");
         storage::GetQueryiv.load(&mut loadfn, "glGetQueryiv");
         storage::GetRenderbufferParameteriv.load(&mut loadfn, "glGetRenderbufferParameteriv");
         storage::GetShaderInfoLog.load(&mut loadfn, "glGetShaderInfoLog");
         storage::GetShaderSource.load(&mut loadfn, "glGetShaderSource");
         storage::GetShaderiv.load(&mut loadfn, "glGetShaderiv");
         storage::GetString.load(&mut loadfn, "glGetString");
         storage::GetStringi.load(&mut loadfn, "glGetStringi");
         storage::GetSynciv.load(&mut loadfn, "glGetSynciv");
         storage::GetTexImage.load(&mut loadfn, "glGetTexImage");
         storage::GetTexLevelParameterfv.load(&mut loadfn, "glGetTexLevelParameterfv");
         storage::GetTexLevelParameteriv.load(&mut loadfn, "glGetTexLevelParameteriv");
         storage::GetTexParameterIiv.load(&mut loadfn, "glGetTexParameterIiv");
         storage::GetTexParameterIuiv.load(&mut loadfn, "glGetTexParameterIuiv");
         storage::GetTexParameterfv.load(&mut loadfn, "glGetTexParameterfv");
         storage::GetTexParameteriv.load(&mut loadfn, "glGetTexParameteriv");
         storage::GetTransformFeedbackVarying.load(&mut loadfn, "glGetTransformFeedbackVarying");
         storage::GetUniformBlockIndex.load(&mut loadfn, "glGetUniformBlockIndex");
         storage::GetUniformIndices.load(&mut loadfn, "glGetUniformIndices");
         storage::GetUniformLocation.load(&mut loadfn, "glGetUniformLocation");
         storage::GetUniformfv.load(&mut loadfn, "glGetUniformfv");
         storage::GetUniformiv.load(&mut loadfn, "glGetUniformiv");
         storage::GetUniformuiv.load(&mut loadfn, "glGetUniformuiv");
         storage::GetVertexAttribIiv.load(&mut loadfn, "glGetVertexAttribIiv");
         storage::GetVertexAttribIuiv.load(&mut loadfn, "glGetVertexAttribIuiv");
         storage::GetVertexAttribPointerv.load(&mut loadfn, "glGetVertexAttribPointerv");
         storage::GetVertexAttribdv.load(&mut loadfn, "glGetVertexAttribdv");
         storage::GetVertexAttribfv.load(&mut loadfn, "glGetVertexAttribfv");
         storage::GetVertexAttribiv.load(&mut loadfn, "glGetVertexAttribiv");
         storage::Hint.load(&mut loadfn, "glHint");
         storage::IsBuffer.load(&mut loadfn, "glIsBuffer");
         storage::IsEnabled.load(&mut loadfn, "glIsEnabled");
         storage::IsEnabledi.load(&mut loadfn, "glIsEnabledi");
         storage::IsFramebuffer.load(&mut loadfn, "glIsFramebuffer");
         storage::IsProgram.load(&mut loadfn, "glIsProgram");
         storage::IsQuery.load(&mut loadfn, "glIsQuery");
         storage::IsRenderbuffer.load(&mut loadfn, "glIsRenderbuffer");
         storage::IsShader.load(&mut loadfn, "glIsShader");
         storage::IsSync.load(&mut loadfn, "glIsSync");
         storage::IsTexture.load(&mut loadfn, "glIsTexture");
         storage::IsVertexArray.load(&mut loadfn, "glIsVertexArray");
         storage::LineWidth.load(&mut loadfn, "glLineWidth");
         storage::LinkProgram.load(&mut loadfn, "glLinkProgram");
         storage::LogicOp.load(&mut loadfn, "glLogicOp");
         storage::MapBuffer.load(&mut loadfn, "glMapBuffer");
         storage::MapBufferRange.load(&mut loadfn, "glMapBufferRange");
         storage::MultiDrawArrays.load(&mut loadfn, "glMultiDrawArrays");
         storage::MultiDrawElements.load(&mut loadfn, "glMultiDrawElements");
         storage::MultiDrawElementsBaseVertex.load(&mut loadfn, "glMultiDrawElementsBaseVertex");
         storage::PixelStoref.load(&mut loadfn, "glPixelStoref");
         storage::PixelStorei.load(&mut loadfn, "glPixelStorei");
         storage::PointParameterf.load(&mut loadfn, "glPointParameterf");
         storage::PointParameterfv.load(&mut loadfn, "glPointParameterfv");
         storage::PointParameteri.load(&mut loadfn, "glPointParameteri");
         storage::PointParameteriv.load(&mut loadfn, "glPointParameteriv");
         storage::PointSize.load(&mut loadfn, "glPointSize");
         storage::PolygonMode.load(&mut loadfn, "glPolygonMode");
         storage::PolygonOffset.load(&mut loadfn, "glPolygonOffset");
         storage::PrimitiveRestartIndex.load(&mut loadfn, "glPrimitiveRestartIndex");
         storage::ProvokingVertex.load(&mut loadfn, "glProvokingVertex");
         storage::ReadBuffer.load(&mut loadfn, "glReadBuffer");
         storage::ReadPixels.load(&mut loadfn, "glReadPixels");
         storage::RenderbufferStorage.load(&mut loadfn, "glRenderbufferStorage");
         storage::RenderbufferStorageMultisample.load(&mut loadfn, "glRenderbufferStorageMultisample");
         storage::SampleCoverage.load(&mut loadfn, "glSampleCoverage");
         storage::SampleMaski.load(&mut loadfn, "glSampleMaski");
         storage::Scissor.load(&mut loadfn, "glScissor");
         storage::ShaderSource.load(&mut loadfn, "glShaderSource");
         storage::StencilFunc.load(&mut loadfn, "glStencilFunc");
         storage::StencilFuncSeparate.load(&mut loadfn, "glStencilFuncSeparate");
         storage::StencilMask.load(&mut loadfn, "glStencilMask");
         storage::StencilMaskSeparate.load(&mut loadfn, "glStencilMaskSeparate");
         storage::StencilOp.load(&mut loadfn, "glStencilOp");
         storage::StencilOpSeparate.load(&mut loadfn, "glStencilOpSeparate");
         storage::TexBuffer.load(&mut loadfn, "glTexBuffer");
         storage::TexImage1D.load(&mut loadfn, "glTexImage1D");
         storage::TexImage2D.load(&mut loadfn, "glTexImage2D");
         storage::TexImage2DMultisample.load(&mut loadfn, "glTexImage2DMultisample");
         storage::TexImage3D.load(&mut loadfn, "glTexImage3D");
         storage::TexImage3DMultisample.load(&mut loadfn, "glTexImage3DMultisample");
         storage::TexParameterIiv.load(&mut loadfn, "glTexParameterIiv");
         storage::TexParameterIuiv.load(&mut loadfn, "glTexParameterIuiv");
         storage::TexParameterf.load(&mut loadfn, "glTexParameterf");
         storage::TexParameterfv.load(&mut loadfn, "glTexParameterfv");
         storage::TexParameteri.load(&mut loadfn, "glTexParameteri");
         storage::TexParameteriv.load(&mut loadfn, "glTexParameteriv");
         storage::TexSubImage1D.load(&mut loadfn, "glTexSubImage1D");
         storage::TexSubImage2D.load(&mut loadfn, "glTexSubImage2D");
         storage::TexSubImage3D.load(&mut loadfn, "glTexSubImage3D");
         storage::TransformFeedbackVaryings.load(&mut loadfn, "glTransformFeedbackVaryings");
         storage::Uniform1f.load(&mut loadfn, "glUniform1f");
         storage::Uniform1fv.load(&mut loadfn, "glUniform1fv");
         storage::Uniform1i.load(&mut loadfn, "glUniform1i");
         storage::Uniform1iv.load(&mut loadfn, "glUniform1iv");
         storage::Uniform1ui.load(&mut loadfn, "glUniform1ui");
         storage::Uniform1uiv.load(&mut loadfn, "glUniform1uiv");
         storage::Uniform2f.load(&mut loadfn, "glUniform2f");
         storage::Uniform2fv.load(&mut loadfn, "glUniform2fv");
         storage::Uniform2i.load(&mut loadfn, "glUniform2i");
         storage::Uniform2iv.load(&mut loadfn, "glUniform2iv");
         storage::Uniform2ui.load(&mut loadfn, "glUniform2ui");
         storage::Uniform2uiv.load(&mut loadfn, "glUniform2uiv");
         storage::Uniform3f.load(&mut loadfn, "glUniform3f");
         storage::Uniform3fv.load(&mut loadfn, "glUniform3fv");
         storage::Uniform3i.load(&mut loadfn, "glUniform3i");
         storage::Uniform3iv.load(&mut loadfn, "glUniform3iv");
         storage::Uniform3ui.load(&mut loadfn, "glUniform3ui");
         storage::Uniform3uiv.load(&mut loadfn, "glUniform3uiv");
         storage::Uniform4f.load(&mut loadfn, "glUniform4f");
         storage::Uniform4fv.load(&mut loadfn, "glUniform4fv");
         storage::Uniform4i.load(&mut loadfn, "glUniform4i");
         storage::Uniform4iv.load(&mut loadfn, "glUniform4iv");
         storage::Uniform4ui.load(&mut loadfn, "glUniform4ui");
         storage::Uniform4uiv.load(&mut loadfn, "glUniform4uiv");
         storage::UniformBlockBinding.load(&mut loadfn, "glUniformBlockBinding");
         storage::UniformMatrix2fv.load(&mut loadfn, "glUniformMatrix2fv");
         storage::UniformMatrix2x3fv.load(&mut loadfn, "glUniformMatrix2x3fv");
         storage::UniformMatrix2x4fv.load(&mut loadfn, "glUniformMatrix2x4fv");
         storage::UniformMatrix3fv.load(&mut loadfn, "glUniformMatrix3fv");
         storage::UniformMatrix3x2fv.load(&mut loadfn, "glUniformMatrix3x2fv");
         storage::UniformMatrix3x4fv.load(&mut loadfn, "glUniformMatrix3x4fv");
         storage::UniformMatrix4fv.load(&mut loadfn, "glUniformMatrix4fv");
         storage::UniformMatrix4x2fv.load(&mut loadfn, "glUniformMatrix4x2fv");
         storage::UniformMatrix4x3fv.load(&mut loadfn, "glUniformMatrix4x3fv");
         storage::UnmapBuffer.load(&mut loadfn, "glUnmapBuffer");
         storage::UseProgram.load(&mut loadfn, "glUseProgram");
         storage::ValidateProgram.load(&mut loadfn, "glValidateProgram");
         storage::VertexAttrib1d.load(&mut loadfn, "glVertexAttrib1d");
         storage::VertexAttrib1dv.load(&mut loadfn, "glVertexAttrib1dv");
         storage::VertexAttrib1f.load(&mut loadfn, "glVertexAttrib1f");
         storage::VertexAttrib1fv.load(&mut loadfn, "glVertexAttrib1fv");
         storage::VertexAttrib1s.load(&mut loadfn, "glVertexAttrib1s");
         storage::VertexAttrib1sv.load(&mut loadfn, "glVertexAttrib1sv");
         storage::VertexAttrib2d.load(&mut loadfn, "glVertexAttrib2d");
         storage::VertexAttrib2dv.load(&mut loadfn, "glVertexAttrib2dv");
         storage::VertexAttrib2f.load(&mut loadfn, "glVertexAttrib2f");
         storage::VertexAttrib2fv.load(&mut loadfn, "glVertexAttrib2fv");
         storage::VertexAttrib2s.load(&mut loadfn, "glVertexAttrib2s");
         storage::VertexAttrib2sv.load(&mut loadfn, "glVertexAttrib2sv");
         storage::VertexAttrib3d.load(&mut loadfn, "glVertexAttrib3d");
         storage::VertexAttrib3dv.load(&mut loadfn, "glVertexAttrib3dv");
         storage::VertexAttrib3f.load(&mut loadfn, "glVertexAttrib3f");
         storage::VertexAttrib3fv.load(&mut loadfn, "glVertexAttrib3fv");
         storage::VertexAttrib3s.load(&mut loadfn, "glVertexAttrib3s");
         storage::VertexAttrib3sv.load(&mut loadfn, "glVertexAttrib3sv");
         storage::VertexAttrib4Nbv.load(&mut loadfn, "glVertexAttrib4Nbv");
         storage::VertexAttrib4Niv.load(&mut loadfn, "glVertexAttrib4Niv");
         storage::VertexAttrib4Nsv.load(&mut loadfn, "glVertexAttrib4Nsv");
         storage::VertexAttrib4Nub.load(&mut loadfn, "glVertexAttrib4Nub");
         storage::VertexAttrib4Nubv.load(&mut loadfn, "glVertexAttrib4Nubv");
         storage::VertexAttrib4Nuiv.load(&mut loadfn, "glVertexAttrib4Nuiv");
         storage::VertexAttrib4Nusv.load(&mut loadfn, "glVertexAttrib4Nusv");
         storage::VertexAttrib4bv.load(&mut loadfn, "glVertexAttrib4bv");
         storage::VertexAttrib4d.load(&mut loadfn, "glVertexAttrib4d");
         storage::VertexAttrib4dv.load(&mut loadfn, "glVertexAttrib4dv");
         storage::VertexAttrib4f.load(&mut loadfn, "glVertexAttrib4f");
         storage::VertexAttrib4fv.load(&mut loadfn, "glVertexAttrib4fv");
         storage::VertexAttrib4iv.load(&mut loadfn, "glVertexAttrib4iv");
         storage::VertexAttrib4s.load(&mut loadfn, "glVertexAttrib4s");
         storage::VertexAttrib4sv.load(&mut loadfn, "glVertexAttrib4sv");
         storage::VertexAttrib4ubv.load(&mut loadfn, "glVertexAttrib4ubv");
         storage::VertexAttrib4uiv.load(&mut loadfn, "glVertexAttrib4uiv");
         storage::VertexAttrib4usv.load(&mut loadfn, "glVertexAttrib4usv");
         storage::VertexAttribI1i.load(&mut loadfn, "glVertexAttribI1i");
         storage::VertexAttribI1iv.load(&mut loadfn, "glVertexAttribI1iv");
         storage::VertexAttribI1ui.load(&mut loadfn, "glVertexAttribI1ui");
         storage::VertexAttribI1uiv.load(&mut loadfn, "glVertexAttribI1uiv");
         storage::VertexAttribI2i.load(&mut loadfn, "glVertexAttribI2i");
         storage::VertexAttribI2iv.load(&mut loadfn, "glVertexAttribI2iv");
         storage::VertexAttribI2ui.load(&mut loadfn, "glVertexAttribI2ui");
         storage::VertexAttribI2uiv.load(&mut loadfn, "glVertexAttribI2uiv");
         storage::VertexAttribI3i.load(&mut loadfn, "glVertexAttribI3i");
         storage::VertexAttribI3iv.load(&mut loadfn, "glVertexAttribI3iv");
         storage::VertexAttribI3ui.load(&mut loadfn, "glVertexAttribI3ui");
         storage::VertexAttribI3uiv.load(&mut loadfn, "glVertexAttribI3uiv");
         storage::VertexAttribI4bv.load(&mut loadfn, "glVertexAttribI4bv");
         storage::VertexAttribI4i.load(&mut loadfn, "glVertexAttribI4i");
         storage::VertexAttribI4iv.load(&mut loadfn, "glVertexAttribI4iv");
         storage::VertexAttribI4sv.load(&mut loadfn, "glVertexAttribI4sv");
         storage::VertexAttribI4ubv.load(&mut loadfn, "glVertexAttribI4ubv");
         storage::VertexAttribI4ui.load(&mut loadfn, "glVertexAttribI4ui");
         storage::VertexAttribI4uiv.load(&mut loadfn, "glVertexAttribI4uiv");
         storage::VertexAttribI4usv.load(&mut loadfn, "glVertexAttribI4usv");
         storage::VertexAttribIPointer.load(&mut loadfn, "glVertexAttribIPointer");
         storage::VertexAttribPointer.load(&mut loadfn, "glVertexAttribPointer");
         storage::Viewport.load(&mut loadfn, "glViewport");
         storage::WaitSync.load(&mut loadfn, "glWaitSync");

    }
}

