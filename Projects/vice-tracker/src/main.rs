// Zamiast "ui::tui", używasz nazwy crate'a
use vice_tracker::ui::tui::Menu;

fn main() {
    // Tworzysz opcje (używamy vec! dla wygody)
    let opcje = vec!["Dodaj nawyk", "Pokaż raporty", "Wyjdź"];

    // Tworzysz instancję Menu
    let glowne_menu = Menu::new(
        "--- VICE TRACKER v1.0 ---",
        opcje,
        "Twój wybór: "
    );

    // Wyświetlasz
    glowne_menu.display();

    // Pobierasz wybór
    let wybor = glowne_menu.get_choice();

    println!("Wybrałeś opcję numer: {}", wybor + 1);
}