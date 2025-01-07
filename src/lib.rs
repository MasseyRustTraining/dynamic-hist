use std::ops::Range;

pub struct DynHist {
    count: usize,
    bins: Vec<usize>,
    range: Range<f64>,
}

pub enum DynHistError {
    OutOfRange { value: f64, range: Range<f64> },
    IndexFail,
}

impl DynHist {
    pub fn new(nbins: usize, range: Range<f64>) -> Self {
        Self {
            count: 0,
            bins: vec![0; nbins],
            range,
        }
    }

    pub fn sample(&mut self, value: f64) -> Result<(), DynHistError> {
        if !self.range.contains(&value) {
            let range = self.range.clone();
            return Err(DynHistError::OutOfRange { value, range });
        }
        let offset = value - self.range.start;
        let interval = self.range.end - self.range.start;
        let position: f64 = offset / interval;
        let nbins: f64 = self.bins.len() as f64;
        let bin = (position * nbins).floor();
        if bin.is_nan() || bin > usize::MAX as f64 {
            return Err(DynHistError::IndexFail);
        }
        self.bins[bin as usize] += 1;
        self.count += 1;
        Ok(())
    }

    pub fn hist(&self) -> &[usize] {
        &self.bins
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
