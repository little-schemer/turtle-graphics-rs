//!
//! Tree
//!

use clap::Parser;
use turtle::*;
use turtle_graphics::*;
use turtle_graphics::recursive_figure::*;


// コマンドライン処理
#[derive(Parser)]
#[command(
    name  = "tree",
    about = "Turtle Graphics による木構造"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "再帰のレベル",
        default_value = "7",
        value_name    = "LEVEL"
    )]
    level: usize,

    #[arg(
        short,
        long,
        help = "レベル 0 の枝の長さ",
        default_value = "80.0",
        value_name    = "SIZE"
    )]
    size: f64,

    #[arg(
        short,
        long,
        help = "枝の角度",
        default_value = "30.0",
        value_name    = "ANGLE"
    )]
    angle: f64,
}


fn main() {
    let args = Args::parse();

    let mut drawing = Drawing::new();
    let mut turtle  = drawing.add_turtle();

    drawing.set_size((800, 600));
    drawing.set_title("Tree");
    turtle.set_speed(25);

    turtle.set_start((0.0, -200.0), 90.0);

    turtle.tree(args.size, args.angle, args.level);
}
