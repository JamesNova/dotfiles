#!/bin/sh

pkgs=$(sudo find $HOME -iname novapkgs.txt)
aurpkgs=$(sudo find $HOME -iname aurpkgs.txt)

command_exists() {
  command -v "$@" >/dev/null 2>&1
}

if command_exists paru; then
    echo "Skipping Paru"
else
    echo "Installing Paru"
    sudo chown $USER /etc
    cd /etc
    git clone https://aur.archlinux.org/paru.git
    cd paru
    makepkg -si
fi

echo "Installing Packages"
sudo pacman -Syyu --noconfirm
sudo pacman -S --needed --noconfirm - < $pkgs
paru -S --needed - < $aurpkgs
