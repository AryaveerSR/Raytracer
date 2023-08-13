//! A structure representing an interval.

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub fn contains<T: Into<f64>>(&self, t: T) -> bool {
        let t = t.into();
        (self.min <= t) & (self.max >= t)
    }

    pub fn surrounds<T: Into<f64>>(&self, t: T) -> bool {
        let t = t.into();
        (self.min < t) & (self.max > t)
    }

    pub fn excludes<T: Into<f64>>(&self, t: T) -> bool {
        !self.surrounds(t)
    }

    pub fn new<X: Into<f64>, Y: Into<f64>>(min: X, max: Y) -> Self {
        Interval {
            min: min.into(),
            max: max.into(),
        }
    }
}