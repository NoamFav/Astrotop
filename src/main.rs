mod backend;

use crate::backend::{Collector, PredictedLoad};

fn main() {
    let mut collector = Collector::new();

    let snapshot = collector.snapshot();
    let history = vec![snapshot.clone()];
    let prediction: PredictedLoad = backend::forecast(&history);

    backend::emit(&snapshot, &prediction);
}
