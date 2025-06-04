use crate::machines::Overclock;
use crate::model::GregTechRecipe;
use crate::optimization_request::MachineConfiguration;

pub struct MagneticFluxExhibitor();

impl Overclock for MagneticFluxExhibitor {
    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        match machine.tier {
            ..=1 => 8,
            2 => 24,
            3 => 48,
            4 => 96,
            5.. => 256,
        }
    }

    fn speed_modifier(&self, machine: &MachineConfiguration, _speed_modifier: f64) -> f64 {
        match machine.tier {
            ..=1 => 1.10,
            2 => 1.25,
            3 => 1.50,
            4 => 2.00,
            5.. => 2.50,
        }
    }

    fn energy_modifier(
        &self,
        machine: &MachineConfiguration,
        _recipe: &GregTechRecipe,
        _tier: u64,
        _energy_modifier: f64,
    ) -> f64 {
        match machine.tier {
            ..=1 => 0.80,
            2 => 0.75,
            3 => 0.70,
            4 => 0.60,
            5.. => 0.50,
        }
    }
}
