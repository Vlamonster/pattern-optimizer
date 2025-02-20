use crate::machines::Overclock;

pub struct IndustrialExtrusionMachine();

impl Overclock for IndustrialExtrusionMachine {
    const PARALLELS_PER_TIER: u64 = 4;
    const SPEED_MODIFIER: f64 = 3.50;
}
