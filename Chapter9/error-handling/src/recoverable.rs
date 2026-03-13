pub fn demo(){
    let input = "42";
    let bad_input = "nie-liczba";

    //1. match - pełna kontrola
    match input.parse::<i32>(){
        Ok(n) => println!("Parsowanie udane: {}",n),
        Err(e) => println!("Błąd parsowania: {}",e),
    }

    //2. unwrap - "daj mi to albo crash"
    let n = input.parse::<i32>().unwrap();
    println!("Unwrap zadziałał {}",n);

    //3. expect
    let _m = input.parse::<i32>().expect("Input powinien być liczbą!");
}