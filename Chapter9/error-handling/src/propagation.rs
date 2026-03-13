use std::num::ParseIntError;

//funkcja próbująca sparsować dwie liczby
pub fn parse_int(val1: &str, val2: &str) -> Result<i32, ParseIntError> {
    let n1 = val1.parse::<i32>()?; // operator ? - Jeśli błąd funkcja kończy tutaj i zwraca Err
    let n2 = val2.parse::<i32>()?; // to samo co wyzej

    Ok(n1 + n2)
}

pub fn demo(){
    match parse_int("10","20"){
        Ok(n) => println!("Suma: {}",n),
        Err(e) => println!("błąd propagacji: {}",e)
    }
}