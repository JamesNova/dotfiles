#!/bin/sh

#      _ _   _
#     | | \ | |
#  _  | |  \| |  JamesNova (JN)
# | |_| | |\  |  gitlab.com/JamesNova
#  \___/|_| \_|
#

source ~/.session

command_exists() {
  command -v "$@" >/dev/null 2>&1
}

screen_resolution() {
  xrandr --output DP-0 --mode 1920x1080 --rate 144
}

start_pipewire() {
    killall -p pipewire pulseaudio wireplumber
    pipewire &
    pipewire-pulse &
    wireplumber &
}

start_pulseaudio() {
    pulseaudio --start
}

start_dunst() {
    dunst &
}

start_compositor() {
    picom -b &
}

run_unclutter() {
  if command_exists unclutter; then
    pidof unclutter || unclutter &
  fi
}

start_wallpaper() {
  bash ~/.fehbg
  #nitrogen --restore &
  # feh --randomize --bg-fill /usr/share/backgrounds/wallpapers-novaos/*.jpg /usr/share/backgrounds/wallpapers-novaos/*.jpeg /usr/share/backgrounds/wallpapers-novaos/*.png
  # feh --bg-fill '/usr/share/backgrounds/wallpapers-novaos/undefined - Imgur(20).jpg'
}

start_polybar() {
    ./~/.config/polybar/launch.sh
    nm-applet &
}

run_colemak() {
    setxkbmap us -variant colemak_dh
}

case $SESSION in
    awesome)
        screen_resolution
        start_compositor
        run_unclutter
        start_dunst
        start_wallpaper
    ;;
    dwm)
        screen_resolution
        start_pulseaudio
        # start_pipewire
        start_compositor
        run_unclutter
        start_wallpaper
    ;;
    xmonad)
        screen_resolution
        nm-applet &
    ;;
esac

exec $SESSION
