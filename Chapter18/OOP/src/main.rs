use std::fs::File;
use std::io::Write;

pub trait Component {
    fn render(&self) -> String;
}
enum Visibility {
    Visible,
    Hidden,
}
pub struct Document {
    components: Vec<Box<dyn Component>>,
}

impl Document {
    pub fn new() -> Self {
        Document { components: vec![] }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn generate_file(&self, filename: &str) {
        let mut html = String::from("<html>\n<body style=\"font-family: sans-serif; margin: 40px;\">\n");

        for component in &self.components {
            html.push_str(&component.render());
            html.push('\n');
        }

        html.push_str("</body>\n</html>");

        let mut file = File::create(filename).expect("nie mozna stworzyc pliku");
        file.write_all(html.as_bytes()).expect("nie udalo sie zapisac");
        println!("gotowa strona: {}", filename);
    }
}

pub struct Header {
    text: String,
    level: u8,
}
impl Header {
    pub fn new(text: String, mut level: u8) -> Self {
        if level > 6 { level = 6; }
        if level == 0 { level = 1; }
        Header { text, level }
    }
}
impl Component for Header {
    fn render(&self) -> String {
        format!("<h{}>{}</h{}>", self.level, self.text, self.level)
    }
}

pub struct Paragraph {
    pub text: String,
    pub visibility: Visibility,
}
impl Component for Paragraph {
    fn render(&self) -> String {
        match self.visibility {
            Visibility::Visible => format!("<p>{}</p>", self.text),
            Visibility::Hidden => String::new(),
        }
    }
}
pub struct Button {
    pub label: String,
}
impl Component for Button {
    fn render(&self) -> String {
        format!("<button>{}</button>", self.label)
    }
}

pub struct List {
    pub items: Vec<Box<dyn Component>>,
}
impl Component for List {
    fn render(&self) -> String {
        format!("<ul>{}</ul>", self.items.iter().map(|x| format!("<li>{}</li>", x.render())).collect::<Vec<String>>().join(" "))
    }
}

fn main() {
    let mut doc = Document::new();

    doc.add_component(Box::new(Header::new(String::from("Header"), 0)));

    doc.add_component(Box::new(Paragraph { text: String::from("Dzisiaj zrobiłem generator HTML!"), visibility: Visibility::Hidden }));
    doc.add_component(Box::new(Button { label: String::from("Kliknij mnie") }));
    let list = List { items: vec![Box::new(Paragraph { text: String::from("Dzisiaj zrobiłem generator HTML!"), visibility: Visibility::Visible }), Box::new(Button { label: String::from("Kliknij mnie w list") })] };
    doc.add_component(Box::new(list));

    doc.generate_file("raport.html");
}