use dynamic_hist::Hist;

fn main() {
    let mut h = Hist::new(4);
    let samples = [0.1, 0.3, 0.35, 0.78];
    for s in samples {
        h.sample(s);
    }
    println!("{} {:?}", h.total_count(), h.counts());
}
