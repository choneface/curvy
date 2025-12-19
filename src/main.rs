use curvy::{run, AppRunner, Image, RunConfig, View};

fn main() {
    let image = Image::from_file("src/image.ppm").expect("Failed to load image");
    let (width, height) = image.size();

    let app = AppRunner::new(image);

    run(app, RunConfig {
        width,
        height,
        resizable: false,
    });
}
