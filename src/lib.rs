#![allow(irrefutable_let_patterns)]
#![allow(non_upper_case_globals)]

mod game;
pub use game::*;
mod context;
pub use context::*;

pub mod graphics {
    mod graphics;
    pub use graphics::*;

    pub(crate) mod backend_opengl {
        mod game;
        pub use game::*;
        mod bindings {
            pub mod gl32;
            pub mod gles20;
        }
        pub mod gl;
        pub mod graphics;
    }
}