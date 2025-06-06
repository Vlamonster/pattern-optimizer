# Installation Steps
1. Prepare the Hardware:
   - Use a **Computer Case** that supports an **Internet Card**.
   - Set up an **Interface** with **Pattern Capacity Cards** containing the patterns to be optimized.
   - Place an **Adapter** with a **Database** next to the Interface.
   - Ensure everything is configured as shown in the image below.
2. Download the script:
   - `wget https://raw.githubusercontent.com/Vlamonster/pattern-optimizer/refs/heads/master/src/lua/optimize.lua`
3. Run the script once to download dependencies & config:
   - `optimize -h`
4. Configure machines.lua as needed.
5. Start Optimizing:
   - `optimize list` to show available machines
   - `optimize run [-q] [-b] [-s] [-r] [-t <ticks>] [-m <multiplier>] [-h] <machine>`
     - `-q, --quiet`      → Suppress all output except for errors.
     - `-b, --batch`      → Run for approximately 127 ticks, unless the recipe requires more time.
     - `-t, --ticks`      → Run for approximately <ticks> ticks, unless the recipe requires more time.
     - `-m, --multiplier` → Generate a pattern for <multiplier> times the base recipe without optimizing.
     - `-s, --skip`       → Skip patterning non-consumed items.
     - `-r, --restore`    → Restores missing outputs.
     - `-h, --help`       → Show help information.

# Example Setup
<img src="setup.png" width="600">
