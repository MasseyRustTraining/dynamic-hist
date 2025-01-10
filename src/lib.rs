/// Record of samples. Each bin in the record
/// represents its fraction of the space 0.0..1.0.
pub struct Hist {
    bins: Vec<usize>,
    count: usize,
}

/// Keeps track of which bin samples fall in and report as
/// needed.
impl Hist {
    /// Create a new histogram with `n` bins.
    pub fn new(n: usize) -> Hist {
        Hist { bins: vec![0; n], count: 0 }
    }

    /// Count a new sample.
    pub fn sample(&mut self, posn: f32) {
        if posn.is_nan() || posn < 0.0 || posn >= 1.0 {
            panic!("position out of range");
        }
        let nbins = self.bins.len();
        let bin_number = (nbins as f32 * posn).floor() as usize;
        self.bins[bin_number] += 1;
        self.count += 1;
    }

    /// Return a slice containing the counts in the histogram.
    pub fn counts(&self) -> &[usize] {
        &self.bins
    }

    /// Return the number of samples taken.
    pub fn total_count(&self) -> usize {
        self.count
    }
}

#[test]
fn smoke_test() {
    let mut h = Hist::new(4);
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
    let mut h = Hist::new(4);
    h.sample(-1.5);
}
