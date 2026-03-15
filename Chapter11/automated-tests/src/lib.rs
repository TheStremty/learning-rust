pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

/// Dodaje jeden do liczby.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = automated_tests::add_one(arg);
///
/// assert_eq!(6, answer, "Wynik add_one jest zły! Expected: 6 Got: `{}`", answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 2 //zostawiam to specjalnie
}

#[cfg(test)]
mod tests {
    use super::*;

    //podstawowe asercje
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    //assert_eq - testowanie równości
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    //własne wiadomości o błędach
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Stremty");
        assert!(
            result.contains("Stremty"),
            "Greeting did not contain name, value was `{}`", result
        )
    }

    //testowanie panic - (Should Panic)
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    //używanie Result<T,E> w testach (zamiast panic)
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}