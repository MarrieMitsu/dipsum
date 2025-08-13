use std::collections::HashMap;

use arboard::Clipboard;
use clap::Parser;
use rand::prelude::*;

#[macro_use]
extern crate lazy_static;

const DICTIONARY: &[u8] = include_bytes!("../lorem-ipsum.txt");
const DICTIONARY_LEN: usize = DICTIONARY.len();

#[derive(Debug)]
struct Location {
    start: usize,
    end: usize,
}

lazy_static! {
    static ref SENTENTIAE: HashMap<usize, Location> = {
        let mut m: HashMap<usize, Location> = HashMap::new();
        let mut count = 0;
        let mut start = 0;

        for i in 0..DICTIONARY_LEN {
            let byte = DICTIONARY[i];

            if byte == 10 {
                count += 1;

                let end = if DICTIONARY[i - 1] == 13 {
                    i - 2
                } else {
                    i - 1
                };

                m.insert(count, Location { start, end });

                if DICTIONARY_LEN > i + 1 {
                    start = i + 1;
                }
            }
        }

        m
    };
    static ref SENTENTIA_TOTAL: usize = SENTENTIAE.len();
}

struct Generator<'a> {
    rng: &'a mut ThreadRng,
}

impl<'a> Generator<'a> {
    fn new(rng: &'a mut ThreadRng) -> Self {
        Generator { rng }
    }

    fn sententia_slice(&mut self) -> &[u8] {
        let index = self.rng.random_range(1..=*SENTENTIA_TOTAL);
        let loc = SENTENTIAE.get(&index).unwrap();

        &DICTIONARY[loc.start..=loc.end]
    }

    fn paragraph(&mut self, amount: usize) -> String {
        let mut buffer: Vec<u8> = vec![];

        for i in 0..amount {
            let len = self.rng.random_range(3..=5);

            for j in 0..len {
                buffer.extend_from_slice(self.sententia_slice());

                // next sentence delimiter
                if j + 1 != len {
                    buffer.push(32);
                }
            }

            // next paragraph delimiter
            if i + 1 != amount {
                buffer.extend_from_slice(&[13, 10, 13, 10]);
            }
        }

        String::from_utf8(buffer).unwrap()
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Copy to clipboard
    #[arg(short, long)]
    copy: bool,

    /// Paragraph amount
    amount: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let mut rng = rand::rng();
    let mut clipboard = Clipboard::new().unwrap();
    let mut generator = Generator::new(&mut rng);

    let amount = if let Some(e) = cli.amount { e } else { 1 };

    let paragraph = generator.paragraph(amount);

    if cli.copy {
        clipboard.set_text(paragraph.clone()).unwrap();
    }

    println!("{paragraph}");
}
