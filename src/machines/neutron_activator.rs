use crate::machines::Overclock;
use crate::model::GregTechRecipe;
use crate::optimization_request::MachineConfiguration;

pub struct NeutronActivator();

impl Overclock for NeutronActivator {
    fn speed_modifier(&self, machine: &MachineConfiguration, _speed_modifier: f64) -> f64 {
        0.90f64.powf(-(machine.height.saturating_sub(4) as f64))
    }

    fn advised_batch(
        &self,
        machine: &MachineConfiguration,
        ticks: u64,
        recipe: &GregTechRecipe,
    ) -> (u64, u64) {
        let speed_modifier = machine.speed_modifier.unwrap_or(Self::SPEED_MODIFIER);

        // Always 1 for Neutron Activator
        let mut effective_parallels = 1;

        // Compute adjusted processing time based on overclocks and speed modifier
        let overclocked_processing_time =
            recipe.duration as f64 / self.speed_modifier(machine, speed_modifier);

        // Ensure processing time is at least 1 tick
        let corrected_processing_time = f64::max(overclocked_processing_time, 1.0) as u64;

        // Adjust effective parallels for sub-tick cases
        if overclocked_processing_time < 1.0 {
            effective_parallels = u64::max(
                effective_parallels,
                (1.00 / overclocked_processing_time) as u64,
            )
        };

        // Compute the advised batch size
        if corrected_processing_time <= ticks {
            let advised_batch = (effective_parallels as f64 * (ticks as f64 + 0.99)
                / corrected_processing_time as f64) as u64;
            let duration = (corrected_processing_time as f64
                * (advised_batch as f64 / effective_parallels as f64))
                as u64;
            (advised_batch, duration)
        } else {
            (effective_parallels, corrected_processing_time)
        }
    }
}
