use crate::machines::Overclock;
use crate::model::Recipe;
use crate::optimization_request::MachineConfiguration;

pub struct LargeFluidExtractor();

impl Overclock for LargeFluidExtractor {
    const SPEED_MODIFIER: f64 = 1.50;
    const ENERGY_MODIFIER: f64 = 0.80;

    fn get_max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        8 * machine.solenoid_tier
    }

    fn get_speed_modifier(&self, machine: &MachineConfiguration, speed_modifier: f64) -> f64 {
        speed_modifier + 0.10 * machine.coil_tier as f64
    }

    fn get_energy_modifier(
        &self,
        machine: &MachineConfiguration,
        _recipe: &Recipe,
        _tier: u64,
        energy_modifier: f64,
    ) -> f64 {
        energy_modifier * f64::powi(0.90, machine.coil_tier as i32)
    }
}
