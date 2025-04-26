//! Cryptography utilities for RA2 MIX files

use crate::{MixError, constants::*};
use blowfish::{
    Blowfish,
    cipher::{BlockDecrypt, KeyInit, generic_array},
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use num::BigUint;
use std::io::Cursor;

/// Decrypts the Blowfish key from the encrypted key in the MIX file header
///
/// # Arguments
/// * `encrypted_blowfish_key` - The encrypted Blowfish key from the MIX file header
///
/// # Returns
/// The decrypted Blowfish key or an error if decryption fails
pub fn decrypt_blowfish_key(encrypted_blowfish_key: &[u8]) -> Result<Vec<u8>, MixError> {
    const BLOCK_SIZE: usize = 40;
    // 65537
    let public_exponent = BigUint::from_bytes_le(&[1, 0, 1]);
    // 681994811107118991598552881669230523074742337494683459234572860554038768387821901289207730765589
    let public_modulus = BigUint::from_bytes_le(&[
        21, 127, 67, 170, 61, 79, 251, 209, 230, 193, 176, 248, 106, 14, 221, 171, 74, 176, 130, 102, 250, 84, 170, 232, 162,
        63, 113, 81, 214, 96, 81, 86, 228, 252, 57, 109, 8, 218, 188, 81,
    ]);
    if encrypted_blowfish_key.len() < SIZE_OF_ENCRYPTED_KEY {
        return Err(MixError::CryptoError { message: "Length of blowfish key must > 80".to_string() });
    }
    let mut decrypted_blowfish_key = Vec::new();
    // Process each 40-byte block
    for i in (0..SIZE_OF_ENCRYPTED_KEY).step_by(BLOCK_SIZE) {
        let end = std::cmp::min(i + BLOCK_SIZE, encrypted_blowfish_key.len());
        let encrypted_block = &encrypted_blowfish_key[i..end];
        let block_int = BigUint::from_bytes_le(encrypted_block);
        // Perform RSA decryption (actually encryption with public key in this case)
        let decrypted_int = block_int.modpow(&public_exponent, &public_modulus);
        let mut decrypted = decrypted_int.to_bytes_le();
        // Trim end zeros
        while let Some(&0) = decrypted.last() {
            decrypted.pop();
        }
        decrypted_blowfish_key.extend_from_slice(&decrypted);
    }
    Ok(decrypted_blowfish_key)
}

/// Decrypts a MIX file header using the provided Blowfish key
///
/// # Arguments
/// * `mix_data` - The MIX file data
/// * `key` - The decrypted Blowfish key
///
/// # Returns
/// A tuple containing the file count, data size, and decrypted index data
pub fn decrypt_mix_header(mix_data: &[u8], key: &[u8]) -> Result<(u16, u32, Vec<u8>), MixError> {
    // Create Blowfish cipher with LittleEndian byte order
    let cipher = Blowfish::<BigEndian>::new_from_slice(key)
        .map_err(|e| MixError::CryptoError { message: format!("Failed to create Blowfish cipher: {}", e) })?;
    let header_start = SIZE_OF_FLAGS + SIZE_OF_ENCRYPTED_KEY;

    // Decrypt the first block
    let mut decrypted_block = [0u8; BLOCK_SIZE];
    let first_block = &mix_data[header_start..header_start + BLOCK_SIZE];

    // Copy the block to decrypt
    let mut block = [0u8; BLOCK_SIZE];
    block.copy_from_slice(first_block);

    // Decrypt the block
    cipher.decrypt_block(generic_array::GenericArray::from_mut_slice(&mut block));
    decrypted_block.copy_from_slice(&block);

    // Read file count and data size from the decrypted block
    let mut cursor = Cursor::new(&decrypted_block[..SIZE_OF_FILE_COUNT + SIZE_OF_DATA_SIZE]);
    let file_count = cursor.read_u16::<LittleEndian>()?;
    let data_size = cursor.read_u32::<LittleEndian>()?;

    // Calculate decryption sizes
    let (decrypt_size, padding_size) = get_decryption_block_sizing(file_count);

    // Decrypt the rest of the index data
    let encrypted_data = &mix_data[header_start + BLOCK_SIZE..header_start + BLOCK_SIZE + decrypt_size];
    let mut data_decrypted = Vec::with_capacity(decrypt_size);

    // Process each block
    for chunk in encrypted_data.chunks(BLOCK_SIZE) {
        let mut block = [0u8; BLOCK_SIZE];
        if chunk.len() < BLOCK_SIZE {
            // Handle the last partial block
            block[..chunk.len()].copy_from_slice(chunk);
        }
        else {
            block.copy_from_slice(chunk);
        }

        // Decrypt the block
        cipher.decrypt_block(generic_array::GenericArray::from_mut_slice(&mut block));

        // Add the decrypted block to the result
        if chunk.len() < BLOCK_SIZE {
            data_decrypted.extend_from_slice(&block[..chunk.len()]);
        }
        else {
            data_decrypted.extend_from_slice(&block);
        }
    }

    // Combine the index data from the first block and the rest of the decrypted data
    let num_bytes_index_data_in_first_block = BLOCK_SIZE - SIZE_OF_FILE_COUNT - SIZE_OF_DATA_SIZE;
    let mut index_decrypted = Vec::with_capacity(num_bytes_index_data_in_first_block + decrypt_size - padding_size);

    // Add the index data from the first block
    index_decrypted.extend_from_slice(&decrypted_block[SIZE_OF_FILE_COUNT + SIZE_OF_DATA_SIZE..]);

    // Add the rest of the index data, excluding padding
    if data_decrypted.len() > padding_size {
        index_decrypted.extend_from_slice(&data_decrypted[..data_decrypted.len() - padding_size]);
    }

    Ok((file_count, data_size, index_decrypted))
}

/// Calculates the decryption block sizing for a MIX file
///
/// # Arguments
/// * `file_count` - The number of files in the MIX file
///
/// # Returns
/// A tuple containing the decrypt size and padding size
pub fn get_decryption_block_sizing(file_count: u16) -> (usize, usize) {
    let index_len = file_count as usize * FILE_ENTRY_SIZE;
    let remaining_index_len = index_len - SIZE_OF_FILE_COUNT;
    let padding_size = BLOCK_SIZE - (remaining_index_len % BLOCK_SIZE);
    let decrypt_size = remaining_index_len + padding_size;

    (decrypt_size, padding_size)
}

#[test]
fn rsa_keys() {
    // RA2 public exponent
    let public_exponent = BigUint::parse_bytes(b"65537", 10).unwrap();
    assert_eq!(public_exponent.to_bytes_le(), &[1, 0, 1]);

    let public_modulus = BigUint::parse_bytes(
        b"681994811107118991598552881669230523074742337494683459234572860554038768387821901289207730765589",
        10,
    )
    .unwrap();
    // RA2 public modulus
    assert_eq!(
        public_modulus.to_bytes_le(),
        &[
            21, 127, 67, 170, 61, 79, 251, 209, 230, 193, 176, 248, 106, 14, 221, 171, 74, 176, 130, 102, 250, 84, 170, 232,
            162, 63, 113, 81, 214, 96, 81, 86, 228, 252, 57, 109, 8, 218, 188, 81,
        ]
    );
}
