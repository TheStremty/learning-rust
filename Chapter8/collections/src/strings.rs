pub fn run_demo() {
    let mut hello = String::from("Cześć");
    hello.push_str( ", co tam? 🦀");

    let s1 = String::from("Analiza");
    let s2 = String::from("Danych");
    let s3 = format!("{} - {}", s1, s2); //format! nie zabiera własności

    println!("{}", hello);
    println!("Wynik formatowania: {}", s3);
}