use crate::backend::collector::SystemSnapshot;

pub struct PredictedLoad {
    pub cpu_future: f32,
    pub mem_future: f32,
}

pub fn forecast(_history: &[SystemSnapshot]) -> PredictedLoad {
    PredictedLoad {
        cpu_future: 0.0,
        mem_future: 0.0,
    }
}
