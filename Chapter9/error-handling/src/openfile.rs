use std::fs::File;
use std::io::ErrorKind;

//akcja w zależności od rodzaju błędu.
pub fn demo(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("Próba stworzenia pliku.");
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Błąd podczas tworzenia pliku: {:?}", e),
                }
            }
            _ => panic!("Coś poszło nie tak: {}", error),
        }
    };
}