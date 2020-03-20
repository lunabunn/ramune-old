#![allow(irrefutable_let_patterns)]
#![allow(non_upper_case_globals)]

mod game;
pub use game::*;
mod context;
pub use context::*;

pub mod graphics {
    mod graphics;
    pub use graphics::*;
    mod texture;
    pub use texture::*;

    pub(crate) use backend_opengl as backend;
    pub use backend::{GraphicsContext, Renderer, run};

    pub(crate) mod backend_opengl {
        mod game;
        pub use game::*;
        mod graphics;
        pub use graphics::*;

        mod bindings {
            pub mod gl33;
            pub mod gles20;
        }
        pub mod gl;
    }
}
