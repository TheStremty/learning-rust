fn main() {

    println!("31.2 Celcius is {} Fahrenheit.", celsius_to_fahrenheit(31.2));
    println!("102 Fahrenheit is {} Celcius", fahrenheit_to_celsius(102.0));

    let result = fibonacci(10);
    println!("Fibonacci(10): {result}");

    the_twelve_days_of_christmas();
}

fn celsius_to_fahrenheit(degrees: f32) -> f32 {

    degrees * 9.0 / 5.0 + 32.0
}
fn fahrenheit_to_celsius(degrees: f32) -> f32 {
    (degrees - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u128 {
    let mut tup: (u128, u128) = (1,0);
    for _ in 0..n {
        tup = (tup.1, tup.0+tup.1);
    }
    tup.1
}

fn the_twelve_days_of_christmas(){
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for day in 0..=11{
        println!("On the {} day of Christmas, my true love sent to me:",days[day]);
        for gift in (0..=day).rev(){
            if (gift == 0 && day > 0){
                print!("and ");
            }
            print!("{}",gifts[gift]);
            if {gift == 0}{
                print!(".\n");
            }
            else {
                print!(",\n");
            }

        }
    }
}