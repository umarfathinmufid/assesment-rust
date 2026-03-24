use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== KONVERSI SUHU ===");
        println!("1. Celsius ke Fahrenheit");
        println!("2. Fahrenheit ke Celsius");
        println!("3. Celsius ke Kelvin");
        println!("4. Kelvin ke Celsius");
        println!("5. Keluar");
        print!("Pilih menu (1-5): ");
        io::stdout().flush().unwrap();
        
        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca input");
        let pilihan = pilihan.trim();
        
        match pilihan {
            "1" => {
                let celsius = baca_suhu("Celsius");
                let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
                println!("{}°C = {:.2}°F", celsius, fahrenheit);
            },
            "2" => {
                let fahrenheit = baca_suhu("Fahrenheit");
                let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                println!("{}°F = {:.2}°C", fahrenheit, celsius);
            },
            "3" => {
                let celsius = baca_suhu("Celsius");
                let kelvin = celsius + 273.15;
                println!("{}°C = {:.2}K", celsius, kelvin);
            },
            "4" => {
                let kelvin = baca_suhu("Kelvin");
                let celsius = kelvin - 273.15;
                println!("{}K = {:.2}°C", kelvin, celsius);
            },
            "5" => {
                println!("Terima kasih!");
                break;
            },
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn baca_suhu(satuan: &str) -> f64 {
    loop {
        let simbol = match satuan {
            "Celsius" => "°C",
            "Fahrenheit" => "°F",
            "Kelvin" => "K",
            _ => "",
        };
        
        print!("Masukkan suhu dalam {} {}: ", satuan, simbol);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        
        match input.trim().parse::<f64>() {
            Ok(suhu) => return suhu,
            Err(_) => println!("Input tidak valid, masukkan angka!"),
        }
    }
}
