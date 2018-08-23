//! Elliptic Curves (presently Weierstrass form only)

use core::{fmt::Debug, hash::Hash, str::FromStr};
use generic_array::ArrayLength;
#[cfg(feature = "digest")]
use generic_array::GenericArray;

pub mod nistp256;
pub mod point;
pub mod secp256k1;

pub use self::nistp256::NistP256;
pub use self::secp256k1::Secp256k1;
use error::Error;

/// Elliptic curve in short Weierstrass form suitable for use with ECDSA
pub trait WeierstrassCurve:
    Clone + Debug + Default + Hash + Eq + PartialEq + Send + Sized + Sync
{
    /// Elliptic curve kind
    const CURVE_KIND: WeierstrassCurveKind;

    /// Size of a private scalar for this elliptic curve in bytes
    type PrivateScalarSize: ArrayLength<u8>;

    /// Size of a compressed point for this curve in bytes when serialized
    /// using `Octet-String-to-Elliptic-Curve-Point` encoding defined in
    /// section 2.3.4 of SEC 1: Elliptic Curve Cryptography (Version 2.0).
    /// <http://www.secg.org/sec2-v2.pdf>
    type CompressedPointSize: ArrayLength<u8>;

    /// Size of a raw uncompressed elliptic curve point sans the `0x04`
    /// tag byte added in the `UncompressedPointSize` value.
    type UntaggedPointSize: ArrayLength<u8>;

    /// Size of an uncompressed elliptic curve point serialized using
    /// the `Octet-String-to-Elliptic-Curve-Point` encoding (including the
    /// `0x04` tag)
    type UncompressedPointSize: ArrayLength<u8>;

    /// Maximum size of an ASN.1 DER encoded ECDSA signature using this curve
    type Asn1SignatureMaxSize: ArrayLength<u8>;

    /// Size of a compact, fixed-sized ECDSA signature using this curve
    type FixedSignatureSize: ArrayLength<u8>;
}

/// Types of Weierstrass curves known to this library
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WeierstrassCurveKind {
    /// Placeholder type for unknown elliptic curves. This can be used for
    /// unsupported curve types and also to prevent exhaustiveness checking
    /// so new curve types can be added to this enum without breaking existing
    /// providers (i.e. when matching on this enum, providers should include
    /// an `other` or `_` option to handle unsupported curve types.
    Unknown,

    /// The NIST P-256 (a.k.a. prime256v1, secp256r1) elliptic curve defined in
    /// FIPS 186-4: Digital Signature Standard (DSS)
    ///
    /// <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf>
    NistP256,

    /// The secp256k1 elliptic curve as defined by Certicom's SECG in
    /// SEC 2: Recommended Elliptic Curve Domain Parameters:
    ///
    /// <http://www.secg.org/sec2-v2.pdf>
    Secp256k1,
}

impl FromStr for WeierstrassCurveKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(unused_variables)] // for no_std
        match s {
            "nistp256" => Ok(WeierstrassCurveKind::NistP256),
            "secp256k1" => Ok(WeierstrassCurveKind::Secp256k1),
            other => fail!(ParseError, "invalid elliptic curve type: {}", other),
        }
    }
}

impl WeierstrassCurveKind {
    /// Get the string identifier for this elliptic curve. This name matches
    /// the Signatory module name for this curve (or `"unknown"`).
    pub fn to_str(self) -> &'static str {
        match self {
            WeierstrassCurveKind::Unknown => "unknown",
            WeierstrassCurveKind::NistP256 => "nistp256",
            WeierstrassCurveKind::Secp256k1 => "secp256k1",
        }
    }

    /// Get the SECG identifier name for this particular elliptic curve
    /// (if applicable).
    pub fn to_secg_name(self) -> Option<&'static str> {
        match self {
            WeierstrassCurveKind::NistP256 => Some("secp256r1"),
            WeierstrassCurveKind::Secp256k1 => Some("secp256k1"),
            _ => None,
        }
    }
}

/// Digest input type for a particular Weierstrass curve
#[cfg(feature = "digest")]
pub type CurveDigest<C> = GenericArray<u8, <C as WeierstrassCurve>::PrivateScalarSize>;