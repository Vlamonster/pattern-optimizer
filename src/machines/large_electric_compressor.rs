use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct LargeElectricCompressor();

impl Overclock for LargeElectricCompressor {
    const SPEED_MODIFIER: f64 = 2.00;
    const ENERGY_MODIFIER: f64 = 0.90;

    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        tier: u64,
        _machine: &MachineConfiguration,
    ) -> u64 {
        2 * tier
    }
}
