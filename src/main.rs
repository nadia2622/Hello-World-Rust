fn main() {
    println!("Hello, world!");
    println!("Hello, Nadiaaa!");
    println!("Hello, Tambunan!");
}
// ini buat test
#[test] //jangan lupa kasih attribute ini buat nandain kalau ini unit test
fn hello_test() {
    println!("Hello TESTTT");
}

#[test]
fn test_variable() {
    let nama = "Nadia Tambunan"; // ini variable immutable (gabisa digonta-ganti isinya)
    print!("Hallo {}", nama);
}

// Tapi sebetulnya kita bisa buat variable mutable. pake kata kunci let mut
#[test]
fn test_mutable () {
    let mut nama = "Nadia Tambunan";
    println!("Hello {}", nama);
    
    nama = "Sangkam Tambunan"; //bisa ubah isi variable seperti ini
    println!("Hello {}", nama);

}

/*Rust adalah bahasa yang static typing, artinya setiap kita membuat variable dengan tipe data tertentu,
maka dia tidak akan bisa berubah menjadi tipe data lainnya.
 */
#[test]
fn static_typing () {
    let mut nama = "Nadia Tambunan";
    println!("Hello {}", nama);
    
    nama = "Budi";
    //nama = 10; ini gabisa dicompile
    println!("Hello {}", nama);

}

/* Di Rust, kita bisa bikin variable dengan nama yang sama.
tapi ntar variable yang sebelumnya bakal tertutup atau bisa disebut shadowing.
Boleh di Rust, tapi sebaiknya jangan terlalu sering karena dapat membingungkan pembaca kode
*/
#[test]
fn shadowing() {
    let nama = "Nadia Tambunan";
    println!("Hai {}", nama);
// ini adalah 2 variable yang berbeda. bahkan lokasi memorinya pun berbeda
    let nama = 20; //bedanya sama immutable, variable ke dua pake kata kunci let lagi. (artinya bikin variable baru)
    println!("Umurku {}", nama);
}

// bikin variable yang tipe datanya disebutkan (explicit type)
#[test]
fn explicit () {
    let umur: i32 = 20; // itukan pake i (Signed). kalo Unsigned pake u/ cth: u32 (dia gabisa nerima angka negatif)
    println!("{}", umur);
}