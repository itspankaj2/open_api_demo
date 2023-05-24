
clear

cargo fix --allow-dirty
cargo fmt

echo "Running cargo build";
echo "Running diesel migration";
diesel migration revert
diesel migration run



echo "Running cargo watch";
cargo watch --clear  -x check -x test -x run