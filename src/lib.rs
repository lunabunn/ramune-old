mod game;
pub use game::*;

pub mod graphics {
    pub(crate) mod backend_opengl {
        mod game;
        pub use game::*;
    }
}