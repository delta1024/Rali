pkgname=rali 
_pkgname=archInstaller
pkgver=0.1.r32.
pkgrel=1
pkgdesc="Rust Arch Linux Installer"
arch=(x86_64)
url="ssh://gituser@synas.local:99/volume1/git/archInstaller.git"
license=('GPL V3.0')
makedepends=('git' 'cargo' 'rust') # 'bzr', 'git', 'mercurial' or 'subversion'
provides=("${pkgname}")
conflicts=("${pkgname}")
source=("git+$url#branch=testing")
md5sums=('SKIP')

# Please refer to the 'USING VCS SOURCES' section of the PKGBUILD man page for
# a description of each element in the source array.

pkgver() {
	cd "${_pkgname}"
# Git, tags available
	printf "0.1.r%s.%s" "$(git rev-list --count HEAD)" # "$(git rev-parse --short HEAD)"  

}


build() {
	cd "${_pkgname}"
	cargo build --target-dir=target
	# RUSTUP_TOOLCHAIN=stable cargo build --release --locked --all-features --target-dir=target
}




package() {
	cd "${_pkgname}"
	install -Dm 755 target/debug/"${pkgname}" -t "${pkgdir}/usr/bin"
	install -Dm 644 man/rali.1 -t "${pkgdir}/usr/share/man/man1/"
	install -Dm 644 man/rali.conf.5 -t "${pkgdir}/usr/share/man/man5/"
}
