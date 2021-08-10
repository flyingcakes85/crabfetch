use ansi_term::Color::Red;
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{} : {}", Red.bold().paint("OS"), sys.name().unwrap());
    println!(
        "{} : {}",
        Red.bold().paint("Kernel"),
        sys.kernel_version().unwrap()
    );
    println!(
        "{} : {}",
        Red.bold().paint("Host"),
        sys.host_name().unwrap()
    );

    println!(
        "{} : {}",
        Red.bold().paint("CPU"),
        sys.processors()[0].brand()
    );
    println!("{} : {}", Red.bold().paint("Cores"), sys.processors().len());

    let mut usage = 0;

    for processor in sys.processors() {
        usage = usage + processor.cpu_usage() as usize;
    }
    println!(
        "{} : {}%",
        Red.bold().paint("Usage"),
        usage / sys.processors().len()
    );

    for component in sys.components() {
        if component.label().contains("Package id") {
            println!(
                "{} : {}C",
                Red.bold().paint("Temp"),
                component.temperature()
            );
        }
    }

    println!(
        "{} : {}MB/{}MB",
        Red.bold().paint("Mem"),
        sys.used_memory() / 1024,
        sys.total_memory() / 1024
    );
    println!(
        "{} : {}MB/{}MB",
        Red.bold().paint("Swap"),
        sys.used_swap() / 1024,
        sys.total_swap() / 1024
    );
}
