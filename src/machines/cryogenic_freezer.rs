use crate::machines::Overclock;

pub struct CryogenicFreezer();

impl Overclock for CryogenicFreezer {
    const PARALLELS_OFFSET: u64 = 4;
    const SPEED_MODIFIER: f64 = 2.00;
}
