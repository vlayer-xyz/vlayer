use crate::{with_fixture, Benchmark};

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new(
            "ed25519sha512",
            with_fixture!(ed25519sha512::setup(), ed25519sha512::verify),
            8_500_000,
        ),
        Benchmark::new(
            "ristretto255",
            with_fixture!(ristretto255::setup(), ristretto255::verify),
            8_500_000,
        ),
        Benchmark::new(
            "secp256k1",
            with_fixture!(secp256k1::setup(), secp256k1::verify),
            8_500_000,
        ),
        Benchmark::new("p256", with_fixture!(p256::setup(), p256::verify), 19_700_000),
        Benchmark::new("ed448", with_fixture!(ed448::setup(), ed448::verify), 87_500_000),
    ]
}

const MESSAGE: &'static str = "ala ma kota";

mod secp256k1 {
    use std::collections::BTreeMap;

    use frost_core;
    use frost_secp256k1 as frost;
    use rand::thread_rng;

    use super::MESSAGE;

    type Crypto = frost::Secp256K1Sha256;
    type Sig = (frost_core::keys::PublicKeyPackage<Crypto>, frost_core::Signature<Crypto>);

    pub(super) fn setup() -> Sig {
        let mut rng = thread_rng();
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap();

        let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

        for (identifier, secret_share) in shares {
            let key_package = frost::keys::KeyPackage::try_from(secret_share).unwrap();
            key_packages.insert(identifier, key_package);
        }

        let mut nonces_map = BTreeMap::new();
        let mut commitments_map = BTreeMap::new();

        for participant_index in 1..=min_signers {
            let participant_identifier = participant_index.try_into().expect("should be nonzero");
            let key_package = &key_packages[&participant_identifier];
            let (nonces, commitments) =
                frost::round1::commit(key_package.signing_share(), &mut rng);
            nonces_map.insert(participant_identifier, nonces);
            commitments_map.insert(participant_identifier, commitments);
        }

        let mut signature_shares = BTreeMap::new();
        let message = MESSAGE.as_bytes();
        let signing_package = frost::SigningPackage::new(commitments_map, message);
        for participant_identifier in nonces_map.keys() {
            let key_package = &key_packages[participant_identifier];

            let nonces = &nonces_map[participant_identifier];

            let signature_share =
                frost::round2::sign(&signing_package, nonces, key_package).unwrap();

            signature_shares.insert(*participant_identifier, signature_share);
        }

        let group_signature =
            frost::aggregate(&signing_package, &signature_shares, &pubkey_package).unwrap();

        (pubkey_package, group_signature)
    }

    pub(super) fn verify((pubkey_package, group_signature): Sig) {
        let is_signature_valid = pubkey_package
            .verifying_key()
            .verify(MESSAGE.as_bytes(), &group_signature)
            .is_ok();
        assert!(is_signature_valid);
    }
}
mod p256 {
    use std::collections::BTreeMap;

    use frost_core;
    use frost_p256 as frost;
    use rand::thread_rng;

    use super::MESSAGE;

    type Crypto = frost::P256Sha256;
    type Sig = (frost_core::keys::PublicKeyPackage<Crypto>, frost_core::Signature<Crypto>);

    pub(super) fn setup() -> Sig {
        let mut rng = thread_rng();
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap();

        let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

        for (identifier, secret_share) in shares {
            let key_package = frost::keys::KeyPackage::try_from(secret_share).unwrap();
            key_packages.insert(identifier, key_package);
        }

        let mut nonces_map = BTreeMap::new();
        let mut commitments_map = BTreeMap::new();

        for participant_index in 1..=min_signers {
            let participant_identifier = participant_index.try_into().expect("should be nonzero");
            let key_package = &key_packages[&participant_identifier];
            let (nonces, commitments) =
                frost::round1::commit(key_package.signing_share(), &mut rng);
            nonces_map.insert(participant_identifier, nonces);
            commitments_map.insert(participant_identifier, commitments);
        }

        let mut signature_shares = BTreeMap::new();
        let message = MESSAGE.as_bytes();
        let signing_package = frost::SigningPackage::new(commitments_map, message);
        for participant_identifier in nonces_map.keys() {
            let key_package = &key_packages[participant_identifier];

            let nonces = &nonces_map[participant_identifier];

            let signature_share =
                frost::round2::sign(&signing_package, nonces, key_package).unwrap();

            signature_shares.insert(*participant_identifier, signature_share);
        }

        let group_signature =
            frost::aggregate(&signing_package, &signature_shares, &pubkey_package).unwrap();

        (pubkey_package, group_signature)
    }

    pub(super) fn verify((pubkey_package, group_signature): Sig) {
        let is_signature_valid = pubkey_package
            .verifying_key()
            .verify(MESSAGE.as_bytes(), &group_signature)
            .is_ok();
        assert!(is_signature_valid);
    }
}
mod ristretto255 {
    use std::collections::BTreeMap;

    use frost_core;
    use frost_ristretto255 as frost;
    use rand::thread_rng;

    use super::MESSAGE;

    type Crypto = frost_ristretto255::Ristretto255Sha512;

    type Sig = (frost_core::keys::PublicKeyPackage<Crypto>, frost_core::Signature<Crypto>);

    pub(super) fn setup() -> Sig {
        let mut rng = thread_rng();
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap();

        let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

        for (identifier, secret_share) in shares {
            let key_package = frost::keys::KeyPackage::try_from(secret_share).unwrap();
            key_packages.insert(identifier, key_package);
        }

        let mut nonces_map = BTreeMap::new();
        let mut commitments_map = BTreeMap::new();

        for participant_index in 1..=min_signers {
            let participant_identifier = participant_index.try_into().expect("should be nonzero");
            let key_package = &key_packages[&participant_identifier];
            let (nonces, commitments) =
                frost::round1::commit(key_package.signing_share(), &mut rng);
            nonces_map.insert(participant_identifier, nonces);
            commitments_map.insert(participant_identifier, commitments);
        }

        let mut signature_shares = BTreeMap::new();
        let message = MESSAGE.as_bytes();
        let signing_package = frost::SigningPackage::new(commitments_map, message);
        for participant_identifier in nonces_map.keys() {
            let key_package = &key_packages[participant_identifier];

            let nonces = &nonces_map[participant_identifier];

            let signature_share =
                frost::round2::sign(&signing_package, nonces, key_package).unwrap();

            signature_shares.insert(*participant_identifier, signature_share);
        }

        let group_signature =
            frost::aggregate(&signing_package, &signature_shares, &pubkey_package).unwrap();

        (pubkey_package, group_signature)
    }

    pub(super) fn verify((pubkey_package, group_signature): Sig) {
        let is_signature_valid = pubkey_package
            .verifying_key()
            .verify(MESSAGE.as_bytes(), &group_signature)
            .is_ok();
        assert!(is_signature_valid);
    }
}
mod ed448 {
    use std::collections::BTreeMap;

    use frost_core;
    use frost_ed448 as frost;
    use rand::thread_rng;

    use super::MESSAGE;

    type Crypto = frost::Ed448Shake256;

    type Sig = (frost_core::keys::PublicKeyPackage<Crypto>, frost_core::Signature<Crypto>);

    pub(super) fn setup() -> Sig {
        let mut rng = thread_rng();
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap();

        let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

        for (identifier, secret_share) in shares {
            let key_package = frost::keys::KeyPackage::try_from(secret_share).unwrap();
            key_packages.insert(identifier, key_package);
        }

        let mut nonces_map = BTreeMap::new();
        let mut commitments_map = BTreeMap::new();

        for participant_index in 1..=min_signers {
            let participant_identifier = participant_index.try_into().expect("should be nonzero");
            let key_package = &key_packages[&participant_identifier];
            let (nonces, commitments) =
                frost::round1::commit(key_package.signing_share(), &mut rng);
            nonces_map.insert(participant_identifier, nonces);
            commitments_map.insert(participant_identifier, commitments);
        }

        let mut signature_shares = BTreeMap::new();
        let message = MESSAGE.as_bytes();
        let signing_package = frost::SigningPackage::new(commitments_map, message);
        for participant_identifier in nonces_map.keys() {
            let key_package = &key_packages[participant_identifier];

            let nonces = &nonces_map[participant_identifier];

            let signature_share =
                frost::round2::sign(&signing_package, nonces, key_package).unwrap();

            signature_shares.insert(*participant_identifier, signature_share);
        }

        let group_signature =
            frost::aggregate(&signing_package, &signature_shares, &pubkey_package).unwrap();

        (pubkey_package, group_signature)
    }

    pub(super) fn verify((pubkey_package, group_signature): Sig) {
        let is_signature_valid = pubkey_package
            .verifying_key()
            .verify(MESSAGE.as_bytes(), &group_signature)
            .is_ok();
        assert!(is_signature_valid);
    }
}

mod ed25519sha512 {
    use std::collections::BTreeMap;

    use frost_core;
    use frost_ed25519 as frost;
    use rand::thread_rng;

    use super::MESSAGE;

    type Crypto = frost_ed25519::Ed25519Sha512;

    type Sig = (frost_core::keys::PublicKeyPackage<Crypto>, frost_core::Signature<Crypto>);

    pub(super) fn setup() -> Sig {
        let mut rng = thread_rng();
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap();

        let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

        for (identifier, secret_share) in shares {
            let key_package = frost::keys::KeyPackage::try_from(secret_share).unwrap();
            key_packages.insert(identifier, key_package);
        }

        let mut nonces_map = BTreeMap::new();
        let mut commitments_map = BTreeMap::new();

        for participant_index in 1..=min_signers {
            let participant_identifier = participant_index.try_into().expect("should be nonzero");
            let key_package = &key_packages[&participant_identifier];
            let (nonces, commitments) =
                frost::round1::commit(key_package.signing_share(), &mut rng);
            nonces_map.insert(participant_identifier, nonces);
            commitments_map.insert(participant_identifier, commitments);
        }

        let mut signature_shares = BTreeMap::new();
        let message = MESSAGE.as_bytes();
        let signing_package = frost::SigningPackage::new(commitments_map, message);
        for participant_identifier in nonces_map.keys() {
            let key_package = &key_packages[participant_identifier];

            let nonces = &nonces_map[participant_identifier];

            let signature_share =
                frost::round2::sign(&signing_package, nonces, key_package).unwrap();

            signature_shares.insert(*participant_identifier, signature_share);
        }

        let group_signature =
            frost::aggregate(&signing_package, &signature_shares, &pubkey_package).unwrap();

        (pubkey_package, group_signature)
    }

    pub(super) fn verify((pubkey_package, group_signature): Sig) {
        let is_signature_valid = pubkey_package
            .verifying_key()
            .verify(MESSAGE.as_bytes(), &group_signature)
            .is_ok();
        assert!(is_signature_valid);
    }
}
