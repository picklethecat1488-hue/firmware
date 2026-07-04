use defmt_decoder::Table;
use defmt_host::{dump_logs, stream_logs, DefmtLogSource};
use probe_rs::probe::list::Lister;
use std::io;
use std::time::Duration;

struct ProbeRttSource<'a, 'b> {
    channel: probe_rs::rtt::UpChannel,
    core: &'a mut probe_rs::Core<'b>,
}

impl<'a, 'b> DefmtLogSource for ProbeRttSource<'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        self.channel
            .read(self.core, buf)
            .map_err(|e| format!("RTT read failed: {:?}", e))
    }
}

pub fn run_rtt(chip: &str, table: &Table, dump: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to probe for chip '{}'...", chip);
    let lister = Lister::new();
    let probes = lister.list_all();
    let probe_info = probes.first().ok_or("No debug probes connected")?;
    let probe = probe_info.open()?;

    let mut session = probe.attach(chip, probe_rs::Permissions::default())?;
    let mut core = session.core(0)?;

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

    if dump {
        println!("Draining buffered defmt logs:\n");
        dump_logs(source, table, io::stdout())?;
    } else {
        println!("Streaming defmt logs (Ctrl+C to stop):\n");
        stream_logs(
            source,
            table,
            io::stdout(),
            Duration::from_millis(10),
            || false, // Keep running forever
        )?;
    }

    Ok(())
}
