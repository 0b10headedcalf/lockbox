# Lockbox

<pre align="center>                                               
  ▄▄▄                                          
 ▀██▀                        █▄                
  ██                  ▄▄     ██                
  ██      ▄███▄ ▄███▀ ██ ▄█▀ ████▄ ▄███▄▀██ ██▀
  ██      ██ ██ ██    ████   ██ ██ ██ ██  ███  
 ████████▄▀███▀▄▀███▄▄██ ▀█▄▄████▀▄▀███▀▄██ ██▄
</pre>                            
                                               

Encrypt a file with AES-256-GCM and email the key + nonce to a recipient. The file is then deleted and key+nonce zeroed in memory. The onus is then on the recipient to provide you with the ability to decrypt your file. Currently supports markdown as format as this tool was originally intended to stop myself from accessing and ruminating over old letters/files in my Obsidian vaults.

There is a sample file to decrypt in the repo and the keys+nonce are provided to test the tool.

## Build

```sh
cargo build --release
```

## Configure

Copy `config.toml.example` to `config.toml` and add your [Resend](https://resend.com) API key:

```toml
[mail]
RESEND_KEY = "re_xxxxxxxxxxxxx"
```

This is done for developer experience and ease of use but setting up an SMTP send protocol in Rust is not very difficult either, there's some really good tutorials online and the lettr crate also makes it rather simple.

## Usage

### Lock

Encrypts `<file>` to `<file>.lckbx` and emails the key + nonce to the recipient.

```sh
lockbox lock --target secret.md --recipient trusteduser@example.com
```

### Unlock

Decrypts `<file>.lckbx` back to `<file>.md` using the key and nonce.

```sh
lockbox unlock --target secret.md.lckbx <key> <nonce>
```

## How it works

- Generates a random 256-bit key and 96-bit nonce per lock.
- Encrypts the file with AES-256-GCM (tampering fails decrypt).
- Sends key and nonce (hex-encoded) to the recipient via Resend.
- Zeroes the key in memory after sending.

## Issues

- Key and nonce travel the same channel (email). Security depends on the recipient's inbox.
- Sender currently hardcoded to `lockbox@resend.dev` due to API use.
- Unlock always writes output to markdown.
