use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// Represents a crafting or processing request sent by the client to the server.
///
/// # Fields
/// - `machine`: The name of the machine that should process the pattern.
/// - `ticks`: The number of ticks to run the machine for at least
/// - `inputs`: A list of input `Item`s required for the recipe.
/// - `outputs`: A list of expected output `Item`s.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct OptimizationRequest {
    #[serde(rename = "machine")]
    pub machine: MachineConfiguration,
    #[serde(rename = "ticks")]
    pub ticks: u64,
    #[serde(rename = "inputs")]
    pub inputs: Vec<RequestItem>,
    #[serde(rename = "outputs")]
    pub outputs: Vec<RequestItem>,
}

/// Represents the configuration of a machine.
///
/// # Fields
/// - `id`: A unique identifier for the machine.
/// - `recipes`: A list of recipe categories that the machine can process.
/// - `energy_usage`: The maximum energy usage for the machine.
/// - `parallels_offset`: The base number of parallel processes the machine can handle.
/// - `parallels_per_tier`: The number of additional parallel processes available per tier.
/// - `speed_modifier`: A modifier applied to the speed of the machine.
/// - `energy_modifier`: A modifier that affects the energy consumption of the machine.
/// - `maximum_overclock_tier`: The highest tier of overclocking that the machine can handle.
/// - `tier`: The tier of the machine (e.g. Forge Hammer, Macerator).
/// - `expansion_width`: The expansion width of the machine (e.g. Fluid Solidifier).
/// - `solenoid_tier`: The tier of solenoids used in the machine.
/// - `coil_tier`: The tier of coils used in the machine.
/// - `laser_amperage`: The amperage of the laser source hatch.
/// - `pipe_casing_tier`: The tier of pipe casing used in the machine.
/// - `item_pipe_casing_tier`: The tier of item pipe casing used in the machine.
/// - `glass_tier`: The tier of glass used in the machine.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct MachineConfiguration {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "recipes")]
    pub recipes: Vec<String>,
    #[serde(rename = "energyUsage", default)]
    pub energy_usage: u64,
    #[serde(rename = "parallelsOffset")]
    pub parallels_offset: Option<u64>,
    #[serde(rename = "parallelsPerTier")]
    pub parallels_per_tier: Option<u64>,
    #[serde(rename = "speedModifier")]
    pub speed_modifier: Option<f64>,
    #[serde(rename = "energyModifier")]
    pub energy_modifier: Option<f64>,
    #[serde(rename = "maximumOverclockTier", default = "maximum_overclock_tier")]
    pub maximum_overclock_tier: u64,
    #[serde(rename = "tier", default = "one")]
    pub tier: u64,
    #[serde(rename = "width", default)]
    pub width: u64,
    #[serde(rename = "height", default)]
    pub height: u64,
    #[serde(rename = "solenoidTier", default = "one")]
    pub solenoid_tier: u64,
    #[serde(rename = "coilTier", default = "one")]
    pub coil_tier: u64,
    #[serde(rename = "laserAmperage", default = "laser_amperage")]
    pub laser_amperage: u64,
    #[serde(rename = "pipeCasingTier", default = "one")]
    pub pipe_casing_tier: u64,
    #[serde(rename = "itemPipeCasingTier", default = "one")]
    pub item_pipe_casing_tier: u64,
    #[serde(rename = "glassTier", default = "one")]
    pub glass_tier: u64,
    #[serde(rename = "upgrades", default)]
    pub upgrades: GorgeUpgrades,
    #[serde(rename = "dtr", default)]
    pub dtr: u64,
    #[serde(rename = "rings", default = "one")]
    pub rings: u64,
}

fn one() -> u64 {
    1
}

fn maximum_overclock_tier() -> u64 {
    u64::MAX
}

fn laser_amperage() -> u64 {
    32
}

#[derive(Deserialize, Debug, Default)]
#[allow(unused)]
pub struct GorgeUpgrades {
    /// Forge of the Gods
    ///
    /// Unlocks the base functionality of the Forge of the Gods, meaning:
    /// - 8 module slots
    /// - 1 ring
    /// - The Helioflare Power Forge module
    /// - 2 billion EU/t processing voltage
    /// - 15,000K heat bonus cap
    #[serde(rename = "START", default)]
    pub start: bool,
    /// Improved Gravitational Convection Coils
    ///
    /// Unlocks a recipe time reduction multiplier based on the current heat the multi is running at.
    /// This bonus is calculated via the following formula:
    /// `Multiplier = 1 / (Heat^0.01)`
    #[serde(rename = "IGCC", default)]
    pub igcc: bool,
    /// Graviton-Induced Superconductivity System
    ///
    /// Increases the base processing voltage of all modules by:
    /// `Stellar Fuel Units/sec * 10^8 EU/t`
    #[serde(rename = "GISS", default)]
    pub giss: bool,
    /// Superluminal Amplifier
    ///
    /// Unlocks a multiplier to maximum parallel based on fuel consumption rate.
    /// This bonus is calculated via the formula:
    /// `Multiplier = 1 + (Stellar Fuel Units/sec) / 15`
    #[serde(rename = "SA", default)]
    pub sa: bool,
    /// Relativistic Electron Capacitor
    ///
    /// - Unlocks a configuration window for maximum battery size and increases the limit to `max int`.
    /// - Unlocks an energy discount multiplier that scales with battery size (capped at `5%`).
    /// - Discount formula:
    ///   `Discount = (1 - 1.05^(-0.05 * Max Battery Capacity)) / 20`
    #[serde(rename = "REC", default)]
    pub rec: bool,
    /// Closed Timelike Curve Disruption Device
    ///
    /// Adds a `2×` multiplier to maximum parallel.
    #[serde(rename = "CTCDD", default)]
    pub ctcdd: bool,
    /// Singularity Exposure Fuel Compression Process
    ///
    /// Improves the fuel consumption → heat conversion formula.
    /// Improved formula:
    /// - **Power Forge Heat**: `log1.12(Stellar Fuel Units/sec) * 1000 + 12601`
    /// - **Melting Core Heat**: `log1.18(Stellar Fuel Units/sec) * 1000 + 12601`
    #[serde(rename = "SEFCP", default)]
    pub sefcp: bool,
    /// Transfinite Construction Techniques
    ///
    /// Improves the formula of **SA** to:
    /// `Multiplier = 1 + (Stellar Fuel Units/sec) / 5`
    #[serde(rename = "TCT", default)]
    pub tct: bool,
    /// Gravitationally Guided Electron Beam Emitter
    ///
    /// Improves the **OC** formula from `4/2 OCs` to `4/2.3 OCs`.
    #[serde(rename = "GGEBE", default)]
    pub ggebe: bool,
    /// Temporal Plasma Transformation Process
    ///
    /// Allows the Heliothermal Plasma Fabricator to process multi-step plasmas.
    /// **Tier restriction still applies.**
    #[serde(rename = "TPTP", default)]
    pub tptp: bool,
    /// Critical Neutrino Tunneling Integration
    ///
    /// Increases the cap of **EBF** heat bonuses to `30,000K`.
    #[serde(rename = "CNTI", default)]
    pub cnti: bool,
    /// Extreme Pulsar Exposure Chambers
    ///
    /// Unlocks a multiplier to maximum parallel based on current heat.
    /// Multiplier formula:
    /// `Multiplier = 1 + Heat / 15000`
    #[serde(rename = "EPEC", default)]
    pub epec: bool,
    /// Internal Micro-Kugelblitz Generator
    ///
    /// - Improves **EBF** energy reduction heat bonus from `5%` to `8%`.
    /// - Adds an energy discount based on the fill level of the internal battery.
    /// - Discount formula:
    ///   `Discount = (Current fill level / Max Capacity - 0.5)^2 * (-0.6) + 0.15`
    ///   *(multiplied by `2/3` for modules other than the Plasma Fabricator).*
    #[serde(rename = "IMKG", default)]
    pub imkg: bool,
    /// Duplicity of Potency
    ///
    /// Allows the Helioflare Power Forge to receive the full benefits of the
    /// Helioflux Melting Core upgrade path.
    #[serde(rename = "DoP", default)]
    pub dop: bool,
    /// Neutron Degeneracy Pressure Exposure
    ///
    /// **EBF** heat bonuses are granted above `30,000K`, but heat used in calculations
    /// follows this formula:
    /// - **Power Forge Heat**: `30000 + (Current Heat - 30000)^0.85`
    /// - **Melting Core Heat**: `30000 + (Current Heat - 30000)^0.8`
    #[serde(rename = "NDPE", default)]
    pub ndpe: bool,
    /// Parity of Singularity
    ///
    /// Unlocks a multiplier to maximum parallel based on total purchased upgrades.
    /// Multiplier formula:
    /// `Multiplier = 1 + Upgrade Amount / 5`
    #[serde(rename = "PoS", default)]
    pub pos: bool,
    /// Disparity of Rarity
    ///
    /// Improves **IGCC** based on current maximum parallel.
    /// Improved formula:
    /// `Multiplier = (1 / Heat^0.01) / (Parallel^0.02)`
    #[serde(rename = "DoR", default)]
    pub dor: bool,
    /// Null-Gravity Modulation Sheath
    ///
    /// Multiplies maximum processing voltage by `4` per active ring.
    /// *(Applies after other bonuses.)*
    #[serde(rename = "NGMS", default)]
    pub ngms: bool,
    /// Paradoxical Attainment
    ///
    /// Allows the Heliofusion Exoticizer to be affected by other upgrade benefits,
    /// but those benefits are square rooted first.
    /// The overclock bonus is adjusted via the following formula: OC Factor = 2 + (Base OC Factor - 2)^2
    #[serde(rename = "PA", default)]
    pub pa: bool,
    /// Cosmically Duplicate
    ///
    /// Allows construction of the second ring and adds 4 module slots.
    #[serde(rename = "CD", default)]
    pub cd: bool,
    /// Transfinite Stellar Existence
    ///
    /// Uncaps maximum fuel consumption,
    /// but fuel consumption in bonus calculations scales according to this formula:
    /// Actual FC = Current Max FC + (Current FC - Current Max FC)^0.4,
    /// where FC refers to fuel consumption and max FC refers to the max fuel consumption without this upgrade.
    #[serde(rename = "TSE", default)]
    pub tse: bool,
    /// The Boundless Flow
    ///
    /// Uncaps maximum processing voltage. Voltage can be set in each module's GUI.
    #[serde(rename = "TBF", default)]
    pub tbf: bool,
    /// Effortless Existence
    ///
    /// Allows construction of the third ring and adds 4 module slots.
    #[serde(rename = "EE", default)]
    pub ee: bool,
    /// Orion's Arm Genesis Schema
    ///
    /// Unlocks Magmatter production in the Heliofusion Exoticizer,
    /// creation of exotic plasmas in the Heliothermal Plasma Fabricator and Graviton Shard ejection & injection.
    #[serde(rename = "END", default)]
    pub end: bool,
}

/// This struct is used as part of an `OptimizationRequest`, describing the input
/// and output items involved in a machine process.
///
/// # Fields
/// - `name`: The unique identifier or registry name of the item.
/// - `label`: The human-readable name of the item.
/// - `size`: The quantity of this item in the request.
/// - `max_size`: The maximum stack size for this item.
/// - `damage`: The item's current damage or metadata value.
/// - `max_damage`: The maximum possible damage value (durability).
/// - `has_tag`: Whether the item has an NBT tag.
/// - `capacity`: Optional capacity (used for items like fluid containers).
/// - `fluid`: Optional fluid information if the item represents a fluid container.
/// - `fluid_drop`: Optional details if the item represents a fluid source.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct RequestItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "size")]
    pub size: u64,
    #[serde(rename = "maxSize")]
    pub max_size: u64,
    #[serde(rename = "damage")]
    pub damage: u64,
    #[serde(rename = "maxDamage")]
    pub max_damage: u64,
    #[serde(rename = "hasTag")]
    pub has_tag: bool,
    #[serde(rename = "capacity")]
    pub capacity: Option<u64>,
    #[serde(rename = "fluid")]
    pub fluid: Option<Fluid>,
    #[serde(rename = "fluidDrop")]
    pub fluid_drop: Option<FluidDrop>,
}

/// Represents a quantity of fluid associated with an item in an optimization request.
///
/// # Fields
/// - `amount`: The volume of fluid, typically measured in millibuckets (mB).
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Fluid {
    amount: u64,
}

/// Represents a fluid drop associated with an item in an optimization request.
///
/// # Fields
/// - `label`: The human-readable name of the fluid.
/// - `id`: The unique registry identifier of the fluid.
/// - `amount`: The quantity of fluid in millibuckets.
/// - `has_tag`: Indicates whether the fluid has associated NBT data.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct FluidDrop {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "amount")]
    pub amount: u64,
    #[serde(rename = "hasTag")]
    pub has_tag: bool,
}

impl Display for OptimizationRequest {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Machine: {}", self.machine.id)?;
        writeln!(f, "Using recipes: {}", self.machine.recipes.join(", "))?;
        writeln!(f, "- Energy usage: {} EU/t", self.machine.energy_usage)?;

        if let Some(parallels_offset) = self.machine.parallels_offset {
            writeln!(f, "- Parallels offset: {}", parallels_offset)?;
        }
        if let Some(parallels_per_tier) = self.machine.parallels_per_tier {
            writeln!(f, "- Parallels per tier: {}", parallels_per_tier)?;
        }
        if let Some(speed_modifier) = self.machine.speed_modifier {
            writeln!(f, "- Speed modifier: {:.2}", speed_modifier)?;
        }
        if let Some(energy_modifier) = self.machine.energy_modifier {
            writeln!(f, "- Energy modifier: {:.2}", energy_modifier)?;
        }

        writeln!(f, "- Maximum overclock tier: {}", self.machine.maximum_overclock_tier)?;
        writeln!(f, "- Tier: {}", self.machine.tier)?;
        writeln!(f, "- Dimensions (W x H): {} x {}", self.machine.width, self.machine.height)?;
        writeln!(f, "- Solenoid tier: {}", self.machine.solenoid_tier)?;
        writeln!(f, "- Coil tier: {}", self.machine.coil_tier)?;
        writeln!(f, "- Laser amperage: {} A", self.machine.laser_amperage)?;
        writeln!(f, "- Pipe casing tier: {}", self.machine.pipe_casing_tier)?;
        writeln!(f, "- Item pipe casing tier: {}", self.machine.item_pipe_casing_tier)?;
        writeln!(f, "- Glass tier: {}", self.machine.glass_tier)
    }
}
