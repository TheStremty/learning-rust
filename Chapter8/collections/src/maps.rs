use std::collections::HashMap;

pub fn run_demo(){
    let mut scores = HashMap::new();

    let team_name = String::from("Blue");
    let score = 10;

    scores.insert(team_name, score); //team_name moved do mapy

    let name = String::from("Blue");

    let team_score = scores.get(&name);
    if let Some(s) = team_score{
        println!("Wynik drużyny {}: {}",name,s);
    }
    let fast_score = scores.get("Blue").copied().unwrap_or(0);
    println!("Blue: {}", fast_score);


    let text = "niebo niebo ziemia słońce ziemia niebo";
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Statystyka słów: {:?}", counts);
}