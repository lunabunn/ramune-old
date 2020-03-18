use crate::graphics::{GraphicsHandle, TextureManagerHandle};

pub struct Context {
    pub graphics: GraphicsHandle,
    pub(crate) texture_manager: TextureManagerHandle,
}
