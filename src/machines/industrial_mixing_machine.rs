use crate::optimize::Overclock;

pub struct IndustrialMixingMachine();

impl Overclock for IndustrialMixingMachine {
    const PARALLELS_PER_TIER: u64 = 8;
    const SPEED_MODIFIER: f64 = 3.50;
    const ENERGY_MODIFIER: f64 = 1.00;
}
