// All types are copied from tlsn_core crate, so we can use them in our tests.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tlsn_core::{
    attestation::{Field, FieldId, Header},
    connection::{
        ConnectionInfo, ServerCertCommitment, ServerCertOpening, ServerEphemKey, ServerName,
    },
    hash::{Hash, HashAlgId, TypedHash},
    signing::{Signature, VerifyingKey},
    transcript::{Direction, Idx, TranscriptProof},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePath(&'static str);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPresentation {
    pub attestation: TestAttestationProof,
    pub identity: Option<ServerIdentityProof>,
    pub transcript: Option<TranscriptProof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerIdentityProof {
    pub name: ServerName,
    pub opening: ServerCertOpening,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAttestationProof {
    pub signature: Signature,
    pub header: Header,
    pub body: BodyProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyProof {
    pub body: TestBody,
    pub proof: MerkleProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestBody {
    pub verifying_key: Field<VerifyingKey>,
    pub connection_info: Field<ConnectionInfo>,
    pub server_ephemeral_key: Field<ServerEphemKey>,
    pub cert_commitment: Field<ServerCertCommitment>,
    pub encoding_commitment: Option<Field<EncodingCommitment>>,
    pub plaintext_hashes: Index<Field<PlaintextHash>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodingCommitment {
    pub root: TypedHash,
    pub seed: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct Index<T> {
    items: Vec<T>,
    field_ids: HashMap<FieldId, usize>,
    transcript_idxs: HashMap<Idx, usize>,
}

impl<T: Serialize> Serialize for Index<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.items.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Index<T>
where
    Index<T>: From<Vec<T>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Vec::<T>::deserialize(deserializer).map(Index::from)
    }
}

impl<T> From<Index<T>> for Vec<T> {
    fn from(value: Index<T>) -> Self {
        value.items
    }
}

impl<T> Index<T> {
    pub(crate) fn new<F>(items: Vec<T>, f: F) -> Self
    where
        F: Fn(&T) -> (&FieldId, &Idx),
    {
        let mut field_ids = HashMap::new();
        let mut transcript_idxs = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            let (id, idx) = f(item);
            field_ids.insert(*id, i);
            transcript_idxs.insert(idx.clone(), i);
        }
        Self {
            items,
            field_ids,
            transcript_idxs,
        }
    }
}

impl From<Vec<Field<PlaintextHash>>> for Index<Field<PlaintextHash>> {
    fn from(items: Vec<Field<PlaintextHash>>) -> Self {
        Self::new(items, |field: &Field<PlaintextHash>| (&field.id, &field.data.idx))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PlaintextHash {
    pub direction: Direction,
    pub idx: Idx,
    pub hash: TypedHash,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MerkleProof {
    pub alg: HashAlgId,
    pub tree_len: usize,
    pub proof: rs_merkle::MerkleProof<Hash>,
}
mod rs_merkle {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MerkleProof<H> {
        pub proof_hashes: Vec<H>,
    }
}
