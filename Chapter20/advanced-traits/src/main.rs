use std::fmt;
use std::fmt::{Debug, Formatter, Display};
use std::ops::Add;
use std::error::Error;

type SmartResult<T> = Result<T, Box<dyn Error>>;

fn sprawdz_sensor() -> SmartResult<i32> {
    Ok(100)
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

trait Wydruk {
    type Format;
    fn drukuj(&self) -> Self::Format;
}

struct Dokument {
    tekst: String,
}

impl Wydruk for Dokument {
    type Format = String;
    fn drukuj(&self) -> Self::Format {
        self.tekst.clone()
    }
}

trait Zarys: fmt::Display {
    fn drukuj_zarys(&self){
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4))
    }
}

struct Wspolrzedne {
    x: i32,
    y: i32,
}

impl Zarys for Wspolrzedne {}

impl Display for Wspolrzedne {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

struct ListaDokumentow(Vec<Dokument>);

impl Display for ListaDokumentow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let docs = self.0.iter().map(|d| d.drukuj()).collect::<Vec<_>>();
        write!(f, "{}", docs.join(", "))
    }
}

trait Pilot {
    fn wlacz(&self);
}

trait Odtwarzacz {
    fn wlacz(&self);
}

struct SmartTV;

impl Pilot for SmartTV {
    fn wlacz(&self) {
        println!("Pilot: Włączam zasilanie TV.");
    }
}

impl Odtwarzacz for SmartTV {
    fn wlacz(&self) {
        println!("Odtwarzacz: Uruchamiam aplikację Netflix.");
    }
}

impl SmartTV {
    fn wlacz(&self) {
        println!("SmartTV: Wyświetlam logo powitalne.");
    }
}

fn main() {
    let doc = Dokument { tekst: String::from("Jakis tekst dokumentu") };
    let doc2 = Dokument { tekst: String::from("dokument2") };
    println!("{}", doc.drukuj());

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 10, y: 20 };
    let p3 = p1 + p2;
    println!("p3 = {:?}", p3);

    let tv = SmartTV;
    Pilot::wlacz(&tv);
    tv.wlacz();
    Odtwarzacz::wlacz(&tv);

    let w = Wspolrzedne { x: 1024, y: -512 };
    w.drukuj_zarys();

    let lista = ListaDokumentow(vec![doc, doc2]);
    println!("Dokumenty: {}", lista);

    match sprawdz_sensor() {
        Ok(val) => println!("Status sensora: {}", val),
        Err(e) => println!("Błąd: {}", e),
    }
}