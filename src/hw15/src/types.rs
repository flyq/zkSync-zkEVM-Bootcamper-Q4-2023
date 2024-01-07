use ark_crypto_primitives::crh::constraints::{CRHSchemeGadget, TwoToOneCRHSchemeGadget};
use ark_crypto_primitives::crh::injective_map::constraints::{
    PedersenCRHCompressorGadget, PedersenTwoToOneCRHCompressorGadget, TECompressorGadget,
};
use ark_crypto_primitives::crh::injective_map::{
    PedersenCRHCompressor, PedersenTwoToOneCRHCompressor, TECompressor,
};
use ark_crypto_primitives::crh::pedersen::Window;
use ark_crypto_primitives::crh::{CRHScheme, TwoToOneCRHScheme};
use ark_crypto_primitives::merkle_tree::constraints::{
    BytesVarDigestConverter, ConfigGadget, PathVar,
};
use ark_crypto_primitives::merkle_tree::{ByteDigestConverter, Config, MerkleTree, Path};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective};
use ark_r1cs_std::prelude::UInt8;

pub type ConstraintF = ark_ed_on_bls12_381::Fq;

pub type LeafVar<ConstraintF> = [UInt8<ConstraintF>];

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LeafWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl Window for LeafWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 144;
}

pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, LeafWindow>;

pub type LeafHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    LeafWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashParamsVar =
    <LeafHashGadget as CRHSchemeGadget<LeafHash, ConstraintF>>::ParametersVar;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwoToOneWindow;
// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl Window for TwoToOneWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 128;
}

pub type TwoToOneHash =
    PedersenTwoToOneCRHCompressor<EdwardsProjective, TECompressor, TwoToOneWindow>;

pub type TwoToOneHashGadget = PedersenTwoToOneCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    TwoToOneWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHSchemeGadget<TwoToOneHash, ConstraintF>>::ParametersVar;

#[derive(Clone)]
pub struct MerkleConfig;

impl Config for MerkleConfig {
    type Leaf = [u8];

    type LeafDigest = <LeafHash as CRHScheme>::Output;
    type LeafInnerDigestConverter = ByteDigestConverter<Self::LeafDigest>;
    type InnerDigest = <TwoToOneHash as TwoToOneCRHScheme>::Output;

    // Our Merkle tree relies on two hashes: one to hash leaves, and one to hash pairs
    // of internal nodes.
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

pub struct SimpleMerkleTreeParamsVar;

impl ConfigGadget<MerkleConfig, ConstraintF> for SimpleMerkleTreeParamsVar {
    type Leaf = LeafVar<ConstraintF>;
    type LeafDigest = <LeafHashGadget as CRHSchemeGadget<LeafHash, ConstraintF>>::OutputVar;
    type LeafInnerConverter = BytesVarDigestConverter<Self::LeafDigest, ConstraintF>;
    type InnerDigest =
        <TwoToOneHashGadget as TwoToOneCRHSchemeGadget<TwoToOneHash, ConstraintF>>::OutputVar;
    type LeafHash = LeafHashGadget;
    type TwoToOneHash = TwoToOneHashGadget;
}

/// The R1CS equivalent of the the Merkle tree root.
pub type RootVar =
    <TwoToOneHashGadget as TwoToOneCRHSchemeGadget<TwoToOneHash, ConstraintF>>::OutputVar;

/// The R1CS equivalent of the the Merkle tree path.
pub type SimplePathVar = PathVar<MerkleConfig, ConstraintF, SimpleMerkleTreeParamsVar>;

/// A Merkle tree containing account information.
pub type SimpleMerkleTree = MerkleTree<MerkleConfig>;
/// The root of the account Merkle tree.
pub type Root = <TwoToOneHash as TwoToOneCRHScheme>::Output;
/// A membership proof for a given account.
pub type SimplePath = Path<MerkleConfig>;
