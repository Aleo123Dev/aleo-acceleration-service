pub mod consts;

//TODO: config
pub struct Config {
    pub password: Option<String>,
    pub db: rocksdb::DB,
}
