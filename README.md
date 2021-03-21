# EternalPatchManifestRust
Tool to patch DOOM Eternal's build manifest file for modding purposes, rewritten on Rust.

## Usage
To patch your build manifest, place the compiled binary in your "base" folder, then run it using:
```
./DEternal_patchManifest <AES Key>
```
on Linux or 
```
.\DEternal_patchManifest.exe <AES Key>
```
on Windows, where "AES Key" is the key used to encrypt/decrypt the build manifest file.

## Compiling
### Linux / macOS
To compile, you'll need a Rust environment set up with rustup. You can set it up by running:
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
and following the given instructions.

Afterwards, clone this repo:
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
The compiled binary will be located at the ./target/release folder.

### Windows
To compile, you'll need a Rust environment set up with rustup and the Visual Studio C++ build tools. You can set it up by downloading rustup from [here](https://www.rust-lang.org/tools/install) and follow the given instructions, then downloading Visual Studio 2019 and selecting the C++ tools for download.

NOTE: All the following commands are for PowerShell.

Afterwards, clone this repo using the Git Bash:
```
git clone https://github.com/PowerBall253/EternalPatchManifestRust.git
```
Then, set the following environment variable for maximum speed:
```
$RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3"
```

Finally, cd into the directory and compile with cargo:
```
cd EternalPatchManifestRust
cargo build --release
```
The compiled binary will be located at the .\target\release folder.

## Credits
* SutandoTsukai181 and Visual Studio: for creating the original DEternal_patchManifest Python script.
