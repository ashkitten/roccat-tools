-- helpers

function match_active_window_class(match_class)
    -- I'd like to be able to avoid shelling out to do this, but I can't seem to find
    -- a good way to bind to xlib or xcb in lua

    local handle = io.popen("xprop -id `xdotool getactivewindow` WM_CLASS")
    local result = handle:read("*a")
    handle:close()

    local instance, class = string.match(result, '"(.*)", "(.*)"')

    if class == nil then return false end
    return string.match(class, match_class)
end

-- init

devices = { tyon = {} }

for i, device in ipairs(libroccat.find_devices()) do
    if device:name() == "tyon" then
        devices.tyon[#devices.tyon + 1] = {
            device = device,
        }
    end
end

-- event loop

while true do
    if match_active_window_class("Alacritty") then
        for i, tyon in pairs(devices.tyon) do
            tyon.device:set_profile(2)
        end
    else
        for i, tyon in pairs(devices.tyon) do
            tyon.device:set_profile(1)
        end
    end
end