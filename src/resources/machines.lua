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

local machines = {
    ["bender"] = {
        id = "Industrial Material Press",
        recipes = {"Bending Machine"},
        energyUsage = 4 * UMV
    },
    ["extruder"] = {
        id = "Industrial Extrusion Machine",
        recipes = {"Extruder"},
        energyUsage = 4 * UMV
    },
    ["wiremill"] = {
        id = "Industrial Wire Factory",
        recipes = {"Wiremill"},
        energyUsage = 4 * UMV
    },
    ["hammer"] = {
        id = "Industrial Sledgehammer",
        recipes = {"Forge Hammer"},
        energyUsage = 4 * UMV,
        tier = 4
    },
    ["extractor"] = {
        id = "Dissection Apparatus",
        recipes = {"Extractor"},
        energyUsage = 4 * UMV
    },
    ["solidifier"] = {
        id = "Fluid Shaper",
        recipes = {"Fluid Solidifier"},
        energyUsage = 4 * UMV,
        width = 6 -- width equals the expansion width
    },
    ["cutter"] = {
        id = "Industrial Cutting Factory",
        recipes = {"Cutting Machine"},
        energyUsage = 4 * UMV
    },
    ["fextractor"] = {
        id = "Large Fluid Extractor",
        recipes = {"Fluid Extractor"},
        energyUsage = 4 * UMV,
        solenoidTier = 12,
        coilTier = 14
    },
    ["macerator"] = {
        id = "Industrial Maceration Stack",
        recipes = {"Macerator"},
        energyUsage = 4 * UMV,
        tier = 2
    },
    ["bfurnace"] = {
        id = "Blast Furnace",
        recipes = {"Blast Furnace"},
        energyUsage = 1 * ULV,
        coilTier = 1
    },
    ["mebf"] = {
        id = "Mega Blast Furnace",
        recipes = {"Blast Furnace"},
        energyUsage = 1 * ULV,
        coilTier = 1
    },
    ["volcanus"] = {
        id = "Volcanus",
        recipes = {"Blast Furnace"},
        energyUsage = 4 * UMV,
        coilTier = 14
    },
    ["freezer"] = {
        id = "Vacuum Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = 1 * ULV
    },
    ["mvf"] = {
        id = "Mega Vacuum Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = 64 * UMV,
        tier = 0 -- tier equals perfect overclocks from subspace cooling
    },
    ["cryo"] = {
        id = "Cryogenic Freezer",
        recipes = {"Vacuum Freezer"},
        energyUsage = 4 * UMV
    },
    ["mixer"] = {
        id = "Industrial Mixing Machine",
        recipes = {"Multiblock Mixer"},
        energyUsage = 4 * UMV
    },
    ["engraver"] = {
        id = "Hyper-Intensity Laser Engraver",
        recipes = {"Laser Engraver"},
        energyUsage = 4 * UMV,
        laserAmperage = 4194304 -- 4_194_304
    },
    ["centrifuge"] = {
        id = "Industrial Centrifuge",
        recipes = {"Multiblock Centrifuge"},
        energyUsage = 4 * UMV
    },
    ["autoclave"] = {
        id = "Industrial Autoclave",
        recipes = {"Autoclave"},
        energyUsage = 4 * UMV,
        coilTier = 14,
        pipeCasingTier = 4,
        itemPipeCasingTier = 8
    },
    ["mabs"] = {
        id = "Mega Alloy Blast Smelter",
        recipes = {"Alloy Blast Smelter"},
        energyUsage = 64 * UMV,
        coilTier = 14,
        glassTier = 10
    },
    ["assembler"] = {
        id = "Precise Auto-Assembler MT-3662",
        recipes = {"Assembler"},
        energyUsage = 64 * UMV,
        tier = 5 -- Mk4 is tier 5
    },
    ["blackhole"] = {
        id = "Pseudostable Black Hole Containment Field",
        recipes = {"Neutronium Compressor", "Compressor"},
        energyUsage = 64 * UMV,
        maximumOverclockTier = 12
    },
    ["electrolyzer"] = {
        id = "Industrial Electrolyzer",
        recipes = {"Multiblock Electrolyzer"},
        energyUsage = 4 * UMV
    },
    ["dehydrator"] = {
        id = "Utupu-Tanuri",
        recipes = {"Multiblock Dehydrator"},
        energyUsage = 4 * UEV
    },
    ["implosion"] = {
        id = "Electric Implosion Compressor",
        recipes = {"Implosion Compressor"},
        energyUsage = 64 * UMV,
        tier = 4
    },
    ["chembath"] = {
        id = "Ore Washing Plant",
        recipes = {"Chemical Bath"},
        energyUsage = 4 * UMV
    },
    ["megachem"] = {
        id = "Mega Chemical Reactor",
        recipes = {"Large Chemical Reactor"},
        energyUsage = 64 * UMV
    },
    ["lathe"] = {
        id = "Industrial Precision Lathe",
        recipes = {"Lathe"},
        energyUsage = 4 * UEV,
        itemPipeCasingTier = 8
    },
    ["alloysmelter"] = {
        id = "Zyngen",
        recipes = {"Alloy Smelter"},
        energyUsage = 4 * UEV,
        coilTier = 14
    },
    ["arcfurnace"] = {
        id = "High Current Industrial Arc Furnace",
        recipes = {"Arc Furnace"},
        energyUsage = 4 * UEV,
        tier = 1,
        width = 7
    },
    ["plasmafurnace"] = {
        id = "High Current Industrial Arc Furnace",
        recipes = {"Plasma Arc Furnace"},
        energyUsage = 4 * UEV,
        tier = 2,
        width = 7
    }
    --["godforge"] = { id = "God Forge", energyUsage = 2 * G },
    --["compressor"] = { id = "Large Electric Compressor", recipes = { "Compressor" }, energyUsage = 1 * ULV },
    --["hipcompressor"] = { id = "Hot Isostatic Pressurization Unit", recipes = { "Compressor" }, energyUsage = 1 * ULV },
    --["neutroniumcompressor"] = { id = "Neutronium Compressor", recipes = { "Neutronium Compressor" }, energyUsage = 1 * ULV },
}


return machines
