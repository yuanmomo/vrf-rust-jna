#![crate_type = "dylib"]

//! # Basic example
//!
//! This example shows a basic usage of the `vrf-rs` crate:
//!
//! 1. Instantiate the `ECVRF` by specifying the `CipherSuite`
//! 2. Generate a VRF proof by using the `prove()` function
//! 3. (Optional) Convert the VRF proof to a hash (e.g. to be used as pseudo-random value)
//! 4. Verify a VRF proof by using `verify()` function

use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;
use std::ffi::{CStr,CString};
use std::str;
use std::mem;
use std::os::raw::c_char;


/// Example of just calling into Rust
/// It is marked as "no_mangle", so that our Java code can still see the Rust function after it's
/// compiled (normally the Rust compiler changes the name during compilation. It is marked as
/// allow(snake_case) because Rust functions are supposed to be written in snake_case, but we need
/// to use camelCase to match the name of the function in Java.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn prove(sk: *const c_char, preSeed: *const c_char ) -> *const c_char {
    // init vrf
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();

    // Inputs: Secret Key, Public Key (derived) & Message
    let secret_key = hex::decode(to_string(sk)).unwrap();

    // parse preSeed
    let inputMessage = to_string(preSeed);
    let message: &[u8] = inputMessage.as_bytes();

    // get proof
    let pi = vrf.prove(&secret_key, &message).unwrap();

    // return proof
    return to_ptr(hex::encode(&pi));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn verify(pk: *const c_char, preSeed: *const c_char, pi: *const c_char, ) -> bool {
    // init vrf
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();

    // parse preSeed
    let inputMessage = to_string(preSeed);
    let message: &[u8] = inputMessage.as_bytes();

    // init proof
    let proof = hex::decode(to_string(pi)).unwrap();

    // init public key
    let public_key = hex::decode(to_string(pk)).unwrap();

    // VRF proof verification (returns VRF hash output)
    let beta = vrf.verify(&public_key, &proof, &message);

    return match beta {
        Ok(_beta) => {
        //     println!("VRF proof is valid!\nHash output: {}", hex::encode(&beta));
            true
        }
        Err(_e) => {
            // println!("VRF proof is not valid: {}", e);
            false
        }
    }
}

/// Convert a native string to a Rust string
fn to_string(pointer: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    str::from_utf8(slice).unwrap().to_string()
}

/// Convert a Rust string to a native string
fn to_ptr(string: String) -> *const c_char {
    let cs = CString::new(string.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    // Tell Rust not to clean up the string while we still have a pointer to it.
    // Otherwise, we'll get a segfault.
    mem::forget(cs);
    ptr
}
