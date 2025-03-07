use crate::machines::Overclock;
use crate::model::GregTechRecipe;
use crate::optimization_request::MachineConfiguration;

pub struct ComponentAssemblyLine();

impl Overclock for ComponentAssemblyLine {
    const PARALLELS_OFFSET: u64 = 1;

    fn perfect_overclocks(
        &self,
        machine: &MachineConfiguration,
        recipe: &GregTechRecipe,
        _tier: u64,
    ) -> u64 {
        u64::saturating_sub(machine.tier, recipe.special as u64)
    }
}
