//! Host command-line utility for streaming defmt logs via RTT from an attached target device.

use clap::Parser;
use log_tool::{decode_project_info, stream_logs, RttLogSource};
use probe_rs::probe::list::Lister;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Streams defmt logs from a target device via RTT"
)]
pub struct Cli {
    /// Path to the ELF binary containing defmt table and symbols
    #[arg(short, long)]
    pub elf: PathBuf,

    /// Chip name (e.g., "rp2040"). If omitted, --project must be specified.
    #[arg(short, long)]
    pub chip: Option<String>,

    /// Project name (e.g., "cat_detector") to auto-detect chip name
    #[arg(short, long)]
    pub project: Option<String>,
}

struct ProbeRttSource<'a, 'b> {
    channel: probe_rs::rtt::UpChannel,
    core: &'a mut probe_rs::Core<'b>,
}

impl<'a, 'b> RttLogSource for ProbeRttSource<'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        self.channel
            .read(self.core, buf)
            .map_err(|e| format!("RTT read failed: {:?}", e))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 1. Get chip name
    let chip = if let Some(chip_name) = cli.chip {
        chip_name
    } else if let Some(project_name) = cli.project {
        let (project_chip, _, _) = decode_project_info(&project_name)?;
        project_chip
    } else {
        return Err("Either --chip or --project must be specified".into());
    };

    // 2. Parse defmt table from ELF
    let elf_data = fs::read(&cli.elf)?;
    let table = defmt_decoder::Table::parse(&elf_data)?
        .ok_or("Could not parse defmt table from the specified ELF file")?;

    // 3. Connect to the debug probe
    println!("Connecting to probe for chip '{}'...", chip);
    let lister = Lister::new();
    let probes = lister.list_all();
    let probe_info = probes.first().ok_or("No debug probes connected")?;
    let probe = probe_info.open()?;

    let mut session = probe.attach(chip, probe_rs::Permissions::default())?;
    let mut core = session.core(0)?;

    // 4. Attach to RTT
    println!("Attaching to RTT...");
    let mut rtt = match probe_rs::rtt::Rtt::attach(&mut core) {
        Ok(rtt) => rtt,
        Err(e) => {
            return Err(format!("Failed to attach to RTT: {:?}", e).into());
        }
    };

    let up_channel = rtt.up_channels.remove(0);
    let source = ProbeRttSource {
        channel: up_channel,
        core: &mut core,
    };

    println!("Streaming defmt logs (Ctrl+C to stop):\n");

    stream_logs(
        source,
        &table,
        io::stdout(),
        Duration::from_millis(10),
        || false, // Keep running forever
    )?;

    Ok(())
}
