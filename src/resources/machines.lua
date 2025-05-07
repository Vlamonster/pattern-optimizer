local ULV = 8       -- tier  0
local  LV = 4 * ULV -- tier  1
local  MV = 4 *  LV -- tier  2
local  HV = 4 *  MV -- tier  3
local  EV = 4 *  HV -- tier  4
local  IV = 4 *  EV -- tier  5
local LuV = 4 *  IV -- tier  6
local ZPM = 4 * LuV -- tier  7
local  UV = 4 * ZPM -- tier  8
local UHV = 4 *  UV -- tier  9
local UEV = 4 * UHV -- tier 10
local UIV = 4 * UEV -- tier 11
local UMV = 4 * UIV -- tier 12
local UXV = 4 * UMV -- tier 13
local MAX = 4 * UXV -- tier 14

local K = 1000
local M = 1000 * K
local G = 1000 * M
local T = 1000 * G

local INF = (1 << 63) - 1

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
        energyUsage = 4 * UMV,
    },
    ["press"] = {
        id = "Industrial Material Press",
        recipes = {"Forming Press"},
        energyUsage = 16 * UMV,
    },
    ["extruder"] = {
        id = "Industrial Extrusion Machine",
        recipes = {"Extruder"},
        energyUsage = 4 * UMV,
    },
    ["wiremill"] = {
        id = "Industrial Wire Factory",
        recipes = {"Wiremill"},
        energyUsage = 4 * UMV,
    },
    ["hammer"] = {
        id = "Industrial Sledgehammer",
        recipes = {"Forge Hammer"},
        energyUsage = 4 * UMV,
        tier = 4,
    },
    ["extractor"] = {
        id = "Dissection Apparatus",
        recipes = {"Extractor"},
        energyUsage = 4 * UMV,
    },
    ["solidifier"] = {
        id = "Fluid Shaper",
        recipes = {"Fluid Solidifier", "Tool Casting Machine"},
        energyUsage = 64 * UMV,
        width = 6, -- width equals the expansion width
    },
    ["cutter"] = {
        id = "Industrial Cutting Factory",
        recipes = {"Cutting Machine"},
        energyUsage = 16 * UMV,
    },
    ["fextractor"] = {
        id = "Large Fluid Extractor",
        recipes = {"Fluid Extractor"},
        energyUsage = 4 * UMV,
        solenoidTier = 12,
        coilTier = 14,
    },
    ["macerator"] = {
        id = "Industrial Maceration Stack",
        recipes = {"Macerator"},
        energyUsage = 4 * UMV,
        tier = 2,
    },
    ["bfurnace"] = {
        id = "Blast Furnace",
        recipes = {"Blast Furnace"},
        energyUsage = 1 * ULV,
        coilTier = 1,
    },
    ["mebf"] = {
        id = "Mega Blast Furnace",
        recipes = {"Blast Furnace"},
        energyUsage = 1 * ULV,
        coilTier = 1,
    },
    ["volcanus"] = {
        id = "Volcanus",
        recipes = {"Blast Furnace"},
        energyUsage = 4 * UMV,
        coilTier = 14,
    },
    ["freezer"] = {
        id = "Vacuum Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = 1 * ULV,
    },
    ["mvf"] = {
        id = "Mega Vacuum Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = 64 * UMV,
        tier = 0, -- tier equals perfect overclocks from subspace cooling
    },
    ["cryo"] = {
        id = "Cryogenic Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = 4 * UMV,
    },
    ["mixer"] = {
        id = "Industrial Mixing Machine",
        recipes = {"Multiblock Mixer"},
        energyUsage = 16 * UMV,
    },
    ["engraver"] = {
        id = "Hyper-Intensity Laser Engraver",
        recipes = {"Laser Engraver"},
        energyUsage = 16 * UMV,
        laserAmperage = 4194304, -- 4_194_304
    },
    ["centrifuge"] = {
        id = "Industrial Centrifuge",
        recipes = {"Multiblock Centrifuge"},
        energyUsage = 4 * UMV,
    },
    ["autoclave"] = {
        id = "Industrial Autoclave",
        recipes = {"Autoclave"},
        energyUsage = 16 * UMV,
        coilTier = 14,
        pipeCasingTier = 4,
        itemPipeCasingTier = 8,
    },
    ["mabs"] = {
        id = "Mega Alloy Blast Smelter",
        recipes = {"Alloy Blast Smelter"},
        energyUsage = 64 * UMV,
        coilTier = 14,
        glassTier = 10,
    },
    ["assembler"] = {
        id = "Precise Auto-Assembler MT-3662",
        recipes = {"Assembler"},
        energyUsage = 64 * UMV,
        tier = 5, -- Mk4 is tier 5
    },
    ["compressor"] = {
        id = "Large Electric Compressor",
        recipes = {"Compressor"},
        energyUsage = 4 * UMV,
    },
    ["blackhole"] = {
        id = "Pseudostable Black Hole Containment Field",
        recipes = {"Neutronium Compressor", "Compressor"},
        energyUsage = 64 * UMV,
        maximumOverclockTier = 12,
    },
    ["electrolyzer"] = {
        id = "Industrial Electrolyzer",
        recipes = {"Multiblock Electrolyzer"},
        energyUsage = 4 * UMV,
    },
    ["dehydrator"] = {
        id = "Utupu-Tanuri",
        recipes = {"Multiblock Dehydrator"},
        energyUsage = 4 * UEV,
    },
    ["chembath"] = {
        id = "Ore Washing Plant",
        recipes = {"Chemical Bath"},
        energyUsage = 16 * UMV,
    },
    ["megachem"] = {
        id = "Mega Chemical Reactor",
        recipes = {"Large Chemical Reactor"},
        energyUsage = 64 * UMV,
    },
    ["lathe"] = {
        id = "Industrial Precision Lathe",
        recipes = {"Lathe"},
        energyUsage = 4 * UEV,
        itemPipeCasingTier = 8,
    },
    ["alloysmelter"] = {
        id = "Zyngen",
        recipes = {"Alloy Smelter"},
        energyUsage = 4 * UEV,
        coilTier = 14,
    },
    ["arcfurnace"] = {
        id = "High Current Industrial Arc Furnace",
        recipes = {"Arc Furnace"},
        energyUsage = 4 * UEV,
        tier = 1,
        width = 7,
    },
    ["plasmafurnace"] = {
        id = "High Current Industrial Arc Furnace",
        recipes = {"Plasma Arc Furnace"},
        energyUsage = 4 * UEV,
        tier = 2,
        width = 7,
    },
    ["nactivator"] = {
        id = "Neutron Activator",
        recipes = {}, -- recipes not yet extracted
        height = 64,
    },
    ["implosion"] = {
        id = "Electric Implosion Compressor",
        recipes = {"Electric Implosion Compressor"},
        energyUsage = 64 * UMV,
        tier = 4,
    },
    ["sifter"] = {
        id = "Large Sifter Control Block",
        recipes = {"Sifter"},
        energyUsage = 16 * UMV,
    },
    ["cal"] = {
        id = "Circuit Assembly Line",
        recipes = {"Circuit Assembly Line", "Circuit Assembler"},
        energyUsage = 1 * UMV,
        tier = 12,
    },
    ["coal"] = {
        id = "Component Assembly Line",
        recipes = {"Component Assembly Line"},
        energyUsage = 65536 * UEV, -- 65_536
        tier = 12,
    },
    ["mdt"] = {
        id = "Mega Distillation Tower",
        recipes = {"Distillation Tower"},
        energyUsage = 64 * UMV,
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
        energyUsage = 4 * UEV,
        coilTier = 11,
    },
}

return machines
