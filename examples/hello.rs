use ramune::graphics::Graphics;

fn main() {
    let options = ramune::GameOptions {
        title: "Hello, Ramune!",
        ..Default::default()
    };

    ramune::run(options, |e| match e {
        ramune::Event::Draw(ctx) => {
            let g = &mut ctx.graphics;
            g.fill_rect(50.0, 50.0, 100.0, 100.0);
            g.flush(Some(ramune::graphics::Color::rgb(0.5, 0.7, 0.3)));
        },
        _ => {}
    });
}
