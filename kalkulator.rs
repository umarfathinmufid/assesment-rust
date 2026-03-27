use std::io::{self, Write};

fn main() {
    println!("=== KALKULATOR SEDERHANA ===");
    println!("Operasi yang didukung: +, -, *, /");
    println!("Ketik 'exit' untuk keluar.\n");

    loop {
        print!("Masukkan ekspresi (contoh: 5 + 3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        let input = input.trim();

        if input.to_lowercase() == "exit" {
            println!("Terima kasih telah menggunakan kalkulator!");
            break;
        }

        match hitung(input) {
            Ok(hasil) => println!("Hasil: {}", hasil),
            Err(pesan) => println!("Error: {}", pesan),
        }
    }
}

fn hitung(ekspresi: &str) -> Result<f64, String> {
    let bagian: Vec<&str> = ekspresi.split_whitespace().collect();
    
    if bagian.len() != 3 {
        return Err("Format ekspresi salah. Gunakan: angka operator angka".to_string());
    }

    let a = bagian[0].parse::<f64>().map_err(|_| "Angka pertama tidak valid".to_string())?;
    let op = bagian[1];
    let b = bagian[2].parse::<f64>().map_err(|_| "Angka kedua tidak valid".to_string())?;

    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Tidak bisa membagi dengan nol".to_string())
            } else {
                Ok(a / b)
            }
        },
        _ => Err("Operator tidak didukung. Gunakan +, -, *, atau /".to_string()),
    }
}