use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct ElectricImplosionCompressor();

impl Overclock for ElectricImplosionCompressor {
    fn get_max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        1 << (2 * (machine.tier - 1))
    }
}
