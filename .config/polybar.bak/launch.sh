#!/bin/bash

killall -q polybar
polybar nova 2>&1 | tee -a /tmp/polybar.log & disown
