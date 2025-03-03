use crate::machines::Overclock;
use crate::model::Recipe;
use crate::optimization_request::MachineConfiguration;

pub struct ComponentAssemblyLine();

impl Overclock for ComponentAssemblyLine {
    const PARALLELS_OFFSET: u64 = 1;

    fn perfect_overclocks(
        &self,
        machine: &MachineConfiguration,
        recipe: &Recipe,
        _tier: u64,
    ) -> u64 {
        u64::saturating_sub(machine.tier, recipe.special as u64)
    }
}
