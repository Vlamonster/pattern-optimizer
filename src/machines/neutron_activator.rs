use crate::{
    model::GregTechRecipe,
    optimize::Overclock,
    request::{
        MachineConfiguration,
        OptimizationRequest,
    },
    MainError,
};

pub struct NeutronActivator();

impl Overclock for NeutronActivator {
    fn speed_modifier(&self, machine: &MachineConfiguration, _speed_modifier: f64) -> f64 {
        0.90f64.powf(-(machine.height.saturating_sub(4) as f64))
    }

    fn optimize_batch_size(&self, request: &OptimizationRequest, recipe: &GregTechRecipe) -> Result<(u64, u64), MainError> {
        let machine = &request.machine;

        let speed_modifier = machine.speed_modifier.unwrap_or(Self::SPEED_MODIFIER);

        // Always 1 for Neutron Activator
        let mut effective_parallels = 1;

        // Compute adjusted processing time based on overclocks and speed modifier
        let overclocked_processing_time = recipe.duration as f64 / self.speed_modifier(machine, speed_modifier);

        // Ensure processing time is at least 1 tick
        let corrected_processing_time = f64::max(overclocked_processing_time, 1.0) as u64;

        // Adjust effective parallels for sub-tick cases
        if overclocked_processing_time < 1.0 {
            effective_parallels = u64::max(effective_parallels, (1.00 / overclocked_processing_time) as u64)
        };

        // Compute the advised batch size
        if corrected_processing_time <= request.ticks {
            let optimize_batch_size =
                (effective_parallels as f64 * (request.ticks as f64 + 0.99) / corrected_processing_time as f64) as u64;
            let duration = (corrected_processing_time as f64 * (optimize_batch_size as f64 / effective_parallels as f64)) as u64;
            Ok((optimize_batch_size, duration))
        } else {
            Ok((effective_parallels, corrected_processing_time))
        }
    }
}
