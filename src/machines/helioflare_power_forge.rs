use crate::machines::Overclock;
use crate::model::GregTechRecipe;
use crate::optimization_request::{GorgeUpgrades, MachineConfiguration};

pub struct HelioflarePowerForge();

impl HelioflarePowerForge {
    fn heat(machine: &MachineConfiguration) -> u64 {
        if machine.upgrades.sefcp {
            (f64::log2(machine.dtr as f64) / f64::log2(1.12) * 1000.0) as u64 + 12_601
        } else {
            (f64::log2(machine.dtr as f64) / f64::log2(1.5) * 1000.0) as u64 + 12_601
        }
    }

    fn effective_heat(machine: &MachineConfiguration) -> u64 {
        let heat = Self::heat(machine);
        if machine.upgrades.ndpe && heat > 30_000 {
            30_000 + f64::powf((heat - 30_000) as f64, 0.85) as u64
        } else {
            u64::min(Self::effective_heat_capacity(&machine.upgrades), heat)
        }
    }

    fn effective_heat_capacity(upgrades: &GorgeUpgrades) -> u64 {
        if upgrades.cnti {
            30_000
        } else if upgrades.start {
            15_000
        } else {
            todo!("Case not handled yet!")
        }
    }

    fn energy_usage(machine: &MachineConfiguration) -> u64 {
        let mut energy_usage = 2_000_000_000;

        if machine.upgrades.giss {
            energy_usage += 100_000_000 * machine.dtr
        }

        if machine.upgrades.ngms {
            energy_usage <<= 2 * machine.rings;
        }

        energy_usage
    }

    fn parallels(machine: &MachineConfiguration) -> u64 {
        let mut multiplier = 1.0;

        if machine.upgrades.sa {
            multiplier *= 1.0 + machine.dtr as f64 / 15.0;
        }

        if machine.upgrades.ctcdd {
            multiplier *= 2.0;
        }

        (1024.0 * multiplier) as u64
    }
}

impl Overclock for HelioflarePowerForge {
    fn energy_modifier(
        &self,
        machine: &MachineConfiguration,
        recipe: &GregTechRecipe,
        _tier: u64,
        mut energy_modifier: f64,
    ) -> f64 {
        // Heat discounts
        let heat = Self::effective_heat(machine);
        let recipe_heat = u64::min(
            recipe.special as u64,
            Self::effective_heat_capacity(&machine.upgrades),
        );
        let discounts = (heat - recipe_heat) / 900;
        energy_modifier *= f64::powi(0.95, discounts as i32);

        // Assume the user always uses an appropriate battery size
        if machine.upgrades.rec {
            energy_modifier *= 0.95;
        }

        energy_modifier
    }

    fn speed_modifier(&self, machine: &MachineConfiguration, speed_modifier: f64) -> f64 {
        if machine.upgrades.igcc {
            speed_modifier * f64::powf(Self::effective_heat(machine) as f64, 0.01)
        } else {
            speed_modifier
        }
    }

    fn perfect_overclocks(
        &self,
        machine: &MachineConfiguration,
        recipe: &GregTechRecipe,
        _tier: u64,
    ) -> u64 {
        let heat = Self::effective_heat(machine);
        (heat - recipe.special as u64) / 1800
    }

    fn advised_batch(
        &self,
        machine: &MachineConfiguration,
        ticks: u64,
        recipe: &GregTechRecipe,
    ) -> (u64, u64) {
        if !machine.upgrades.start {
            panic!("Missing upgrade START");
        }
        let parallels = Self::parallels(machine);
        let energy_usage = Self::energy_usage(machine);
        let speed_modifier = machine.speed_modifier.unwrap_or(Self::SPEED_MODIFIER);
        let energy_modifier = machine.energy_modifier.unwrap_or(Self::ENERGY_MODIFIER);

        // Adjust energy consumption based on machine and recipe specifics
        let energy_modifier = self.energy_modifier(machine, recipe, 0, energy_modifier);
        let energy_used = f64::ceil(recipe.energy_usage as f64 * energy_modifier) as u64;

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
            (parallels * advised_batch, duration)
        } else {
            (parallels * effective_parallels, corrected_processing_time)
        }
    }
}
