use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct PreciseAutoAssemblerMT3662();

impl Overclock for PreciseAutoAssemblerMT3662 {
    const SPEED_MODIFIER: f64 = 2.00;

    fn get_max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        8 << machine.tier
    }
}
