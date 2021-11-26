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
makedepends=('cargo')
checkdepends=()
optdepends=()
provides=(sns)
conflicts=(sns)
replaces=()
backup=()
options=()
install=
changelog=
source=("https://github.com/edfloreshz/sensei/releases/download/v$pkgver/sensei-$pkgver-amd64.tar.gz")
noextract=()
md5sums=('SKIP')
validpgpkeys=()

build() {
	tar xvzf sensei-$pkgver-amd64.tar.gz 
}

package() {
	sudo mv sns ${pkgdir}
	install -Dm644 LICENSE ${pkgdir}/usr/share/licenses/${pkgname}/LICENSE
	install -Dm644 README.md ${pkgdir}/usr/share/doc/${pkgname}/README.md
}
