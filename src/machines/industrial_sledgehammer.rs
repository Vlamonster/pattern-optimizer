use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct IndustrialSledgehammer();

impl Overclock for IndustrialSledgehammer {
    const SPEED_MODIFIER: f64 = 2.00;

    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        8 * tier * machine.tier
    }
}
