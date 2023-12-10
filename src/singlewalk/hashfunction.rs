
// takes in a message and uses a math function
// to convert the message into a random message
// takes the random message and pass a different
// math function to get the original message

use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use anyhow::{Result, Ok};

fn _rsa_hash_fn(msg:&str, random_bit_size: usize) -> Result<(RsaPrivateKey, RsaPublicKey)> {
  let mut random_walk = rand::thread_rng();
  let rand_bit_sz = random_bit_size;
  let rsa_private = RsaPrivateKey::new(&mut random_walk, rand_bit_sz)?;
  let rsa_public = RsaPublicKey::from(&rsa_private);
  let signed_data = rsa_public.encrypt(&mut random_walk, Pkcs1v15Encrypt, msg.as_bytes())?;
  let verify_data = rsa_private.decrypt(Pkcs1v15Encrypt, &signed_data)?;
  dbg!(verify_data);
  Ok((rsa_private, rsa_public))
}

// for RSA pick two random prime numbers
// compute them to get the max value available
// select a number to stand as the public key
// using the Extended Euclidean Algorithm 
// generate the associated Private key.
