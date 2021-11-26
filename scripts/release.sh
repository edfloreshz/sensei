cd ..
mkdir release
cargo build --release
mv target/release/sns release 
cp LICENSE release/LICENSE
cp README.md release/README.md
tar -czvf sensei-0.2.8-amd64.tar.gz release
mv sensei-0.2.8-amd64.tar.gz release
