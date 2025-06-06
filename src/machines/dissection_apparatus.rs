use crate::{
    optimize::Overclock,
    request::MachineConfiguration,
};

pub struct DissectionApparatus();

impl Overclock for DissectionApparatus {
    const SPEED_MODIFIER: f64 = 3.00;
    const ENERGY_MODIFIER: f64 = 0.85;

    fn max_parallels(&self, _parallels_offset: u64, _parallels_per_tier: u64, _tier: u64, machine: &MachineConfiguration) -> u64 {
        machine.item_pipe_casing_tier * 8
    }
}
