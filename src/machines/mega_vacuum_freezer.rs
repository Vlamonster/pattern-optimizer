use crate::machines::Overclock;

pub struct MegaVacuumFreezer();

impl Overclock for MegaVacuumFreezer {
    const PARALLELS_OFFSET: u64 = 256;
}
