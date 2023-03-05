echo "== TESTING -f test.json ==" && \
cargo run -- -f test.json && \
echo "== TESTING -c test.toml ==" && \
cargo run -- -c test.toml && \
echo "== TESTING -c . ==" && \
cargo run -- -c . && \
echo ""
echo "== ======= =="
echo "== SUCCESS =="
echo "== ======= =="
