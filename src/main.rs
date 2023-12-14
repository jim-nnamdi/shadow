use merkle::merkle_bt::Nodex;
use rand::rngs::OsRng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use secp256k1::{PublicKey, SecretKey};

use anyhow::{Ok, Result};
pub mod euclidean;
pub mod randomwalk;
pub mod singlewalk;
pub mod merkle;
pub mod p2psim;

pub fn shadow_tx_id_hash() -> Result<(SecretKey, PublicKey)> {
    let secp = secp256k1::Secp256k1::new();
    let (sk, pk) = secp.generate_keypair(&mut OsRng);
    Ok((sk, pk))
}

pub fn shadow_block_id_hash() -> Result<(SecretKey, PublicKey)> {
    let secp = secp256k1::Secp256k1::new();
    let (sk, pk) = secp.generate_keypair(&mut OsRng);
    dbg!(sk, pk);
    Ok((sk, pk))
}

pub fn shadow_encrypt_block_data(bits_size: usize, block_data: &[u8]) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let shadow_private = RsaPrivateKey::new(&mut rng, bits_size)?;
    let shadow_public = RsaPublicKey::from(&shadow_private);
    let shadow_data_encrypt = shadow_public.encrypt(&mut rng, Pkcs1v15Encrypt, block_data)?;
    Ok(shadow_data_encrypt)
}

pub fn shadow_decrypt_block_data(bits_size: usize, block_data: Vec<u8>) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let shadow_private = RsaPrivateKey::new(&mut rng, bits_size)?;
    let shadow_data_decrypt = shadow_private.decrypt(Pkcs1v15Encrypt, &block_data)?;
    Ok(shadow_data_decrypt)
}

fn main() {
    let node = Nodex {
        values: vec![1, 2, 3],
        children: vec![
            Nodex {
                values: vec![4, 5],
                children: vec![],
            },
            Nodex {
                values: vec![6, 7],
                children: vec![],
            },
        ],
    };
    let v:Vec<_> = node.vals().collect();
    println!("{:?}", v);
}