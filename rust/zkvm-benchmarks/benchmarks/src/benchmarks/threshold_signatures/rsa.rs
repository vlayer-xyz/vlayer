use crate::{with_fixture, Benchmark};

pub fn benchmarks() -> Vec<Benchmark> {
    vec![
        Benchmark::new("rsa-1", with_fixture!(rsa::hardcoded(1), rsa::verify), 10_000_000),
        Benchmark::new("rsa-3", with_fixture!(rsa::hardcoded(3), rsa::verify), 30_000_000),
    ]
}

const MESSAGE: &'static str = "ala ma kota";

mod rsa {

    use risc0_zkvm::sha::rust_crypto::Sha256;
    use rsa::{
        pkcs1::DecodeRsaPublicKey,
        pkcs1v15::{Signature, VerifyingKey},
        signature::Verifier,
    };

    use super::MESSAGE;

    pub(super) fn hardcoded(min_signers: usize) -> (Vec<Signature>, Vec<VerifyingKey<Sha256>>) {
        let sigs: [[u8; 256]; 3] = [
            [
                166, 242, 200, 142, 73, 181, 184, 173, 121, 220, 58, 155, 154, 116, 173, 205, 189,
                183, 201, 201, 217, 240, 37, 144, 183, 9, 52, 39, 200, 116, 225, 199, 116, 65, 149,
                39, 35, 153, 9, 69, 84, 26, 115, 207, 100, 161, 185, 158, 13, 237, 125, 210, 154,
                93, 82, 123, 33, 127, 28, 37, 243, 97, 140, 48, 13, 120, 190, 201, 244, 92, 125, 6,
                143, 175, 132, 199, 156, 184, 14, 226, 182, 221, 122, 128, 249, 166, 204, 216, 45,
                177, 85, 84, 23, 48, 11, 7, 18, 88, 144, 205, 102, 239, 18, 254, 65, 9, 40, 7, 177,
                116, 165, 78, 55, 94, 0, 150, 134, 173, 103, 242, 226, 83, 208, 37, 235, 0, 125,
                175, 174, 249, 210, 3, 173, 42, 8, 254, 121, 238, 83, 37, 60, 240, 136, 7, 9, 197,
                17, 115, 255, 17, 140, 182, 1, 36, 172, 213, 4, 220, 91, 138, 244, 230, 24, 174,
                13, 195, 130, 7, 147, 29, 14, 98, 124, 82, 165, 196, 184, 18, 62, 102, 192, 90,
                118, 192, 34, 196, 171, 47, 184, 140, 61, 13, 182, 81, 181, 128, 143, 134, 145, 75,
                94, 208, 5, 226, 69, 237, 96, 34, 46, 86, 231, 19, 219, 45, 83, 15, 85, 17, 238,
                72, 57, 86, 114, 247, 109, 75, 44, 30, 155, 75, 56, 97, 156, 113, 100, 220, 159,
                138, 210, 67, 245, 209, 161, 196, 26, 202, 50, 100, 20, 218, 163, 180, 98, 165, 61,
                21,
            ],
            [
                168, 117, 29, 120, 49, 79, 113, 52, 14, 191, 158, 102, 139, 222, 13, 13, 215, 225,
                103, 49, 143, 83, 255, 80, 230, 158, 141, 220, 167, 249, 158, 133, 29, 228, 96, 16,
                176, 199, 247, 225, 226, 76, 163, 121, 98, 207, 14, 151, 84, 186, 206, 198, 213,
                138, 118, 36, 198, 37, 229, 81, 25, 115, 113, 156, 49, 44, 177, 105, 177, 118, 132,
                201, 254, 156, 242, 175, 184, 149, 238, 143, 95, 88, 158, 204, 131, 22, 162, 206,
                75, 158, 124, 109, 100, 158, 114, 229, 131, 71, 117, 238, 116, 247, 114, 148, 193,
                176, 229, 12, 203, 172, 195, 211, 103, 50, 212, 190, 0, 5, 95, 255, 149, 71, 246,
                142, 67, 192, 231, 201, 29, 252, 202, 193, 231, 63, 93, 88, 145, 191, 15, 161, 243,
                147, 71, 15, 177, 37, 10, 212, 228, 237, 252, 182, 245, 90, 79, 58, 17, 124, 42,
                104, 215, 136, 202, 212, 138, 207, 255, 121, 62, 107, 99, 172, 159, 175, 27, 175,
                81, 32, 97, 50, 42, 50, 117, 239, 35, 24, 227, 98, 238, 60, 45, 123, 181, 96, 235,
                178, 126, 167, 12, 94, 154, 54, 146, 248, 231, 52, 91, 138, 40, 202, 155, 196, 16,
                182, 52, 87, 147, 83, 57, 196, 49, 104, 231, 88, 144, 217, 119, 130, 85, 187, 124,
                18, 36, 79, 132, 41, 247, 124, 83, 172, 67, 121, 140, 138, 208, 75, 213, 94, 71,
                199, 86, 41, 123, 52, 107, 1,
            ],
            [
                37, 111, 220, 207, 198, 222, 92, 219, 126, 242, 220, 135, 6, 120, 57, 79, 190, 77,
                82, 178, 67, 150, 74, 69, 55, 35, 114, 69, 217, 11, 147, 170, 141, 104, 29, 85,
                176, 92, 167, 96, 6, 53, 19, 202, 15, 174, 153, 30, 214, 253, 110, 8, 251, 197,
                126, 79, 119, 123, 62, 73, 25, 253, 162, 194, 155, 245, 210, 165, 139, 115, 189,
                234, 76, 232, 197, 249, 44, 139, 149, 16, 133, 236, 111, 186, 56, 11, 234, 150,
                102, 62, 128, 213, 125, 132, 35, 113, 183, 86, 14, 169, 180, 54, 89, 39, 215, 48,
                147, 223, 233, 248, 91, 92, 101, 192, 233, 44, 42, 54, 199, 205, 230, 63, 140, 93,
                134, 12, 114, 179, 195, 65, 1, 213, 181, 28, 225, 10, 106, 63, 104, 249, 15, 196,
                150, 186, 190, 74, 161, 93, 235, 87, 23, 113, 101, 23, 167, 149, 97, 148, 152, 207,
                64, 215, 121, 108, 1, 195, 138, 249, 10, 121, 32, 192, 67, 181, 222, 0, 66, 28,
                198, 14, 64, 162, 58, 235, 132, 75, 131, 54, 120, 197, 12, 229, 175, 45, 133, 110,
                246, 183, 169, 147, 228, 103, 22, 5, 52, 224, 224, 223, 171, 3, 55, 9, 18, 137,
                105, 100, 27, 243, 218, 59, 244, 105, 34, 103, 177, 10, 79, 136, 59, 63, 34, 3, 98,
                209, 179, 52, 68, 4, 242, 32, 168, 28, 131, 40, 122, 166, 13, 121, 164, 52, 146,
                190, 229, 108, 207, 21,
            ],
        ];

        let  pub_keys_enc = [
            "-----BEGIN RSA PUBLIC KEY-----\nMIIBCgKCAQEAvmoPl2/NPxsZBYePgyQkdb8oWLMUgh2t3pUHmsAKBBmn29OYjz+D\nb+RODuFh0HYhGjv+cQJJINLnBG+aYT5faMUBO/7C2azvy6lwPMlpRDT2npaORFdg\n8YBVRATlZhbtGoERJRkXPDz1jxzMqj3VxOdjcjknKksVlyWvVFnXqVUh2ZkxE/nB\nI+K9qM9b34RfYjUDLWMhBrTgPdYtSV3XCD4hIo8FmDMvK9lEQ/RT3kaU+TJPkMIe\nKvVJMMkzGO8XmD7aXaiDFSzf/Tv20dirGwB7I+QVkiLB3wGiokcpLlzbj/rrEm+x\nJfMqusFn8ArBf4EkduPIlV2l0IKo7RJJvQIDAQAB\n-----END RSA PUBLIC KEY-----\n",
            "-----BEGIN RSA PUBLIC KEY-----\nMIIBCgKCAQEAvm1Ka+L1QwBoHSdewM7U7G64VSTXyO/O+KmQ4DT87g6YCqP4sTzr\nUjC3SV+qaD+lF8UhnAAj0iEFtemjknt4HqkILosOzTzlIfMovsFvCKjscExotmgD\nSgUYuAcPubyMg9zDUCjdOpzviDA/JAqdGlBntIAMlOX8/J/kUWL4C2/7CcVv1mQT\n0B7wJQFzrk5Th8O7ePgWwHNTwZKPiL+oj/HoEY5bqd+cBtfJLqwTZ934XmE32Frj\nxSLclLvAeAIDB7LNsY+7XBx+RlRaWUZV+uq62jxBRj0j50dsTVO6Q7uU3Ln7S26T\nm2K9bJikr+OTTU3HUddBahpLpWWtSfSNewIDAQAB\n-----END RSA PUBLIC KEY-----\n",
            "-----BEGIN RSA PUBLIC KEY-----\nMIIBCgKCAQEA48khj491EpC4uoeW5u8AOCIiMsH1t0/jzpfS2xcchzosubwMkXFO\nzJqsEers+YqNdLuyJKnk7cXe29qVrF48B9tpp3zMb/NtB6rooQEnBPE/kmF5MqiL\nKFkNrsrE36qd7li0sXni6Xct8r1x48wvedMWbaROMDI3L4ctnBXIKCGB0jAL54yc\nwOgWHn6UIGlLFeYYn5TE8+b69NHMf/JANdRG5ykxgKiHzpQRrM9ydFSpL8Sgnbkz\nfqUsgsb8Lf+pzZd87r8UfDQJnPxYXpFJPMBDI++qk79kAx42Cb8AZnJsHdjN0OxJ\nkc5obcW+tlgc2QTB4XZ4W7a7UlNzVc/kJQIDAQAB\n-----END RSA PUBLIC KEY-----\n",
            "-----BEGIN RSA PUBLIC KEY-----\nMIIBCgKCAQEA4xC7lzs3zFHQ89cRc4d4GUupcWU3BQfXJ18FkbYijVhFSEnWHZzL\nfKLxjQSoWYWURON4J0WOWQ1ewRGTfFt6P3qAMt3LF3r1cf715Dv96uEHIWxW0sNv\nWoA0nZALrWa23grmtHfwiFoKAwP5ri4cFXLqRdq2xYf3tR11lhETXwKgt+DjPRgN\nW/Gztchs0x4SmQ8xCOLokuNJ06LDwbtvWt6/9Lpo/qbfHc5BMjojtod6kF9djU+G\nZRZmWXJmPDMDf11cftm/RqRz4SI8Y9E6ZuP9kCq/LAsxabuD/xVO46ZGUWAjziP4\nKFuM/8yeCD5ZjAoj3WEDJLJhXO+3ZPrC4QIDAQAB\n-----END RSA PUBLIC KEY-----\n",
            "-----BEGIN RSA PUBLIC KEY-----\nMIIBCgKCAQEA0ySunX2GKe4Ffashna5/iSbZFrJAKaWHU8KPOU2fDCMyBlm087OI\neJW9qqY1OJIiaMbLdgP7G5PCV0EBpCUJPuVaoYjpCLs9Vwefp180SeXqLa2zlZU1\n4mLViC+essMEyMhYAyr+peQb6Yw4J4/pT2SqLfOot1zHxLWz2CTyMm7LtpgYBHGk\nap1b7eA0gVYa8tb5ZXZGNQUoCpfU+wx+l5oZIjoRdMqsEG7xePClqycqSqsS2JWX\nl8oOkLtZfdw8GRYxi4z1adzB/WG+Y/U4zVTTLi+MQtLlXJ+6kAazsAsmf500C2K0\nnu0fvcNvpUVcur0jSNJx0JvV89+7wZjgLwIDAQAB\n-----END RSA PUBLIC KEY-----\n",
        ];

        let signatures = sigs
            .iter()
            .take(min_signers)
            .map(|s| Signature::try_from(s.as_slice()).unwrap())
            .collect();
        let keys = pub_keys_enc
            .iter()
            .map(|k| VerifyingKey::from_pkcs1_pem(k).unwrap())
            .collect::<Vec<_>>();

        (signatures, keys)
    }

    pub(super) fn verify((signatures, pub_keys): (Vec<Signature>, Vec<VerifyingKey<Sha256>>)) {
        let result = signatures
            .iter()
            .zip(pub_keys)
            .all(|(sig, key)| key.verify(MESSAGE.as_bytes(), sig).is_ok());

        assert!(result);
    }
}
