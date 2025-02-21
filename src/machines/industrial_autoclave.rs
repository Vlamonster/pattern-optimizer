use crate::machines::Overclock;
use crate::model::Recipe;
use crate::optimization_request::MachineConfiguration;

pub struct IndustrialAutoclave();

impl Overclock for IndustrialAutoclave {
    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        12 * machine.item_pipe_casing_tier
    }

    fn speed_modifier(&self, machine: &MachineConfiguration, _speed_modifier: f64) -> f64 {
        1.00 + 0.25 * machine.coil_tier as f64
    }

    fn energy_modifier(
        &self,
        machine: &MachineConfiguration,
        _recipe: &Recipe,
        _tier: u64,
        _energy_modifier: f64,
    ) -> f64 {
        (12.0 - machine.pipe_casing_tier as f64) / 12.0
    }
}
