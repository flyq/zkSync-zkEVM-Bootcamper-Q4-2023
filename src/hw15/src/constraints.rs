use ark_crypto_primitives::crh::{CRHScheme, TwoToOneCRHScheme};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::prelude::UInt8;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use crate::types::{
    ConstraintF, LeafHash, LeafHashParamsVar, Root, RootVar, SimplePath, SimplePathVar,
    TwoToOneHash, TwoToOneHashParamsVar,
};

pub struct MerkleTreeVerification {
    // These are constants that will be embedded into the circuit
    pub leaf_crh_params: <LeafHash as CRHScheme>::Parameters,
    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRHScheme>::Parameters,

    // These are the public inputs to the circuit.
    pub root: Root,
    pub leaf: u8,

    // This is the private witness to the circuit.
    pub authentication_path: Option<SimplePath>,
}

impl ConstraintSynthesizer<ConstraintF> for MerkleTreeVerification {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // First, we allocate the public inputs
        let root = RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.root))?;

        let leaf = UInt8::new_input_vec(ark_relations::ns!(cs, "leaf_var"), &[self.leaf])?;

        // Then, we allocate the public parameters as constants:
        let leaf_crh_params = LeafHashParamsVar::new_constant(cs.clone(), &self.leaf_crh_params)?;
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        // Finally, we allocate our path as a private witness variable:
        let path = SimplePathVar::new_witness(ark_relations::ns!(cs, "path_var"), || {
            Ok(self.authentication_path.as_ref().unwrap())
        })?;

        // let leaf_bytes = vec![leaf; 1];

        // Now, we have to check membership. How do we do that?
        // Hint: look at https://github.com/arkworks-rs/crypto-primitives/blob/6be606259eab0aec010015e2cfd45e4f134cd9bf/src/merkle_tree/constraints.rs#L135

        let is_member =
            path.verify_membership(&leaf_crh_params, &two_to_one_crh_params, &root, &leaf)?;

        is_member.enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}
