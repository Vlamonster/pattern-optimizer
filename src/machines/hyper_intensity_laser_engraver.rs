use crate::machines::Overclock;
use crate::optimization_request::MachineConfiguration;

pub struct HyperIntensityLaserEngraver();

impl Overclock for HyperIntensityLaserEngraver {
    const SPEED_MODIFIER: f64 = 3.00;
    const ENERGY_MODIFIER: f64 = 0.80;

    fn max_parallels(
        &self,
        _parallels_offset: u64,
        _parallels_per_tier: u64,
        _tier: u64,
        machine: &MachineConfiguration,
    ) -> u64 {
        (machine.laser_amperage as f64).cbrt() as u64
    }
}
