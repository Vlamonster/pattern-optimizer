use crate::optimize::Overclock;

pub struct IndustrialMaterialPress();

impl Overclock for IndustrialMaterialPress {
    const PARALLELS_PER_TIER: u64 = 4;
    const SPEED_MODIFIER: f64 = 6.00;
}
