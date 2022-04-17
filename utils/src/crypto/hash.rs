pub use hex::encode;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};

use super::{Error, Result};

/// create a cryptographic hash from a string (sha1, sha256, sha512)
///
/// ```
/// use homedisk_utils::crypto::{CryptographicHash, encode};
///
/// let mut sha1 = CryptographicHash::new("SHA-1").unwrap();
/// sha1.update(b"test sha1 hash");
///
/// let hash = encode(sha1.finalize());
///
/// assert_eq!(hash, "7726bd9560e1ad4a1a4f056cae5c0c9ea8bacfc2".to_string())
/// ```
#[derive(Debug, Clone)]
pub enum CryptographicHash {
    Sha1(Sha1),
    Sha256(Sha256),
    Sha512(Sha512),
}

impl CryptographicHash {
    pub fn new(algo: &str) -> Result<Self> {
        match algo {
            "SHA-1" | "SHA1" | "Sha1" | "sha1" => Ok(Self::Sha1(Sha1::new())),
            "SHA-256" | "SHA256" | "Sha256" | "sha256" => Ok(Self::Sha256(Sha256::new())),
            "SHA-512" | "SHA512" | "Sha512" | "sha512" => Ok(Self::Sha512(Sha512::new())),
            _ => Err(Error::UnknownAlgorithm("digest", algo.to_string())),
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        match self {
            Self::Sha1(sha1) => sha1.update(input),
            Self::Sha256(sha256) => sha256.update(input),
            Self::Sha512(sha512) => sha512.update(input),
        }
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        match self {
            Self::Sha1(sha1) => sha1.finalize_reset().to_vec(),
            Self::Sha256(sha256) => sha256.finalize_reset().to_vec(),
            Self::Sha512(sha512) => sha512.finalize_reset().to_vec(),
        }
    }

    pub fn hash(algo: &str, input: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Self::new(algo)?;

        hasher.update(input);

        Ok(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::{encode, CryptographicHash, Error};

    #[test]
    fn sha1() {
        let mut sha1 = CryptographicHash::new("SHA-1").unwrap();
        sha1.update(b"test sha1 hash");

        let hash = encode(sha1.finalize());

        assert_eq!(hash, "7726bd9560e1ad4a1a4f056cae5c0c9ea8bacfc2".to_string())
    }

    #[test]
    fn sha256() {
        let mut sha256 = CryptographicHash::new("SHA-256").unwrap();
        sha256.update(b"test sha256 hash");

        let hash = encode(sha256.finalize());

        assert_eq!(
            hash,
            "eaf6e4198f39ccd63bc3e957d43bf4ef67f12c318c8e3cdc2567a37339902dac".to_string()
        )
    }

    #[test]
    fn sha512() {
        let mut sha512 = CryptographicHash::new("SHA-512").unwrap();
        sha512.update(b"test sha512 hash");

        let hash = encode(sha512.finalize());

        assert_eq!(
            hash,
            "b43b4d7178014c92f55be828d66c9f98211fc67b385f7790a5b4b2fcb89fe1831645b5a4c17f3f7f11d8f34d2800a77a2b8faa5a0fb9d6b8f7befbc29a9ce795".to_string()
        )
    }

    #[test]
    fn unknown_algorithm() {
        let algo = "unknow_algo";
        let err = CryptographicHash::new(algo).unwrap_err();

        assert_eq!(err, Error::UnknownAlgorithm("digest", algo.to_string()))
    }
}
