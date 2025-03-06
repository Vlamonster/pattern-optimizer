use crate::machines::Overclock;
use crate::model::Recipe;
use crate::optimization_request::{GorgeUpgrades, MachineConfiguration};

pub struct HelioflarePowerForge();

impl HelioflarePowerForge {
    fn heat(dtr: u64) -> u64 {
        (dtr as f64 * 300.0 * f64::powi(1.15, dtr as i32)) as u64
    }

    fn effective_heat_capacity(upgrades: &GorgeUpgrades) -> u64 {
        if upgrades.start {
            15_000
        } else {
            todo!("Case not handled yet!")
        }
    }
}

impl Overclock for HelioflarePowerForge {
    fn energy_modifier(
        &self,
        machine: &MachineConfiguration,
        recipe: &Recipe,
        _tier: u64,
        energy_modifier: f64,
    ) -> f64 {
        let heat = u64::min(
            Self::effective_heat_capacity(&machine.upgrades),
            Self::heat(machine.dtr),
        );
        let discounts = (heat - recipe.special as u64) / 900;
        energy_modifier * f64::powi(0.95, discounts as i32)
    }

    fn perfect_overclocks(
        &self,
        machine: &MachineConfiguration,
        recipe: &Recipe,
        _tier: u64,
    ) -> u64 {
        let heat = u64::min(
            Self::effective_heat_capacity(&machine.upgrades),
            Self::heat(machine.dtr),
        );
        (heat - recipe.special as u64) / 1800
    }

    fn advised_batch(
        &self,
        machine: &MachineConfiguration,
        ticks: u64,
        recipe: &Recipe,
    ) -> (u64, u64) {
        if !machine.upgrades.start {
            panic!("Missing upgrade START");
        }
        let energy_usage = 2_000_000_000;

        let speed_modifier = machine.speed_modifier.unwrap_or(Self::SPEED_MODIFIER);
        let energy_modifier = machine.energy_modifier.unwrap_or(Self::ENERGY_MODIFIER);

        // Adjust energy consumption based on machine and recipe specifics
        let energy_modifier = self.energy_modifier(machine, recipe, 0, energy_modifier);
        let energy_used = f64::ceil(energy_usage as f64 * energy_modifier) as u64;

        // Determine overclock levels
        let overclocks = u64::ilog(energy_usage / energy_used, 4) as u64;

        // Split into regular and perfect overclocks
        let perfect_overclocks = u64::min(overclocks, self.perfect_overclocks(machine, recipe, 0));
        let regular_overclocks = overclocks - perfect_overclocks;

        // Compute adjusted processing time based on overclocks and speed modifier
        let overclocked_processing_time = recipe.duration as f64
            / self.speed_modifier(machine, speed_modifier)
            / 2f64.powi(regular_overclocks as i32)
            / 4f64.powi(perfect_overclocks as i32);

        // Ensure processing time is at least 1 tick
        let corrected_processing_time = f64::max(overclocked_processing_time, 1.0) as u64;

        // Adjust effective parallels for sub-tick cases
        let mut effective_parallels = 1;
        if overclocked_processing_time < 1.0 {
            effective_parallels = (1.00 / overclocked_processing_time) as u64
        };

        // Compute the advised batch size
        if corrected_processing_time <= ticks {
            let advised_batch = (effective_parallels as f64 * (ticks as f64 + 0.99)
                / corrected_processing_time as f64) as u64;
            let duration = (corrected_processing_time as f64
                * (advised_batch as f64 / effective_parallels as f64))
                as u64;
            (1024 * advised_batch, duration)
        } else {
            (1024 * effective_parallels, corrected_processing_time)
        }
    }
}
