mod big_barrel_brewery;
mod blast_furnace;
mod circuit_assembly_line;
mod component_assembly_line;
mod cryogenic_freezer;
mod dissection_apparatus;
mod electric_implosion_compressor;
mod fluid_shaper;
mod helioflare_power_forge;
mod helioflux_melting_core;
mod high_current_industrial_arc_furnace;
mod hyper_intensity_laser_engraver;
mod industrial_autoclave;
mod industrial_centrifuge;
mod industrial_coke_oven;
mod industrial_cutting_factory;
mod industrial_electrolyzer;
mod industrial_extrusion_machine;
mod industrial_maceration_stack;
mod industrial_material_press;
mod industrial_mixing_machine;
mod industrial_precision_lathe;
mod industrial_sledgehammer;
mod industrial_wire_factory;
mod large_electric_compressor;
mod large_fluid_extractor;
mod large_sifter_control_block;
mod magnetic_flux_exhibitor;
mod mega_alloy_blast_smelter;
mod mega_blast_furnace;
mod mega_chemical_reactor;
mod mega_distillation_tower;
mod mega_vacuum_freezer;
mod multi_smelter;
mod neutron_activator;
mod ore_washing_plant;
mod precise_auto_assembler_mt_3662;
mod pseudostable_black_hole_containment_field;
mod utupu_tanuri;
mod vacuum_freezer;
mod volcanus;
mod zyngen;

use {
    crate::{
        model::GregTechRecipe,
        optimize::Overclock,
        request::OptimizationRequest,
        MainError,
    },
    big_barrel_brewery::BigBarrelBrewery,
    blast_furnace::BlastFurnace,
    circuit_assembly_line::CircuitAssemblyLine,
    component_assembly_line::ComponentAssemblyLine,
    cryogenic_freezer::CryogenicFreezer,
    dissection_apparatus::DissectionApparatus,
    electric_implosion_compressor::ElectricImplosionCompressor,
    fluid_shaper::FluidShaper,
    helioflare_power_forge::HelioflarePowerForge,
    helioflux_melting_core::HeliofluxMeltingCore,
    high_current_industrial_arc_furnace::HighCurrentIndustrialArcFurnace,
    hyper_intensity_laser_engraver::HyperIntensityLaserEngraver,
    industrial_autoclave::IndustrialAutoclave,
    industrial_centrifuge::IndustrialCentrifuge,
    industrial_coke_oven::IndustrialCokeOven,
    industrial_cutting_factory::IndustrialCuttingFactory,
    industrial_electrolyzer::IndustrialElectrolyzer,
    industrial_extrusion_machine::IndustrialExtrusionMachine,
    industrial_maceration_stack::IndustrialMacerationStack,
    industrial_material_press::IndustrialMaterialPress,
    industrial_mixing_machine::IndustrialMixingMachine,
    industrial_precision_lathe::IndustrialPrecisionLathe,
    industrial_sledgehammer::IndustrialSledgehammer,
    industrial_wire_factory::IndustrialWireFactory,
    large_electric_compressor::LargeElectricCompressor,
    large_fluid_extractor::LargeFluidExtractor,
    large_sifter_control_block::LargeSifterControlBlock,
    magnetic_flux_exhibitor::MagneticFluxExhibitor,
    mega_alloy_blast_smelter::MegaAlloyBlastSmelter,
    mega_blast_furnace::MegaBlastFurnace,
    mega_chemical_reactor::MegaChemicalReactor,
    mega_distillation_tower::MegaDistillationTower,
    mega_vacuum_freezer::MegaVacuumFreezer,
    multi_smelter::MultiSmelter,
    neutron_activator::NeutronActivator,
    ore_washing_plant::OreWashingPlant,
    precise_auto_assembler_mt_3662::PreciseAutoAssemblerMT3662,
    pseudostable_black_hole_containment_field::PseudostableBlackHoleContainmentField,
    utupu_tanuri::UtupuTanuri,
    vacuum_freezer::VacuumFreezer,
    volcanus::Volcanus,
    zyngen::Zyngen,
};

pub fn optimize_batch_size(request: &OptimizationRequest, recipe: &GregTechRecipe) -> Result<(u64, u64), MainError> {
    match request.machine.id.as_str() {
        "Industrial Material Press" => IndustrialMaterialPress().optimize_batch_size(request, recipe),
        "Industrial Extrusion Machine" => IndustrialExtrusionMachine().optimize_batch_size(request, recipe),
        "Industrial Wire Factory" => IndustrialWireFactory().optimize_batch_size(request, recipe),
        "Industrial Sledgehammer" => IndustrialSledgehammer().optimize_batch_size(request, recipe),
        "Dissection Apparatus" => DissectionApparatus().optimize_batch_size(request, recipe),
        "Fluid Shaper" => FluidShaper().optimize_batch_size(request, recipe),
        "Industrial Cutting Factory" => IndustrialCuttingFactory().optimize_batch_size(request, recipe),
        "Large Fluid Extractor" => LargeFluidExtractor().optimize_batch_size(request, recipe),
        "Industrial Maceration Stack" => IndustrialMacerationStack().optimize_batch_size(request, recipe),
        "Blast Furnace" => BlastFurnace().optimize_batch_size(request, recipe),
        "Mega Blast Furnace" => MegaBlastFurnace().optimize_batch_size(request, recipe),
        "Volcanus" => Volcanus().optimize_batch_size(request, recipe),
        "Mega Alloy Blast Smelter" => MegaAlloyBlastSmelter().optimize_batch_size(request, recipe),
        "Vacuum Freezer" => VacuumFreezer().optimize_batch_size(request, recipe),
        "Mega Vacuum Freezer" => MegaVacuumFreezer().optimize_batch_size(request, recipe),
        "Cryogenic Freezer" => CryogenicFreezer().optimize_batch_size(request, recipe),
        "Industrial Mixing Machine" => IndustrialMixingMachine().optimize_batch_size(request, recipe),
        "Hyper-Intensity Laser Engraver" => HyperIntensityLaserEngraver().optimize_batch_size(request, recipe),
        "Industrial Centrifuge" => IndustrialCentrifuge().optimize_batch_size(request, recipe),
        "Industrial Autoclave" => IndustrialAutoclave().optimize_batch_size(request, recipe),
        "Precise Auto-Assembler MT-3662" => PreciseAutoAssemblerMT3662().optimize_batch_size(request, recipe),
        "Pseudostable Black Hole Containment Field" => PseudostableBlackHoleContainmentField().optimize_batch_size(request, recipe),
        "Industrial Electrolyzer" => IndustrialElectrolyzer().optimize_batch_size(request, recipe),
        "Utupu-Tanuri" => UtupuTanuri().optimize_batch_size(request, recipe),
        "Electric Implosion Compressor" => ElectricImplosionCompressor().optimize_batch_size(request, recipe),
        "Ore Washing Plant" => OreWashingPlant().optimize_batch_size(request, recipe),
        "Mega Chemical Reactor" => MegaChemicalReactor().optimize_batch_size(request, recipe),
        "Industrial Precision Lathe" => IndustrialPrecisionLathe().optimize_batch_size(request, recipe),
        "Zyngen" => Zyngen().optimize_batch_size(request, recipe),
        "High Current Industrial Arc Furnace" => HighCurrentIndustrialArcFurnace().optimize_batch_size(request, recipe),
        "Large Sifter Control Block" => LargeSifterControlBlock().optimize_batch_size(request, recipe),
        "Circuit Assembly Line" => CircuitAssemblyLine().optimize_batch_size(request, recipe),
        "Component Assembly Line" => ComponentAssemblyLine().optimize_batch_size(request, recipe),
        "Mega Distillation Tower" => MegaDistillationTower().optimize_batch_size(request, recipe),
        "Helioflare Power Forge" => HelioflarePowerForge().optimize_batch_size(request, recipe),
        "Helioflux Melting Core" => HeliofluxMeltingCore().optimize_batch_size(request, recipe),
        "Multi Smelter" => MultiSmelter().optimize_batch_size(request, recipe),
        "Large Electric Compressor" => LargeElectricCompressor().optimize_batch_size(request, recipe),
        "Magnetic Flux Exhibitor" => MagneticFluxExhibitor().optimize_batch_size(request, recipe),
        "Industrial Coke Oven" => IndustrialCokeOven().optimize_batch_size(request, recipe),
        "Big Barrel Brewery" => BigBarrelBrewery().optimize_batch_size(request, recipe),
        "Neutron Activator" => NeutronActivator().optimize_batch_size(request, recipe),
        machine => panic!("Unknown Machine: {machine}"),
    }
}
