mod bus;
mod facility;
mod taxi;
mod traffic;

use crate::capture::Capture;

/// Every capture, in coordinator run order.
pub fn all() -> Vec<Capture> {
    let mut all = vec![];
    all.extend(bus::captures());
    all.extend(traffic::captures());
    all.extend(taxi::captures());
    all.extend(facility::captures());
    all
}
