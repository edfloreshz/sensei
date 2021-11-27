pkgver="0.2.8"
pkgname="sensei-amd64.tar.gz"
echo Building v$pkgver\n

cd ..
mkdir release
cargo build --release
mv target/release/sns release 
cp LICENSE release/LICENSE
cp README.md release/README.md
tar -czvf $pkgname release
mv $pkgname release
rm -rf release/{LICENSE,sns,README.md}