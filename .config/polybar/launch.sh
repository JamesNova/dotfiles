#!/bin/bash

killall -q polybar
polybar powermenu 2>&1 | tee -a /tmp/polybar.log & disown
polybar left 2>&1 | tee -a /tmp/polybar.log & disown
polybar middle 2>&1 | tee -a /tmp/polybar.log & disown
polybar right 2>&1 | tee -a /tmp/polybar.log & disown
