use crate::machines::Overclock;
use crate::model::GregTechRecipe;
use crate::optimization_request::MachineConfiguration;

pub struct MegaChemicalReactor();

impl Overclock for MegaChemicalReactor {
    const PARALLELS_OFFSET: u64 = 256;

    // Always performs perfect overclocks
    fn perfect_overclocks(
        &self,
        _machine: &MachineConfiguration,
        _recipe: &GregTechRecipe,
        _tier: u64,
    ) -> u64 {
        u64::MAX
    }
}
