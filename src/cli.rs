use std::path::PathBuf;

const DEF_HOST: &str = "localhost"; 
const DEF_PORT: &str = "8000"; 
const DEF_ASSETS: &str = "dist"; 
pub enum Flags {
    HOST,
    PORT,
    INDEX,
    ASSETS
}

pub struct Cli<'c> {
    pub host: &'c str,
    pub port: &'c str,
    pub address: String,
    pub index: PathBuf, 
}

impl<'c> Default for Cli<'c> {
    fn default() -> Self {
        Self {
            host: DEF_HOST,
            port: DEF_PORT,
            address: format!("{DEF_HOST}:{DEF_PORT}"),
            index: PathBuf::from(DEF_ASSETS), 
        }
    }
}


