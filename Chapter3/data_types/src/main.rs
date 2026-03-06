fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {guess}");

    //floating-point types
    {
        let x = 2.0; //f64

        let y: f32 = 3.0; //f32
    }
    //numeric operations
    {
        //addition
        let sum = 5 + 10;

        //subtraction
        let difference = 95.5 - 4.3;

        //multiplication
        let product = 4 * 30;

        //division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3;

        //remainder
        let remainder = 43 % 5;
    }

    //the boolean type
    {
        let t = true;
        let f: bool = false;
    }


    //the character type
    {
        let c = 'z';
        let z: char = 'Z';
        let heart_eyed_cat = '😻';
    }


    //the tuple type
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is {y}");

        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }

    //the array type
    {
        let a = [1, 2, 3, 4, 5];

        let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];

        let array = [3;5];

        let first = a[0];
        let second = a[1];

    }

}