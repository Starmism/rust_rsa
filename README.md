# rust_rsa

## About this repo
This is my Rust implementation of the RSA encryption standard, based on [this book](https://www.amazon.com/Introduction-Algorithms-3rd-MIT-Press/dp/0262033844).\
This is for my CS 3000 - Advanced Algorithms & Data Structures class at SUU, taught by Professor Nathan Barker in the Spring 2022 semester.

## How to use

### Prerequisities
A Rust compiler that supports version `2021`.
Having `e.txt` in the same working directory as the binary, once built. Should be a small-ish prime number, `65537` is a great starter.\
Having `message.txt` in the same working directory as the binary, once built. Should be a message you wish to encode!

### Building and Testing
Just run `cargo test` to ensure everything works as intended, and then run `cargo build --release` to get a binary.\
Once you have this binary, a command line interface is exposed.

### CLI
Running anything other than one of three preset commands will give you the help screen. Here it is now:\
```
Help for RSA
List of possible args: [gen, encrypt, decrypt]
gen -> Generates and writes `d.txt` and `n.txt` to the current directory. Takes A WHILE even on good computers.
encrypt -> Takes `message.txt`, `e.txt`, and `n.txt` and encrypts the message, outputting `encryptedMessage.txt`.
decrypt -> Takes `encryptedMessage.txt`, `d.txt`, and `n.txt` and decrypts the message, printing it out in the console.
```

### Using it to encrypt then decrypt a message
You basically just go in the order of CLI args.\
First, you `gen` your public and private keys. Note that this took about 2 minutes on my decently-powered computer, as it has to keep randomly trying until it gets a prime.\
Next, you have it encrypt your `message.txt` using `encrypt` and it spits out `encryptedMessage.txt`.\
Finally, you run `decrypt` to decrypt `encryptedMessage.txt` and output it to your console!
