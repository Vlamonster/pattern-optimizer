use crate::{
    model::GregTechRecipe,
    optimize::Overclock,
    request::MachineConfiguration,
};

pub struct IndustrialCokeOven();

impl Overclock for IndustrialCokeOven {
    fn max_parallels(&self, _parallels_offset: u64, _parallels_per_tier: u64, _tier: u64, _machine: &MachineConfiguration) -> u64 {
        match _machine.tier {
            ..=1 => 12,
            2.. => 24,
        }
    }

    fn energy_modifier(&self, _machine: &MachineConfiguration, _recipe: &GregTechRecipe, tier: u64, _energy_modifier: f64) -> f64 {
        1.00 - 0.04 * tier as f64
    }
}
