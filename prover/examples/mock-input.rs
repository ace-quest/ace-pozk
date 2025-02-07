use ark_ec::{AffineRepr, CurveGroup};
use ark_ed_on_bn254::{EdwardsProjective, Fr};
use ark_ff::{BigInteger, One, PrimeField, UniformRand};
use ark_serialize::{CanonicalSerialize, Compress};
use ethabi::{encode, ethereum_types::U256, Token};
use rand_chacha::{rand_core::SeedableRng, ChaChaRng};
use std::fs::write;
use zshuffle::{keygen::Keypair, mask::mask};
use uzkge::anemoi::{AnemoiJive, AnemoiJive254};

const NUM: usize = 52;
const N_CARDS_PUBLIC: usize = 17;

#[inline]
fn parse_point_to_tokens<F: PrimeField, G: CurveGroup<BaseField = F>>(p: G) -> (Token, Token) {
    let affine = G::Affine::from(p);
    let (cx, cy) = affine.xy().unwrap();

    let x_bytes = cx.into_bigint().to_bytes_be();
    let y_bytes = cy.into_bigint().to_bytes_be();

    let x = U256::from_big_endian(&x_bytes);
    let y = U256::from_big_endian(&y_bytes);

    (Token::Uint(x), Token::Uint(y))
}

/// cargo run --example mock-input
/// python3 -m http.server 8000
/// INPUT=http://localhost:8000/test_inputs cargo run --release
fn main() {
    let mut prng = ChaChaRng::from_entropy();

    let joint_keypair = Keypair::generate(&mut prng);
    let joint_pk = joint_keypair.public;

    let mut joint_pk_bytes = Vec::new();
    joint_pk
        .serialize_with_mode(&mut joint_pk_bytes, Compress::Yes)
        .unwrap();
    let joint_pk_token = Token::Uint(U256::from_big_endian(&joint_pk_bytes));

    let mut new_cards_token = vec![];
    let mut new_cards_digest = vec![];
    for _ in 0..NUM {
        let point = EdwardsProjective::rand(&mut prng);
        let (nc, _) = mask(&mut prng, &joint_pk, &point, &Fr::one()).unwrap();

        new_cards_digest.push(nc);

        let (x1, y1) = parse_point_to_tokens(nc.e1);
        let (x2, y2) = parse_point_to_tokens(nc.e2);

        // e2, e1
        new_cards_token.push(x2);
        new_cards_token.push(y2);
        new_cards_token.push(x1);
        new_cards_token.push(y1);
    }

    let hash = AnemoiJive254::eval_variable_length_hash(
        &new_cards_digest
            .iter()
            .skip(N_CARDS_PUBLIC)
            .flat_map(|x| x.flatten())
            .collect::<Vec<_>>(),
    );
    let hash_bytes = hash.into_bigint().to_bytes_be();
    let digest = U256::from_big_endian(&hash_bytes);

    // encode
    // let bytes = encode(&[joint_pk_token, Token::Array(new_cards_token)]);

    let inputs_bytes = encode(&[joint_pk_token]);
    let publics_bytes = encode(&[Token::Array(new_cards_token), Token::Uint(digest)]);

    let inputs_str = format!("0x{}", hex::encode(&inputs_bytes));
    let publics_str = format!("0x{}", hex::encode(&publics_bytes));

    let mut bytes = (inputs_bytes.len() as u32).to_be_bytes().to_vec();
    bytes.extend(inputs_bytes);
    bytes.extend(publics_bytes);

    // let content = format!("0x{}", hex::encode(&bytes));
    write(format!("./test_inputs_{NUM}"), bytes).unwrap();
    write(format!("./test_miner_{NUM}"), format!("{}\n{}", inputs_str, publics_str)).unwrap();
    write("../test/test_publics", publics_str).unwrap();
}
