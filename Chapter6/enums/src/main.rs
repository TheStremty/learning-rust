enum IpAddr {
    V4(u8,u8,u8,u8), // Opcja nr 1: cztery małe liczby
    V6(String),      // Opcja nr 2: jeden długi napis
}



fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip(&home);

    if let IpAddr::V6(ip) = &loopback {
        println!("V6 pasuje {}", ip);
    }

    print_ip(&loopback);

    if let IpAddr::V4(a, b, c, d) = &home {
        println!("V4 pasuje {}.{}.{}.{}", a,b,c,d);
    }

}


fn print_ip(addr: &IpAddr) {
    match addr {
        IpAddr::V4(a,b,c,d) => {println!("IPV4: {}.{}.{}.{}", a,b,c,d)},
        IpAddr::V6(ip) => println!("IPV6: {}", ip),
    }
}
