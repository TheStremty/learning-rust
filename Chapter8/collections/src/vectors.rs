pub fn run_demo(){
    let mut numbers = vec![10, 20, 30];
    numbers.push(40);

    let index = 10;
    match numbers.get(index){
        Some(val) => println!("Liczba pod indeksem {} to {}", index, val),
        None => println!("Brak wartości pod indeksem {}", index),
    }

    for n in &mut numbers {
        *n += 5;
    }

    println!("Zaktualizowany wektor: {:?}", numbers);

}