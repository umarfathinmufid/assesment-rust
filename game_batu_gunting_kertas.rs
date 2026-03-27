use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("=== GAME BATU GUNTING KERTAS ===");
    println!("Pilih: batu, gunting, atau kertas");
    println!("Ketik 'exit' untuk keluar.\n");

    let pilihan = ["batu", "gunting", "kertas"];

    loop {
        print!("Pilihan Anda: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        let input = input.trim().to_lowercase();

        if input == "exit" {
            println!("Terima kasih telah bermain!");
            break;
        }

        if !pilihan.contains(&input.as_str()) {
            println!("Pilihan tidak valid. Pilih batu, gunting, atau kertas.\n");
            continue;
        }

        // Pilihan komputer random sederhana
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as usize;
        let komputer = pilihan[seed % 3];

        println!("Komputer memilih: {}", komputer);

        let hasil = tentukan_pemenang(&input, komputer);
        println!("{}\n", hasil);
    }
}

fn tentukan_pemenang(player: &str, komputer: &str) -> String {
    if player == komputer {
        "Seri!".to_string()
    } else if (player == "batu" && komputer == "gunting") ||
              (player == "gunting" && komputer == "kertas") ||
              (player == "kertas" && komputer == "batu") {
        "Anda menang!".to_string()
    } else {
        "Komputer menang!".to_string()
    }
}