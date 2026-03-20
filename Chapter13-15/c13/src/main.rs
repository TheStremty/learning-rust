fn main()
{
    let rabat = 20;

    let procent_to_value = |v| {v * rabat / 100};

    let oblicz_cene = |c| {c - procent_to_value(c)};

    let ceny = vec![100,200,50,10];

    let po_rabacie = ceny.iter()
        .filter(|&&x| x >= 20)
        .map(|x| oblicz_cene(x))
        .collect::<Vec<i32>>();
    println!("{:?}", po_rabacie);

    let suma: i31 = po_rabacie.iter().sum();
    println!("{:?}", suma);

}
