// definicja traita (cechy)
// Każdy, kto chce być "Opisywalny", musi mieć funkcję summary.

trait Summary {
    fn summarize(&self) -> String;
}

// struktury
struct NewsArticle {
    headline: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

//implementacja
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("#{}\n{}", self.headline, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}:\n{}", self.username, self.content)
    }
}

// generics + trait bounds
// ta funkcja przyjmuje dowolny typ T, o ile ten typ ma implementację Summary.
fn notify<T: Summary>(item: &T) {
    println!("Nowe powiadomienie!\n\n{}", item.summarize());
}

// 'a to generyczny czas życia.
// tzn. "Zwracana referencja będzie żyła conajmniej tak długo, jak KRÓTSZA z tych dwóch".
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//demo
fn main() {
    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Generics są super!")
    };

    let article = NewsArticle {
        headline: String::from("Przełom w Rust!"),
        content: String::from("Rozdział 10 opanowany! 🦀")
    };

    notify(&tweet);
    notify(&article);
    println!("{}", longest("abcde", "fgh"));
}