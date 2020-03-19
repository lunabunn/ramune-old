use crate::graphics;
use crate::graphics::Graphics;
use crate::Context;

/// Runs the game with the given `options`, firing `callback` upon game loop events.
///
/// # Examples
/// ```rust
/// use ramune::{GameOptions, Event};
/// use ramune::graphics::{Graphics, Color};
///
/// fn main() {
///     let options = GameOptions {
///         title: "My Awesome Game",
///         ..Default::default()
///     };
///
///     ramune::run(options, |e| match e {
///         Event::Draw(ctx) => {
///             let g = &mut ctx.graphics;
///             g.fill_rect(50.0, 50.0, 100.0, 100.0);
///             g.flush(Some(Color::rgb(0.5, 0.7, 0.3)));
///         }
///         _ => { }
///     });
/// }
/// ```
pub fn run(options: GameOptions, callback: impl FnMut(Event) + 'static) {
    graphics::backend_opengl::run(options, callback);
}

/// Options for initializing the game.
pub struct GameOptions {
    /// The title of the window.
    pub title: &'static str,
    /// The size of the window.
    pub size: (u32, u32),
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            title: "Ramune",
            size: (1024, 786),
        }
    }
}

/// An event in the game loop.
pub enum Event<'a> {
    /// Event fired once on initialization.
    Init(&'a mut Context),
    /// Event fired every frame for game logic.
    Update(&'a mut Context),
    /// Event fired every frame for rendering.
    Draw(&'a mut Context, &'a mut Graphics),
    /// Event fired on window resize.
    WindowResized(&'a mut Context),
}
