use crate::backend::{PredictedLoad, SystemSnapshot};

pub fn emit(snapshot: &SystemSnapshot, forecast: &PredictedLoad) {
    println!(
        "[SysDash-Ultra] CPU: {:.1}%, MEM: {:.1}%, net_in: {}, net_out: {}, future CPU: {:.1}%, future MEM: {:.1}%",
        snapshot.cpu,
        snapshot.mem,
        snapshot.net_in,
        snapshot.net_out,
        forecast.cpu_future,
        forecast.mem_future,
    );
}
