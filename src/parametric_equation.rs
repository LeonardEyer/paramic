pub trait ParametricEquation {
    fn get_position(&self, t: f64) -> (f64, f64);

    fn get_period(&self) -> f64;
}

#[derive(Debug, Clone)]
pub struct EquationA {
    pub a: i32,
    pub b : i32,
    pub c : i32,
    pub d : i32,
    pub j : i32,
    pub k : i32,
}

impl ParametricEquation for EquationA {
    /// Taken from https://en.wikipedia.org/w/index.php?title=Parametric_equation&diff=prev&oldid=1145357253
    fn get_position(&self, t: f64) -> (f64, f64) {
        let x = (t * self.a as f64).cos() - (t * self.b as f64).cos().powi(self.j);
        let y = (t * self.c as f64).sin() - (t * self.d as f64).sin().powi(self.k);
        (x, y)
    }

    fn get_period(&self) -> f64 {
        for i in 1..1000 {
            let (x, y) = self.get_position(i as f64);
            if x > 0.0 && y > 0.0 {
                return i as f64;
            }
        }
        1000.0
    }
}