use std::ops::Deref;

use jwt_simple::prelude::*;

use crate::{models::User, AppError};

/// 有效期 7 天
const JWT_DURATION: u32 = 60 * 60 * 24 * 7;

const JWT_ISSUER: &'static str = "chat_server";
const JWT_AUDIENCE: &'static str = "chat_web";

/// 私钥: 用于生成签名
pub struct EncodingKey(Ed25519KeyPair);

/// 公钥: 用于验证签名
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    /// 从pem文件加载私钥
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    /// 生成签名
    pub fn sign(&self, user: User) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user, Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISSUER).with_audience(JWT_AUDIENCE);
        self.0.sign(claims)
    }
}

impl DecodingKey {
    /// 从pem文件加载公钥
    pub fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    /// 验证签名
    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let mut options = VerificationOptions::default();
        options.allowed_issuers = Some(HashSet::from_strings(&[JWT_ISSUER]));
        options.allowed_audiences = Some(HashSet::from_strings(&[JWT_AUDIENCE]));

        let claims = self.0.verify_token::<User>(token, Some(options))?;
        Ok(claims.custom)
    }
}

impl Deref for EncodingKey {
    type Target = Ed25519KeyPair;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DecodingKey {
    type Target = Ed25519PublicKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
