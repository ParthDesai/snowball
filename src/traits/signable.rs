pub trait Signable {
    fn sign_bytes(&self) -> Vec<u8>;
}
