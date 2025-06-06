use crate::{
    optimize::Overclock,
    request::MachineConfiguration,
};

pub struct PreciseAutoAssemblerMT3662();

impl Overclock for PreciseAutoAssemblerMT3662 {
    const SPEED_MODIFIER: f64 = 2.00;

    fn max_parallels(&self, _parallels_offset: u64, _parallels_per_tier: u64, _tier: u64, machine: &MachineConfiguration) -> u64 {
        match machine.tier {
            0 => 16,
            1 => 32,
            2 => 64,
            3 => 128,
            4.. => 256,
        }
    }
}
