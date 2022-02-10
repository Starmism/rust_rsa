mod key_genner;
mod utils;

use std::{env, fs};
use num_bigint::{BigInt, Sign};
use crate::key_genner::gen_and_write;
use crate::utils::modular_exponentiation;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "gen" => gen_and_write(),
        "encrypt" => {
            let message = fs::read_to_string("./message.txt").expect("Could not read message.txt");
            let e = fs::read_to_string("./e.txt").expect("Could not read e.txt");
            let e = str::parse::<BigInt>(&e).expect("Could not convert e to BigInt");
            let n = fs::read_to_string("./n.txt").expect("Could not read n.txt");
            let n = str::parse::<BigInt>(&n).expect("Could not convert n to BigInt");
            let encrypted_message = encrypt(message, e, n);
            fs::write("./encryptedMessage.txt", encrypted_message).expect("Could not write encryptedMessage.txt");
            println!("Message successfully encrypted!");
        },
        "decrypt" => {
            let encrypted_message = fs::read("./encryptedMessage.txt").expect("Could not read encryptedMessage.txt");
            let d = fs::read_to_string("./d.txt").expect("Could not read d.txt");
            let d = str::parse::<BigInt>(&d).expect("Could not convert d to BigInt");
            let n = fs::read_to_string("./n.txt").expect("Could not read n.txt");
            let n = str::parse::<BigInt>(&n).expect("Could not convert n to BigInt");
            println!("{}", decrypt(encrypted_message, d, n));
        },
        _ => {
            println!("Help for RSA");
            println!("List of possible args: [gen, encrypt, decrypt]");
            println!("gen -> Generates and writes `d.txt` and `n.txt` to the current directory. Takes A WHILE even on good computers.");
            println!("encrypt -> Takes `message.txt`, `e.txt`, and `n.txt` and encrypts the message, outputting `encryptedMessage.txt`.");
            println!("decrypt -> Takes `encryptedMessage.txt`, `d.txt`, and `n.txt` and decrypts the message, printing it out in the console.");
        }
    }
}

fn encrypt(message: String, e: BigInt, n: BigInt) -> Vec<u8> {
    modular_exponentiation(BigInt::from_bytes_le(Sign::Plus, message.as_bytes()), e, n).to_signed_bytes_le()
}

fn decrypt(ciphertext: Vec<u8>, d: BigInt, n: BigInt) -> String {
    String::from_utf8(modular_exponentiation(BigInt::from_bytes_le(Sign::Plus, &ciphertext), d, n).to_signed_bytes_le()).expect("Error decrypting ciphertext.")
}

#[test]
fn test_barker_cryption() {
    use std::str::FromStr;
    let test_string = String::from("Now this is the kind of iron oxide I'm talking about!");
    let encryption = encrypt(test_string.clone(),
                             BigInt::from_str("65537").unwrap(),
                             BigInt::from_str("14931823167293929236223177684374184515136992234755488826951468395109113501101304516998668362165719343372106967334250694839718724443572508041949473758390652263990639435390634416880126951778717653345208347572103653655972934581128879105316217301422711546019112749439273130716783984471041382343896641040543055537898403369241189094829012821210799245011795962327313713627911528770053912398648739161912301667222256038164537822906602005257263275428796823783275759313456555640232837834657105163583854840700043276834789983820908923090318606142621667390456623339356140346686668637128955155935613475792152616029280551356631118863").unwrap()
    );
    let decryption = decrypt(encryption,
    BigInt::from_str("5808503590750132473995967039770442040815149915816354611201317351494754712560487317948091173307488120297366297285798666619524683170795695866784562674464978391110019710481847411751254352635101207781600341990069892523389444042469134136608810959953470683037234851064357983955531850711883806116778323186712920302682226319760181252786848116792458118650001376715039438483815636679926124746479699856546069936706268564892008467952781146096151291632793067334616739840398442551227073612889821477463894492922039776436335306983051853815773723723990663209720281074976327596651429202547688050439896228852167186346324668367293489889").unwrap()
    ,
    BigInt::from_str("14931823167293929236223177684374184515136992234755488826951468395109113501101304516998668362165719343372106967334250694839718724443572508041949473758390652263990639435390634416880126951778717653345208347572103653655972934581128879105316217301422711546019112749439273130716783984471041382343896641040543055537898403369241189094829012821210799245011795962327313713627911528770053912398648739161912301667222256038164537822906602005257263275428796823783275759313456555640232837834657105163583854840700043276834789983820908923090318606142621667390456623339356140346686668637128955155935613475792152616029280551356631118863").unwrap()
    );
    assert_eq!(test_string, decryption);
}
