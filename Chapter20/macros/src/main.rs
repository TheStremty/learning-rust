macro_rules! witaj {
    // Reguła dla braku argumentów
    () => {
        println!("Witaj, świecie!");
    };

    // Reguła dla DOWOLNEJ liczby argumentów (obsłuży też 1 lub 2 osoby!)
    ( $( $imie:expr ),* ) => {
        $(
            println!("Witaj {}!", $imie);
        )*
    };
}

fn main() {
    witaj!();
    witaj!("Młody", "Stremty");
}