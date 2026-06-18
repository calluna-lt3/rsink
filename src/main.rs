use md4::{Md4, Digest};
use std::{
    collections::HashMap, fs::File, io::{BufReader, Read}
};

#[derive(Debug)]
struct Args {
    src: String,
    dst: String,
    recursive: bool,
}

fn print_usage() {
    println!("Usage: rsink [OPTS..] SRC DST");
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut recursive = false;
    let mut src: Option<String> = None;
    let mut dst: Option<String> = None;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                print_usage();
                std::process::exit(0);
            }
            Short('r') | Long("recursive") => {
                recursive = true;
            }
            Value(val) if src.is_none() => {
                src = Some(val.string()?);
            }
            Value(val) if dst.is_none() => {
                dst = Some(val.string()?);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        src: src.ok_or("Missing src")?,
        dst: dst.ok_or("Missing dst")?,
        recursive,
    })
}

fn copyyyy(args: &Args) -> Result<(), anyhow::Error> {
    let dst = match File::open(&args.dst) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("bad src file {e:?}");
            std::process::exit(1);
        }
    };


    let rolling_to_md4: HashMap<[u8; 32], [u8; 128]> = HashMap::new();
    // calc rolling checksums
    let dst_reader = BufReader::new(dst);
    let mut hasher = Md4::new();
    const S: usize = 512;
    const M: u32 = 2_u32.pow(16);

    let mut dst_bytes = dst_reader.bytes();
    let mut buf: [u8; S] = [0; 512];
    let mut buf_offset = 0;
    while let Some(Ok(byte)) = dst_bytes.next() {
        if buf_offset == 512 {
            buf_offset = 0;
            md4 = Md4::digest(buf);
            // add to hm
        }

        // calc rolling
        buf[buf_offset] = byte;
    }

    // store in hashmap

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = parse_args()?;

    if !args.recursive {
        copyyyy(&args)?;
    } else {
        todo!("recursive");
        // NOTE: bfs
    }

    Ok(())
}
