#!/bin/env zsh
arch_repo=$HOME/NAS/archRepo/x86_64
makepkg -f
rm $arch_repo/rali-testing*.pkg.tar.zst
mv rali-*.pkg.tar.zst $arch_repo/
pushd $arch_repo/..
./update-repo.sh
popd
