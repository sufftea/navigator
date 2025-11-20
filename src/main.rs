use std::{
    io::{BufRead, BufReader, Read, Stdin, Write, stdout},
    process::{Command, Stdio},
    u64,
};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg()]
    root_dir: String,
}

fn main() {
    let args = Args::parse();

    let mut fzf = Command::new("fzf")
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut fzf_input = fzf.stdin.take().unwrap();

    // find dir -mindepth $i -maxdepth $i -follow \( -type d -o -type l \)
    'outer: for depth in 0..u32::MAX {
        let mut find = Command::new("find")
            .arg(args.root_dir.clone())
            .arg("-mindepth")
            .arg(format!("{depth}"))
            .arg("-maxdepth")
            .arg(format!("{depth}"))
            .arg("(")
            .arg("-type")
            .arg("d")
            .arg("-o")
            .arg("-type")
            .arg("l")
            .arg(")")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let find_output = find.stdout.take().unwrap();
        let mut find_reader = BufReader::new(find_output);

        loop {
            let mut dir = String::new();
            find_reader.read_line(&mut dir).unwrap();

            if fzf.try_wait().unwrap().is_some() {
                let output = fzf.wait_with_output().unwrap();

                println!("{}", String::from_utf8(output.stdout).unwrap());
                find.kill().unwrap();
                break 'outer;
            }

            fzf_input.write_all(dir.as_bytes()).unwrap();

            if find.try_wait().unwrap().is_some() {
                break;
            }
        }

        // find.kill().unwrap();
        // find.wait().unwrap();
    }
}
