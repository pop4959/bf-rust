use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::option::Option::Some;

fn main() {
    let contents = fs::read_to_string(&env::args().nth(1).expect("No file argument provided")).expect("Unable to read file");
    let code = contents.as_bytes();
    let is = code.len();
    let ds = match env::args().nth(2) {
        Some(n) => n.parse().expect("Unable to read size"),
        None => 30000
    };
    let mut data = vec![0u8; ds];
    let mut jump = HashMap::new();
    let mut jump_stack = vec![];
    for ip in 0..is {
        match code[ip] as char {
            '[' => jump_stack.push(ip),
            ']' => {
                let jp = jump_stack.pop().expect("Unmatched brackets");
                jump.insert(ip, jp);
                jump.insert(jp, ip);
            }
            _ => continue
        }
    }
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut ip = 0;
    let mut dp = 0;
    while ip < is && dp < ds {
        match code[ip] as char {
            '>' => dp += 1,
            '<' => dp -= 1,
            '+' => data[dp] = if data[dp] == 255 { 0 } else { data[dp] + 1 },
            '-' => data[dp] = if data[dp] == 0 { 255 } else { data[dp] - 1 },
            '.' => { print!("{}", data[dp] as char); io::stdout().flush().expect("Unable to flush output"); },
            ',' => input.read_exact(&mut data[dp..dp + 1]).expect("Unable to read input"),
            '[' => if data[dp] == 0 { ip = *jump.get(&ip).expect("Unknown jump"); },
            ']' => if data[dp] != 0 { ip = *jump.get(&ip).expect("Unknown jump"); },
            _ => ()
        }
        ip += 1;
    }
}
