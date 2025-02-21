use crate::machines::Overclock;
use crate::model::Recipe;
use crate::optimization_request::MachineConfiguration;

pub struct Volcanus();

impl Overclock for Volcanus {
    const PARALLELS_OFFSET: u64 = 8;
    const SPEED_MODIFIER: f64 = 2.20;
    const ENERGY_MODIFIER: f64 = 0.90;

    fn energy_modifier(
        &self,
        machine: &MachineConfiguration,
        recipe: &Recipe,
        _tier: u64,
        energy_modifier: f64,
    ) -> f64 {
        let heat = 901 + 900 * machine.coil_tier;
        let discounts = (heat - recipe.special as u64) / 900;
        energy_modifier * f64::powi(0.95, discounts as i32)
    }

    fn perfect_overclocks(
        &self,
        machine: &MachineConfiguration,
        recipe: &Recipe,
        _tier: u64,
    ) -> u64 {
        let heat = 901 + 900 * machine.coil_tier;
        (heat - recipe.special as u64) / 1800
    }
}
