use crate::optimize::Overclock;

pub struct UtupuTanuri();

impl Overclock for UtupuTanuri {
    const PARALLELS_OFFSET: u64 = 4;
    const SPEED_MODIFIER: f64 = 2.20;
    const ENERGY_MODIFIER: f64 = 0.50;
}
