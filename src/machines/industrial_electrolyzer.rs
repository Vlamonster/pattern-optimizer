use crate::optimize::Overclock;

pub struct IndustrialElectrolyzer();

impl Overclock for IndustrialElectrolyzer {
    const PARALLELS_PER_TIER: u64 = 2;
    const SPEED_MODIFIER: f64 = 2.80;
    const ENERGY_MODIFIER: f64 = 0.90;
}
