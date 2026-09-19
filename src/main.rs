mod math;
mod rsa;

use clap::Parser;
use num_bigint::BigInt;

#[derive(Parser, Debug)]
#[command(author, version, about = "Educational RSA step-by-step computation tracer", long_about = None)]

struct Args {
    #[arg(long, default_value = "61")]
    p: String,

    #[arg(long, default_value = "53")]
    q: String,

    #[arg(long, default_value = "17")]
    e: String,

    #[arg(long, default_value = "65")]
    msg: String,
}

fn main() {
    let args = Args::parse();

    let p = match args.p.parse::<BigInt>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Failed to parse 'p' as a big integer.");
            std::process::exit(1);
        }
    };

    let q = match args.q.parse::<BigInt>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Failed to parse 'q' as a big integer.");
            std::process::exit(1);
        }
    };

    let e = match args.e.parse::<BigInt>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Failed to parse 'e' as a big integer.");
            std::process::exit(1);
        }
    };

    let msg = match args.msg.parse::<BigInt>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Failed to parse 'msg' as a big integer.");
            std::process::exit(1);
        }
    };

    println!("[INFO] Starting RSA Key Generation & Calculation Trace");

    println!("[STEP 1] Modulus & Totient Calculation");
    println!("  p = {}, q = {}", p, q);
    let n = &p * &q;
    let one = BigInt::from(1);
    let phi = (&p - &one) * (&q - &one);
    println!("  Modulus n = p * q = {}", n);
    println!("  Totient phi(n) = (p - 1) * (q - 1) = {}", phi);

    println!("\n[STEP 2] Public Exponent Verification");
    println!("  Chosen e = {}", e);
    let g = math::gcd(&e, &phi);
    println!("  gcd(e, phi(n)) = {}", g);
    if g != one {
        eprintln!("[ERROR] Public exponent e is not coprime to phi(n).");
        std::process::exit(1);
    } else {
        println!("  [OK] e and phi(n) are coprime.");
    }

    println!("\n[STEP 3] Private Exponent Derivation");
    println!("  Computing d = e^(-1) mod phi(n)...");
    let d = match math::mod_inverse(&e, &phi) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("[ERROR] {}", err);
            std::process::exit(1);
        }
    };
    println!("  Private Exponent d = {}", d);

    println!("\n[STEP 4] Message Encryption");
    println!("  Plaintext message m = {}", msg);
    if msg >= n {
        eprintln!("[WARNING] Message is larger than or equal to modulus n.");
    }
    let ciphertext = rsa::encrypt(&msg, &e, &n);
    println!("  Ciphertext c = m^e mod n = {}", ciphertext);

    println!("\n[STEP 5] Message Decryption");
    println!("  Ciphertext c = {}", ciphertext);
    let decrypted = rsa::decrypt(&ciphertext, &d, &n);
    println!("  Plaintext m = c^d mod n = {}", decrypted);

    if msg == decrypted {
        println!("\n[SUCCESS] Decrypted message matches original plaintext.");
    } else {
        println!("\n[FAILURE] Decrypted message does not match original plaintext.");
        std::process::exit(1);
    }
}
