//! A structure representing an interval.

use std::ops::RangeInclusive;

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

    pub const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
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

    pub fn from_intervals(a: Self, b: Self) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn clamp<T: Into<f64>>(&self, t: T) -> f64 {
        let t = t.into();

        if t < self.min {
            self.min
        } else if t > self.max {
            self.max
        } else {
            t
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn to_range(&self) -> RangeInclusive<f64> {
        self.min..=self.max
    }

    pub fn new<X: Into<f64>, Y: Into<f64>>(min: X, max: Y) -> Self {
        Interval {
            min: min.into(),
            max: max.into(),
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: 0.0, max: 0.0 }
    }
}

#[macro_export]
macro_rules! interval {
    ($min:expr, $max:expr) => {
        Interval::new($min, $max)
    };
}
