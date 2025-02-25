local args = {...}

-- Optimization server parameters
local HOST = "vlamonster.duckdns.org"
local PORT = 3939
local RETRY_DELAY = 5
local TIMEOUT = 30

-- Interface and pattern parameters
local MAX_SLOT = 36
local MAX_ITEMS = 9

-- Terminal colors
local RED = "\27[0;31m"
local YELLOW = "\27[0;33m"
local CYAN = "\27[0;36m"
local WHITE = "\27[0;97m"

-- Required OpenComputer libraries
local computer = require("computer")
local component = require("component")
local internet = require("internet")

-- Computer components
local database = component.database
local interface = component.me_interface

local function downloadFile(filename, url)
    if not os.execute("wget " .. url .. " " .. filename) then
        print("Failed to install " .. filename)
        os.exit(1)
    end
end

local function ensureFile(filename, url)
  local file = io.open(filename, "r")
  if file then
    file:close()
  else
    print("Missing " .. filename)
    download_file(filename, url)
  end
  return require(filename:gsub("%.lua$", ""))
end

-- Ensure external dependencies are loaded
local json = ensureFile("dkjson.lua", "https://raw.githubusercontent.com/Vlamonster/pattern-optimizer/refs/heads/master/src/resources/dkjson.lua")
local argparse = ensureFile("argparse.lua", "https://raw.githubusercontent.com/Vlamonster/pattern-optimizer/refs/heads/master/src/resources/argparse.lua")
package.loaded.machines = nil -- Do not use cached version since user likely changes settings frequently
local machines = ensureFile("machines.lua", "https://raw.githubusercontent.com/Vlamonster/pattern-optimizer/refs/heads/master/src/resources/machines.lua")

local function log(...)
  if not args.quiet then print(...) end
end

local function formatIdentifier(id, meta)
    return string.format("%s:%d", id, meta)
end

local function formatNumber(n)
    return tostring(n):reverse():gsub("(%d%d%d)", "%1,"):reverse():gsub("^,", "")
end

local function formatNBT(nbt)
    return nbt ~= "" and ("(NBT: " .. nbt .. ")") or ""
end

local function listMachines()
  print("Available machines:")
  for k, v in pairs(machines) do
    print(string.format("%-30s -> %s", k, v.id))
  end
  os.exit(0)
end

local function openSocket()
    local socket
    repeat
        socket = internet.open(HOST, PORT)
        if not socket then
            print("Connection failed. Retrying in " .. RETRY_DELAY .. " seconds...")
            os.sleep(RETRY_DELAY)
        end
    until socket
    socket:setTimeout(TIMEOUT)
    socket.stream.socket.finishConnect()
    return socket
end

local function prepareMessage(slot)
    local message = { machine = machines[args.machine], ticks = tonumber(args.ticks), inputs = {}, outputs = {} }
    for i = 1, MAX_ITEMS do
        interface.storeInterfacePatternInput(slot, i, database.address, i)
        local input = database.get(i)
        if not input then break end
        table.insert(message.inputs, input)
        database.clear(i)
    end
    for i = 1, MAX_ITEMS do
        interface.storeInterfacePatternOutput(slot, i, database.address, i)
        local output = database.get(i)
        if not output then break end
        table.insert(message.outputs, output)
        database.clear(i)
    end
    return message
end

local function sendAndGetResponse(socket, message)
    local encodedMessage = json.encode(message)
    socket:write(encodedMessage)
    socket:flush()
    local response = socket:read()
    while not response do
        print("Connection failed. Retrying in " .. tostring(RETRY_DELAY) .. " seconds...")
        os.sleep(RETRY_DELAY)
        socket:close()
        socket = openSocket()
        socket:write(encodedMessage)
        socket:flush()
        response = socket:read()
    end
    return json.decode(response)
end

local function handleItems(type, items, slot)
    log(string.format("  %s>>%s %s:", YELLOW, WHITE, type:upper()))
    local setFunction = type == "inputs" and interface.setInterfacePatternInput or interface.setInterfacePatternOutput
    for i, v in ipairs(items) do
        local identifier = formatIdentifier(v.id, v.meta)
        local amount = formatNumber(v.amount)
        local nbt = formatNBT(v.nbt)
        log(string.format("    %s-%s %-40s %sx%s %-15s%s", CYAN, WHITE, identifier, CYAN, WHITE, amount, nbt))
        database.set(i, v.id, v.meta, v.nbt)
        setFunction(slot, database.address, i, v.amount, i)
        database.clear(i)
    end
end

local function optimize()
    -- Set up connection to optimization server
    local socket = openSocket()
    if not machines[args.machine] then
        print("Machine '" .. args.machine .. "' not found in machines.lua. Use `optimize list` to view available machines")
        os.exit(1)
    end
    for slot = 1, MAX_SLOT do
        if interface.getInterfacePattern(slot) then
            log(string.format("%s[%s SLOT %d %s]%s", RED, WHITE, slot, RED, WHITE));
            local message = prepareMessage(slot)
            local response = sendAndGetResponse(socket, message)
            if response.error then
                print(response.error)
            else
                handleItems("inputs", response.inputs, slot)
                handleItems("outputs", response.outputs, slot)
                log(string.format("  %s>>%s DURATION: ", YELLOW, WHITE))
                log(string.format("    %s-%s %.2fs / %dt", CYAN, WHITE, response.duration / 20, response.duration))
                log("--------------------------------------------------------")
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
