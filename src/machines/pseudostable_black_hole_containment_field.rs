use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct PseudostableBlackHoleContainmentField();

impl Overclock for PseudostableBlackHoleContainmentField {
    const SPEED_MODIFIER: f64 = 5.00;
    const ENERGY_MODIFIER: f64 = 0.70;

    fn get_max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        tier: u64,
        _machine: &MachineConfiguration,
    ) -> u64 {
        // Assumed to run at stability <= 20
        4 * 8 * tier
    }
}
