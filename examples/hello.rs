fn main() {
    let options = ramune::GameOptions {
        title: "Hello, Ramune!",
        ..Default::default()
    };

    ramune::run(options, |e| {
        println!("{:?}", e);
    });
}
