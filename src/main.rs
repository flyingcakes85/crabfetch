use ansi_term::Color::Red;
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};
use termion::{color, style};
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};
fn main() {
    let left_pad = 30;
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    // println!("{}\r", stdout.cursor_pos().unwrap().1);

    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{} : {}\r", Red.bold().paint("OS"), sys.name().unwrap());
    println!(
        "{} : {}\r",
        Red.bold().paint("Kernel"),
        sys.kernel_version().unwrap()
    );
    println!(
        "{} : {}\r",
        Red.bold().paint("Host"),
        sys.host_name().unwrap()
    );

    println!(
        "{} : {}\r",
        Red.bold().paint("CPU"),
        sys.processors()[0].brand()
    );
    println!(
        "{} : {}\r",
        Red.bold().paint("Cores"),
        sys.processors().len()
    );

    let mut usage = 0;

    for processor in sys.processors() {
        usage = usage + processor.cpu_usage() as usize;
    }
    println!(
        "{} : {}%\r",
        Red.bold().paint("Usage"),
        usage / sys.processors().len()
    );

    for component in sys.components() {
        if component.label().contains("Package id") {
            println!(
                "{} : {}C\r",
                Red.bold().paint("Temp"),
                component.temperature()
            );
        }
    }

    println!(
        "{} : {}MB/{}MB\r",
        Red.bold().paint("Mem"),
        sys.used_memory() / 1024,
        sys.total_memory() / 1024
    );
    println!(
        "{} : {}MB/{}MB\r",
        Red.bold().paint("Swap"),
        sys.used_swap() / 1024,
        sys.total_swap() / 1024
    );
}
