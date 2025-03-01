use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct HighCurrentIndustrialArcFurnace();

impl Overclock for HighCurrentIndustrialArcFurnace {
    const SPEED_MODIFIER: f64 = 3.50;

    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        let plasma_modifier = if machine.tier == 1 { 1 } else { 8 };
        plasma_modifier * machine.width * tier
    }
}
