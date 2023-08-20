#      _ _   _
#     | | \ | |
#  _  | |  \| |  JamesNova (JN)
# | |_| | |\  |  gitlab.com/JamesNova
#  \___/|_| \_|
#

## Startx automaticaly after login in tty1
if [[ $(tty) = /dev/tty1 ]]; then
  startx
fi

### EXPORT
export TERM="xterm-256color"                      # getting proper colors
export HISTCONTROL=ignoredups:erasedups           # no duplicate entries
export ALTERNATE_EDITOR=""                        # setting for emacsclient
export EDITOR="nvim"                              # $EDITOR use Neovim in terminal
export VISUAL="nvim"                              # $VISUAL use Neovim in GUI mode

### "bat" as manpager
export MANPAGER="sh -c 'col -bx | bat -l man -p'"

### "vim" as manpager
# export MANPAGER='/bin/bash -c "vim -MRn -c \"set buftype=nofile showtabline=0 ft=man ts=8 nomod nolist norelativenumber nonu noma\" -c \"normal L\" -c \"nmap q :qa<CR>\"</dev/tty <(col -b)"'

### "nvim" as manpager
# export MANPAGER="nvim -c 'set ft=man' -"

### SET VI MODE ###
# Comment this line out to enable default emacs-like bindings
set -o vi
bind -m vi-command 'Control-l: clear-screen'
bind -m vi-insert 'Control-l: clear-screen'

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

### PROMPT
# This is commented out if using starship prompt
 # PS1='[\u@\h \W]\$ '

### PATH
if [ -d "$HOME/.bin" ] ;
  then PATH="$HOME/.bin:$PATH"
fi

if [ -d "$HOME/.local/bin" ] ;
  then PATH="$HOME/.local/bin:$PATH"
fi

if [ -d "$HOME/Applications" ] ;
  then PATH="$HOME/Applications:$PATH"
fi

### CHANGE TITLE OF TERMINALS
case ${TERM} in
  xterm*|rxvt*|Eterm*|aterm|kterm|gnome*|alacritty|st|konsole*)
    PROMPT_COMMAND='echo -ne "\033]0;${USER}@${HOSTNAME%%.*}:${PWD/#$HOME/\~}\007"'
        ;;
  screen*)
    PROMPT_COMMAND='echo -ne "\033_${USER}@${HOSTNAME%%.*}:${PWD/#$HOME/\~}\033\\"'
    ;;
esac

### SHOPT
shopt -s autocd # change to named directory
shopt -s cdspell # autocorrects cd misspellings
shopt -s cmdhist # save multi-line commands in history as single line
shopt -s dotglob
shopt -s histappend # do not overwrite history
shopt -s expand_aliases # expand aliases
shopt -s checkwinsize # checks term size when bash regains control

#ignore upper and lowercase when TAB completion
bind "set completion-ignore-case on"

### ARCHIVE EXTRACTION
# usage: ex <file>
ex ()
{
  if [ -f $1 ] ; then
    case $1 in
      *.tar.bz2)   tar xjf $1   ;;
      *.tar.gz)    tar xzf $1   ;;
      *.bz2)       bunzip2 $1   ;;
      *.rar)       unrar x $1   ;;
      *.gz)        gunzip $1    ;;
      *.tar)       tar xf $1    ;;
      *.tbz2)      tar xjf $1   ;;
      *.tgz)       tar xzf $1   ;;
      *.zip)       unzip $1     ;;
      *.Z)         uncompress $1;;
      *.7z)        7z x $1      ;;
      *.deb)       ar x $1      ;;
      *.tar.xz)    tar xf $1    ;;
      *.tar.zst)   unzstd $1    ;;
      *)           echo "'$1' cannot be extracted via ex()" ;;
    esac
  else
    echo "'$1' is not a valid file"
  fi
}


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
alias ..='cd ..'
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

### Run Pfetch on Start ###
pfetch

### SETTING THE STARSHIP PROMPT ###
# eval "$(starship init bash)"
