use nannou::prelude::*;
use test_lib::prelude::*;

const INPUT_FILE: &str = "./assets/input.txt";
const OUTPUT_FILE: &str = "./assets/output.txt";

fn main() { nannou::sketch(view).run(); }

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    // Background Color
    draw.background().color(CORNFLOWERBLUE);

    draw.to_frame(app, &frame).unwrap();
}
