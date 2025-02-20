use crate::machines::Overclock;

pub struct IndustrialCuttingFactory();

impl Overclock for IndustrialCuttingFactory {
    const PARALLELS_PER_TIER: u64 = 4;
    const SPEED_MODIFIER: f64 = 3.00;
    const ENERGY_MODIFIER: f64 = 0.75;
}
