use md4::{Md4, Digest};
use std::{
    collections::HashMap, fs::File, io::{BufReader, Read}
};

const M: u64 = 2_u64.pow(16);
const S: usize = 512;

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

// returns a u32 because as stated in the paper (pg 2)
// 'We have found that values of S between 500 and 1000 are quite good for most purposes'
// and u8::MAX (256) * 1024 < u64::MAX
fn a_init(input: &[u8]) -> u64 {
    input.iter().map(|x| *x as u64).sum::<u64>() % M
}

// regarding S - i, this is safe as S is some # and i is in the range [i..=#]
// so it will never overflow into negatives
fn b_init(input: &[u8]) -> u64 {
    input.iter().enumerate().map(|(i, x)| ((S - i + 1) as u64) * (*x as u64)).sum::<u64>() % M
}

// xk => value to be swapped out
// xn => value to be swapped in
// e.g. if going from 0 - 511 to 1 - 512, xk is buf[0] and xn is buf[512]
fn _a_rolling(a: u64, xk: u8, xn: u8) -> u64 {
    (a - xk as u64 + xn as u64) % M
}

// k => lower bound new index
// l => upper bound new index
// e.g. if going from 0 - 511 to 1 - 512, k is 1, l is 512
fn _b_rolling(a_next: u64, b: u64, xk: u8, k: usize, l: usize) -> u64 {
    (b - (l as u64 - k as u64 + 1) * xk as u64 + a_next) % M
}

fn s(a: u64, b: u64) -> u64 {
    a + M * b
}

fn copyyyy(args: &Args) -> Result<(), anyhow::Error> {
    let dst = match File::open(&args.dst) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("bad dst file {e:?}");
            std::process::exit(1);
        }
    };


    // calc checksums !!!
    let mut rolling_to_md4: HashMap<u64, u128> = HashMap::new();
    let dst_reader = BufReader::new(dst);
    let mut dst_bytes = dst_reader.bytes();
    let mut buf: [u8; S] = [0; S];
    let mut buf_offset = 0;
    let mut total_bytes_read = 0;

    while let Some(Ok(byte)) = dst_bytes.next() {
        buf[buf_offset] = byte;
        buf_offset += 1;
        total_bytes_read += 1;

        if buf_offset == S {
            // buffer is now filled with data, calc rolling + md4
            // TODO: use a_next//b_next
            let rolling = s(a_init(&buf), b_init(&buf));
            let md4 = Md4::digest(buf);
            let md4 = u128::from_be_bytes(md4.into());
            rolling_to_md4.insert(rolling, md4);
            buf_offset = 0;

            println!("Checksums for {} -> {}:", total_bytes_read as i64 - S as i64, total_bytes_read - 1);
            println!("  rolling: {:x}", rolling);
            println!("      md4: {:x}", md4);
        }
    }

    // remaining bytes
    let rolling = s(a_init(&buf[0..(total_bytes_read % S)]), b_init(&buf[0..(total_bytes_read % S)]));
    let md4 = Md4::digest(&buf[0..(total_bytes_read % S)]);
    let md4 = u128::from_be_bytes(md4.into());
    rolling_to_md4.insert(rolling, md4);

    println!("Checksums for {} -> {}:", total_bytes_read - (total_bytes_read % S), total_bytes_read - 1);
    println!("  rolling: {:x}", rolling);
    println!("      md4: {:x}", md4);

    println!("hashmap: {:?}", rolling_to_md4);

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
