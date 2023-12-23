use ark_bls12_381::Fq as F; // Prime Field
use ark_ff::{Field, PrimeField};
use ark_std::UniformRand;

fn main() {
    let mut rng = ark_std::rand::thread_rng();
    // select a random value from the field
    let a = F::rand(&mut rng);

    let modulus = <F as PrimeField>::MODULUS;

    // show that 1 + 1 = 2
    assert_eq!(<F as Field>::ONE + <F as Field>::ONE, F::from(2));

    // show that the multiplicative inverse of a number multipled by itself equals one.
    assert_eq!(a * a.inverse().unwrap(), <F as Field>::ONE);

    // show that a value raised to the power of the modulus is equal to itself
    // use the `pow` function to raise to a power
    assert_eq!(a.pow(modulus), a);

    println!("modulus: {}", modulus);
}
