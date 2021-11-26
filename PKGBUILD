# Maintainer: Eduardo Flores <edfloreshz@gmail.com>

pkgname=sensei
pkgver=0.2.8
pkgrel=1
epoch=
pkgdesc="Sensei is a simple command line tool to open documentation for any crate."
arch=(x86_64)
url="https://sensei.edfloreshz.dev"
license=('GPL')
groups=()
depends=()
makedepends=()
checkdepends=()
optdepends=()
provides=(sns)
conflicts=(sensei sns)
replaces=()
backup=()
options=()
install=
changelog=
source=("https://github.com/edfloreshz/sensei/releases/download/v$pkgver/sensei-amd64.tar.gz")
noextract=()
md5sums=('SKIP')
validpgpkeys=()

build() {
	mkdir -p pkg/sensei
	mv sensei-amd64.tar.gz pkg/sensei
	cd pkg/sensei
	tar -xzvf sensei-amd64.tar.gz
	cd release
}

package() {
	sudo mv pkg/sensei/sns ${pkgdir}
	install -Dm644 LICENSE ${pkgdir}/usr/share/licenses/${pkgname}/LICENSE
	install -Dm644 README.md ${pkgdir}/usr/share/doc/${pkgname}/README.md
}
