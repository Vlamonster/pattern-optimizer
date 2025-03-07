use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct MultiSmelter();

impl Overclock for MultiSmelter {
    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        4 << machine.coil_tier
    }
}
