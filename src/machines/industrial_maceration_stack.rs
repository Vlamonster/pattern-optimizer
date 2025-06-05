use crate::{
    optimize::Overclock,
    request::MachineConfiguration,
};

pub struct IndustrialMacerationStack();

impl Overclock for IndustrialMacerationStack {
    const SPEED_MODIFIER: f64 = 1.60;

    fn max_parallels(&self, _parallels_offset: u64, _parallels_per_tier: u64, tier: u64, machine: &MachineConfiguration) -> u64 {
        if machine.tier == 1 {
            2 * tier
        } else {
            8 * tier
        }
    }
}
