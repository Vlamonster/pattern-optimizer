use crate::optimize::Overclock;

pub struct BigBarrelBrewery();

impl Overclock for BigBarrelBrewery {
    const PARALLELS_PER_TIER: u64 = 4;
    const SPEED_MODIFIER: f64 = 1.50;
}
