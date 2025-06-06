use crate::optimize::Overclock;

pub struct MegaDistillationTower();

impl Overclock for MegaDistillationTower {
    const PARALLELS_OFFSET: u64 = 256;
}
