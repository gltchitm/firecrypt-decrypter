use argon2::{Config, ThreadMode, Variant, Version};

use chacha20poly1305::{XChaCha20Poly1305, Key, XNonce};
use chacha20poly1305::aead::{Aead, NewAead};

use clap::{App, Arg};

use rpassword::read_password_from_tty;

use std::io::{BufReader, Read, Cursor, copy};
use std::fs::{File, create_dir, create_dir_all};
use std::env::{current_dir, set_current_dir};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const MAGIC_VERSION_PREFIX: &str = "@@5c53512d-FIRECRYPT-VERSION-2-6062fceb@@\n\n\n";

const ARGON2_SALT_LEN: u32 = 32;
const ARGON2_TIME_COST: u32 = 3;
const ARGON2_MEM_COST: u32 = 96 * 1024;
const ARGON2_LANES: u32 = 2;
const ARGON2_HASH_LENGTH: u32 = 32;

const XNONCE_LEN: usize = 24;

fn main() {
    let matches = App::new("Firecrypt Decrypter")
        .version(VERSION)
        .about("Decryption tool for profiles encrypted with Firecrypt 2")
        .arg(
            Arg::with_name("profile")
                .required(true)
                .help("Path to the .firecrypt file")
        )
        .arg(
            Arg::with_name("output")
                .required(true)
                .help("Location to place the decrypted profile folder")
        )
        .get_matches();

    let profile_path = Path::new(matches.value_of("profile").unwrap());
    let output_folder = Path::new(matches.value_of("output").unwrap());
    let output_path = Path::join(
        output_folder,
        profile_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    );

    assert!(output_folder.is_dir());
    assert!(profile_path.is_file());
    assert!(profile_path.extension().unwrap() == "firecrypt");
    assert!(!output_path.exists());

    let mut encrypted_reader = BufReader::new(File::open(profile_path).unwrap());

    let mut magic_version_prefix = [0u8; MAGIC_VERSION_PREFIX.len()];
    let mut salt = [0u8; ARGON2_SALT_LEN as usize];
    let mut nonce = [0u8; XNONCE_LEN];
    let mut encrypted_zip_data = vec![];

    encrypted_reader.read_exact(&mut magic_version_prefix).unwrap();
    encrypted_reader.read_exact(&mut salt).unwrap();
    encrypted_reader.read_exact(&mut nonce).unwrap();
    encrypted_reader.read_to_end(&mut encrypted_zip_data).unwrap();

    assert!(magic_version_prefix == MAGIC_VERSION_PREFIX.as_bytes());

    let password = read_password_from_tty(Some("Password:")).unwrap();

    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        thread_mode: ThreadMode::Parallel,
        mem_cost: ARGON2_MEM_COST,
        time_cost: ARGON2_TIME_COST,
        lanes: ARGON2_LANES,
        hash_length: ARGON2_HASH_LENGTH,
        secret: &[],
        ad: &[],
    };

    let key = argon2::hash_raw(password.as_bytes(), &salt, &config).unwrap();

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    let plaintext = cipher.decrypt(
        &XNonce::from_iter(nonce),
        encrypted_zip_data.as_ref()
    ).unwrap();

    let mut archive = zip::ZipArchive::new(Cursor::new(plaintext)).unwrap();

    create_dir(&output_path).unwrap();

    let original_dir = current_dir().unwrap();
    set_current_dir(output_path).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let output_path = file.enclosed_name().unwrap();

        if file.name().ends_with('/') {
            create_dir_all(&output_path).unwrap();
        } else {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    create_dir_all(&parent).unwrap();
                }
            }

            let mut output_file = File::create(&output_path).unwrap();
            copy(&mut file, &mut output_file).unwrap();
        }
    }

    set_current_dir(original_dir).unwrap();
}
