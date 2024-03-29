use std::io::{stdin, stdout, BufWriter, Error, ErrorKind, Read as _, Write as _};
use std::time::Instant;

use env_logger;
#[cfg(debug_assertions)]
use log::debug;
use log::{error, info, warn};

use humnum::HumanNumericLine;
use humnum::Options;

fn main() -> Result<(), Error> {
    let args: Options = Options::new();
    let now = Instant::now();
    env_logger::init();
    let mut out = BufWriter::new(stdout().lock());
    let mut stdinbuf = vec![];
    {
        let mut lines = stdin().lock();
        let res = lines.read_to_end(&mut stdinbuf);
        if let Err(e) = res {
            let ek = e.kind();
            match ek {
                ErrorKind::UnexpectedEof => warn!("Got EOF we didn't anticipate"),
                _ => error!("Got unexpected I/O error: {:?}", e),
            }
        }
    }
    let unsorted: Vec<&[u8]> = stdinbuf.split_inclusive(|c| *c == '\n' as u8).collect();
    info!(
        "Reading done; got {} lines in {:.4}µs",
        unsorted.len(),
        now.elapsed().as_micros()
    );
    let now = Instant::now();
    let sorted_refs = {
        let mut unsorted_refs: Vec<HumanNumericLine> = unsorted
            .into_iter()
            .map(|l| HumanNumericLine::new(l, args.mode))
            .collect();
        // Strangely, for my tested data, this is significantly faster than .sort_unstable()
        unsorted_refs.sort();
        unsorted_refs
    };
    info!(
        "Sorting done; sorted {} lines in {:.4}µs",
        sorted_refs.len(),
        now.elapsed().as_micros()
    );
    let now = Instant::now();
    #[cfg(debug_assertions)]
    debug!("Sorted now {:#?}", &sorted_refs);
    for r in sorted_refs {
        let res = out.write(r.buf);
        if let Err(e) = res {
            error!("Failed to write to stdout! {:?}", e);
            break;
        }
    }
    if let Err(e) = out.flush() {
        error!("Failed to write to (flush) stdout! {:?}", e);
        Err(e)
    } else {
        info!("Writing done; wrote in {0:.4}µs", now.elapsed().as_micros());
        Ok(())
    }
}
