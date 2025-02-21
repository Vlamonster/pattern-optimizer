local args = {...}

local computer = require("computer")
local component = require("component")
local internet = require("internet")

-- Computer components
local database = component.database
local interface = component.me_interface

local function ensure_file(filename, url)
  if not io.open(filename, "r") then
    print("Missing " .. filename)
    if not os.execute("wget " .. url .. " " .. filename) then
      print("Failed to install " .. filename)
      os.exit(1)
    end
  end
  return require(filename:gsub("%.lua$", ""))
end

local json = ensure_file("dkjson.lua", "https://raw.githubusercontent.com/Vlamonster/pattern-optimizer/refs/heads/master/src/resources/dkjson.lua")
local argparse = ensure_file("argparse.lua", "https://raw.githubusercontent.com/Vlamonster/pattern-optimizer/refs/heads/master/src/resources/argparse.lua")
local machines = ensure_file("machines.lua", "https://raw.githubusercontent.com/Vlamonster/pattern-optimizer/refs/heads/master/src/resources/machines.lua")

local function log(...)
  if not args.quiet then print(...) end
end

local function listMachines()
  print("Available machines:")
  for k, v in pairs(machines) do
    print(string.format("%-30s -> %s", k, v.id))
  end
  os.exit(0)
end

local function optimize()
    -- Set up connection to optimization server
    local host = "vlamonster.duckdns.org"
    local port = 3939
    local socket repeat socket = internet.open(host, port) until socket

    if not machines[args.machine] then
        print("Machine '" .. args.machine .. "' not found in machines.lua. Use `optimize list` to view available machines")
        os.exit(1)
    end

    for slot = 1, 36 do
      if interface.getInterfacePattern(slot) then
        log("PROCESSING SLOT " .. slot)
        local msg = {machine = machines[args.machine], ticks = tonumber(args.ticks), inputs = {}, outputs = {}}
        for i = 1, 9 do
          interface.storeInterfacePatternInput(slot, i, database.address, i)
          local input = database.get(i)
          if not input then break end
          table.insert(msg.inputs, input)
          database.clear(i)
        end
        for i = 1, 9 do
          interface.storeInterfacePatternOutput(slot, i, database.address, i)
          local output = database.get(i)
          if not output then break end
          table.insert(msg.outputs, output)
          database.clear(i)
        end
        socket:write(json.encode(msg))
        socket:flush()
        local response repeat response = socket:read() until response
        response = json.decode(response)
        if response.error then
          print(response.error)
        else
          log("INPUTS:")
          for i, v in ipairs(response.inputs) do
            log("id: " .. v.id .. ", amount: " .. tostring(v.amount) .. ", meta: " .. tostring(v.meta) .. ", nbt = '" .. v.nbt .. "'")
            database.set(i, v.id, v.meta, v.nbt)
            interface.setInterfacePatternInput(slot, database.address, i, v.amount, i)
            database.clear(i)
          end
          log("OUTPUTS:")
          for i, v in ipairs(response.outputs) do
            log("id: " .. v.id .. ", amount: " .. tostring(v.amount) .. ", meta: " .. tostring(v.meta) .. ", nbt = '" .. v.nbt .. "'")
            database.set(i + 9, v.id, v.meta, v.nbt)
            interface.setInterfacePatternOutput(slot, database.address, i + 9, v.amount, i)
            database.clear(i + 9)
          end
          log("--------------------")
        end
      end
    end
    computer.beep()
    socket:close()
    os.exit(0)
end

-- Argument parsing
local parser = argparse("optimize", "A tool for optimizing crafting patterns.")
local run = parser:command("run", "Optimize patterns for a specified machine.")
run:argument("machine", "The target machine to use for optimization.")
run:flag("-q --quiet", "Suppress all output except for errors.")
run:flag("-b --batch", "Run for approximately 127 ticks, unless the recipe requires more time.")
run:option("-t --ticks", "Run for approximately <ticks> ticks, unless the recipe requires more time.", "20")
parser:command("list", "Display a list of available machines."):action(listMachines)
args = parser:parse(args)

if args.machine then
    if args.batch then args.ticks = 127 end
    optimize()
end
