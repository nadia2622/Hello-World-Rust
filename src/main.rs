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

/* Sekarang kita masuk ke CONSTANT */