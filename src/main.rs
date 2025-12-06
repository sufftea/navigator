use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

    'outer: for depth in 0..u32::MAX {
        let mut find = Command::new("find")
            .args([
                "-L".into(),
                args.root_dir.clone(),
                "-mindepth".into(),
                format!("{depth}"),
                "-maxdepth".into(),
                format!("{depth}"),
                "-type".into(),
                "d".into(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let find_output = find.stdout.take().unwrap();
        let mut find_reader = BufReader::new(find_output);

        loop {
            let mut dir = String::new();
            find_reader.read_line(&mut dir).unwrap();

            let write_result = fzf_input.write_all(dir.as_bytes());

            if fzf.try_wait().unwrap().is_some() || write_result.is_err() {
                let output = fzf.wait_with_output().unwrap();
                println!("{}", String::from_utf8(output.stdout).unwrap());
                find.kill().unwrap();
                break 'outer;
            }

            if find.try_wait().unwrap().is_some() {
                break;
            }
        }
    }
}
