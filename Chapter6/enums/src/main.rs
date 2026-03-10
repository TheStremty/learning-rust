enum IpAddr {
    V4(u8,u8,u8,u8), // Opcja nr 1: cztery małe liczby
    V6(String),      // Opcja nr 2: jeden długi napis
}
impl IpAddr {
    fn print_ip(&self) {
        match self {
            IpAddr::V4(a,b,c,d) => {println!("IPV4: {}.{}.{}.{}", a,b,c,d)},
            IpAddr::V6(ip) => println!("IPV6: {}", ip),
        }
    }
}


fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    home.print_ip();

    if let IpAddr::V6(ip) = &loopback {
        println!("V6 pasuje {}", ip);
    }

    loopback.print_ip();

    if let IpAddr::V4(a, b, c, d) = &home {
        println!("V4 pasuje {}.{}.{}.{}", a,b,c,d);
    }

    let number:Option<u32> = Some(512);

    if let Some(n) = number {
        println!("liczba to: {}", n);
    }

    let number:Option<u32> = None;

    if let Some(n) = number {
        println!("liczba to: {}", n);
    }
    else {
        println!("Nie ma liczby!");
    }


}
