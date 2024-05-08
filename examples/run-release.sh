echo "================================="
# cargo clean
cargo build --release
date -Ins
cargo run --release -- ./examples/media/tar-fiit-example.xml >./examples/output/!!!.txt
date -Ins
echo "================================="