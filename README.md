# Dynamic Histograms
Bart Massey 2025

Take the existing code and make it accept a value range for
the histogram. The range should be given when creating the
histogram, and should be of generic type.

  1. Modify the `Hist` struct to take a generic type
     for values to be binned.

     ```rust
     pub struct Hist<T> {
     ```

  2. Accept a range of values to be binned when creating
     a histogram.

     ```rust
     /// Create a new histogram with `n` bins.
     pub fn new(n: usize, range: std::ops::Range<T>) -> Hist {
     ```

  3. Adjust the other methods in the libary to work with all
     this.

Now you should be able to call, for example,
`Hist::new(4, 0u8..15)` and get a histogram for samples
that are `u8`.

## Hints

* Read the documentation for `std::ops::Range` carefully, as
  it is both a good example of generics and a thing you will
  need to understand to be able to use the `..` operator.

* You may need quite a few traits from `std::ops` to allow
  the arithmetic you need, for example `std::ops::Add`, `std::ops::Div`.

* It will be impossible to maintain our `NaN` checks in this
  code without some work. Oh well. You could fix it in one
  of several ways.
  
  The obvious plan is to define an `IsValid` trait and
  require it for histogram types. I do not recommend this,
  because it makes the library hard to use.
  
  The normal plan is to use the `NotNan` type from the
  [`ordered-float`](https://crates.io/crates/ordered-float)
  crate for floating-point histograms, but that is ugly.
