use rand::rngs::OsRng;
use anyhow::{Result, Ok};
use secp256k1::{SecretKey, PublicKey, Message, hashes::sha256};

use crate::merkle::merkle::Node;

pub fn _secp256k1() -> Result<(SecretKey, PublicKey)>{
  let secp256k1 = secp256k1::Secp256k1::new();
  let secp256_key = secp256k1.generate_keypair(&mut OsRng);
  Ok(secp256_key)
}

pub fn _hash_merkle_nodex(_node_data:Box<Node>) -> Result<String> {
  let sec = secp256k1::Secp256k1::new();
  let kpair = sec.generate_keypair(&mut OsRng);
  let serialise_node = serde_json::to_string(&_node_data).unwrap();
  let sec_msg = Message::from_hashed_data::<sha256::Hash>(serialise_node.as_bytes());
  let sec_sig = sec.sign_ecdsa(&sec_msg, &kpair.0);
  let sec_sig_str = sec_sig.to_string();
  Ok(sec_sig_str)
}

pub fn _secp256sig(msg:&[u8]) -> Result<bool>{
  let secp = secp256k1::Secp256k1::new();
  let secp_keys = secp.generate_keypair(&mut OsRng);
  let secp_msg = Message::from_hashed_data::<sha256::Hash>(msg);
  let sig_secp = secp.sign_ecdsa(&secp_msg, &secp_keys.0);
  let ver_secp = secp.verify_ecdsa(&secp_msg, &sig_secp, &secp_keys.1);
  Ok(ver_secp.is_ok())
}