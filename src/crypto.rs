use std::borrow::Borrow;
use std::str::FromStr;
use secp256k1::hashes::sha256;
use secp256k1::{Error, PublicKey, SecretKey};
use secp256k1::rand::thread_rng;
use web3::signing::{Key, SecretKeyRef, SigningError, Signature};
use secp256k1::generate_keypair;
use prost::Message;
use crate::pb;


pub fn new_secp256k1() -> (SecretKey, PublicKey) {
    generate_keypair(&mut thread_rng())
}

pub fn recovery(key: &str) -> Result<SecretKey, Error> {
    secp256k1::SecretKey::from_str(key)
}

pub fn sign_transaction(secret: &SecretKey, tx: & mut pb::abi::BxhTransaction) {
    let sr = SecretKeyRef::new(&secret);
    let message = secp256k1::Message::from_hashed_data::<sha256::Hash>(&tx.encode_to_vec());
    let signature = sr.sign_message(message.as_ref()).unwrap();
    let mut sigs: Vec<u8> = Vec::new();
    sigs.push(3); // bitxhub key type
    sigs.append(signature.r.as_bytes().to_vec().as_mut());
    sigs.append(signature.s.as_bytes().to_vec().as_mut());
    sigs.push(signature.v as u8);
    tx.signature = sigs
}


#[cfg(test)]
mod test {
    use super::*;
    use chrono::Local;
    use pb::abi::*;
    use prost::Message;
    use secp256k1::hashes::hex::ToHex;
    use web3::signing::{Key, recover, SecretKeyRef};

    #[test]
    fn test_new_address() {
        let (k0, k1) = new_secp256k1();
        let sr = SecretKeyRef::new(&k0);
        let address = sr.address();
        println!("address: {}", address);
        assert_eq!(address.as_bytes().len(), 20)
    }

    #[test]
    fn test_sign_transaction(){
        let (k0, k1) = new_secp256k1();
        let sr = SecretKeyRef::new(&k0);
        let address = sr.address();
        let data = TransactionData {
            r#type: 0,
            amount: String::from("1000000000"),
            vm_type: 0,
            payload: vec![],
            extra: vec![],
        };
        let mut tx = BxhTransaction {
            version: vec![],
            from: address.as_bytes().to_vec(),
            to: "0x79a1215469FaB6f9c63c1816b45183AD3624bE34"
                .as_bytes()
                .to_vec(),
            timestamp: Local::now().timestamp_nanos(),
            transaction_hash: vec![],
            payload: data.encode_to_vec(),
            nonce: 1,
            amount: String::from(""),
            typ: 0,
            signature: vec![],
            extra: vec![],
            ibtp: None,
        };
        let message = secp256k1::Message::from_hashed_data::<sha256::Hash>(&tx.encode_to_vec());
        sign_transaction(&k0, &mut tx);
        assert_eq!(66, tx.signature.len());
        let x:Vec<u8> = tx.encode_to_vec();
        println!("{}", x.to_hex());
        let signature = tx.signature.as_slice()[1..65].to_vec();
        let result = recover(message.as_ref(), signature.as_slice(), tx.signature.as_slice()[65] as i32).unwrap();
        assert_eq!(address, result)
    }
}
