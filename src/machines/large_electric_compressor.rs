use crate::optimize::Overclock;

pub struct LargeElectricCompressor();

impl Overclock for LargeElectricCompressor {
    const PARALLELS_PER_TIER: u64 = 2;
    const SPEED_MODIFIER: f64 = 2.00;
    const ENERGY_MODIFIER: f64 = 0.90;
}
