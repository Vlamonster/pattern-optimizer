use serde::Deserialize;

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
    #[serde(rename = "energyUsage")]
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
    #[serde(rename = "tier", default = "tier")]
    pub tier: u64,
    #[serde(rename = "expansionWidth", default = "expansion_width")]
    pub expansion_width: u64,
    #[serde(rename = "solenoidTier", default = "solenoid_tier")]
    pub solenoid_tier: u64,
    #[serde(rename = "coilTier", default = "coil_tier")]
    pub coil_tier: u64,
    #[serde(rename = "laserAmperage", default = "laser_amperage")]
    pub laser_amperage: u64,
    #[serde(rename = "pipeCasingTier", default = "pipe_casing_tier")]
    pub pipe_casing_tier: u64,
    #[serde(rename = "itemPipeCasingTier", default = "item_pipe_casing_tier")]
    pub item_pipe_casing_tier: u64,
    #[serde(rename = "glassTier", default = "glass_tier")]
    pub glass_tier: u64,
}

fn maximum_overclock_tier() -> u64 {
    u64::MAX
}

fn coil_tier() -> u64 {
    1
}

fn expansion_width() -> u64 {
    0
}

fn solenoid_tier() -> u64 {
    2
}

fn tier() -> u64 {
    1
}

fn laser_amperage() -> u64 {
    32
}

fn pipe_casing_tier() -> u64 {
    1
}

fn item_pipe_casing_tier() -> u64 {
    1
}

fn glass_tier() -> u64 {
    1
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
