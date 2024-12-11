use alloy_primitives::B256;
use digest::Digest;

pub fn hash<D: Digest>(data: impl AsRef<[u8]>) -> B256 {
    let digest = D::digest(data.as_ref());
    B256::from_slice(digest.as_slice())
}

pub fn keccak256(data: impl AsRef<[u8]>) -> B256 {
    hash::<sha3::Keccak256>(data)
}

pub fn sha2(data: impl AsRef<[u8]>) -> B256 {
    hash::<sha2::Sha256>(data)
}
