#![allow(dead_code)]

struct SensorData {
    id: u32,
    val: i32,
    location: String,
    active: bool,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
    TemperatureChange(i32),
    SensorUpdate(SensorData),
}

struct State {
    color: (i32, i32, i32),
    position: (i32, i32),
    quit: bool,
    temp: i32,
}

impl State {
    fn new() -> Self {
        State {
            color: (0, 0, 0),
            position: (0, 0),
            quit: false,
            temp: 20,
        }
    }
}

fn process_message(msg: Message, state: &mut State) {
    match msg {
        Message::Quit => state.quit = true,

        Message::Move { x, y } => { state.position = (x, y); },

        Message::TemperatureChange(temp @ 35..=100) => {
            println!("Wykryto upał! {}", temp);
            state.temp = temp;
        },

        Message::ChangeColor(Color::Rgb(_, g @ 255, _)) => {
            println!("Maks zielony!");
            state.color = (state.color.0, g, state.color.2);
        },

        Message::SensorUpdate(SensorData { id: 7 | 8, val, .. }) => {
            println!("id 7 albo 8 wartosc: {}", val)
        },

        _ => println!("Cos innego"),
    }
}

fn print_battery_status(&(id, level): &(u32, i32)) {
    println!("Battery status: {} for sensor with id:({})", level, id);
}

fn main() {
    let mut state = State::new();

    let messages = vec![
        Message::TemperatureChange(40),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(Color::Rgb(100, 255, 100)),
        Message::SensorUpdate(SensorData { id: 8, val: 230, location: String::from("Living room"), active: true }),
        Message::Write(String::from("Witaj w inteligentnym domu")),
        Message::Quit,
    ];

    for msg in messages {
        process_message(msg, &mut state);
    }

    let battery_reports = vec![(1, 85), (2, 12), (3, 100)];

    for (id, level) in battery_reports {
        print_battery_status(&(id, level));
    }
}