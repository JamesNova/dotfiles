---------------------------
-- Default awesome theme --
---------------------------

local xresources = require("beautiful.xresources")
local dpi = xresources.apply_dpi

local theme = {}

theme.font          = "sans 8"

theme.bg_normal     = "#222222"
theme.bg_focus      = "#FFD866"
theme.bg_urgent     = "#ff0000"
theme.bg_minimize   = "#444444"
theme.bg_systray    = theme.bg_normal

theme.fg_normal     = "#aaaaaa"
theme.fg_focus      = "#000000"
theme.fg_urgent     = "#ffffff"
theme.fg_minimize   = "#ffffff"

theme.useless_gap   = dpi(0)
theme.border_width  = dpi(1)
theme.border_normal = "#000000"
theme.border_focus  = "#FFD866"
theme.border_marked = "#91231c"

theme.icon_theme = nil

return theme
