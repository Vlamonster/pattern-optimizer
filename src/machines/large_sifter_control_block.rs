use crate::machines::Overclock;

pub struct LargeSifterControlBlock();

impl Overclock for LargeSifterControlBlock {
    const PARALLELS_PER_TIER: u64 = 4;
    const SPEED_MODIFIER: f64 = 5.00;
    const ENERGY_MODIFIER: f64 = 0.75;
}
