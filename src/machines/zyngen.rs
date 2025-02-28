use crate::machines::Overclock;
use crate::model::Recipe;
use crate::optimization_request::MachineConfiguration;

pub struct Zyngen();

impl Overclock for Zyngen {
    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        tier * machine.coil_tier
    }

    fn speed_modifier(&self, machine: &MachineConfiguration, speed_modifier: f64) -> f64 {
        speed_modifier + machine.coil_tier as f64 * 0.05
    }

    fn perfect_overclocks(
        &self,
        _machine: &MachineConfiguration,
        _recipe: &Recipe,
        _tier: u64,
    ) -> u64 {
        u64::MAX
    }
}
