//! OCSP Request

use crate::{ext::Nonce, CertId, Version};
use alloc::vec::Vec;
use const_oid::db::rfc6960::ID_PKIX_OCSP_NONCE;
use core::{default::Default, option::Option};
use der::{asn1::BitString, Decode, Sequence};
use spki::AlgorithmIdentifierOwned;
use x509_cert::{
    certificate::Certificate,
    ext::{pkix::name::GeneralName, Extensions},
};

/// OCSPRequest structure as defined in [RFC 6960 Section 4.1.1].
///
/// ```text
/// OCSPRequest ::= SEQUENCE {
///    tbsRequest              TBSRequest,
///    optionalSignature   [0] EXPLICIT Signature OPTIONAL }
/// ```
///
/// [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1
#[derive(Clone, Debug, Eq, PartialEq, Sequence)]
#[allow(missing_docs)]
pub struct OcspRequest {
    pub tbs_request: TbsRequest,

    #[asn1(context_specific = "0", optional = "true", tag_mode = "EXPLICIT")]
    pub optional_signature: Option<Signature>,
}

impl OcspRequest {
    /// Returns the request's nonce value, if any. This method will return `None` if the request
    /// has no `Nonce` extension or decoding of the `Nonce` extension fails.
    pub fn nonce(&self) -> Option<Nonce> {
        self.tbs_request.nonce()
    }
}

/// TBSRequest structure as defined in [RFC 6960 Section 4.1.1].
///
/// ```text
/// TBSRequest ::= SEQUENCE {
///    version             [0] EXPLICIT Version DEFAULT v1,
///    requestorName       [1] EXPLICIT GeneralName OPTIONAL,
///    requestList             SEQUENCE OF Request,
///    requestExtensions   [2] EXPLICIT Extensions OPTIONAL }
/// ```
///
/// [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1
#[derive(Clone, Debug, Eq, PartialEq, Sequence)]
#[allow(missing_docs)]
pub struct TbsRequest {
    #[asn1(
        context_specific = "0",
        default = "Default::default",
        tag_mode = "EXPLICIT"
    )]
    pub version: Version,

    #[asn1(context_specific = "1", optional = "true", tag_mode = "EXPLICIT")]
    pub requestor_name: Option<GeneralName>,

    pub request_list: Vec<Request>,

    #[asn1(context_specific = "2", optional = "true", tag_mode = "EXPLICIT")]
    pub request_extensions: Option<Extensions>,
}

impl TbsRequest {
    /// Returns the request's nonce value, if any. This method will return `None` if the request
    /// has no `Nonce` extension or decoding of the `Nonce` extension fails.
    pub fn nonce(&self) -> Option<Nonce> {
        match &self.request_extensions {
            Some(extns) => {
                let mut filter = extns.iter().filter(|e| e.extn_id == ID_PKIX_OCSP_NONCE);
                match filter.next() {
                    Some(extn) => Nonce::from_der(extn.extn_value.as_bytes()).ok(),
                    None => None,
                }
            }
            None => None,
        }
    }
}

/// Signature structure as defined in [RFC 6960 Section 4.1.1].
///
/// ```text
/// Signature ::= SEQUENCE {
///    signatureAlgorithm      AlgorithmIdentifier,
///    signature               BIT STRING,
///    certs                  [0] EXPLICIT SEQUENCE OF Certificate OPTIONAL }
/// ```
///
/// [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1
#[derive(Clone, Debug, Eq, PartialEq, Sequence)]
#[allow(missing_docs)]
pub struct Signature {
    pub signature_algorithm: AlgorithmIdentifierOwned,
    pub signature: BitString,

    #[asn1(context_specific = "0", optional = "true", tag_mode = "EXPLICIT")]
    pub certs: Option<Vec<Certificate>>,
}

/// Request structure as defined in [RFC 6960 Section 4.1.1].
///
/// ```text
/// Request ::= SEQUENCE {
///    reqCert                     CertID,
///    singleRequestExtensions     [0] EXPLICIT Extensions OPTIONAL }
/// ```
///
/// [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1
#[derive(Clone, Debug, Eq, PartialEq, Sequence)]
#[allow(missing_docs)]
pub struct Request {
    pub req_cert: CertId,

    #[asn1(context_specific = "0", optional = "true", tag_mode = "EXPLICIT")]
    pub single_request_extensions: Option<Extensions>,
}
