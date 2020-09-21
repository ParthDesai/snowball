/// Any object that can be signed
pub trait Signable {
    /// Returns bytes that need to be signed
    fn sign_bytes(&self) -> Vec<u8>;
}
