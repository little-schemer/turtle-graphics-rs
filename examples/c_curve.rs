//!
//! C 曲線
//!

use clap::Parser;
use turtle::*;
use turtle_graphics::*;
use turtle_graphics::recursive_figure::*;


// コマンドライン処理
#[derive(Parser)]
#[command(
    name  = "c_curve",
    about = "Turtle Graphics による C 曲線"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "再帰のレベル",
        default_value = "10",
        value_name    = "LEVEL"
    )]
    level: usize,

    #[arg(
        short,
        long,
        help = "レベル 0 の辺の長さ",
        default_value = "300.0",
        value_name    = "SIZE"
    )]
    size: f64,
}


fn main() {
    let args = Args::parse();

    let mut drawing = Drawing::new();
    let mut turtle  = drawing.add_turtle();

    drawing.set_size((800, 600));
    drawing.set_title("C Curve");
    turtle.set_speed("instant");

    turtle.set_start((-args.size / 2.0, -args.size / 3.0), 0.0);

    turtle.c_curve(args.size, args.level);
}
