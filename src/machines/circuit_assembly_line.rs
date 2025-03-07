use crate::machines::Overclock;
use crate::model::GregTechRecipe;
use crate::optimization_request::MachineConfiguration;

pub struct CircuitAssemblyLine();

impl Overclock for CircuitAssemblyLine {
    const PARALLELS_OFFSET: u64 = 1;

    fn perfect_overclocks(
        &self,
        _machine: &MachineConfiguration,
        _recipe: &GregTechRecipe,
        _tier: u64,
    ) -> u64 {
        u64::MAX
    }
}
