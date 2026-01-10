use sui_types::crypto::SignatureScheme as SuiSignatureScheme;

pub enum SignatureScheme {
    BLS12381,
    ED25519,
    MultiSig,
    Secp256k1,
    Secp256r1,
    ZkLoginAuthenticator,
    PasskeyAuthenticator,
}

impl From<SignatureScheme> for SuiSignatureScheme {
    fn from(scheme: SignatureScheme) -> Self {
        match scheme {
            SignatureScheme::BLS12381 => SuiSignatureScheme::BLS12381,
            SignatureScheme::ED25519 => SuiSignatureScheme::ED25519,
            SignatureScheme::MultiSig => SuiSignatureScheme::MultiSig,
            SignatureScheme::Secp256k1 => SuiSignatureScheme::Secp256k1,
            SignatureScheme::Secp256r1 => SuiSignatureScheme::Secp256r1,
            SignatureScheme::ZkLoginAuthenticator => SuiSignatureScheme::ZkLoginAuthenticator,
            SignatureScheme::PasskeyAuthenticator => SuiSignatureScheme::PasskeyAuthenticator,
        }
    }
}
