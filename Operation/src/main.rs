mod circuit;
mod sensor;
mod overseer;
use std::time::{Duration, Instant};

fn main() {
    let mut running = true;
    let mut overseer = overseer::OverSeer::default();
    overseer.init();
    let mut last_update = Instant::now();
    while running {
        let now = Instant::now();
        let delta_time = now - last_update;
        last_update = now;
        running = overseer.run(delta_time);
    }
}
