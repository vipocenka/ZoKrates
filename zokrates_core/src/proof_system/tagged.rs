use serde::Serialize;
use zokrates_field::Field;

use super::{Fr, Scheme};

#[derive(Serialize)]
pub struct TaggedVerificationKey<T: Field, S: Scheme<T>> {
    scheme: String,
    curve: String,
    #[serde(flatten)]
    vk: S::VerificationKey,
}

#[derive(Serialize)]
pub struct TaggedProof<T: Field, S: Scheme<T>> {
    scheme: String,
    curve: String,
    pub proof: S::ProofPoints,
    pub inputs: Vec<Fr>,
}

impl<T: Field, S: Scheme<T>> TaggedProof<T, S> {
    pub fn new(proof: S::ProofPoints, inputs: Vec<Fr>) -> Self {
        TaggedProof {
            scheme: S::NAME.to_string(),
            curve: T::name().to_string(),
            proof,
            inputs,
        }
    }
}

impl<T: Field, S: Scheme<T>> TaggedVerificationKey<T, S> {
    pub fn new(vk: S::VerificationKey) -> Self {
        TaggedVerificationKey {
            scheme: S::NAME.to_string(),
            curve: T::name().to_string(),
            vk,
        }
    }
}
