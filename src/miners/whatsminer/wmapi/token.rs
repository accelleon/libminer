use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Sha256, Digest};
use openssl::symm::{Cipher, Crypter, Mode};
use base64;
use chrono::{Duration, Utc, DateTime};

use crate::error::Error;
use crate::util::md5::do_md5_crypt;

#[derive(Deserialize, Debug)]
pub struct TokenData {
    pub salt: String,
    pub time: String,
    pub newsalt: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    #[serde(rename = "STATUS")]
    pub status: super::StatusCode,
    #[serde(rename = "When")]
    pub when: usize,
    #[serde(rename = "Code")]
    pub code: usize,
    #[serde(rename = "Msg")]
    pub msg: Option<TokenData>,
    #[serde(rename = "Description")]
    pub description: String,
}

impl TokenResponse {
    /// Make an from a given get_token response and a password
    pub fn make_token(self, password: &str) -> Result<WhatsminerToken, Box<dyn std::error::Error>> {
        /*
         * key = md5_crypt.hash(passwd, salt=token_msg.salt).split('$')[1]
         * aeskey = unhexlify(sha256(key.encode()).hexdigest().encode())
         * self.cipher = AES.new(aeskey, AES.MODE_ECB)
         * self.token = md5_crypt.hash(key + token_msg.time, salt=token_msg.newsalt).split('$')[3]
         * self.expires = datetime.now() + timedelta(minutes=30)
         */
        if let Some(token) = self.msg {
            let hashkey = do_md5_crypt(password.as_bytes(), &token.salt)?;
            let key = hashkey.split('$').nth(3).unwrap();
            let mut sha256 = Sha256::new();
            sha256.update(&key);
            let aeskey = sha256.finalize_reset().to_vec();
            
            let hashkey2 = do_md5_crypt((key.to_string() + &token.time).as_bytes(), &token.newsalt)?;
            let token = hashkey2.split('$').nth(3).unwrap();
            Ok(WhatsminerToken{
                token: token.to_string(),
                cipher: aeskey,
                expires: Utc::now() + Duration::minutes(30),
            })
        } else {
            Err(Box::new(Error::ApiCallFailed("Failed to get token".into())))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhatsminerToken {
    token: String,
    pub expires: DateTime<Utc>,
    cipher: Vec<u8>,
}

impl WhatsminerToken {
    #[inline]
    pub fn is_expired(&self) -> bool {
        self.expires < Utc::now()
    }

    /// Decrypt a given JSON using the token and return the writable JSON
    pub fn encrypt(&self, data: &serde_json::Value) -> Result<serde_json::Value, Error> {
        if self.is_expired() {
            return Err(Error::TokenExpired);
        }
        // Pad serialized data to a multiple of 16 bytes
        let mut data = data.to_string().as_bytes().to_vec();
        let padding = 16 - (data.len() % 16);
        data.extend(vec![0; padding]);
        // Encrypt at a lower level, we don't want padding
        let aes = Cipher::aes_256_ecb();
        let mut crypter = Crypter::new(aes, Mode::Encrypt, &self.cipher, None).map_err(|_| Error::EncodingError)?;
        crypter.pad(false);
        let mut out = vec![0; data.len() + aes.block_size()];
        let count = crypter.update(&data, &mut out).map_err(|_| Error::EncodingError)?;
        let res = crypter.finalize(&mut out[count..]).map_err(|_| Error::EncodingError)?;
        out.truncate(count + res);
        // Base64 encode the result
        let b64 = base64::encode(&out);
        Ok(json!({
            "enc": 1,
            "data": b64,
        }))
    }

    pub fn decrypt(&self, data: &serde_json::Value) -> Result<serde_json::Value, Error> {
        if let Some(data) = data.get("enc") {
            let data = base64::decode(data.as_str().unwrap().as_bytes()).map_err(|_| Error::EncodingError)?;
            // Decrypt at a lower level as we need to disable padding
            let aes = Cipher::aes_256_ecb();
            let mut crypter = Crypter::new(aes, Mode::Decrypt, &self.cipher, None).map_err(|_| Error::EncodingError)?;
            crypter.pad(false);
            let mut out = vec![0; data.len() + aes.block_size()];
            let count = crypter.update(&data, &mut out).map_err(|_| Error::EncodingError)?;
            let rest = crypter.finalize(&mut out[count..]).map_err(|_| Error::EncodingError)?;
            out.truncate(count + rest);
            let mut out = String::from_utf8(out).map_err(|_| Error::EncodingError)?;
            // Trim null padding
            out = out.trim_end_matches('\0').to_string();
            Ok(serde_json::from_str(&out).map_err(|_| Error::EncodingError)?)
        } else {
            Err(Error::ExpectedReturn)
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }
}
