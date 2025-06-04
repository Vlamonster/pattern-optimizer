-- Tier constants
local ULV =  0
local  LV =  1
local  MV =  2
local  HV =  3
local  EV =  4
local  IV =  5
local LuV =  6
local ZPM =  7
local  UV =  8
local UHV =  9
local UEV = 10
local UIV = 11
local UMV = 12
local UXV = 13
local MAX = 14

-- Coil constants
local Cupronickel       =  1
local Kanthal           =  2
local Nichrome          =  3
local TPVAlloy          =  4
local HSSG              =  5
local HSSS              =  6
local Naquadah          =  7
local NaquadahAlloy     =  8
local Trinium           =  9
local ElectrumFlux      = 10 -- Do not confuse with FluxedElectum Item Pipe Casing
local AwakenedDraconium = 11
local Infinity          = 12
local Hypogen           = 13
local Eternal           = 14

-- Pipe Casing constants
local Bronze        = 1
local Steel         = 2
local Titanium      = 3
local TungstenSteel = 4

-- Item Pipe Casing constants
local Tin            = 1
local Brass          = 2
local Electrum       = 3
local Platinum       = 4
local Osmium         = 5
local Quantium       = 6
local FluxedElectrum = 7 -- Do not confuse with ElectrumFlux Coil
local BlackPlutonium = 8

-- Voltage constants indexed by tier
local EU = {
    [ULV] =          8, --             8
    [LV]  =         32, --            32
    [MV]  =        128, --           128
    [HV]  =        512, --           512
    [EV]  =       2048, --         2,048
    [IV]  =       8192, --         8,192
    [LuV] =      32768, --        32,768
    [ZPM] =     131072, --       131,072
    [UV]  =     524288, --       524,288
    [UHV] =    2097152, --     2,097,152
    [UEV] =    8388608, --     8,388,608
    [UIV] =   33554432, --    33,554,432
    [UMV] =  134217728, --   134,217,728
    [UXV] =  536870912, --   536,870,912
    [MAX] = 2147483648, -- 2,147,483,648
}

-- Fluid Hatch constants indexed by tier
local HATCH = {
    [ULV] =      8000, --       8,000
    [LV]  =     16000, --      16,000
    [MV]  =     32000, --      32,000
    [HV]  =     64000, --      64,000
    [EV]  =    128000, --     128,000
    [IV]  =    256000, --     256,000
    [LuV] =    512000, --     512,000
    [ZPM] =   1024000, --   1,024,000
    [UV]  =   2048000, --   2,048,000
    [UHV] =   4096000, --   4,096,000
    [UEV] =   8192000, --   8,192,000
    [UIV] =  16384000, --  16,384,000
    [UMV] =  32768000, --  32,768,000
    [UXV] =  65536000, --  65,536,000
    [MAX] = 131072000, -- 131,072,000
}

local K = 1000     -- Thousand
local M = 1000 * K -- Million
local G = 1000 * M -- Billion
local T = 1000 * G -- Trillion

local INF = (1 << 63) - 1 -- Infinity

-- Upgrades for Gorge
local upgrades = {
    START = true,
    IGCC = true,
    GISS = true,
    SA = true,
    REC = true,
    CTCDD = true,
    SEFCP = true,
    TCT = false,
    GGEBE = false,
    TPTP = false,
    CNTI = true,
    EPEC = false,
    IMKG = false,
    DoP = false,
    NDPE = true,
    PoS = false,
    DoR = false,
    NGMS = true,
    PA = false,
    CD = false,
    TSE = false,
    TSE = false,
    TBF = false,
    EE = false,
    END = false,
}

local machines = {
    ["bender"] = {
        id = "Industrial Material Press",
        recipes = {"Bending Machine"},
        energyUsage = EU[ULV],
    },
    ["press"] = {
        id = "Industrial Material Press",
        recipes = {"Forming Press"},
        energyUsage = EU[ULV],
    },
    ["extruder"] = {
        id = "Industrial Extrusion Machine",
        recipes = {"Extruder"},
        energyUsage = EU[ULV],
    },
    ["wiremill"] = {
        id = "Industrial Wire Factory",
        recipes = {"Wiremill"},
        energyUsage = EU[ULV],
    },
    ["hammer"] = {
        id = "Industrial Sledgehammer",
        recipes = {"Forge Hammer"},
        energyUsage = EU[ULV],
        tier = 4,
    },
    ["extractor"] = {
        id = "Dissection Apparatus",
        recipes = {"Extractor"},
        energyUsage = EU[ULV],
        itemPipeCasingTier = Tin,
    },
    ["solidifier"] = {
        id = "Fluid Shaper",
        recipes = {"Fluid Solidifier", "Tool Casting Machine"},
        energyUsage = EU[ULV],
        width = 0, -- The number of width expansions, min = 0, max = 6
    },
    ["cutter"] = {
        id = "Industrial Cutting Factory",
        recipes = {"Cutting Machine"},
        energyUsage = EU[ULV],
    },
    ["fextractor"] = {
        id = "Large Fluid Extractor",
        recipes = {"Fluid Extractor"},
        energyUsage = EU[ULV],
        solenoidTier = MV,
        coilTier = Cupronickel,
    },
    ["macerator"] = {
        id = "Industrial Maceration Stack",
        recipes = {"Macerator"},
        energyUsage = EU[ULV],
        tier = 1, -- No upgrade = 0, Upgrade = 2
    },
    ["ebf"] = {
        id = "Blast Furnace",
        recipes = {"Blast Furnace"},
        energyUsage = EU[ULV],
        coilTier = Cupronickel,
    },
    ["mebf"] = {
        id = "Mega Blast Furnace",
        recipes = {"Blast Furnace"},
        energyUsage = EU[ULV],
        coilTier = Cupronickel,
    },
    ["volcanus"] = {
        id = "Volcanus",
        recipes = {"Blast Furnace"},
        energyUsage = EU[ULV],
        coilTier = Cupronickel,
    },
    ["freezer"] = {
        id = "Vacuum Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = EU[ULV],
    },
    ["mvf"] = {
        id = "Mega Vacuum Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = EU[ULV],
        tier = 0, -- No Subspace Cooling = 0, SpaceTime = 1, Spatially Enlarged = 2, Eternity = 3
    },
    ["cryo"] = {
        id = "Cryogenic Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = EU[ULV],
    },
    ["mixer"] = {
        id = "Industrial Mixing Machine",
        recipes = {"Multiblock Mixer"},
        energyUsage = EU[ULV],
    },
    ["engraver"] = {
        id = "Hyper-Intensity Laser Engraver",
        recipes = {"Laser Engraver"},
        energyUsage = EU[ULV],
        laserAmperage = 256,
    },
    ["centrifuge"] = {
        id = "Industrial Centrifuge",
        recipes = {"Multiblock Centrifuge"},
        energyUsage = EU[ULV],
    },
    ["autoclave"] = {
        id = "Industrial Autoclave",
        recipes = {"Autoclave"},
        energyUsage = EU[ULV],
        coilTier = Cupronickel,
        pipeCasingTier = Bronze,
        itemPipeCasingTier = Tin,
    },
    ["mabs"] = {
        id = "Mega Alloy Blast Smelter",
        recipes = {"Alloy Blast Smelter"},
        energyUsage = EU[ULV],
        coilTier = Cupronickel,
        glassTier = HV,
    },
    ["assembler"] = {
        id = "Precise Auto-Assembler MT-3662",
        recipes = {"Assembler"},
        energyUsage = EU[ULV],
        tier = 5, -- Mk4 is tier 5
    },
    ["compressor"] = {
        id = "Large Electric Compressor",
        recipes = {"Compressor"},
        energyUsage = EU[ULV],
    },
    ["blackhole"] = {
        id = "Pseudostable Black Hole Containment Field",
        recipes = {"Neutronium Compressor", "Compressor"},
        energyUsage = EU[ULV],
        maximumOverclockTier = ULV,
    },
    ["electrolyzer"] = {
        id = "Industrial Electrolyzer",
        recipes = {"Multiblock Electrolyzer"},
        energyUsage = EU[ULV],
    },
    ["dehydrator"] = {
        id = "Utupu-Tanuri",
        recipes = {"Multiblock Dehydrator"},
        energyUsage = EU[ULV],
    },
    ["chembath"] = {
        id = "Ore Washing Plant",
        recipes = {"Chemical Bath"},
        energyUsage = EU[ULV],
    },
    ["mcr"] = {
        id = "Mega Chemical Reactor",
        recipes = {"Large Chemical Reactor"},
        energyUsage = EU[ULV],
    },
    ["lathe"] = {
        id = "Industrial Precision Lathe",
        recipes = {"Lathe"},
        energyUsage = EU[ULV],
        itemPipeCasingTier = Tin,
    },
    ["alloysmelter"] = {
        id = "Zyngen",
        recipes = {"Alloy Smelter"},
        energyUsage = EU[ULV],
        coilTier = Cupronickel,
    },
    ["arcfurnace"] = {
        id = "High Current Industrial Arc Furnace",
        recipes = {"Arc Furnace"},
        energyUsage = EU[ULV],
        tier = 1,
        width = 7,
    },
    ["plasmafurnace"] = {
        id = "High Current Industrial Arc Furnace",
        recipes = {"Plasma Arc Furnace"},
        energyUsage = EU[ULV],
        tier = 2,
        width = 7,
    },
    ["nactivator"] = {
        id = "Neutron Activator",
        recipes = {"Neutron Activator"},
        height = 4, -- Height is the number of Speeding Pipe Casings
    },
    ["implosion"] = {
        id = "Electric Implosion Compressor",
        recipes = {"Electric Implosion Compressor"},
        energyUsage = EU[ULV],
        tier = 4,
    },
    ["sifter"] = {
        id = "Large Sifter Control Block",
        recipes = {"Sifter"},
        energyUsage = EU[ULV],
    },
    ["cal"] = {
        id = "Circuit Assembly Line",
        recipes = {"Circuit Assembly Line", "Circuit Assembler"},
        energyUsage = EU[LV],
        tier = ULV,
    },
    ["coal"] = {
        id = "Component Assembly Line",
        recipes = {"Component Assembly Line"},
        energyUsage = EU[ULV],
        tier = LV,
    },
    ["mdt"] = {
        id = "Mega Distillation Tower",
        recipes = {"Distillation Tower"},
        energyUsage = EU[ULV],
    },
    ["gorge1"] = {
        id = "Helioflare Power Forge",
        recipes = {"Blast Furnace"},
        dtr = 18, -- Dimensionally Transcendent Residue Factor
        rings = 1,
        upgrades = upgrades,
    },
    ["gorge2"] = {
        id = "Helioflux Melting Core",
        recipes = {"Helioflux Melting Core"},
        dtr = 18, -- Dimensionally Transcendent Residue Factor
        rings = 1,
        upgrades = upgrades,
    },
    ["smelter"] = {
        id = "Multi Smelter",
        recipes = {"Multi Smelter"},
        energyUsage = EU[ULV],
        coilTier = Cupronickel,
    },
    ["magnet"] = {
        id = "Magnetic Flux Exhibitor",
        recipes = {"Electromagnetic Polarizer", "Electromagnetic Separator"},
        energyUsage = EU[ULV],
        tier = 1, -- Tier of the electromagnet, iron = 1, .., tengam = 5
    },
    ["coke"] = {
        id = "Industrial Coke Oven",
        recipes = {"Coke Oven"},
        energyUsage = EU[ULV],
        tier = 1, -- Tier of the casings, heat resistant = 1, heat proof = 2
    },
    ["brewery"] = {
        id = "Big Barrel Brewery",
        recipes = {"Brewery"},
        energyUsage = EU[ULV],
    },
}

return machines
