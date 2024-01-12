use std::{cmp::Ordering, io};

use rand::Rng;

// Fungsi main
fn main() {
    println!("Game Tebak Angka Random");

    // Menghasilkan angka random 1 - 100
    let angka_rahasia = rand::thread_rng().gen_range(1..=100);

    // Variable untuk menyimpan kesempatan pengguna untuk memasukan input
    let mut kesempatan: i32 = 10;

    // Looping
    loop {
        println!("Kesempatan menjawab : {kesempatan}");

        // Cek apakah pengguna memiliki kesempatan untuk menjawab
        if kesempatan == 0 {
            println!("Angka rahasia nya adalah {angka_rahasia}");
            println!("Kesempatan kamu sudah habis, kamu kalah!");
            break;
        }

        println!("Silahkan masukkan tebakan kamu.");

        // Membuat variable untuk menyimpan input pengguna
        let mut tebakan = String::new();

        // Membaca input pengguna dan memasukkan ke variable di atas
        io::stdin()
            .read_line(&mut tebakan)
            .expect("Gagal membaca tebakan.");

        println!("Tebakan kamu adalah {tebakan}");

        // Merubah input pengguna dari string ke angka u32
        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Cek apakah input pengguna lebih besar, kecil atau sama
        match tebakan.cmp(&angka_rahasia) {
            Ordering::Less => {
                println!("Terlalu kecil");
                kesempatan -= 1;
            }
            Ordering::Greater => {
                println!("Terlalu besar");
                kesempatan -= 1;
            }
            Ordering::Equal => {
                println!("Kamu menang!");
                println!("Angka rahasia nya adalah {angka_rahasia}");
                break;
            }
        }
    }
}
