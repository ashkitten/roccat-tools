-- luacheck: globals libroccat

-- helpers

local function match_active_window_class(match_class)
    -- I'd like to be able to avoid shelling out to do this, but I can't seem to find
    -- a good way to bind to xlib or xcb in lua

    local handle = io.popen("xdotool getactivewindow")
    local result = handle:read("*l")
    handle:close()

    if result then
        local command = string.format("xprop -id %s WM_CLASS", result)
        handle = io.popen(command)
        result = handle:read("*l")
        handle:close()

        if result == nil then return false end
        local class = string.match(result, '"(.*)"$')

        if class == nil then return false end
        return string.match(class, match_class)
    end
    return false
end

-- init

local devices = { tyon = {} }

for _, device in ipairs(libroccat.find_devices()) do
    if device:name() == "tyon" then
        devices.tyon[#devices.tyon + 1] = {
            device = device,
        }
    end
end

-- event loop

while true do
    if match_active_window_class("Alacritty") then
        for _, tyon in pairs(devices.tyon) do
            tyon.device:set_profile(2)
        end
    else
        for _, tyon in pairs(devices.tyon) do
            tyon.device:set_profile(1)
        end
    end

    libroccat.sleep(500)
end
