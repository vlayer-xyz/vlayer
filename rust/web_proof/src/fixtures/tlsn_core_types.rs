// These types are copied from the `tlsn_core` crate to be used in our tests.
// The only change is that all fields have increased visibility, with the `pub(crate)` modifier applied throughout.

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Presentation {
    pub(crate) attestation: AttestationProof,
    pub(crate) identity: Option<ServerIdentityProof>,
    pub(crate) transcript: Option<TranscriptProof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ServerIdentityProof {
    pub(crate) name: ServerName,
    pub(crate) opening: ServerCertOpening,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AttestationProof {
    pub(crate) signature: Signature,
    pub(crate) header: Header,
    pub(crate) body: BodyProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BodyProof {
    pub(crate) body: Body,
    pub(crate) proof: MerkleProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Body {
    pub(crate) verifying_key: Field<VerifyingKey>,
    pub(crate) connection_info: Field<ConnectionInfo>,
    pub(crate) server_ephemeral_key: Field<ServerEphemKey>,
    pub(crate) cert_commitment: Field<ServerCertCommitment>,
    pub(crate) encoding_commitment: Option<Field<EncodingCommitment>>,
    pub(crate) plaintext_hashes: Index<Field<PlaintextHash>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EncodingCommitment {
    pub(crate) root: TypedHash,
    pub(crate) seed: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct Index<T> {
    items: Vec<T>,
    #[allow(dead_code)]
    field_ids: HashMap<FieldId, usize>,
    #[allow(dead_code)]
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
    pub(crate) direction: Direction,
    pub(crate) idx: Idx,
    pub(crate) hash: TypedHash,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct MerkleProof {
    pub(crate) alg: HashAlgId,
    pub(crate) tree_len: usize,
    pub(crate) proof: rs_merkle::MerkleProof<Hash>,
}
mod rs_merkle {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct MerkleProof<H> {
        pub(crate) proof_hashes: Vec<H>,
    }
}
