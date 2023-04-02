use std::env;
use std::net::Ipv4Addr;

fn main() {
    match env::args().nth(1) {
        Some(s) => match s.trim().parse::<u32>() {
            Ok(o) => {
                println!("{}", Ipv4Addr::from(o));
            }
            Err(_) => {
                let first_ip: Ipv4Addr = s
                    .trim()
                    .parse()
                    .expect("Invalid IPv4 address in position 1.");
                let conversion1: u32 = first_ip.into();
                if env::args().nth(2).is_some() {
                    let second_ip: Ipv4Addr = env::args()
                        .nth(2)
                        .unwrap()
                        .trim()
                        .parse()
                        .expect("Invalid IPv4 address in position 2.");

                    let conversion2: u32 = second_ip.into();
                    for ip in conversion1..conversion2 {
                        println!("{}", ip);
                    }
                } else {
                    println!("{}", conversion1);
                }
            }
        },
        None => eprintln!("You must provide an IP address."),
    }
}
