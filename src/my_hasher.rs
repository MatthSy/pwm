use rand::{Rng, SeedableRng};
use rand_chacha::{ChaCha20Rng, ChaCha8Rng};

/// Prend le mdp en parametre et renvoie une version hashée de taille cst = 32 octets
pub(crate) fn hash(str: &str) -> [u8; 32] {
    if str.len() > 32 {panic!("str trop long in fn my_hasher::hash")};
    let mut result: [u8; 32] = [0; 32];

    let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
    for i in 0..str.len() {
        seed[i] = str.as_bytes()[i].clone();
    }
    let mut rng = ChaCha20Rng::from_seed(seed);

    for i in 0..result.len() {
        result[i] = rng.gen_range(33..127);
    }
    result
}

