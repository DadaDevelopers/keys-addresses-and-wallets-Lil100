use bitcoin::key::PrivateKey;
use bitcoin::Network;
use rand::thread_rng;
use secp256k1::{Secp256k1, SecretKey};
use bitcoin::Address;
use bitcoin::CompressedPublicKey;
use bitcoin::key::UntweakedPublicKey;
fn main() {
    // Create a Secp256k1 context
    let secp = Secp256k1::new();

    // Generate a random private key
    let mut rng = thread_rng();
    let secret_key = SecretKey::new(&mut rng);

    // Convert it to a Bitcoin private key
    let private_key = PrivateKey::new(secret_key, Network::Bitcoin);

    println!("Private Key (WIF): {}", private_key);

    // Derive the public key
    let public_key = private_key.public_key(&secp);

    println!("Public Key: {}", public_key);
    let legacy_address = Address::p2pkh(public_key, Network::Bitcoin);

    println!("Legacy Address: {}", legacy_address);


    let compressed = CompressedPublicKey::try_from(public_key)
    .expect("Public key should be compressed");

    let bech32_address = Address::p2wpkh(
    &compressed,
    Network::Bitcoin,
    );

    println!("Bech32 Address: {}", bech32_address);
    let (x_only, _) = public_key.inner.x_only_public_key();
let internal_key = UntweakedPublicKey::from(x_only);

let taproot_address = Address::p2tr(
    &secp,
    internal_key,
    None,
    Network::Bitcoin,
);

println!("Taproot Address: {}", taproot_address);

    
}