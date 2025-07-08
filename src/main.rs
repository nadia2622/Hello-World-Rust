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

//ini latihan bikin var tipe data int dan float explicit
#[test]
fn number () {
    let a: i8 = 10;
    println!("{}", a);
    
    let b: f32 = 10.5;
    println!("{}", b);
}

/* Kita bisa konversi tipe data dari yg ukuran kecil ke gede, & sebaliknya. 
yg penting harus sesuaikan juga ukurannya
dikhawatirkan terjadi integer overflow.
Konversi ini menggunakan kata kunci 'as'
*/ 
#[test]
fn number_conversion () {
    let a: i8 = 10;
    println!("{}", a);
    
    let b: i16 = a as i16;
    println!("{}", b);
    
    let c: i32 = a as i32;
    println!("{}", c);
    
    // let d = 1000000000000000000000; // contoh integer overflow
    // let e : i8 = d as i8;
    // println!("{}", e);
}

// Nahhh kita masuk ke operasi aritmatika ni
#[test]
fn numeric_operator () {
    let a = 10.0;
    let b = 15.0;
    let c = a * b;
    let d = b / a;
    let e = a + b;
    let f = a - b;
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
}

/* Augmented assignments kayak:
a = a + 10 , a += 1-.
-=, *=, /=, %=
Karena kita merubah data variable yang sama, jadi var nya harus mutable
*/
#[test]
fn augmented_assignments() {
    let mut a = 20;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
    
    a *= 10;
    println!("{}", a);
}

//ini buat bool :>
#[test]
fn boolean () {
    let a = true;
    let b = false;
    print!("{} {}", a, b);
}

/* Operator Perbandingan (basically sama aja sih sama bahasa pemrograman lain :v)
 >, <, >=, <=, ==
*/
#[test]
fn comparison () {
    let result = 10 > 20;
    print!("{}", result);
}

/* Operasi boolean (basically sama aja sih sama bahasa pemrograman lain :v)
 &&, ||, !
*/
#[test]
fn boolean_operator () {
    let absen = 78;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    print!("{}", lulus_final);
}

// char pake ''. kalo String pake ""
#[test]
fn char_type () {
    let char1 = 'a';
    let char2 = 'b';
    print!("{} {}", char1, char2);
}

/* Ampun dah apa lagi enih. TUPLE
Tuple = tipe data kumpulan lebih dari 1 tipe data
jumlah data di Tuple ini dah final (defaultnya immutable). gabisa nambah/kurang lagi
Kalo udah bikin 3 data (misal), jumlah sama tipe data gabisa diubah lagi
buat bikin Tuple, pake ()
*/
#[test]
fn tuple () {
    let data = (10, 10.5,true);
    println!("{:?}", data); //:? ini buat debug information
    
    /* Lah tros akses data tuple nya bgimane?
    Nah kita bisa pake (titik) yang diikuti nomor index (lokasi) datanya.
    yups seperti biasa, index dimulai dari 0
     */
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);

    /* Kadang kan ribet tuh kalo harus ambil data dari tuple satu-satu pake index. 
    Alhamdulillah, kita bisa pakai destructuring buat bongkar isi tuple langsung ke variabel. 
    Kalau ada data yang nggak kita butuhin, tinggal pakai tanda _ aja.
     */
    let (a, b, c) = data; // nah ini namanya destructuring (bongkar isi tuple nya)
    println!("{} {} {}", a, b, c); 

    //Kalo gabutuh salah satu data di dalamnya, pake _
    let (_, e, _) = data; // nah ini namanya destructuring (bongkar isi tuple nya)
    println!("{}", e);

    /*  btw, kita bisa loh bikin Tuple jadi Mutable.
     caranya: nomorIndex =
     yahh kayak pas kita ngubah data variable gitu
     */
    let mut data3 = (10, 11.9, false);
    println!("\n{:?}", data3);
    data3.0 = 28;
    data3.1 = 5.5;
    data3.2 = true;
    println!("{:?}", data3);
}

/*Lanjuuut~~ 
Sekarang kita masuk ke yang namanya Unit. Unit adalah tuple tanpa nilai apapun, ditulisnya ()
"Lah kaga guna dong? trs buat apa?"
Biasanya Unit tuh dipake buat function2 yang ga butuh hasil data apapun
*/
fn unit () {
    println!("Helloooo"); // sebetulnya ini tuh returnya tuple kosong
}
#[test]
fn test_unit () {
    let hasil = unit(); // bisa dibilang... dia literli emang tipedatanya tuh tuple kosong :v
    println!("{:?}", hasil);
    
    let test = ();
    println!("{:?}", test);
}

/* OKEEY SAATNYA KITA MASUK KE ARRAY MWAHAHAHHAHAA
bedanya sama tuple, array tuh 1 tipe data aja (yaa tau lah ya)
cara bikinnya: pake []
*/
#[test]
fn array () {
    println!("Array biasa");
    let array: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array); //ini buat print var array nya
    println!();
    /* Nah selanjutnya untuk ngakses array, mirip tuple.
    (pake index yang mau dicari, maksude)
    Bedanya, Tuple itu kan ngaksesnya pake .
    Kalo array pake [index] */
    
    println!("Akses index Array saja");
    let a = array[0];
    let b = array[3];
    println!("{} {}", a, b);
    println!();
    
    // #MUTABLE ARRAY#
    println!("----MUTABLE ARRAY----");
    let mut array1 = [5,6,7,8,9];
    println!("{:?}", array1); //ini buat print var array nya
    
    array1[1] = 10;
    array1[0] = 0;
    println!("{:?}", array1); //ini buat print var array nya
    println!();
    
    // btw, kita bisa nih cari jumlah array nya dengan function len() buat array 
    println!("----Length----");
    let length = array.len(); //usize = ngikutin OS kamuu
    println!("{:?}", length);
    println!();
}

//Wetss saatnya masuk array 2 dimensi
#[test]
fn two_dimensional_array() {
    let matrix = [
        [1,2],
        [3,4]
        ];
        
    print!("Ini Array 2 dimensi");
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]); //[baris][kolom]
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
}

/* Sekarang kita masuk ke variable CONSTANT */
//var constant boleh di luar func

const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    //var constant di dalam function juga boleh
    const MINIMUM: i32 = 0; // variable constant sebaiknya upperase semua dan juga sebut tipe data secara explicit
    println!("{} {}", MINIMUM, MAXIMUM);
}

//VARIABLE SCOPE
/*  -adalah area dimana variable bisa duganakan
    -variable bisa digunakan di dlm scope tempat insialisasi var, dan juga di inner scope
    -Namun variable tidak bisa digunakan di outer scope */
/*  Buat contoh, gini deh. MINIMUM bisa dipake di dalam func constant doang.
    Tapi MAXIMUM bisa dipake di func lainnya */
#[test]
fn variable_scope() {
    let nadia = 1; // variable scope
    { // inner scope
        println!("inner nadia: {}", nadia);
        let tambunan = 2;
        println!("inner tambunan: {}", tambunan);
    }

    // println!("inner tambunan: {}", tambunan); //eror
}

/*  Sekarang kita msuk ke Memory Management:
    
    Garbage Collection
    -adalah fitur yang banyak digunakan bahasa pemrograman untuk melakukan manajemen memory, seperti Java dan Golang
    -Secara berkala Garbage Collection akan memantau data yang sudah tidak digunakan lagi di memory, dan menghapusnya secara otomatis
    -Rust memiliki pendekatan yang berbeda, Rust tidak menggunakan Garbage Collection, Rust juga tidak menggunakan Manual Memory Management
    -Lah trus Rust pake apa? Nah ini akan kita bahas~ */

/* STACK dan HEAP
    -Rust membagi data di memory dalam dua bagian, Stack dan Heap
    -Stack adalah bagian di mana data disimpan dalam struktur data tumpukan (Last In First Out). Semua data di Stack harus yang fixed size (artinya ukuran data sudah pasti)
    -Heap berbeda, heap seperti tempat untuk menyimpan data.
     Untuk menyimpan data di Heap, kita akan melakukan request ke Heap, lalu di dalam Heap terdapat Memory Allocator yang bertugas untuk menemukan area kosong untuk menyimpan dan mengalokasikan data ke area tersebut.
     Setelah itu kita akan diberi pointer (penunjuk) ke lokasi di mana data itu berada di Heap
    -Pointer dari Heap berukuran fixed size, oleh karena itu pointer akan disimpan di Stack
    */ 
#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10; // dia int, dan dia fixed size. makanya dia disimpan di Stack
    let b = String::from("tambunan"); //String ini ukuran datanya ga pasti. makanya nanti dia disimpan di Heap

    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Nadia");
    println!("{}, {}", a, b);
}
/*  DROP FUNCTION
    -Saat variabel keluar dari scope-nya, yang artinya tidak bisa diakses lagi, secara otomatis Rust akan memanggil drop function
    -Drop function adalah function untuk menghapus data, sehingga akan dibersihkan dari Heap
    -Dan jika Rust function() sudah selesai dieksekusi, maka function() tersebut akan dihapus pula dari Stack Frame
    -Oleh karena itu, Rust tidak membutuhkan Garbage Collection ataupun Manual Memory Management
*/

// ----- &str dan String -----

/*  &str = fixed size. (disimpan di Stack)
    String = ukuran data bisa ngembang (disimpan di heap)
     */

    /*Immutable str
    - Karena ukuran &str adalah fixed size, maka operasi &str adalah tipe data yang immutable, artinya isi data &str tidak bisa diubah
    - Ketika kita buat variable mutable, dan mengubah data &str, sebenarnya yang dilakukan adalah mengganti isi variable, bukan mengubah isi dari &str
    - &str memiliki banyak sekali method yang bisa digunakan untuk memanipulasi &str nya, namun akan menghasilkan nilai &str baru
    - Namun perlu diperhatikan, beberapa method dari &str akan mengembalikan bentuk data String, bukan &str

    Ini link buat cari lebih lanjut ttg &str pada Rust : https://doc.rust-lang.org/std/primitive.str.html
     */ 

#[test]
fn sting() {
    let name: &str = "      Nadia Tambunan    ";
    let trim: &str = name.trim(); //buat ngapus spasi kanan kiri
    //aslinya 'name' ga berubah. 'trim' itu data baru, hanya saja ngambil dari 'name'
    println!("{}", name);
    println!("{}", trim);

    let mut username: &'static str = "Nadia";
    username = "Tambun"; // yang berubah bukan &str nya, tapi VARIABLE-NYA.
    println!("{}", username);

    let mut value = 10; //sebetulnya ini udah fixed. dia tetap ada
    value = 11;     // nah kalo ini yg diubah hanya variablenya saja
}

/* Sekarang kita bahas String */
/*  - String di Rust merupakan tipe data text UTF-8, dan bisa berkembang ukurannya
    - Ketika kita buat dalam bentuk immutable variable, maka String tidak bisa berkembang, namun tetap disimpan di Heap
    - Ketika kita buat dalam bentuk mutable variable, maka String bisa berkembang di Heap
    - String juga memiliki method / function untuk memanipulasi data, namun perlu diperhatikan ada method yang digunakan untuk mengubah datanya sendiri, ada juga method yang digunakan untuk mengubah dalam bentuk data baru, tanpa memodifikasi data asli nya 
    
    Ini link buat cari lebih lanjut ttg String pada Rust : https://doc.rust-lang.org/std/string/struct.String.html
*/

#[test]
fn sting_type() {
    let mut name: String = String::from("Nadia"); // di simpen heap
    name.push_str(" Tambunan"); // kalo kita pake push, di heap yg tadinya nadia doang, skrg bertambah karena ketambahan tambunan
    println!("{}", name); // (btw push_str cuma bisa buat var mut)

    let ganti = name.replace("Nadia", "Sangkam");
    println!("{}", ganti); // kalo replace, dia tidak akan mengubah data di heap. tapi dia bikin data di heap baru, dan yang lama tetap ada
}

// OWNERSHIP
/*  - Rust menggunakan Ownership untuk melakukan data management di Memory
    - Ownership adalah salah satu fitur unik di Rust yang mungkin jarang ada di bahasa pemrograman lain
    - Ownership wajib dimengerti, karena akan berdampak ke hampir semua fitur di Rust
    - Ownership adalah fitur yang digunakan oleh Rust untuk menjadikan Rust menjadi bahasa pemrograman yang aman dalam mengelola data di memory, tanpa harus adanya fitur Garbage Collection atau Manual Memory Management
    - Karena Ownership adalah konsep yang baru untuk kebanyakan programmer, maka kadang kita butuh waktu untuk memahaminya

*/

#[test]
fn ownership_rules() {
    // a tidak bisa diakses di sini, karena belum dideklarasikan
    let a = 10; // a bisa diakses mulai dari sini

    { // b tidak bisa diakses di sini, karena belum dideklarasikan
        let b = 21; // a bisa diakses mulai dari sini
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi
// pertanyaannya, kenapa dihapus? karena memang sudah tidak bisa diakses lagi (scope nya sudah selesai).

//  === DATA COPY ===

#[test]
fn data_copy() {
    let a = 10;
    let b = a; // Sebetulnya dia adalah copy dari isi data di var a
    
    println!("{} {}", a, b);
    
    let c = 100;
    let mut d = c; // Sebetulnya dia adalah copy dari isi data di var a
    
    d = 200; // ketika 2 diubah, isi var a tetap sama
    println!("{} {}", c, d);
}

// === OWNERSHIP MOVEMENT ===

#[test]
fn ownership_movement() {
    let name1 = String :: from("Nadia");

    //ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    //name1 tidak bisa diakses di sini

    // println!("{}", name1); eror
    println!("{}", name2);
}