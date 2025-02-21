use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct FluidShaper();

impl Overclock for FluidShaper {
    // Assumed to run continuously
    const SPEED_MODIFIER: f64 = 3.00;
    const ENERGY_MODIFIER: f64 = 0.80;

    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        (2 + 3 * machine.expansion_width) * tier
    }
}
