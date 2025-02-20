use crate::machines::Overclock;

pub struct DissectionApparatus();

impl Overclock for DissectionApparatus {
    const PARALLELS_OFFSET: u64 = 64;
    const SPEED_MODIFIER: f64 = 3.00;
    const ENERGY_MODIFIER: f64 = 0.85;
}
