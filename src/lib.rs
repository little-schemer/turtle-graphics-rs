use turtle::*;


pub trait RecursiveFigure {
    fn set_start<P: Into<Point>>(&mut self, position: P, angle: Angle);
}

impl RecursiveFigure for Turtle {
    fn set_start<P: Into<Point>>(&mut self, position: P, angle: Angle) {
        self.hide();
        self.pen_up();
        self.go_to(position);
        self.set_heading(angle);
        self.pen_down();
    }
}
