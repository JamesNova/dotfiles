#!/bin/bash

killall -q polybar
polybar powermenu 2>&1 | tee -a /tmp/polybar.log & disown
polybar nova1 2>&1 | tee -a /tmp/polybar.log & disown
polybar nova2 2>&1 | tee -a /tmp/polybar.log & disown
polybar nova3 2>&1 | tee -a /tmp/polybar.log & disown
