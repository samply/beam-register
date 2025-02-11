use std::net::SocketAddr;
use std::path::PathBuf;
use clap::Parser;


#[derive(Debug, Parser)]
pub struct Config {
   
    #[clap(long, env, default_value = "0.0.0.0:3000")]
    pub bind_addr: SocketAddr,

    #[clap(long, env, default_value = "APP_{}_KEY")]
    pub beam_app_key_format: String,

    #[clap(long, env)]
    pub api_key: String,

    #[clap(long, env)]
    pub beam_file_path: PathBuf,
    
    #[clap(long, env)]
    pub beam_file_change_check_cron_expression: String,
    
    #[clap(long, env)]
    pub beam_proxy_container_name: String,
    
}

