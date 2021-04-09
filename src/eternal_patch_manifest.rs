/*
* EternalPatchManifestRust
* Copyright (C) 2021 PowerBall253
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(non_snake_case)]

extern crate hex;
extern crate getrandom;
extern crate serde;
extern crate serde_jsonrc;
#[macro_use]
extern crate serde_derive;

use aes_gcm::aead::{AeadInPlace, NewAead, generic_array::GenericArray};
use aes_gcm::Aes128Gcm;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use getrandom::getrandom;
use std::path::Path;

fn decrypt_bm(enc_data: Vec<u8>, key_hex: &String) -> String {
    let size = enc_data.len() as usize;

    let nonce_bytes = enc_data[0..0xC].to_vec();
    let data = enc_data[0xC..(size - 0x50)].to_vec();
    let tag_bytes = enc_data[(size - 0x50)..(size - 0x40)].to_vec();

    let key_bytes = hex::decode(key_hex).unwrap();

    let key = GenericArray::from_slice(&key_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let tag = GenericArray::from_slice(&tag_bytes);

    let cipher = Aes128Gcm::new(key);
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&data);
    cipher.decrypt_in_place_detached(nonce, b"build-manifest", &mut buffer, tag).expect("Failed to decrypt the build manifest data.");
    return String::from_utf8_lossy(&buffer).to_string();
}

fn encrypt_bm(bm_dec: String, key_hex: &String) -> Vec<u8> {
    let mut nonce_bytes = vec![0u8; 0xC];
    getrandom(&mut nonce_bytes).expect("ERROR: Failed to get random string for nonce.");

    let key_bytes = hex::decode(key_hex).unwrap();

    let key = GenericArray::from_slice(&key_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);

    let mut buffer = Vec::new();
    buffer.extend_from_slice(bm_dec.as_bytes());

    let cipher = Aes128Gcm::new(key);
    let tag = cipher.encrypt_in_place_detached(nonce, b"build-manifest", &mut buffer).expect("Failed to encrypt the new build manifest data.");
    buffer.append(&mut tag.to_vec());
    let mut empty_byte_array = vec![0; 0x40];
    buffer.append(&mut empty_byte_array);
    nonce_bytes.append(&mut buffer);
    return nonce_bytes;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FileComponents {
    file_size: i64,
    hashes: Vec<String>,
    chunk_size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BuildManifestJson {
    hash: String,
    files: HashMap<String, FileComponents>,
}

fn optimize_json(dec_data: String) -> String {
    let mut dec_json: BuildManifestJson = serde_jsonrc::from_str(&dec_data).expect("JSON was not well formatted - corrupted file?");

    for (file_path, file_components) in dec_json.files.iter_mut() {
        if Path::new(file_path).exists() {
            file_components.file_size = std::fs::metadata(file_path).unwrap().len() as i64;
            println!("Found file {}, fileSize updated to: {}", file_path, file_components.file_size);
        }
        file_components.hashes = vec!["e2df1b2aa831724ec987300f0790f04ad3f5beb8".to_string()];
        if file_components.file_size > 4294967295 {
            let num_hashes = (file_components.file_size / 4294967295) + ((file_components.file_size % 4294967295 > 0) as i64);
            for _i in 0..(num_hashes - 1) {
                file_components.hashes.push("e2df1b2aa831724ec987300f0790f04ad3f5beb8".to_string());
            }
        }
        file_components.chunk_size = 4294967295;
    }

    return serde_jsonrc::to_string(&dec_json).expect("Failed to convert JSON file to string.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("DEternal_patchManifest Rust by PowerBall253, based on Python script by Visual Studio and SutandoTsukai181.");
        println!("\nUsage: ");
        println!("{} <AES Key>", &args[0]);
        std::process::exit(1);
    }
    let key_hex = &args[1];

    let mut build_manifest = File::open("build-manifest.bin").expect("Failed to open build-manifest.bin for reading."); 
    let mut data_enc: Vec<u8> = Vec::new();
    build_manifest.read_to_end(&mut data_enc).expect("Failed to read build-manifest.bin.");

    let bm_dec = decrypt_bm(data_enc, key_hex);
    let bm_json = optimize_json(bm_dec);
    let bm_enc = encrypt_bm(bm_json, key_hex);

    let mut build_manifest_final = File::create("build-manifest.bin").expect("Failed to open build-manifest.bin for writing.");
    build_manifest_final.write_all(&bm_enc).expect("Failed to write new contents to build-manifest.bin.");
}