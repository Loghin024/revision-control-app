//Binary Large OBject
use::blake3::Hash;
use crate::hex;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Blob(Hash);

impl Ord for Blob {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.as_bytes().cmp(other.0.as_bytes())
    }
}

impl PartialOrd for Blob{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.as_bytes().partial_cmp(other.0.as_bytes())
    }
}

impl From<&Vec<u8>> for Blob {
    fn from(vec: &Vec<u8>) -> Self {
        Blob(blake3::hash(&vec))
    }
}

impl From<&[u8]> for Blob {
    fn from(bytes: &[u8]) -> Self {
        Blob(blake3::hash(&bytes))
    }
}

impl Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b: &[u8] = self.0.as_bytes();
        write!(f, "{}", hex::Hex::from(b))
    }
}
