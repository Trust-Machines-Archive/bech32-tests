use bitcoin::network::constants::Network;
use bitcoin::secp256k1::rand::thread_rng;
use bitcoin::secp256k1::{KeyPair, Secp256k1, Signing, Verification, XOnlyPublicKey};
use bitcoin::util::address::AddressType;

fn taproot<C: Signing + Verification>(secp: &Secp256k1<C>, key_pair: KeyPair) -> bitcoin::Address {
    let xonly = XOnlyPublicKey::from_keypair(&key_pair);
    let address = bitcoin::Address::p2tr(secp, xonly.0, None, Network::Bitcoin);
    return address;
}

#[test]
fn generate_taproot_address() {
    let secp = Secp256k1::new();
    let key_pair = KeyPair::new(&secp, &mut thread_rng());

    let address = taproot(&secp, key_pair);
    assert_eq!(address.address_type(), Some(AddressType::P2tr));
}
