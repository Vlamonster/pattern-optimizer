use crate::{
    optimize::Overclock,
    request::MachineConfiguration,
};

pub struct ElectricImplosionCompressor();

impl Overclock for ElectricImplosionCompressor {
    fn max_parallels(&self, _parallels_offset: u64, _parallels_per_tier: u64, _tier: u64, machine: &MachineConfiguration) -> u64 {
        1 << (2 * (machine.tier - 1))
    }
}
