use crate::{
    constants::{secp256k1_w3, secp256k1_w9},
    PoseidonConstants,
};
use halo2curves::group::ff::PrimeField;

pub trait PoseidonField: PrimeField<Repr = [u8; 32]> {
    fn poseidon_constants(width: usize) -> PoseidonConstants<Self>;
}

use halo2curves::secp256k1::{Fp, Fq};

impl PoseidonField for Fp {
    fn poseidon_constants(width: usize) -> PoseidonConstants<Self> {
        if width == 3 {
            secp256k1_w3()
        } else if width == 9 {
            secp256k1_w9()
        } else {
            panic!("Poseidon constants are only available for width 3 and 9")
        }
    }
}

impl PoseidonField for Fq {
    fn poseidon_constants(width: usize) -> PoseidonConstants<Self> {
        if width == 3 {
            secp256k1_w3()
        } else if width == 9 {
            secp256k1_w9()
        } else {
            panic!("Poseidon constants are only available for width 3 and 9")
        }
    }
}
