use turtle::*;

pub mod recursive_figure;


pub trait Tool {
    fn set_start<P: Into<Point>>(&mut self, position: P, angle: Angle);
}


impl Tool for Turtle {
    fn set_start<P: Into<Point>>(&mut self, position: P, angle: Angle) {
        self.hide();
        self.pen_up();
        self.go_to(position);
        self.set_heading(angle);
        self.pen_down();
    }
}
