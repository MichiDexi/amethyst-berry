# Setup
cargo clean
rm -rf build target
mkdir build

targets="
i586-unknown-linux-gnu
i686-unknown-linux-gnu
i686-pc-windows-gnu
i686-unknown-uefi
x86_64-unknown-linux-gnu
x86_64-pc-windows-gnu
x86_64-unknown-uefi
"

# Build
echo "$targets" | while IFS= read -r line; do
	cargo build --profile intended --target $line
done

# Move
mv "target/i586-unknown-linux-gnu/$2/$1" "build/$1-linux-32-old"
mv "target/i686-unknown-linux-gnu/$2/$1" "build/$1-linux-32"
mv "target/x86_64-unknown-linux-gnu/$2/$1" "build/$1-linux-64"

mv "target/i686-pc-windows-gnu/$2/$1.exe" "build/$1-windows-32.exe"
mv "target/x86_64-pc-windows-gnu/$2/$1.exe" "build/$1-windows-64.exe"

mv "target/i686-unknown-uefi/$2/$1.efi" "build/$1-uefi-32.efi"
mv "target/x86_64-unknown-uefi/$2/$1.efi" "build/$1-uefi-64.efi"

# Clean up
#rm -rf target
