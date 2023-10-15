use nannou::prelude::*;
use test_lib::prelude::*;


const INPUT_FILE: &str = "./files/input.txt";
const OUTPUT_FILE: &str = "./files/output.txt";

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    // Background Color
    draw.background().color(CORNFLOWERBLUE);

    draw.to_frame(app, &frame).unwrap();
}
