// Struktury – czyli jak spakować graty do odpowiednich kartonów, żeby nie szukać pojedynczych skarpet po całym RAM-ie.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Debugowanie – czyli użycie magii #[derive(Debug)], żeby Rust przestał udawać, że nie widzi, co jest w środku struktury.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Stremty"),
        email: String::from("stremty@example.com"),
        sign_in_count: 1,
    };

    println!("Email to: {}", user1.email);


    let rect1 = Rectangle {width: 30, height: 50};

    println!("Mój prostokąt to: {:?}", rect1);
    println!("Pole prostokąta to: {}", rect1.area());
    println!("Mój prostokąt to nadal: {:?}", rect1);

    let rect2 = Rectangle::square(10);
    let rect3 = Rectangle {width: 40, height: 50};

    println!("rect1 może pomieścić rect2? - {}", rect1.can_hold(&rect2));
    println!("rect1 może pomieścić rect3? - {}", rect1.can_hold(&rect3));
}

impl Rectangle {
    fn area(&self) -> u32 {
        // &self – czyli zaimek osobowy, dzięki któremu struktura wie, że mówi o swoich własnych bebechach, a nie o sąsiada.
        self.width * self.height
    }

    // Associated Functions – czyli jak zrobić instrukcję obsługi składania mebli, która nie wymaga posiadania już złożonej szafy pod ręką.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}
