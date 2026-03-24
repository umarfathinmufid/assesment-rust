use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    
    loop {
        println!("\n=== TO-DO LIST ===");
        println!("1. Tambah tugas");
        println!("2. Lihat tugas");
        println!("3. Hapus tugas");
        println!("4. Keluar");
        print!("Pilih menu (1-4): ");
        io::stdout().flush().unwrap();
        
        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca input");
        let pilihan = pilihan.trim();
        
        match pilihan {
            "1" => tambah_tugas(&mut tasks),
            "2" => lihat_tugas(&tasks),
            "3" => hapus_tugas(&mut tasks),
            "4" => {
                println!("Terima kasih!");
                break;
            },
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn tambah_tugas(tasks: &mut Vec<String>) {
    print!("Masukkan tugas: ");
    io::stdout().flush().unwrap();
    
    let mut tugas = String::new();
    io::stdin().read_line(&mut tugas).expect("Gagal membaca input");
    tasks.push(tugas.trim().to_string());
    
    println!("✓ Tugas ditambahkan!");
}

fn lihat_tugas(tasks: &Vec<String>) {
    if tasks.is_empty() {
        println!("\nTidak ada tugas.");
    } else {
        println!("\n--- DAFTAR TUGAS ---");
        for (i, tugas) in tasks.iter().enumerate() {
            println!("{}. {}", i + 1, tugas);
        }
    }
}

fn hapus_tugas(tasks: &mut Vec<String>) {
    if tasks.is_empty() {
        println!("\nTidak ada tugas untuk dihapus.");
        return;
    }
    
    lihat_tugas(tasks);
    print!("\nMasukkan nomor tugas yang dihapus: ");
    io::stdout().flush().unwrap();
    
    let mut nomor = String::new();
    io::stdin().read_line(&mut nomor).expect("Gagal membaca input");
    
    match nomor.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= tasks.len() => {
            let tugas_dihapus = tasks.remove(n - 1);
            println!("✓ Tugas '{}' dihapus!", tugas_dihapus);
        },
        _ => println!("Nomor tidak valid!"),
    }
}
