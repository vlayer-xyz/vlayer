use alloy_primitives::B256;

pub trait Digest: digest::Digest {
    /// Hash of RPL-encoded `Node::Null`
    const EMPTY_ROOT_HASH: B256;
}

impl Digest for sha3::Keccak256 {
    const EMPTY_ROOT_HASH: B256 = alloy_trie::EMPTY_ROOT_HASH;
}

impl Digest for sha2::Sha256 {
    const EMPTY_ROOT_HASH: B256 =
        alloy_primitives::b256!("76be8b528d0075f7aae98d6fa57a6d3c83ae480a8469e668d7b0af968995ac71");
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Node;

    #[test]
    fn empty_root_hash() {
        let empty_node_rlp = Node::<sha2::Sha256>::null().rlp_encoded();
        assert_eq!(keccak256(&empty_node_rlp), sha3::Keccak256::EMPTY_ROOT_HASH);
        assert_eq!(sha2(&empty_node_rlp), sha2::Sha256::EMPTY_ROOT_HASH);
    }
}
