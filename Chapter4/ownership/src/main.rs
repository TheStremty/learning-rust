fn main() {

    // 🦀 Rust Chapter 4: Eksmisja lokatorów ze sterty czyli jak nie zostać z pustym wskaźnikiem

    {
        let s = String::from("Hello");
        let s2 = s; //move
        //s no longer exists, s2 is the owner
        println!("{}",s2);
    }

    {
        let s = String::from("Siema");
        takes_ownership(s); //function takes ownership (Move)
        //println!("{}",s); - ERROR, s doesnt exist anymore, function took ownership.

        let x = 5;
        makes_copy(x);

        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
            // end of scope, drop memory on heap.
        }

        fn makes_copy(some_integer: i32) {
            println!("{}", some_integer);
        }

    }

    //return values
    {
        let s1 = String::from("Hello");
        let s2 = takes_and_gives_back(s1); //s1 goes in, s2 returns

        println!("{}", s2);

        fn takes_and_gives_back(a_string: String) -> String {
            a_string //return ownership back to s2
        }
    }

    //Referencje i Borrowing (Pożyczanie) – czyli jak dać sprzątaczce klucze tylko na chwilę, bez przepisywania własności mieszkania w księdze wieczystej.
    {
        let s1 = String::from("Hello");

        // reference &1 not s1 (ownership)
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len); //s1 still exists

        fn calculate_length(s: &String) -> usize {
            s.len()
        }

        let mut s = String::from("Hello, ");

        let r1 = &mut s; //mutable reference, allows us to change s without giving out ownership of s
        //let r2 = &mut s; // ERROR!  We cant have more than one mutable reference.

        modify_s(r1);
        println!("{}", s);

        fn modify_s(s: &mut String) {
            s.push_str("world!");
        }
    }


    //Slices czyli branie tylko gryza z cudzej kanapki bez przejmowania całej własności
    {
        let s = String::from("hello world");
        let hello = &s[0..5]; //takes hello
        let world = &s[6..11]; //takes world
        println!("{} {}",hello,world);

        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3]; //[2,3]
        println!("{:?}", slice);

    }

}
