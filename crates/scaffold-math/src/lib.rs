pub struct Calc;

impl Calc {
    pub fn add(a: i32, b: i32) -> i32 {
        a.wrapping_add(b)
    }
}

pub struct Engine {
    factor: i32,
    acc: i64,
}

impl Engine {
    pub fn new() -> Self {
        Self { factor: 1, acc: 0 }
    }

    pub fn set_factor(&mut self, factor: i32) {
        self.factor = factor;
    }

    pub fn apply(&mut self, x: i64) -> i64 {
        self.acc = self.acc.wrapping_add(x.wrapping_mul(self.factor as i64));
        self.acc
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(Calc::add(2, 3), 5);
        assert_eq!(Calc::add(-1, 1), 0);
    }

    #[test]
    fn engine_accumulates() {
        let mut engine = Engine::new();
        engine.set_factor(3);
        assert_eq!(engine.apply(2), 6);
        assert_eq!(engine.apply(2), 12);
    }
}
