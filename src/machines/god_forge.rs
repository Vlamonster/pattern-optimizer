// use crate::advice::{pow, Overclock};
// use crate::model::Recipe;
// use crate::optimization_request::MachineConfiguration;
//
// pub struct GodForge();
//
// // enum Fuel {
// //     Residue,
// //     Stellar,
// //     Star,
// // }
// //
// // impl Fuel {
// //     const fn consumption(&self) -> u64 {
// //         match self {
// //             Fuel::Residue => GodForge::FACTOR * 300 * pow(1.15, GodForge::FACTOR) as u64,
// //             Fuel::Stellar => GodForge::FACTOR * 2 * pow(1.08, GodForge::FACTOR) as u64,
// //             Fuel::Star => GodForge::FACTOR / 25,
// //         }
// //     }
// // }
//
// impl GodForge {
//     /// Improved Gravitational Convection Coils
//     /// If enabled: duration *= HEAT^(-0.01)
//     const IGCC: bool = true;
//     /// Superluminal Amplifier
//     /// If enabled: parallels *= 1 + fuel/15
//     const SA: bool = true;
//     const FUEL: Fuel = Fuel::Residue;
//     const EFFECTIVE_HEAT: u64 = 15_000;
//     const FACTOR: u64 = 5;
// }
//
// impl Overclock for GodForge {
//     // const PARALLELS_OFFSET: u64 = if Self::SA {
//     //     (1024.0 * (1.0 + Self::FACTOR as f64 / 15.0)) as u64
//     // } else {
//     //     1024
//     // };
//     // const PARALLELS_PER_TIER: u64 = 0;
//     // const SPEED_MODIFIER: f64 = 1.00;
//     // const ENERGY_MODIFIER: f64 = 1.00;
//
//     fn get_advised_batch(&self, machine: &MachineConfiguration, recipe: &Recipe) -> u64 {
//         // let heat = (f64::log(Self::FACTOR as f64, 1.5) * 1000.0) as u64 + 12601;
//         // let discounted_eut = f64::ceil(
//         //     recipe.energy_usage as f64
//         //         * machine.energy_modifier
//         //         * f64::powi(
//         //             0.95,
//         //             ((Self::EFFECTIVE_HEAT - recipe.special as u64) / 900) as i32,
//         //         ),
//         // ) as u64;
//         // let overclocks = u64::ilog(machine.energy_usage / discounted_eut, 4) as u64;
//         // let perfect_overclocks = u64::min(
//         //     overclocks,
//         //     (Self::EFFECTIVE_HEAT - recipe.special as u64) / 1800,
//         // );
//         // let regular_overclocks = overclocks - perfect_overclocks;
//         //
//         // let overclocked_processing_time = recipe.duration as f64
//         //     / machine.speed_modifier
//         //     / 2f64.powi(regular_overclocks as i32)
//         //     / 4f64.powi(perfect_overclocks as i32)
//         //     / if Self::IGCC {
//         //         f64::powf(heat as f64, 0.01)
//         //     } else {
//         //         1.0
//         //     };
//         //
//         // let corrected_processing_time = if overclocked_processing_time < 1.0 {
//         //     1
//         // } else {
//         //     overclocked_processing_time as u64
//         // };
//         //
//         // let subtick_parallels = if overclocked_processing_time < 1.0 {
//         //     (1.0 / overclocked_processing_time) as u64
//         // } else {
//         //     0
//         // };
//         //
//         // let advised_batch = if corrected_processing_time <= 20 {
//         //     (u64::max(1, subtick_parallels) as f64 * 20.99 / corrected_processing_time as f64)
//         //         as u64
//         // } else {
//         //     1
//         // };
//         //
//         // advised_batch * machine.parallels_offset
//         todo!()
//     }
// }
