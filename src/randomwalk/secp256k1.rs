use rand::rngs::OsRng;
use anyhow::{Result, Ok};
use secp256k1::{SecretKey, PublicKey, Message, hashes::sha256};

pub fn _secp256k1() -> Result<(SecretKey, PublicKey)>{
  let secp256k1 = secp256k1::Secp256k1::new();
  let secp256_key = secp256k1.generate_keypair(&mut OsRng);
  Ok(secp256_key)
}

pub fn _secp256sig(msg:&[u8]) -> Result<bool>{
  let secp = secp256k1::Secp256k1::new();
  let secp_keys = secp.generate_keypair(&mut OsRng);
  let secp_msg = Message::from_hashed_data::<sha256::Hash>(msg);
  let sig_secp = secp.sign_ecdsa(&secp_msg, &secp_keys.0);
  let ver_secp = secp.verify_ecdsa(&secp_msg, &sig_secp, &secp_keys.1);
  Ok(ver_secp.is_ok())
}