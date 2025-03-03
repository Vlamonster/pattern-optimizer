use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct CircuitAssemblyLine();

impl Overclock for CircuitAssemblyLine {
    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        _machine: &MachineConfiguration,
    ) -> u64 {
        u64::MAX
    }
}
