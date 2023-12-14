
use libp2p::{identity,PeerId};

fn _simp2p() {
  let p2p = identity::Keypair::generate_ed25519();
  let peer_id = PeerId::from(p2p.public());
  println!("{:?}", peer_id);
}