use std::io;

fn main() {
    println!("=== Kalkulator Sederhana ===\n");
    
    // Input bilangan pertama
    println!("Masukkan bilangan pertama:");
    let num1: f64 = baca_angka();
    
    // Input operator
    println!("\nPilih operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Gagal membaca input");
    let operator = operator.trim();
    
    // Input bilangan kedua
    println!("Masukkan bilangan kedua:");
    let num2: f64 = baca_angka();
    
    // Hitung hasil
    let hasil = hitung(num1, num2, operator);
    
    println!("\n{} {} {} = {}", num1, operator, num2, hasil);
}

fn baca_angka() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Input tidak valid, masukkan angka:"),
        }
    }
}

fn hitung(a: f64, b: f64, operator: &str) -> f64 {
    match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                println!("Error: Tidak bisa dibagi 0!");
                0.0
            } else {
                a / b
            }
        },
        _ => {
            println!("Operator tidak dikenal!");
            0.0
        }
    }
}
