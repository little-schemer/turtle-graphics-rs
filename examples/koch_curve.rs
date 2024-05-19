//!
//! Koch 曲線
//!

use clap::Parser;
use turtle::*;
use turtle_graphics::*;
use turtle_graphics::recursive_figure::*;


// コマンドライン処理
#[derive(Parser)]
#[command(
    name  = "koch_curve",
    about = "Turtle Graphics による Koch 曲線"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "再帰のレベル",
        default_value = "2",
        value_name    = "LEVEL"
    )]
    level: usize,

    #[arg(
        short,
        long,
        help = "レベル 0 の辺の長さ",
        default_value = "450.0",
        value_name    = "SIZE"
    )]
    size: f64,
}


fn main() {
    let args = Args::parse();

    let mut drawing = Drawing::new();
    let mut turtle  = drawing.add_turtle();

    drawing.set_size((800, 600));
    drawing.set_title("Koch Curve");
    turtle.set_speed("instant");

    turtle.set_start((-args.size / 2.0, args.size / 2.0 / 3.0_f64.sqrt()), 0.0);

    for _ in 0..3 {
        turtle.koch_curve(args.size, args.level);
        turtle.right(120.0);
    }
}
