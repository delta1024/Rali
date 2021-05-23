#!/bin/env zsh
arch_repo=$HOME/NAS/archRepo/x86_64
makepkg -f
mv rali-*.pkg.tar.zst $arch_repo/
pushd $arch_repo/..
./update-repo.sh
popd
