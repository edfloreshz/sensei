cd ..
cargo build --release
cd target/release
tar -czvf release.tar.gz sns
mkdir ../../release
mv sensei-0.2.8-amd64.tar.gz ../../release
