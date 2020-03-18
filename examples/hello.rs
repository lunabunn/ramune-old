fn main() {
    let options = ramune::GameOptions {
        title: "Hello, Ramune!",
        ..Default::default()
    };

    let mut knights = None;
    let mut counter = 0;

    ramune::run(options, move |e| match e {
        ramune::Event::Init(ctx) => {
            let img = image::open(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("../../../res/knights.png"),
            )
            .unwrap()
            .into_rgba();
            let dim = img.dimensions();
            knights = Some(ramune::graphics::Texture::new(
                ctx,
                dim.0,
                dim.1,
                &img.pixels()
                    .map(|x| ramune::graphics::Color::rgba(x[0], x[1], x[2], x[3]))
                    .collect::<Vec<ramune::graphics::Color>>(),
            ));
        }
        ramune::Event::Update(_) => {
            counter += 1;
        }
        ramune::Event::Draw(ctx, g) => {
            g.draw_subimage_scaled(
                knights.unwrap(),
                50.0,
                50.0,
                160.0,
                160.0,
                (counter as f32 / 30.0).floor() % 5.0 * 16.0,
                0.0,
                16.0,
                16.0,
            );
            g.flush(ctx, Some(ramune::graphics::Color::rgb(128, 179, 77)));
        }
        _ => {}
    });
}
