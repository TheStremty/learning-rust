use std::rc::Rc;
use std::cell::RefCell;

struct Paczka {
    nazwa: String,
    zaleznosci: Vec<Rc<RefCell<Paczka>>>,
    pobrana: bool,
}

impl Paczka {
    fn nowa(nazwa: &str) -> Rc<RefCell<Paczka>> {
        Rc::new(RefCell::new(Paczka {
            nazwa: nazwa.to_string(),
            zaleznosci: vec![],
            pobrana: false,
        }))
    }

    fn pobierz(&mut self) {
        if self.pobrana {
            println!("{} — już pobrana, pomijam", self.nazwa);
            return;
        }
        self.pobrana = true;
        println!("{} — pobrano!", self.nazwa);
    }
}

fn main() {
    let a = Paczka::nowa("A");
    let b = Paczka::nowa("B");
    let c = Paczka::nowa("C");

    // B i C zależą od A
    b.borrow_mut().zaleznosci.push(Rc::clone(&a));
    c.borrow_mut().zaleznosci.push(Rc::clone(&a));

    // Pobieramy zależności B
    for dep in &b.borrow().zaleznosci {
        dep.borrow_mut().pobierz(); // A — pobrano!
    }

    // Pobieramy zależności C — A już pobrana!
    for dep in &c.borrow().zaleznosci {
        dep.borrow_mut().pobierz(); // A — już pobrana, pomijam
    }
}

//Rc — gdy nie wiesz kto jest właścicielem:
// Używasz gdy dane muszą być współposiadane przez wiele miejsc i żadne z nich nie jest "głównym" właścicielem. Dane żyją dopóki ktokolwiek ich potrzebuje.

// RefCell jest potrzebny tylko gdy nie możesz napisać &mut self — czyli gdy coś innego (np. Rc, albo trait z zewnątrz) narzuca ci &self i nie masz wyboru.
// Jeśli możesz napisać &mut self — zrób to. To zawsze lepsze rozwiązanie.