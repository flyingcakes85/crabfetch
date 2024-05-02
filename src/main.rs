use ansi_term::Color::Red;
use confy;
use serde::{Deserialize, Serialize};
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};

#[derive(Serialize, Deserialize)]
enum Module {
    OS,
    Kernel,
    Host,
    CPU,
    Cores,
    Usage,
    Temp,
    Mem,
    Swap,
}

#[derive(Serialize, Deserialize)]
struct CustomParameters {
    os: Option<String>,
    kernel: Option<String>,
    host: Option<String>,
    cpu: Option<String>,
    cores: Option<String>,
    usage: Option<String>,
    temp: Option<String>,
    mem: Option<String>,
    swap: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct CustomLabels {
    os: String,
    kernel: String,
    host: String,
    cpu: String,
    cores: String,
    usage: String,
    temp: String,
    mem: String,
    swap: String,
}

#[derive(Serialize, Deserialize)]

struct Config {
    modules: Vec<Module>,
    custom_parameters: CustomParameters,
    custom_labels: CustomLabels,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            modules: vec![
                Module::OS,
                Module::Kernel,
                Module::Host,
                Module::CPU,
                Module::Cores,
                Module::Usage,
                Module::Temp,
                Module::Mem,
                Module::Swap,
            ],
            custom_labels: CustomLabels {
                os: String::from("OS"),
                kernel: String::from("Kernel"),
                host: String::from("Host"),
                cpu: String::from("CPU"),
                cores: String::from("Cores"),
                usage: String::from("Usage"),
                temp: String::from("Temp"),
                mem: String::from("Memory"),
                swap: String::from("Swap"),
            },
            custom_parameters: CustomParameters {
                os: Option::None,
                kernel: Option::None,
                host: Option::None,
                cpu: Option::None,
                cores: Option::None,
                usage: Option::None,
                temp: Option::None,
                mem: Option::None,
                swap: Option::None,
            },
        }
    }
}

fn print_crab() {
    println!("{}",Red.paint("     /\\\r\n    ( /   @ @    ()\r\n     \\  __| |__  /\r\n      -/   \"   \\-\r\n     /-|       |-\\\r\n    / /-\\     /-\\ \\\r\n     / /-`---\'-\\ \\\r\n      /         \\\r\n") );
}

fn main() {
    let cfg: Config = confy::load("crabfetch", None).unwrap();

    println!(" ");
    let left_pad = 25;
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut sys = System::new_all();
    sys.refresh_all();

    print_crab();

    let mut cursor_row = stdout.cursor_pos().unwrap().1;
    cursor_row = cursor_row - 9;
    println!(
        "{}{} : {}\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.os),
        cfg.custom_parameters.os.unwrap_or(sys.name().unwrap())
    );
    cursor_row = cursor_row + 1;
    println!(
        "{}{} : {}\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.kernel),
        cfg.custom_parameters
            .kernel
            .unwrap_or(sys.kernel_version().unwrap())
    );
    cursor_row = cursor_row + 1;

    println!(
        "{}{} : {}\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.host),
        cfg.custom_parameters
            .host
            .unwrap_or(sys.host_name().unwrap())
    );
    cursor_row = cursor_row + 1;

    println!(
        "{}{} : {}\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.cpu),
        cfg.custom_parameters
            .cpu
            .unwrap_or(sys.processors()[0].brand().to_string())
    );
    cursor_row = cursor_row + 1;

    println!(
        "{}{} : {}\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.cores),
        cfg.custom_parameters
            .cores
            .unwrap_or(sys.processors().len().to_string())
    );
    cursor_row = cursor_row + 1;

    let mut usage = 0;

    for processor in sys.processors() {
        usage = usage + processor.cpu_usage() as usize;
    }
    println!(
        "{}{} : {}%\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.usage),
        cfg.custom_parameters
            .usage
            .unwrap_or((usage / sys.processors().len()).to_string())
    );
    cursor_row = cursor_row + 1;

    for component in sys.components() {
        if component.label().contains("Package id") {
            println!(
                "{}{} : {}C\r",
                termion::cursor::Goto(left_pad, cursor_row),
                Red.bold().paint(cfg.custom_labels.temp),
                cfg.custom_parameters
                    .temp
                    .unwrap_or(component.temperature().to_string())
            );
            cursor_row = cursor_row + 1;
            break;
        }
    }

    println!(
        "{}{} : {}\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.mem),
        cfg.custom_parameters.mem.unwrap_or(format!(
            "{}MB/{}MB",
            sys.used_memory() / 1024,
            sys.total_memory() / 1024
        ))
    );
    cursor_row = cursor_row + 1;

    println!(
        "{}{} : {}\r",
        termion::cursor::Goto(left_pad, cursor_row),
        Red.bold().paint(cfg.custom_labels.swap),
        cfg.custom_parameters.swap.unwrap_or(format!(
            "{}MB/{}MB",
            sys.used_swap() / 1024,
            sys.total_swap() / 1024
        ))
    );
}
