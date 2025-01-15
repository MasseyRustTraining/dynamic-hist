use std::marker::PhantomData;
use std::ops::Range;

/// Record of samples. Each bin in the record
/// represents its fraction of the space 0.0..1.0.
pub struct Hist<T> {
    bins: Vec<usize>,
    count: usize,
    start: f64,
    end: f64,
    _sample_type: PhantomData<T>,
}

/// Keeps track of which bin samples fall in and report as
/// needed.
impl<T: Into<f64>> Hist<T> {
    /// Create a new histogram with `n` bins, accepting samples
    /// in `range` `[start..end)`.
    pub fn new(n: usize, range: Range<T>) -> Self {
        Hist {
            bins: vec![0; n],
            count: 0,
            start: range.start.into(),
            end: range.end.into(),
            _sample_type: PhantomData,
        }
    }
}

impl<T> Hist<T> {
    /// Return a slice containing the counts in the histogram.
    pub fn counts(&self) -> &[usize] {
        &self.bins
    }

    /// Return the number of samples taken.
    pub fn total_count(&self) -> usize {
        self.count
    }

    /// Return the lowest bin number, if any,
    /// containing a non-zero count.
    pub fn min(&self) -> Option<usize> {
        for (i, &x) in self.bins.iter().enumerate() {
            if x > 0 {
                return Some(i);
            }
        }
        None
    }

    /// Return the highest bin number, if any,
    /// containing a non-zero count.
    pub fn max(&self) -> Option<usize> {
        for (i, &x) in self.bins.iter().enumerate().rev() {
            if x > 0 {
                return Some(i);
            }
        }
        None
    }
}

impl<T: Into<f64>> Hist<T> {
    /// Count a new sample.
    ///
    /// # Panics
    ///
    /// Panics if sample is outside the range specified when
    /// creating the histogram.
    ///
    /// # Note
    ///
    /// Bin boundaries are computed using `f64` arithmetic.
    /// Slight rounding error may occur.
    pub fn sample(&mut self, posn: T) {
        let mut posn: f64 = posn.into();
        posn -= self.start;
        posn /= self.end - self.start;
        if posn.is_nan() || !(0.0..1.0).contains(&posn) {
            panic!("position out of range");
        }
        let nbins = self.bins.len();
        let bin_number = (nbins as f64 * posn).floor() as usize;
        self.bins[bin_number] += 1;
        self.count += 1;
    }
}

#[test]
fn smoke_test() {
    let mut h = Hist::new(4, 0.0..1.0);
    let samples = [0.1, 0.3, 0.35, 0.78];
    for s in samples {
        h.sample(s);
    }
    let n: usize = h.total_count();
    assert_eq!(n, 4);
    let counts: &[usize] = h.counts();
    assert_eq!(counts, &[1, 2, 0, 1]);
}

#[test]
#[should_panic]
fn test_bad_posn() {
    let mut h = Hist::new(4, 0.0..1.0);
    h.sample(-1.5);
}

#[test]
fn test_min_max() {
    let mut h = Hist::new(4, 0.0..1.0);
    assert!(h.min().is_none());
    assert!(h.max().is_none());
    h.sample(0.55);
    h.sample(0.80);
    assert_eq!(h.min().unwrap(), 2);
    assert_eq!(h.max().unwrap(), 3);
}

#[test]
fn test_u8() {
    let mut h = Hist::new(4, 1u8..5);
    h.sample(1);
    h.sample(3);
    assert_eq!(h.counts(), &[1, 0, 1, 0]);
}
