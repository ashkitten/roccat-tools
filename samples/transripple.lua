-- Shamelessly adapted from the original roccat-tools ripple script

-- config

local BLUE = { state = true, red = 0x5b, green = 0xce, blue = 0xfa }
local PINK = { state = true, red = 0xf5, green = 0xa9, blue = 0xb8 }
local WHITE = { state = true, red = 0xff, green = 0xff, blue = 0xff }

KEY_COLORS = {
    [0]=BLUE, [1]=BLUE, [2]=BLUE, [3]=BLUE, [4]=BLUE, [5]=BLUE, [6]=BLUE,
    [7]=BLUE, [8]=BLUE, [9]=BLUE, [10]=BLUE, [11]=BLUE, [12]=BLUE, [13]=BLUE,
    [14]=BLUE, [15]=BLUE, [16]=PINK, [17]=PINK, [18]=PINK, [19]=PINK,
    [20]=PINK, [21]=PINK, [22]=PINK, [23]=PINK, [24]=PINK, [25]=PINK,
    [26]=PINK, [27]=PINK, [28]=PINK, [29]=PINK, [30]=PINK, [31]=PINK,
    [32]=PINK, [33]=PINK, [34]=PINK, [35]=PINK, [36]=PINK, [37]=PINK,
    [38]=WHITE, [39]=WHITE, [40]=WHITE, [41]=WHITE, [42]=WHITE, [43]=WHITE,
    [44]=WHITE, [45]=WHITE, [46]=WHITE, [47]=WHITE, [48]=WHITE, [49]=WHITE,
    [50]=WHITE, [51]=WHITE, [52]=WHITE, [53]=WHITE, [54]=WHITE, [55]=WHITE,
    [56]=WHITE, [57]=WHITE, [58]=WHITE, [59]=WHITE, [60]=WHITE, [61]=WHITE,
    [62]=WHITE, [63]=WHITE, [64]=WHITE, [65]=WHITE, [66]=WHITE, [67]=WHITE,
    [68]=WHITE, [69]=WHITE, [70]=WHITE, [71]=WHITE, [72]=WHITE, [73]=WHITE,
    [74]=WHITE, [75]=WHITE, [76]=WHITE, [77]=PINK, [78]=PINK, [79]=PINK,
    [80]=PINK, [81]=PINK, [82]=PINK, [83]=PINK, [84]=PINK, [85]=PINK,
    [86]=PINK, [87]=PINK, [88]=PINK, [89]=PINK, [90]=PINK, [91]=PINK,
    [92]=PINK, [93]=PINK, [94]=PINK, [95]=BLUE, [96]=BLUE, [97]=BLUE,
    [98]=BLUE, [99]=BLUE, [100]=BLUE, [101]=BLUE, [102]=BLUE, [103]=BLUE,
    [104]=BLUE, [105]=BLUE, [106]=BLUE, [107]=BLUE, [108]=BLUE, [109]=BLUE,
}

RIPPLE_COLORS = {
    { state = true, red = 0xff, green = 0x00, blue = 0x00 },
    { state = true, red = 0x00, green = 0x80, blue = 0x00 },
    { state = true, red = 0xff, green = 0xff, blue = 0x00 },
    { state = true, red = 0x00, green = 0x80, blue = 0x00 },
    { state = true, red = 0x00, green = 0x00, blue = 0xff },
    { state = true, red = 0xa0, green = 0x00, blue = 0xc0 },
}

SLOWDOWN = 20

-- helpers

MATRIX_ROWS = 25
MATRIX_COLS = 95
KEY_WIDTH = 4

KEYS = {
    [0]={x= 7, y= 2}, [1]={x=15, y= 2}, [2]={x=19, y= 2}, [3]={x=23, y= 2}, [4]={x=27, y= 2},
    [5]={x=33, y= 2}, [6]={x=37, y= 2}, [7]={x=41, y= 2}, [8]={x=45, y= 2}, [9]={x=51, y= 2},
    [10]={x=55, y= 2}, [11]={x=59, y= 2}, [12]={x=63, y= 2}, [13]={x=68, y= 2}, [14]={x=72, y= 2},
    [15]={x=76, y= 2}, [16]={x= 2, y= 7}, [17]={x= 7, y= 7}, [18]={x=11, y= 7}, [19]={x=15, y= 7},
    [20]={x=19, y= 7}, [21]={x=23, y= 7}, [22]={x=27, y= 7}, [23]={x=31, y= 7}, [24]={x=35, y= 7},
    [25]={x=39, y= 7}, [26]={x=43, y= 7}, [27]={x=47, y= 7}, [28]={x=51, y= 7}, [29]={x=55, y= 7},
    [30]={x=63, y= 7}, [31]={x=68, y= 7}, [32]={x=72, y= 7}, [33]={x=76, y= 7}, [34]={x=81, y= 7},
    [35]={x=85, y= 7}, [36]={x=89, y= 7}, [37]={x=93, y= 7}, [38]={x= 2, y=11}, [39]={x= 8, y=11},
    [40]={x=13, y=11}, [41]={x=17, y=11}, [42]={x=21, y=11}, [43]={x=25, y=11}, [44]={x=29, y=11},
    [45]={x=33, y=11}, [46]={x=37, y=11}, [47]={x=41, y=11}, [48]={x=45, y=11}, [49]={x=49, y=11},
    [50]={x=53, y=11}, [51]={x=57, y=11}, [52]={x=62, y=15}, [53]={x=68, y=11}, [54]={x=72, y=11},
    [55]={x=76, y=11}, [56]={x=81, y=11}, [57]={x=85, y=11}, [58]={x=89, y=11}, [59]={x=93, y=13},
    [60]={x= 2, y=15}, [61]={x= 9, y=15}, [62]={x=14, y=15}, [63]={x=18, y=15}, [64]={x=22, y=15},
    [65]={x=26, y=15}, [66]={x=30, y=15}, [67]={x=34, y=15}, [68]={x=38, y=15}, [69]={x=42, y=15},
    [70]={x=46, y=15}, [71]={x=50, y=15}, [72]={x=54, y=15}, [73]={x=58, y=15}, [74]={x=81, y=15},
    [75]={x=85, y=15}, [76]={x=89, y=15}, [77]={x= 2, y=19}, [78]={x= 8, y=19}, [79]={x=12, y=19},
    [80]={x=16, y=19}, [81]={x=20, y=19}, [82]={x=24, y=19}, [83]={x=28, y=19}, [84]={x=32, y=19},
    [85]={x=36, y=19}, [86]={x=40, y=19}, [87]={x=44, y=19}, [88]={x=48, y=19}, [89]={x=52, y=19},
    [90]={x=61, y=19}, [91]={x=72, y=19}, [92]={x=81, y=19}, [93]={x=85, y=19}, [94]={x=89, y=19},
    [95]={x=93, y=21}, [96]={x= 2, y=23}, [97]={x= 8, y=23}, [98]={x=13, y=23}, [99]={x=18, y=23},
    [100]={x=31, y=23}, [101]={x=49, y=23}, [102]={x=53, y=23}, [103]={x=57, y=23}, [104]={x=62, y=23},
    [105]={x=68, y=23}, [106]={x=72, y=23}, [107]={x=76, y=23}, [108]={x=83, y=23}, [109]={x=89, y=23},
}

local function init_light_position_matrix()
    local light_position_matrix = {}

    for sdk, position in pairs(KEYS) do
        local base = position.x - 2 + (position.y - 2) * MATRIX_COLS

        light_position_matrix[base + 1] = sdk
        light_position_matrix[base + 2] = sdk

        base = base + MATRIX_COLS

        light_position_matrix[base    ] = sdk
        light_position_matrix[base + 1] = sdk
        light_position_matrix[base + 2] = sdk
        light_position_matrix[base + 3] = sdk

        base = base + MATRIX_COLS

        light_position_matrix[base    ] = sdk
        light_position_matrix[base + 1] = sdk
        light_position_matrix[base + 2] = sdk
        light_position_matrix[base + 3] = sdk

        base = base + MATRIX_COLS

        light_position_matrix[base + 1] = sdk
        light_position_matrix[base + 2] = sdk
    end

    return light_position_matrix
end

-- Sets corresponding key in array if any.
local function set_led(leds, x, y)
    if y >= MATRIX_ROWS or y < 0 or x >= MATRIX_COLS or x < 0 then
        return
    end

    sdk = light_position_matrix[x + y * MATRIX_COLS]
    if sdk then
        table.insert(leds, sdk)
    end
end

-- QUOTE Midpoint circle algorithm inspired by Perone's programming pad
-- http://www.willperone.net/Code/codecircle.php
-- Copyright (c) Will Perone
--
-- Returns set of sdk key index to set/clear.
local function draw_circle(center, radius)
    local radius = radius * KEY_WIDTH
    local leds = {}

    if radius < 1 then
        return leds
    end

    local x = 0
    local y = radius
    local d = 1 - radius
    local delta_e = 3
    local delta_se = 5 - radius * 2

    set_led(leds, center.x,     center.y - y)
    set_led(leds, center.x,     center.y + y)
    set_led(leds, center.x - y, center.y    )
    set_led(leds, center.x + y, center.y    )

    while y > x do
        if d < 0 then
            d = d + delta_e
            delta_se = delta_se + 2
        else
            d = d + delta_se
            delta_se = delta_se + 4
            y = y - 1
        end
        delta_e = delta_e + 2
        x = x + 1

        set_led(leds, center.x - x, center.y - y)
        set_led(leds, center.x - y, center.y - x)
        set_led(leds, center.x + y, center.y - x)
        set_led(leds, center.x + x, center.y - y)
        set_led(leds, center.x - x, center.y + y)
        set_led(leds, center.x - y, center.y + x)
        set_led(leds, center.x + y, center.y + x)
        set_led(leds, center.x + x, center.y + y)
    end

    return leds
end

function table.shallow_copy(t)
    local t2 = {}
    for k, v in pairs(t) do
        t2[k] = v
    end
    return t2
end

function ryosmkfx_init_lights(ryosmkfx)
    ryosmkfx.profile = ryosmkfx.device:get_profile()
    if ryosmkfx.profile == 1 then
        ryosmkfx.device:set_custom_lights_active(true)
        ryosmkfx.device:set_custom_lights(ryosmkfx.leds)
    else
        ryosmkfx.device:set_custom_lights_active(false)
    end
end

-- init

devices = { ryosmkfx = {} }
light_position_matrix = init_light_position_matrix()

for i, device in ipairs(libroccat.find_devices()) do
    if device:name() == "ryos_mk_fx" then
        devices.ryosmkfx[#devices.ryosmkfx + 1] = {
            device = device,
            profile = device:get_profile(),
            ripples = {},
            leds = table.shallow_copy(KEY_COLORS),
        }

        ryosmkfx_init_lights(devices.ryosmkfx[#devices.ryosmkfx])
    end
end

-- event loop

while true do
    for i, ryosmkfx in pairs(devices.ryosmkfx) do
        local event = ryosmkfx.device:get_event_timed(SLOWDOWN)

        if event then
            if event.type == "profile_start" then
                ryosmkfx_init_lights(ryosmkfx)
            end
        end

        if ryosmkfx.profile == 1 then
            if event then
                if event.type == "effect" then
                    if event.action == "press" then
                        ryosmkfx.ripples[#ryosmkfx.ripples + 1] = {
                            center = KEYS[event.data],
                            radius = 1
                        }
                    end
                end
            end

            for i = #ryosmkfx.ripples, 1, -1 do
                if ryosmkfx.ripples[i].radius <= MATRIX_COLS / KEY_WIDTH + #RIPPLE_COLORS then
                    ryosmkfx.ripples[i].radius = ryosmkfx.ripples[i].radius + 1
                else
                    table.remove(ryosmkfx.ripples, i)
                end
            end

            if #ryosmkfx.ripples > 0 then
                ryosmkfx.leds = table.shallow_copy(KEY_COLORS)

                for _, ripple in pairs(ryosmkfx.ripples) do
                    for i, color in ipairs(RIPPLE_COLORS) do
                        if i <= ripple.radius then
                            leds = draw_circle(ripple.center, ripple.radius - i)
                            for _, led in pairs(leds) do
                                ryosmkfx.leds[led] = color
                            end
                        end
                    end
                end

                ryosmkfx.device:set_custom_lights(ryosmkfx.leds)
            end
        end
    end
end
