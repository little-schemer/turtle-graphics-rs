use turtle::*;


pub trait RecursiveFigure {
    fn koch_curve(&mut self, len: f64, n: usize);
    fn c_curve(&mut self, len: f64, n: usize);
    fn cross_stitch(&mut self, len: f64, n:usize);
    fn tree(&mut self, len: f64, angle: f64, level: usize);
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


    fn c_curve(&mut self, len: f64, n: usize) {
        if n > 0 {
            let len = len / 2.0_f64.sqrt();
            self.left(45.0);
            self.c_curve(len, n - 1);
            self.right(90.0);
            self.c_curve(len, n - 1);
            self.left(45.0);
        } else {
            self.forward(len);
        }
    }


    fn cross_stitch(&mut self, len: f64, n: usize) {
        if n > 0 {
            let len = len / 3.0;
            self.cross_stitch(len, n - 1);
            self.left(90.0);
            self.cross_stitch(len, n - 1);
            self.right(90.0);
            self.cross_stitch(len, n - 1);
            self.right(90.0);
            self.cross_stitch(len, n - 1);
            self.left(90.0);
            self.cross_stitch(len, n - 1);
        } else {
            self.forward(len);
        }
    }


    fn tree(&mut self, len: f64, angle: f64, level: usize) {
        if level > 0 {
            self.forward(len);

            // 左の枝
            self.left(angle);
            self.tree(len * 0.9, angle, level - 1);

            // 右の枝
            self.right(angle * 2.0);
            self.tree(len * 0.9, angle, level - 1);

            // 始点に戻る
            self.left(angle);
            self.backward(len);
        }
    }
}
