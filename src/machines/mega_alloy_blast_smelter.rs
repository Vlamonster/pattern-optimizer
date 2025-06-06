use crate::{
    model::GregTechRecipe,
    optimize::Overclock,
    request::MachineConfiguration,
};

pub struct MegaAlloyBlastSmelter();

impl Overclock for MegaAlloyBlastSmelter {
    const PARALLELS_OFFSET: u64 = 256;

    fn speed_modifier(&self, machine: &MachineConfiguration, _speed_modifier: f64) -> f64 {
        let speedups = u64::min(u64::saturating_sub(machine.coil_tier, 4), machine.glass_tier);
        1.00 / (1.00 - 0.05 * speedups as f64)
    }

    fn energy_modifier(&self, machine: &MachineConfiguration, recipe: &GregTechRecipe, _tier: u64, energy_modifier: f64) -> f64 {
        let recipe_tier = u64::ilog(recipe.energy_usage / 8, 4) as u64;
        let discounts = u64::saturating_sub(machine.coil_tier, recipe_tier);
        energy_modifier * f64::powi(energy_modifier, discounts as i32)
    }
}
