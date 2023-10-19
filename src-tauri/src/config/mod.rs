pub mod consts;

use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use sha2::{Digest, Sha256};
use tauri::api::path::document_dir;

use crate::tls;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
    pub static ref PROGRAM_ID: Mutex<String> = Mutex::new("".to_string());
}

#[tauri::command]
pub async fn input_password(password: String) -> Result<(), String> {
    let mut config = CONFIG.lock().unwrap();
    config
        .decrypt_config(password.as_str())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_password(password: String) -> Result<(), String> {
    let mut config = CONFIG.lock().unwrap();

    config
        .set_password(password.as_str())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn has_password() -> Result<bool, String> {
    let config = CONFIG.lock().unwrap();

    config.has_password().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn try_password() -> Result<bool, String> {
    let mut config = CONFIG.lock().unwrap();
    if config.password.is_some() {
        return Ok(true);
    }
    Ok(config.decrypt_config("").is_ok())
}

#[derive(Clone)]
pub struct Config {
    pub db: Option<Arc<rocksdb::DB>>,
    pub password: Option<String>,
}

const PASSWORD_TEST: &str = "hello world!";
const PASSWORD_TEST_KEY: &str = "password_test";
impl Config {
    pub fn new() -> Self {
        let db = match Self::create_db() {
            Ok(v) => v,
            Err(e) => {
                log::error!("cant create config database: {:#?}", e);
                return Self {
                    db: None,
                    password: None,
                };
            }
        };
        Self {
            db: Some(Arc::new(db)),
            password: None,
        }
    }

    pub fn decrypt_config(&mut self, password: &str) -> Result<()> {
        let db = self.db.clone().context("cant get db")?;
        let pass_test = db.get(PASSWORD_TEST_KEY)?.context("password not set")?;
        match password {
            "" => {
                if pass_test != PASSWORD_TEST.as_bytes() {
                    return Err(anyhow!("password is wrong"));
                }
            }
            _ => {
                let encrypt_key = hash(password);
                let decrypted = tls::aes::aes_decode(&encrypt_key, &pass_test)?;
                if decrypted != PASSWORD_TEST.as_bytes() {
                    return Err(anyhow!("password is wrong"));
                }
            }
        };
        self.password = Some(password.to_string());
        Ok(())
    }

    pub fn set_password(&mut self, password: &str) -> Result<()> {
        let db = self.db.clone().context("cant get db")?;
        self.password = Some(password.to_string());
        self.create_secret_key().context("create server secret")?;
        match password {
            "" => {
                db.put(PASSWORD_TEST_KEY, PASSWORD_TEST)?;
            }
            _ => {
                let encrypt_key = hash(password);
                let encrypted = tls::aes::aes_encode(&encrypt_key, PASSWORD_TEST.as_bytes())?;
                db.put(PASSWORD_TEST_KEY, &encrypted)?;
            }
        };

        Ok(())
    }

    pub fn has_password(&self) -> Result<bool> {
        let db = self.db.clone().context("cant get db")?;
        Ok(db.key_may_exist(PASSWORD_TEST_KEY))
    }

    pub fn get_config() -> Config {
        let config = CONFIG.lock().unwrap();
        config.clone()
    }

    fn create_db() -> Result<rocksdb::DB> {
        let dbdir = document_dir()
            .context("cant find document_dir!")?
            .join("aleo-acc-service");
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        let db = rocksdb::DB::open(&opts, dbdir).context("cant open db")?;
        Ok(db)
    }

    pub fn create_secret_key(&self) -> Result<Vec<u8>> {
        let db = self.db.clone().context("cant get db")?;
        let secret_key = tls::generate_p256_secret().context("generate secret")?;

        match &self.password {
            Some(v) => {
                if v == "" {
                    db.put("secret_key", &secret_key)
                        .context("cant write to db")?;
                } else {
                    let encrypt_key = hash(v);
                    let encrypted = tls::aes::aes_encode(&encrypt_key, &secret_key)?;
                    db.put("secret_key", &encrypted)
                        .context("cant write to db")?;
                }
            }

            None => {
                return Err(anyhow!("no password set!"));
            }
        }

        Ok(secret_key)
    }

    pub fn get_secret_key(&self) -> Result<Vec<u8>> {
        let db = self.db.clone().context("cant get db")?;
        let value = db
            .get("secret_key")
            .context("cant read db")?
            .context("secret not set!")?;

        match self
            .password
            .clone()
            .context("database not decrypted")?
            .as_ref()
        {
            "" => return Ok(value),
            _ => {
                let encrypt_key = hash(self.password.clone().context("no password")?.as_ref());
                let decrypted = tls::aes::aes_decode(&encrypt_key, &value)?;
                return Ok(decrypted);
            }
        }
    }
}

pub fn hash(str: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(str);
    let result = hasher.finalize();
    result.to_vec()
}
