use {
    crate::request::GorgeUpgrades,
    serde::Deserialize,
    std::fmt::{
        Display,
        Formatter,
    },
};

/// Represents the configuration of a machine.
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct MachineConfiguration {
    /// A unique identifier for the machine.
    #[serde(rename = "id")]
    pub id: String,

    /// A list of recipe categories that the machine can process.
    #[serde(rename = "recipes")]
    pub recipes: Vec<String>,

    /// The maximum energy usage for the machine.
    #[serde(rename = "energyUsage", default)]
    pub energy_usage: u64,

    /// The base number of parallel processes the machine can handle.
    #[serde(rename = "parallelsOffset")]
    pub parallels_offset: Option<u64>,

    /// The number of additional parallel processes available per tier.
    #[serde(rename = "parallelsPerTier")]
    pub parallels_per_tier: Option<u64>,

    /// A modifier applied to the speed of the machine.
    #[serde(rename = "speedModifier")]
    pub speed_modifier: Option<f64>,

    /// A modifier that affects the energy consumption of the machine.
    #[serde(rename = "energyModifier")]
    pub energy_modifier: Option<f64>,

    /// The highest tier of overclocking that the machine can handle.
    #[serde(rename = "maximumOverclockTier", default = "maximum_overclock_tier")]
    pub maximum_overclock_tier: u64,

    /// The tier of the machine (e.g. Forge Hammer, Macerator).
    #[serde(rename = "tier", default = "one")]
    pub tier: u64,

    /// The expansion width of the machine (e.g. Fluid Solidifier).
    #[serde(rename = "width", default)]
    pub width: u64,

    /// The expansion height of the machine.
    #[serde(rename = "height", default)]
    pub height: u64,

    /// The tier of solenoids used in the machine.
    #[serde(rename = "solenoidTier", default = "one")]
    pub solenoid_tier: u64,

    /// The tier of coils used in the machine.
    #[serde(rename = "coilTier", default = "one")]
    pub coil_tier: u64,

    /// The amperage of the laser source hatch.
    #[serde(rename = "laserAmperage", default = "laser_amperage")]
    pub laser_amperage: u64,

    /// The tier of pipe casing used in the machine.
    #[serde(rename = "pipeCasingTier", default = "one")]
    pub pipe_casing_tier: u64,

    /// The tier of item pipe casing used in the machine.
    #[serde(rename = "itemPipeCasingTier", default = "one")]
    pub item_pipe_casing_tier: u64,

    /// The tier of glass used in the machine.
    #[serde(rename = "glassTier", default = "one")]
    pub glass_tier: u64,

    /// Optional upgrade configuration.
    #[serde(rename = "upgrades", default)]
    pub upgrades: GorgeUpgrades,

    /// The dynamic transfer rate value for the machine.
    #[serde(rename = "dtr", default)]
    pub dtr: u64,

    /// The number of rings the machine has.
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
    256
}

impl Display for MachineConfiguration {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID:                     {}", self.id)?;
        writeln!(f, "Recipes:              \n{}", self.recipes.iter().map(|r| format!("- {r}")).collect::<Vec<_>>().join("\n"))?;
        writeln!(f, "Energy Usage:           {} EU/t", self.energy_usage)?;
        writeln!(f, "Parallels Offset:       {}", self.parallels_offset.as_ref().map_or("None".into(), u64::to_string))?;
        writeln!(f, "Parallels Per Tier:     {}", self.parallels_per_tier.as_ref().map_or("None".into(), u64::to_string))?;
        writeln!(f, "Speed Modifier:         {}", self.speed_modifier.as_ref().map_or("None".into(), f64::to_string))?;
        writeln!(f, "Energy Modifier:        {}", self.energy_modifier.as_ref().map_or("None".into(), f64::to_string))?;
        writeln!(f, "Maximum Overclock Tier: {}", self.maximum_overclock_tier)?;
        writeln!(f, "Tier:                   {}", self.tier)?;
        writeln!(f, "Width:                  {}", self.width)?;
        writeln!(f, "Height:                 {}", self.height)?;
        writeln!(f, "Solenoid Tier:          {}", self.solenoid_tier)?;
        writeln!(f, "Coil Tier:              {}", self.coil_tier)?;
        writeln!(f, "Laser Amperage:         {}", self.laser_amperage)?;
        writeln!(f, "Pipe Casing Tier:       {}", self.pipe_casing_tier)?;
        writeln!(f, "Item Pipe Casing Tier:  {}", self.item_pipe_casing_tier)?;
        writeln!(f, "Glass Tier:             {}", self.glass_tier)?;
        // writeln!(f, "Upgrades:               {}", self.upgrades)?;
        writeln!(f, "DTR:                    {}", self.dtr)?;
        writeln!(f, "Rings:                  {}", self.rings)
    }
}
