use std::io::{self, Write};
use std::io::Read;
use std::time::Duration;
use dialoguer::MultiSelect;
use serial::SerialPort;

fn list_and_choose_port() -> String {
    let avaliable_ports = serialport::available_ports().unwrap();
    let mut items: Vec<String> = Vec::new();

    for port in avaliable_ports {
        items.push(port.port_name);
    }

    let chosen_port : Vec<usize> = MultiSelect::new()
        .items(&items)
        .interact();

    println!("You chose: {:?}", chosen_port);

    return items[chosen_port[0]].clone();
}

fn main() {
    let port_name = list_and_choose_port();

    let mut port = serial::open("/dev/cu.usbserial-548B0065701").unwrap();

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200).unwrap();
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