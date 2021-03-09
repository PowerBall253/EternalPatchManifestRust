# EternalPatchManifestRust
Tool to patch DOOM Eternal's build manifest file for modding purposes, rewritten on Rust.

## Usage
To patch your build manifest, place the compiled binary in your "base" folder, then run:
```
./DEternal_patchManifest <AES Key>
```
where "AES KEY" is the key used to encrypt/decrypt the build manifest file.

## Compiling (Linux)
To compile, you'll need a Rust environment set up with cargo.

First, clone this repo:
```
git clone https://github.com/PowerBall253/EternalPatchManifestRust.git
```
Then, set the following environment variable for maximum speed:
```
export RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3"
```

Finally, cd into the directory and compile with cargo:
```
cd EternalPatchManifestRust
cargo build --release
```
The compiled binary will be located at the target/release folder.

## Credits
SutandoTsukai181 and Visual Studio: for creating the original DEternal_patchManifest Python script.
