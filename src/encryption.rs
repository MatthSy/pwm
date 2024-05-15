use ring::aead::{Aad, AES_256_GCM, BoundKey, Nonce, NONCE_LEN, NonceSequence, OpeningKey, SealingKey, UnboundKey};
use ring::error::Unspecified;
use crate::access::get_mdp_input;
use crate::memoire::{EncryptedData, ToMdp};
use crate::my_hasher::hash;

#[allow(unused)]
pub(crate) struct CounterNonceSequence(pub(crate) u32);

#[allow(unused)]
impl NonceSequence for CounterNonceSequence {
    // called once for each seal operation
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

        self.0 += 1; // advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}


pub(crate) fn encrypt_mdp(input :String, site: String, counter: u32) -> EncryptedData {
    let mdp_key = get_mdp_input();

    let mut key_bytes = vec![0; AES_256_GCM.key_len()];
    let hashed = hash(&mdp_key);
    for i in 0..AES_256_GCM.key_len() {
        key_bytes[i] = hashed[i];
    }
    println!("{:?}", &key_bytes);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes).expect("Unbound key creation fail");
    let nonce_sequence = CounterNonceSequence(counter);
    let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);

    let associated_data = Aad::from(site.clone());
    let mut in_out = input.clone().to_mdp();

    // Encrypt the data with AEAD using the AES_256_GCM algorithm
    let tag = sealing_key.seal_in_place_separate_tag(associated_data, &mut in_out).expect("Encrypting error");
    EncryptedData { mdp: in_out, site, tag: <[u8; 16]>::try_from(tag.as_ref()).unwrap() }
}

#[allow(unused)]
pub(crate) fn decrypt_mdp(input: EncryptedData, counter: u32) -> String{
    let mdp_key = get_mdp_input();

    let mut key_bytes = vec![0; AES_256_GCM.key_len()];
    let hashed = hash(&mdp_key);
    for i in 0..AES_256_GCM.key_len() {
        key_bytes[i] = hashed[i];
    }
    println!("{:?}", &key_bytes);

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes).expect("Error creating unbound_key");
    let nonce_sequence = CounterNonceSequence(counter);
    let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

    let mut cypher_text_with_tag = [&input.mdp, input.tag.as_ref()].concat();

    let associated_data = Aad::from(input.site.clone());
    let decrypted_data = opening_key.open_in_place(associated_data, &mut cypher_text_with_tag).expect("Error, incorrect password or unknown internal error");

    println!("decrypted_data = {}", String::from_utf8(decrypted_data.to_vec()).unwrap());
    String::from_utf8(decrypted_data.to_vec()).unwrap()
}