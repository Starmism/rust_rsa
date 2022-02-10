use std::fs;
use std::str::FromStr;
use num_bigint::{BigInt, RandBigInt};
use num_traits::{One};
use crate::utils::{extended_euclid, pseudo_prime};

pub fn gen_and_write() {
    let (_e, n, d) = generate_keys();

    fs::write("./n.txt", n.to_string()).expect("Failed to write n.txt.");
    fs::write("./d.txt", d.to_string()).expect("Failed to write d.txt");
}



/// Generates the respective e, n, and d requires for keys
///
/// # Returns
/// * `(BigInt, BigInt, BigInt)` which corresponds to e, n, and d
///
/// You can make the public keys with (e, n)
/// and the private keys with (d, n)
pub fn generate_keys() -> (BigInt, BigInt, BigInt) {
    let mut rng = rand::thread_rng();
    let p = loop {
        let p = rng.gen_bigint(1024);
        if pseudo_prime(p.clone()) { break p }
        println!("Attempting to generate prime #1...");
    };
    println!("Generated prime #1!");
    let q = loop {
        let q = rng.gen_bigint(1024);
        if pseudo_prime(q.clone()) { break q }
        println!("Attempting to generate prime #2...");
    };
    println!("Generated prime #2!");

    let n = p.clone() * q.clone();

    let phi_n = (p.clone() - BigInt::one()) * (q.clone() - BigInt::one());

    let e = BigInt::from_str(&fs::read_to_string("./e.txt").expect("Can't find e.txt in current directory")).expect("Couldn't parse e.txt");

    let (_, d, _) = extended_euclid(e.clone(), phi_n.clone());

    // let n = n + phi_n;
    (e, n, d)
}