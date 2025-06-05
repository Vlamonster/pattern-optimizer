use crate::{
    model::GregTechRecipe,
    request::{
        MachineConfiguration,
        OptimizationRequest,
    },
};

pub trait Overclock {
    const PARALLELS_OFFSET: u64 = 0;
    const PARALLELS_PER_TIER: u64 = 0;
    const SPEED_MODIFIER: f64 = 1.00;
    const ENERGY_MODIFIER: f64 = 1.00;

    fn max_parallels(&self, parallels_offset: u64, parallels_per_tier: u64, tier: u64, _machine: &MachineConfiguration) -> u64 {
        parallels_offset + tier * parallels_per_tier
    }

    fn speed_modifier(&self, _machine: &MachineConfiguration, speed_modifier: f64) -> f64 {
        speed_modifier
    }

    fn energy_modifier(&self, _machine: &MachineConfiguration, _recipe: &GregTechRecipe, _tier: u64, energy_modifier: f64) -> f64 {
        energy_modifier
    }

    fn perfect_overclocks(&self, _machine: &MachineConfiguration, _recipe: &GregTechRecipe, _tier: u64) -> u64 {
        0
    }

    fn optimize_batch_size(&self, request: &OptimizationRequest, recipe: &GregTechRecipe) -> (u64, u64) {
        let machine = &request.machine;

        // Extract machine parameters or fallback to defaults
        let parallels_offset = machine.parallels_offset.unwrap_or(Self::PARALLELS_OFFSET);
        let parallels_per_tier = machine.parallels_per_tier.unwrap_or(Self::PARALLELS_PER_TIER);
        let speed_modifier = machine.speed_modifier.unwrap_or(Self::SPEED_MODIFIER);
        let energy_modifier = machine.energy_modifier.unwrap_or(Self::ENERGY_MODIFIER);

        // Determine the machine tier based on energy usage
        let tier = u64::ilog(machine.energy_usage / 8, 4) as u64;

        // Calculate max parallels allowed at this tier
        let max_parallels = self.max_parallels(parallels_offset, parallels_per_tier, tier, machine);

        // Adjust energy consumption based on machine and recipe specifics
        let energy_modifier = self.energy_modifier(machine, recipe, tier, energy_modifier);
        let adjusted_energy_usage = f64::ceil(recipe.energy_usage as f64 * energy_modifier) as u64;
        let mut effective_parallels = u64::min(machine.energy_usage / adjusted_energy_usage, max_parallels);
        let energy_used = effective_parallels * adjusted_energy_usage;

        // Determine overclock levels
        let overclocks = u64::ilog(machine.energy_usage / energy_used, 4) as u64;
        let overclocks = u64::min(overclocks, machine.maximum_overclock_tier - tier);

        // Split into regular and perfect overclocks
        let perfect_overclocks = u64::min(overclocks, self.perfect_overclocks(machine, recipe, tier));
        let regular_overclocks = overclocks - perfect_overclocks;

        // Compute adjusted processing time based on overclocks and speed modifier
        let overclocked_processing_time = recipe.duration as f64
            / self.speed_modifier(machine, speed_modifier)
            / 2f64.powi(regular_overclocks as i32)
            / 4f64.powi(perfect_overclocks as i32);

        // Ensure processing time is at least 1 tick
        let corrected_processing_time = f64::max(overclocked_processing_time, 1.0) as u64;

        // Adjust effective parallels for sub-tick cases
        if overclocked_processing_time < 1.0 {
            effective_parallels = u64::max(effective_parallels, (max_parallels as f64 / overclocked_processing_time) as u64)
        };

        // Compute the advised batch size
        if corrected_processing_time <= request.ticks {
            let optimize_batch_size =
                (effective_parallels as f64 * (request.ticks as f64 + 0.99) / corrected_processing_time as f64) as u64;
            let duration = (corrected_processing_time as f64 * (optimize_batch_size as f64 / effective_parallels as f64)) as u64;
            (optimize_batch_size, duration)
        } else {
            (effective_parallels, corrected_processing_time)
        }
    }
}
