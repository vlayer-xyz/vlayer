use crate::{KeccakMerkleTrie as MerkleTrie, node::Node, trie::MptError};

#[test]
pub fn empty_key() -> anyhow::Result<()> {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([], [42])?;
    assert_eq!(mpt.get([]).unwrap(), [42]);
    Ok(())
}

#[test]
pub fn one_byte_key() -> anyhow::Result<()> {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x0], [42])?;
    assert_eq!(mpt.get([0x0]).unwrap(), [42]);
    Ok(())
}

#[test]
pub fn duplicate_key() {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0], [42]).unwrap();
    let result = mpt.insert([0], [43]);
    assert_eq!(result.unwrap_err(), MptError::DuplicateKey(Box::from([0])));
}

#[test]
pub fn multi_byte_key() -> anyhow::Result<()> {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x1, 0x1], [42])?;
    assert_eq!(mpt.get([0x1, 0x1]).unwrap(), [42]);
    Ok(())
}

#[test]
pub fn different_length_nibbles() -> anyhow::Result<()> {
    let mut mpt = MerkleTrie(Node::Null);
    mpt.insert([0x0], [42])?;
    mpt.insert([0x10], [43])?;
    assert_eq!(mpt.get([0x0]).unwrap(), [42]);
    assert_eq!(mpt.get([0x10]).unwrap(), [43]);

    Ok(())
}
