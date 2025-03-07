use crate::machines::Overclock;
use crate::model::GregTechRecipe;
use crate::optimization_request::MachineConfiguration;

pub struct MegaVacuumFreezer();

impl Overclock for MegaVacuumFreezer {
    const PARALLELS_OFFSET: u64 = 256;

    fn perfect_overclocks(
        &self,
        machine: &MachineConfiguration,
        _recipe: &GregTechRecipe,
        _tier: u64,
    ) -> u64 {
        machine.tier
    }
}
