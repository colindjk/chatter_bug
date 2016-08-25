extern crate sha1;
extern crate rustc_serialize;

use rustc_serialize::base64::{ ToBase64, STANDARD };

pub fn gen_key(key: &String) -> String {
    let mut m = sha1::Sha1::new();
}

