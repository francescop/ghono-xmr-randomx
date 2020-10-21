use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientConfig {
    pub address: String,
    pub login: String,
    pub pass: String,
    pub keepalive_s: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RandomXConfig {
    pub cores: Vec<u32>,
    pub hard_aes: bool,
    pub jit: bool,
    pub argon2_avx2: bool,
    pub full_mem: bool,
    pub large_pages: bool,
    pub argon2_ssse3: bool,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pool: ClientConfig,
    pub randomx: RandomXConfig,
}
