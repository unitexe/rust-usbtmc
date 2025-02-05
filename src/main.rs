use rust_usbtmc::instrument::Instrument;
use clap::Parser;

// Identifiers for SIGLENT SPD 3303C
const VID: u16 = 0xf4ec;
const PID: u16 = 0x1430;

// Commands for SIGLENT SPD 3303C
const IDN: &str = "*IDN?";
const SYSTEM_STATUS: &str = "SYSTem:STATus?";
const CH1_ENABLE: &str = "OUTPut CH1,ON";
const CH1_DISABLE: &str = "OUTPut CH1,OFF";
const CH2_ENABLE: &str = "OUTPut CH2,ON";
const CH2_DISABLE: &str = "OUTPut CH2,OFF";
const CH3_ENABLE: &str = "OUTPut CH3,ON";
const CH3_DISABLE: &str = "OUTPut CH3,OFF";

#[derive(Parser)]
#[command(name = "psctrl")]
#[command(version = "0.1")]
#[command(about = "Controls a Siglent SPD 3303C")]
struct Cli {
    #[arg(long)]
    cmd: String,
}

fn main() {
    let cli = Cli::parse();
    let mut instr_single = Instrument::new(VID, PID);
    let cli_cmd_str = cli.cmd.as_str();
    
    match cli_cmd_str {
        IDN | SYSTEM_STATUS => println!("Ask: {}", instr_single.ask(cli_cmd_str).unwrap()),
        CH1_ENABLE | CH1_DISABLE | CH2_ENABLE | CH2_DISABLE | CH3_ENABLE | CH3_DISABLE => instr_single.write(cli_cmd_str).unwrap(),
        _ => println!("Command not supported"),
    }
}
