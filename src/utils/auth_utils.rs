use hex;
use sha2::{Digest, Sha256};

pub fn hash_password(password: &str, secret_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(secret_key.as_bytes());
    let result = hasher.finalize();
    let res = hex::encode(result);
    println!("Hashed password: {}", hex::encode(result));
    return res;
}

pub fn verify_password(password: &str, hashed: &str, secret_key: &str) -> bool {
    println!("Verifying password: {}", password);
    let hashed_input = hash_password(password, secret_key);
    println!("Comparing {} against {}", hashed_input, hashed);
    println!("Verification result: {}", hashed_input == hashed);
    return (hashed_input == hashed);
}
