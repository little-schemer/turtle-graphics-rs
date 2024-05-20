use clap::Parser;
use turtle::*;


trait Figure {
    fn spiral(&mut self, size: f64, angle: f64, n: usize);
}


impl Figure for Turtle {
    fn spiral(&mut self, mut size: f64, angle: f64, n: usize) {
        for _ in 0..n {
            self.forward(size);
            self.right(angle);
            size *= 1.02;
        }
    }
}


#[derive(Parser)]
#[command(
    name  = "spiral1",
    about = "螺旋"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "繰り返しの回数",
        default_value = "250",
        value_name    = "TIMES"
    )]
    times: usize,

    #[arg(
        short,
        long,
        help = "開始時の辺の長さ",
        default_value = "5.0",
        value_name    = "SIZE"
    )]
    size: f64,

    #[arg(
        short,
        long,
        help = "角度",
        default_value = "91.3",
        value_name    = "ANGLE"
    )]
    angle: f64,
}


fn main() {
    let args = Args::parse();

    let mut drawing = Drawing::new();
    let mut turtle  = drawing.add_turtle();

    drawing.set_size((800, 600));
    drawing.set_title("Spiral");
    turtle.set_speed(25);

    turtle.spiral(args.size, args.angle, args.times);
    turtle.hide();
}
