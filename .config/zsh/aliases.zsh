#!/bin/sh

### ALIASES ###

## Backup
alias musbak="tar -cf ~/musicbak.tar.gz ~/Music"

# vim
alias v="vim"
alias vim="nvim"

## Edit some config files
alias xmonadhs="vim $HOME/.xmonad/xmonad.hs"
alias xmobarrc="vim $HOME/.config/xmobar/.xmobarrc"
alias alaconf="vim $HOME/.config/alacritty/alacritty.yml"
alias bashrc="vim $HOME/.bashrc"
alias nconf="vim $HOME/.config/nvim/."
alias zshrc="vim $HOME/.zshrc"

# Changing "ls" to "exa"
alias ls='exa -1a --icons --color=always --group-directories-first' # my preferred listing
alias lg='exa -la --icons --color=always --group-directories-first --no-permissions --no-filesize --no-time' # my preferred listing
alias la='exa -la --icons --color=always --group-directories-first'  # all files and dirs
alias ll='exa -l --icons --color=always --group-directories-first'  # long format
alias lt='exa -aT --icons --color=always --group-directories-first' # tree listing
alias l.='exa -a | grep -E "^\."'

# Faster Navigation
alias ...='cd ../..'
alias ....='cd ../../..'
alias .....='cd ../../../..'
alias ......='cd ../../../../..'
alias man='sudo man'
alias chown='sudo chown'
alias chmod='sudo chmod'

# pacman and paru
alias pac="sudo pacman --color=auto -S"
alias pacr="sudo pacman --color=auto -R"
alias pacq="sudo pacman --color=auto -Q"
alias pacsyu='sudo pacman --color=auto -Syyu'    # update only standard pkgs
alias parusua='paru -Sua --noconfirm'            # update only AUR pkgs (paru)
alias parusyu='paru -Syu --noconfirm'            # update standard pkgs and AUR pkgs (paru)
alias unlock='sudo rm /var/lib/pacman/db.lck'    # remove pacman lock
alias cleanup='sudo pacman -Rns (pacman -Qtdq)'  # remove orphaned packages

# Colorize grep output (good for log files)
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'

# confirm before overwriting something
alias cp="cp -ir"
alias mv='mv -i'
alias rm='rm -irf'

# git
alias ginit='git init --initial-branch=master'
alias origin='git remote add origin'
alias addup='git add'
alias addall='git add .'
alias branch='git branch'
alias checkout='git checkout'
alias clone='git clone'
alias commit='git commit -m'
alias fetch='git fetch'
alias pull='git pull origin'
alias push='git push -u origin'
alias stat='git status'
alias tag='git tag'
alias newtag='git tag -a'
alias gitrmc='git rm -r --cached'
alias gitrm='git rm -r'

# switch between shells
alias tobash="sudo chsh $USER -s /bin/bash && echo 'Now log out.'"
alias tozsh="sudo chsh $USER -s /bin/zsh && echo 'Now log out.'"
alias tofish="sudo chsh $USER -s /bin/fish && echo 'Now log out.'"

# music
alias ytaudio="yt-dlp --extract-audio --audio-format mp3 -P ~/Music"
alias vis="ncmpcpp -s visualizer"

# fing
alias fpc="fpc -Co -Cr -Miso -gl"
