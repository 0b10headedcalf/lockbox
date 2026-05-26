//debug
#![allow(warnings)]

use core::fmt;
use std::ffi::c_void;
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf};
use toml::Table;
// use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use clap::{Parser, Subcommand};
// use aes_gcm::aead;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};

//CLI Setup
#[derive(Parser)]
#[command(name = "lockbox", about = "Lock a file and hand off the ability to unlock it to someone else.", version)]
struct Cli{

    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, value_name = "FILE")]
    target: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands{
    Lock{
        #[arg(short,long)]
        target: PathBuf,
        #[arg(short,long)]
        recipient: String

    },
    Unlock{
        #[arg(short,long)]
        target: PathBuf,
        key: String

    },
    
}



// functions 

// mail protocol
#[tokio::main]
async fn send_email(config: String, message_key: String, nonce: String, recipient: String) -> Result<()>{
    let parsed_config = config.parse::<Table>().unwrap();
    let api_key = parsed_config["mail"]["RESEND_KEY"].as_str().unwrap();

    let subject = "Lockbox Key";
    let body = format!(
    "Key: {}\n
     Nonce: {}
     Please keep both of these in a secure location.", message_key, nonce);

    let resend = Resend::new(api_key);
    
    let from = "lockbox@resend.dev";
    let to = [recipient];

    let email = CreateEmailBaseOptions::new(from,to,subject)
        .with_text(&body);

    let _email = resend.emails.send(email).await?;

    Ok(())
}

//generate 256 bit encryption key
fn generate_key() -> [u8;32]{
    let mut key = [0u8;32]; //key is 32 bytes(256 bits)
    rand::fill(&mut key);
    return key; 
}

fn generate_nonce() -> [u8;12]{    
    let mut nonce = [0u8;12];
    rand::fill(&mut nonce);
    return nonce;
}


fn lock(target:PathBuf, key:[u8;32]){
    println!("locking {:#?}",target);
    todo!();
}

fn unlock(target:PathBuf, key: String){
    println!("Unlocking {:#?}",target);
    todo!();
}


fn main() {
    let config = std::fs::read_to_string("config.toml")
        .expect("config not found");
    
    let message_key = hex::encode(generate_key());
    let nonce = hex::encode(generate_nonce());
    
    let cli = Cli::parse();

    match cli.command{
        Commands::Lock{target, recipient} => {
            send_email(config, message_key, nonce, recipient);
            lock_target(target);
        }
        Commands::Unlock { target, key } => {
            unlock_target(target, key);
        }
    }
    

}
