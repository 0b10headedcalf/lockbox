//debug
// #![allow(warnings)]
use std::{fs};
use std::path::{PathBuf};
use toml::Table;
use aes_gcm::{Aes256Gcm,KeyInit, aead::Aead};
use clap::{Parser, Subcommand};
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use zeroize::{Zeroize};

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
        key: String,
        u_nonce: String

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


fn lock(target:PathBuf, key:&[u8;32], nonce_bytes:&[u8;12]) -> std::io::Result<()>{
    println!("locking {:#?}",target);
    let data = fs::read(&target)?;
    let mut new_path = PathBuf::from(&target);
    new_path.set_extension("lckbx");
    let cipher = Aes256Gcm::new(key.into());
    let encrypted_file = cipher
        .encrypt(nonce_bytes.into(), data.as_ref())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    
    println!("Encryption of {:?} succesful!", target);
    fs::write(new_path, encrypted_file)?;
    fs::remove_file(target)?;
    Ok(())
}

fn unlock(target:PathBuf, key_str: String, nonce_str: String) -> std::io::Result<()>{
    println!("Unlocking {:#?}",target);
    let data = fs::read(&target)?;
    let mut new_path = PathBuf::from(target);
    new_path.set_extension("md");
    let mut key_bytes = [0u8;32];
    let mut nonce_bytes = [0u8;12];
    hex::decode_to_slice(key_str, &mut key_bytes).ok();
    hex::decode_to_slice(nonce_str, &mut nonce_bytes).ok();
    let cipher = Aes256Gcm::new(&key_bytes.into());

    let decrypted_file = cipher
        .decrypt(&nonce_bytes.into(), data.as_ref())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    println!("Decryption of {:?} succesful!", new_path);
    fs::write(new_path, decrypted_file)?;
    Ok(())
}

fn main() {
    let config = std::fs::read_to_string("config.toml")
        .expect("config not found");
    
    let mut message_key = generate_key();
    let mut nonce = generate_nonce();
    
    let cli = Cli::parse();

    match cli.command{
        Commands::Lock{target, recipient} => {
            lock(target, &message_key, &nonce).ok();
            send_email(config, hex::encode(message_key), hex::encode(nonce), recipient).ok();
            message_key.zeroize();
            nonce.zeroize();
        }
        Commands::Unlock { target, key, u_nonce} => {
            unlock(target, key, u_nonce).ok();
        }
    }
    

}
