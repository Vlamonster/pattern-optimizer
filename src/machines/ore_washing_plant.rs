use crate::machines::Overclock;

pub struct OreWashingPlant();

impl Overclock for OreWashingPlant {
    const PARALLELS_PER_TIER: u64 = 4;
    const SPEED_MODIFIER: f64 = 5.00;
}
