use crate::optimize::Overclock;

pub struct IndustrialCentrifuge();

impl Overclock for IndustrialCentrifuge {
    const PARALLELS_PER_TIER: u64 = 6;
    const SPEED_MODIFIER: f64 = 2.25;
    const ENERGY_MODIFIER: f64 = 0.90;
}
