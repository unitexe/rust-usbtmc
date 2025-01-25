use rust_usbtmc::instrument::Instrument;

// Identifiers for SIGLENT SPD 3303C
const VID: u16 = 0xf4ec;
const PID: u16 = 0x1430;

// Commands for SIGLENT SPD 3303C
const IDN: &str = "*IDN?";
const SYSTEM_STATUS: &str = "SYSTem:STATus?";

fn main() {
    let mut instr_single = Instrument::new(VID, PID);

    // Query the manufacturer, product model, series NO. and software version.
    // Example output: Siglent Technologies,SPD3303C,SPD3EGGD7R2531,1.02.01.02.02R3,V2.0
    println!("Ask: {}", instr_single.ask(IDN).unwrap());

    // Query the current working state.
    // Return info is in hexidecimal format. Binary correspondence relationship:
    //
    // Bit      Corresponding State
    // ----     --------------------
    // 0        0: CH1 CV mode; 1: CH1 CC mode
    // 1        0: CH2 CV mode; 1: CH2 CC mode
    // 2,3      01: Independent mode; 10: Parallel mode; 11: Series mode
    // 4        0: CH1 OFF 1: CH1 ON
    // 5        0: CH2 OFF 1: CH2 ON
    // 
    // Example output: 0x000C
    println!("Ask: {}", instr_single.ask(SYSTEM_STATUS).unwrap());
}
