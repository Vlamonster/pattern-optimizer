use crate::machines::Overclock;

pub struct MegaDistillationTower();

impl Overclock for MegaDistillationTower {
    const PARALLELS_OFFSET: u64 = 256;
}
