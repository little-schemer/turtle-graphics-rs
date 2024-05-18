use turtle::*;

pub trait RecursiveFigure {
    fn koch_curve(&mut self, len: f64, n: usize);
}

impl RecursiveFigure for Turtle {
    fn koch_curve(&mut self, len: f64, n: usize) {
        if n > 0 {
            let len = len / 3.0;
            self.koch_curve(len, n - 1);
            self.left(60.0);
            self.koch_curve(len, n - 1);
            self.right(120.0);
            self.koch_curve(len, n - 1);
            self.left(60.0);
            self.koch_curve(len, n - 1);
        } else {
            self.forward(len);
        }
    }
}
