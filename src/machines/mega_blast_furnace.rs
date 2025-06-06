use crate::{
    model::GregTechRecipe,
    optimize::Overclock,
    request::MachineConfiguration,
};

pub struct MegaBlastFurnace();

impl Overclock for MegaBlastFurnace {
    const PARALLELS_OFFSET: u64 = 256;

    fn energy_modifier(&self, machine: &MachineConfiguration, recipe: &GregTechRecipe, tier: u64, energy_modifier: f64) -> f64 {
        let heat = 901 + 900 * machine.coil_tier + 100 * (u64::saturating_sub(tier, 2));
        let discounts = (heat - recipe.special as u64) / 900;
        energy_modifier * f64::powi(0.95, discounts as i32)
    }

    fn perfect_overclocks(&self, machine: &MachineConfiguration, recipe: &GregTechRecipe, tier: u64) -> u64 {
        let heat = 901 + 900 * machine.coil_tier + 100 * (u64::saturating_sub(tier, 2));
        (heat - recipe.special as u64) / 1800
    }
}
