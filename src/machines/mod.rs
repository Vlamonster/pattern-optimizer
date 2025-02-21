mod blast_furnace;
mod cryogenic_freezer;
mod dissection_apparatus;
mod electric_implosion_compressor;
mod fluid_shaper;
mod god_forge;
mod hyper_intensity_laser_engraver;
mod industrial_autoclave;
mod industrial_centrifuge;
mod industrial_cutting_factory;
mod industrial_electrolyzer;
mod industrial_extrusion_machine;
mod industrial_maceration_stack;
mod industrial_material_press;
mod industrial_mixing_machine;
mod industrial_sledgehammer;
mod industrial_wire_factory;
mod large_fluid_extractor;
mod mega_alloy_blast_smelter;
mod mega_blast_furnace;
mod mega_vacuum_freezer;
mod ore_washing_plant;
mod precise_auto_assembler_mt_3662;
mod pseudostable_black_hole_containment_field;
mod utupu_tanuri;
mod vacuum_freezer;
mod volcanus;

use crate::machines::mega_alloy_blast_smelter::MegaAlloyBlastSmelter;
use crate::machines::ore_washing_plant::OreWashingPlant;
use crate::model::Recipe;
use crate::optimization_request::MachineConfiguration;
use blast_furnace::BlastFurnace;
use cryogenic_freezer::CryogenicFreezer;
use dissection_apparatus::DissectionApparatus;
use electric_implosion_compressor::ElectricImplosionCompressor;
use fluid_shaper::FluidShaper;
use hyper_intensity_laser_engraver::HyperIntensityLaserEngraver;
use industrial_autoclave::IndustrialAutoclave;
use industrial_centrifuge::IndustrialCentrifuge;
use industrial_cutting_factory::IndustrialCuttingFactory;
use industrial_electrolyzer::IndustrialElectrolyzer;
use industrial_extrusion_machine::IndustrialExtrusionMachine;
use industrial_maceration_stack::IndustrialMacerationStack;
use industrial_material_press::IndustrialMaterialPress;
use industrial_mixing_machine::IndustrialMixingMachine;
use industrial_sledgehammer::IndustrialSledgehammer;
use industrial_wire_factory::IndustrialWireFactory;
use large_fluid_extractor::LargeFluidExtractor;
use mega_blast_furnace::MegaBlastFurnace;
use mega_vacuum_freezer::MegaVacuumFreezer;
use precise_auto_assembler_mt_3662::PreciseAutoAssemblerMT3662;
use pseudostable_black_hole_containment_field::PseudostableBlackHoleContainmentField;
use utupu_tanuri::UtupuTanuri;
use vacuum_freezer::VacuumFreezer;
use volcanus::Volcanus;

macro_rules! machine_batch_match {
    ($machine:expr, $ticks:expr, $recipe:expr, { $($name:literal => $struct:ident),* $(,)? }) => {
        match $machine.id.as_str() {
            $( $name => $struct().advised_batch($machine, $ticks, $recipe), )*
            _ => panic!("unknown machine: {:?}", $machine),
        }
    };
}

pub fn advised_batch(machine: &MachineConfiguration, ticks: u64, recipe: &Recipe) -> u64 {
    machine_batch_match!(machine, ticks, recipe, {
        "Industrial Material Press" => IndustrialMaterialPress,
        "Industrial Extrusion Machine" => IndustrialExtrusionMachine,
        "Industrial Wire Factory" => IndustrialWireFactory,
        "Industrial Sledgehammer" => IndustrialSledgehammer,
        "Dissection Apparatus" => DissectionApparatus,
        "Fluid Shaper" => FluidShaper,
        "Industrial Cutting Factory" => IndustrialCuttingFactory,
        "Large Fluid Extractor" => LargeFluidExtractor,
        "Industrial Maceration Stack" => IndustrialMacerationStack,
        "Blast Furnace" => BlastFurnace,
        "Mega Blast Furnace" => MegaBlastFurnace,
        "Volcanus" => Volcanus,
        "Mega Alloy Blast Smelter" => MegaAlloyBlastSmelter,
        "Vacuum Freezer" => VacuumFreezer,
        "Mega Vacuum Freezer" => MegaVacuumFreezer,
        "Cryogenic Freezer" => CryogenicFreezer,
        "Industrial Mixing Machine" => IndustrialMixingMachine,
        "Hyper-Intensity Laser Engraver" => HyperIntensityLaserEngraver,
        "Industrial Centrifuge" => IndustrialCentrifuge,
        "Industrial Autoclave" => IndustrialAutoclave,
        "Precise Auto-Assembler MT-3662" => PreciseAutoAssemblerMT3662,
        "Pseudostable Black Hole Containment Field" => PseudostableBlackHoleContainmentField,
        "Industrial Electrolyzer" => IndustrialElectrolyzer,
        "Utupu-Tanuri" => UtupuTanuri,
        "Electric Implosion Compressor" => ElectricImplosionCompressor,
        "Ore Washing Plant" => OreWashingPlant,
    })
}

pub trait Overclock {
    const PARALLELS_OFFSET: u64 = 0;
    const PARALLELS_PER_TIER: u64 = 0;
    const SPEED_MODIFIER: f64 = 1.00;
    const ENERGY_MODIFIER: f64 = 1.00;

    fn max_parallels(
        &self,
        parallels_offset: u64,
        parallels_per_tier: u64,
        tier: u64,
        _machine: &MachineConfiguration,
    ) -> u64 {
        parallels_offset + tier * parallels_per_tier
    }

    fn speed_modifier(&self, _machine: &MachineConfiguration, speed_modifier: f64) -> f64 {
        speed_modifier
    }

    fn energy_modifier(
        &self,
        _machine: &MachineConfiguration,
        _recipe: &Recipe,
        _tier: u64,
        energy_modifier: f64,
    ) -> f64 {
        energy_modifier
    }

    fn perfect_overclocks(
        &self,
        _machine: &MachineConfiguration,
        _recipe: &Recipe,
        _tier: u64,
    ) -> u64 {
        0
    }

    #[rustfmt::skip]
    fn advised_batch(&self, machine: &MachineConfiguration, ticks: u64, recipe: &Recipe) -> u64 {
        // Extract machine parameters or fallback to defaults
        let parallels_offset = machine.parallels_offset.unwrap_or(Self::PARALLELS_OFFSET);
        let parallels_per_tier = machine.parallels_per_tier.unwrap_or(Self::PARALLELS_PER_TIER);
        let speed_modifier = machine.speed_modifier.unwrap_or(Self::SPEED_MODIFIER);
        let energy_modifier = machine.energy_modifier.unwrap_or(Self::ENERGY_MODIFIER);

        // Determine the machine tier based on energy usage
        let tier = u64::ilog(machine.energy_usage / 8, 4) as u64;

        // Calculate max parallels allowed at this tier
        let max_parallels = self.max_parallels(parallels_offset, parallels_per_tier, tier, machine);

        // Adjust energy consumption based on machine and recipe specifics
        let energy_modifier = self.energy_modifier(machine, recipe, tier, energy_modifier);
        let adjusted_energy_usage = f64::ceil(recipe.energy_usage as f64 * energy_modifier) as u64;
        let mut effective_parallels = u64::min(machine.energy_usage / adjusted_energy_usage, max_parallels);
        let energy_used = effective_parallels * adjusted_energy_usage;

        // Determine overclock levels
        let overclocks = u64::ilog(machine.energy_usage / energy_used, 4) as u64;
        let overclocks = u64::min(overclocks, machine.maximum_overclock_tier - tier);

        // Split into regular and perfect overclocks
        let perfect_overclocks = u64::min(overclocks, self.perfect_overclocks(machine, recipe, tier));
        let regular_overclocks = overclocks - perfect_overclocks;

        // Compute adjusted processing time based on overclocks and speed modifier
        let overclocked_processing_time = recipe.duration as f64
            / self.speed_modifier(machine, speed_modifier)
            / 2f64.powi(regular_overclocks as i32)
            / 4f64.powi(perfect_overclocks as i32);

        // Ensure processing time is at least 1 tick
        let corrected_processing_time = f64::max(overclocked_processing_time, 1.0) as u64;

        // Adjust effective parallels for sub-tick cases
        if overclocked_processing_time < 1.0 {
            effective_parallels = u64::max(effective_parallels, (max_parallels as f64 / overclocked_processing_time) as u64)
        };

        // Compute the advised batch size
        if corrected_processing_time <= ticks {
            (effective_parallels as f64 * (ticks as f64 + 0.99) / corrected_processing_time as f64) as u64
        } else {
            effective_parallels
        }
    }
}
