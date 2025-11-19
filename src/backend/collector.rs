use sysinfo::{Networks, System};

#[derive(Clone)]
pub struct SystemSnapshot {
    pub cpu: f32,
    pub mem: f32,
    pub net_in: u64,
    pub net_out: u64,
}

pub struct Collector {
    sys: System,
    networks: Networks,
}

impl Collector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();

        Self { sys, networks }
    }

    pub fn snapshot(&mut self) -> SystemSnapshot {
        // refresh existing structures
        self.sys.refresh_all();
        self.networks.refresh();
        // Memory %
        let total = self.sys.total_memory() as f32;
        let used = self.sys.used_memory() as f32;
        let mem = if total > 0.0 {
            (used / total) * 100.0
        } else {
            0.0
        };

        let cpu = self.sys.global_cpu_info().cpu_usage(); // Network totals
        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();

        let mut net_in = 0;
        let mut net_out = 0;
        for (_name, data) in &networks {
            net_in += data.total_received();
            net_out += data.total_transmitted();
        }

        SystemSnapshot {
            cpu,
            mem,
            net_in,
            net_out,
        }
    }
}
