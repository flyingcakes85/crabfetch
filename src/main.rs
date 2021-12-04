use ansi_term::Color::Red;
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};
fn main() {
    let left_pad = 25;
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}",Red.paint("     /\\\r\n    ( /   @ @    ()\r\n     \\  __| |__  /\r\n      -/   \"   \\-\r\n     /-|       |-\\\r\n    / /-\\     /-\\ \\\r\n     / /-`---\'-\\ \\\r\n      /         \\\r\n") );

    let mut cursor_row = crossterm::cursor::position().unwrap().1;
    cursor_row -= 9;
    println!(
        "{}{} : {}\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("OS"),
        sys.name().unwrap()
    );
    cursor_row += 1;
    println!(
        "{}{} : {}\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("Kernel"),
        sys.kernel_version().unwrap()
    );
    cursor_row += 1;

    println!(
        "{}{} : {}\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("Host"),
        sys.host_name().unwrap()
    );
    cursor_row += 1;

    println!(
        "{}{} : {}\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("CPU"),
        sys.processors()[0].brand()
    );
    cursor_row += 1;

    println!(
        "{}{} : {}\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("Cores"),
        sys.processors().len()
    );
    cursor_row += 1;

    let mut usage = 0;

    for processor in sys.processors() {
        usage += processor.cpu_usage() as usize;
    }
    println!(
        "{}{} : {}%\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("Usage"),
        usage / sys.processors().len()
    );
    cursor_row += 1;

    for component in sys.components() {
        if component.label().contains("Package id") {
            println!(
                "{}{} : {}C\r",
                crossterm::cursor::MoveTo(left_pad, cursor_row),
                Red.bold().paint("Temp"),
                component.temperature()
            );
            cursor_row += 1;
            break;
        }
    }

    println!(
        "{}{} : {}MB/{}MB\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("Mem"),
        sys.used_memory() / 1024,
        sys.total_memory() / 1024
    );
    cursor_row += 1;

    println!(
        "{}{} : {}MB/{}MB\r",
        crossterm::cursor::MoveTo(left_pad, cursor_row),
        Red.bold().paint("Swap"),
        sys.used_swap() / 1024,
        sys.total_swap() / 1024
    );
}
