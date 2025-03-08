use std::{
    fmt,
    io::{self, BufRead, Read, stdin},
};

use clap::Parser;
use eyre::eyre;
use serde::Deserialize;
use serde_cbor::from_slice;
use serde_with::{serde_as, skip_serializing_none};

/// Number of bytes that denote the length of the CBOR data
const CBOR_METADATA_LENGTH_LEN: usize = 2;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize)]
/* We must tolerate the explicit lifetime here due to `serde_cbor`'s idea of
 * which Rust types map to which CBOR types. Specifically, `serde_cbor`
 * considers a `Vec<u8>` to be a byte *array* not a *sequence*. We need to
 * deserialise the latter, hence the `&[u8]`.
 * */
struct CanonicalMetadata<'a> {
    pub ipfs: Option<&'a [u8]>,
    bzzr0: Option<&'a [u8]>,
    bzzr1: Option<&'a [u8]>,
    pub experimental: Option<bool>,
    _solc: Option<&'a [u8]>,
}

#[derive(Clone, Debug)]
enum Digest {
    Ipfs(Vec<u8>),
    Swarm(Vec<u8>),
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Digest::Ipfs(bytes) => {
                /* Critically, the human-readable CID (i.e., that one might use
                 * to access via a gateway, for instance), is the
                 * Base58-encoded form of the IPFS digest byte sequence
                 * */
                write!(f, "ipfs://{}", bs58::encode(&bytes).into_string())
            }
            Digest::Swarm(bytes) => write!(f, "bzz://{}", hex::encode(bytes)),
        }
    }
}

#[inline]
fn pad_slice_front(slice: &[u8]) -> [u8; 8] {
    let mut padded = [0u8; 8];
    let start = 8 - slice.len();
    padded[start..].copy_from_slice(slice);
    padded
}

#[inline]
fn cbor_metadata_length(data: &[u8]) -> Option<usize> {
    if data.len() >= CBOR_METADATA_LENGTH_LEN {
        Some(usize::from_be_bytes(pad_slice_front(
            &data[data.len() - CBOR_METADATA_LENGTH_LEN..],
        )))
    } else {
        None
    }
}

#[inline]
fn grab_cbor_data(data: &[u8]) -> Option<&[u8]> {
    cbor_metadata_length(data).and_then(|n| {
        if data.len() >= n {
            Some(
                &data[(data.len() - (n + CBOR_METADATA_LENGTH_LEN))
                    ..(data.len() - CBOR_METADATA_LENGTH_LEN)],
            )
        } else {
            None
        }
    })
}

#[inline]
fn grab_canonical_metadata(data: &[u8]) -> Option<CanonicalMetadata> {
    grab_cbor_data(data).and_then(|bytes| from_slice(bytes).ok())
}

#[inline]
fn grab_ipfs_digest(data: &[u8]) -> Option<Vec<u8>> {
    grab_canonical_metadata(data)
        .and_then(|metadata| metadata.ipfs.map(|ipfs| ipfs.to_vec()))
}

#[inline]
fn grab_swarm_digest(data: &[u8]) -> Option<Vec<u8>> {
    None
}

#[inline]
fn grab_digest(data: &[u8]) -> Option<Digest> {
    match (grab_ipfs_digest(data), grab_swarm_digest(data)) {
        (Some(ipfs), None) => Some(Digest::Ipfs(ipfs)),
        (None, Some(swarm)) => Some(Digest::Swarm(swarm)),
        (Some(ipfs), Some(_)) => Some(Digest::Ipfs(ipfs)),
        (None, None) => None,
    }
}

#[derive(Clone, Debug, Parser)]
struct Opts {
    #[clap(short, long, action)]
    pub live: bool,
    #[clap(short = 'i', long, action)]
    pub hex: bool,
    #[clap(short, long, action)]
    pub metadata: bool,
}

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    if opts.live {
        todo!()
    } else {
        let bytes = if opts.hex {
            let mut line = String::new();
            stdin().lock().read_line(&mut line)?;
            let line = line.trim_end();
            hex::decode(line[2..].trim_end())?
        } else {
            let mut buf = Vec::new();
            io::stdin().lock().read_to_end(&mut buf)?;
            buf
        };
        let cbor_data = match grab_cbor_data(&bytes) {
            Some(t) => t,
            None => return Err(eyre!("No CBOR data present")),
        };
        assert!(cbor_data.len() == cbor_metadata_length(&bytes).unwrap());

        if opts.metadata {
            println!("{:#?}", grab_canonical_metadata(&bytes));
        }

        if let Some(digest) = grab_digest(&bytes) {
            println!("{digest}");
        }
    }

    Ok(())
}
