use std::io::{self, Write};
use std::io::Read;
use std::time::Duration;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use serial::{SerialPort, BaudRate};
use std::process::exit;

fn list_and_choose_port() -> String {
    let avaliable_ports = serialport::available_ports().unwrap();
    let mut items: Vec<String> = Vec::new();

    for port in avaliable_ports {
        items.push(port.port_name);
    }

    let chosen_port = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .with_prompt("Select port")
        .interact_on_opt(&Term::stderr());

    match chosen_port.unwrap() {
        Some(index) => items[index].to_string(),
        None => exit(0),
    }
}

fn select_baud() -> BaudRate {
    let items = vec!["9600", "19200", "38400", "57600", "115200", "230400", "460800", "921600"];
    let chosen_baud = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(4)
        .with_prompt("Select baud rate")
        .interact_on_opt(&Term::stderr());

    match chosen_baud.unwrap() {
        Some(index) => {
            return match items[index].parse::<u32>() {
                Ok(baud) => BaudRate::from_speed(baud.try_into().unwrap()),
                Err(_) => BaudRate::Baud115200,
            };
        },
        None => exit(0),
    }
}

fn main() {
    let port_name = list_and_choose_port();
    let mut port = serial::open(&port_name).unwrap();
    let baude_rate = select_baud();

    println!("\nListening to {} @ {} baud", port_name, baude_rate.speed());

    port.reconfigure(&|settings| {
        settings.set_baud_rate(baude_rate).unwrap();
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }).unwrap();

    port.set_timeout(Duration::from_millis(100)).unwrap();

    loop {
        let mut buf = [0u8; 64];
        match port.read(&mut buf) {
            Ok(t) => {
                io::stdout().write_all(&buf[..t]).unwrap();
                io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {
                eprintln!("{:?}", e);
                std::thread::sleep(Duration::from_millis(100));

                io::Error::new(
                    io::ErrorKind::Other, "device reports readiness to read but returned no data (device disconnected?)"
                );
                break;
            },
        }
    }
}
