use serde::Deserialize;

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
