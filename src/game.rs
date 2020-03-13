use crate::graphics;

/// Runs the game with the given `options`, firing `callback` upon game loop events.
///
/// # Examples
/// ```rust
/// use ramune::{GameOptions, Event};
/// use ramune::graphics::Color;
/// 
/// fn main() {
///     let options = GameOptions {
///         title: "My Awesome Game",
///         ..Default::default()
///     };
/// 
///     ramune::run(options, |e| match e {
///         Event::Render(ctx, frames) => {
///             let g = &mut ctx.graphics;
///             g.begin(frames[0], Some(Color::rgb(0.5, 0.3, 0.7)));
///             g.fill_rect(50, 50, 100, 100);
///             g.end();
///         },
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
#[derive(Debug)]
pub enum Event {
    /// Event fired once on initialization.
    Init(),
    /// Event fired every frame for game logic.
    Update(),
    /// Event fired every frame for rendering.
    Draw(),
    /// Event fired on window resize.
    WindowResized(),
}