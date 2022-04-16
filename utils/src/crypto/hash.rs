use hex::encode;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};

use super::{Error, Result};

/// create a cryptographic hash from a string (sha1, sha256, sha512)
///
/// ```
/// use homedisk_utils::crypto::hasher;
///
/// let sha1 = hasher("SHA-1", "test string".to_string()).unwrap();
/// assert_eq!(sha1, "661295c9cbf9d6b2f6428414504a8deed3020641".to_string())
/// ```
pub fn hasher(algo: &str, input: String) -> Result<String> {
    let hash = match algo {
        "SHA-1" | "SHA1" | "Sha1" | "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(input.as_bytes());

            encode(hasher.finalize())
        }

        "SHA-256" | "SHA256" | "Sha256" | "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());

            encode(hasher.finalize())
        }

        "SHA-512" | "SHA512" | "Sha512" | "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());

            encode(hasher.finalize())
        }

        _ => Err(Error::UnknownAlgorithm("digest", algo.to_string()))?,
    };

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use crate::crypto::{hasher, Error};

    #[test]
    fn sha1() {
        let sha1 = hasher("sha1", "test string".to_string()).unwrap();
        assert_eq!(sha1, "661295c9cbf9d6b2f6428414504a8deed3020641".to_string());
    }

    #[test]
    fn sha256() {
        let sha1 = hasher("sha256", "test string".to_string()).unwrap();
        assert_eq!(
            sha1,
            "d5579c46dfcc7f18207013e65b44e4cb4e2c2298f4ac457ba8f82743f31e930b".to_string()
        );
    }

    #[test]
    fn sha512() {
        let sha1 = hasher("sha512", "test string".to_string()).unwrap();
        assert_eq!(
            sha1,
            "10e6d647af44624442f388c2c14a787ff8b17e6165b83d767ec047768d8cbcb71a1a3226e7cc7816bc79c0427d94a9da688c41a3992c7bf5e4d7cc3e0be5dbac".to_string()
        );
    }

    #[test]
    fn unknown_algorithm() {
        let algo = "unknow_algo";
        let err = hasher(algo, "test string".to_string()).unwrap_err();

        assert_eq!(err, Error::UnknownAlgorithm("digest", algo.to_string()));
    }
}
