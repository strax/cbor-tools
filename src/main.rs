use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::{BufReader, Error, Read};
use std::iter::FromIterator;

use base64;
use cbor;
use cbor::{Cbor, CborFloat};
use clap::{App, Arg, ArgMatches, SubCommand};
use json::number::Number;
use json::{object, JsonValue};

struct Input<'a>(Box<dyn io::BufRead + 'a>);

impl<'a> Input<'a> {
    fn file<S: AsRef<str>>(path: S) -> io::Result<Input<'a>> {
        let file = fs::File::open(path.as_ref())?;
        Ok(Input(Box::new(io::BufReader::new(file))))
    }

    fn stdin(stream: &'a io::Stdin) -> Input<'a> {
        Input(Box::new(stream.lock()))
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'a> io::BufRead for Input<'a> {
    fn fill_buf(&mut self) -> Result<&[u8], Error> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

fn to_json(object: &Cbor) -> JsonValue {
    match object {
        Cbor::Bool(b) => JsonValue::Boolean(*b),
        Cbor::Signed(n) => JsonValue::Number(n.into_i64().into()),
        Cbor::Unsigned(z) => JsonValue::Number(z.into_u64().into()),
        Cbor::Float(r) => JsonValue::Number(r.into_f64().into()),
        Cbor::Null => JsonValue::Null,
        Cbor::Unicode(ref s) => JsonValue::String(s.clone()),
        Cbor::Undefined => JsonValue::Null,
        Cbor::Array(xs) => JsonValue::Array(xs.iter().map(to_json).collect()),
        Cbor::Map(kv) => JsonValue::Object(json::object::Object::from_iter(
            kv.iter().map(|(k, v)| (k, to_json(v))),
        )),
        Cbor::Bytes(cbor::CborBytes(ref bytes)) => JsonValue::String(base64::encode(bytes)),
        Cbor::Tag(x) => to_json(&x.data),
        _ => unreachable!(),
    }
}

fn command_dump(input: Input) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = cbor::Decoder::from_reader(input);
    for item in stream.items() {
        let item = item.expect("parse error");
        print!("{}", to_json(&item).pretty(2));
    }
    Ok(())
}

fn command_head(input: Input) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = cbor::Decoder::from_reader(input);
    let mut stdout = io::stdout();
    let mut encoder = cbor::Encoder::from_writer(stdout);
    let item = stream.items().take(1).filter_map(|res| res.ok());
    encoder.encode(item)?;
    encoder.flush()?;
    Ok(())
}

fn input<'a>(args: &ArgMatches, stdin: &'a io::Stdin) -> Input<'a> {
    match args.value_of("FILE") {
        Some(path) => Input::file(path).expect("cannot open"),
        None => {
            eprintln!("FILE not provided, reading from STDIN");
            Input::stdin(&stdin)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("cbor")
        .about("Utilities for working with CBOR data")
        .bin_name("cbor")
        .version(clap::crate_version!())
        .subcommand(
            SubCommand::with_name("dump")
                .about("converts the input CBOR data to JSON")
                .arg(Arg::with_name("FILE").index(1)),
        )
        .subcommand(
            SubCommand::with_name("head")
                .about("prints the first item of the input CBOR sequence to stdout")
                .arg(Arg::with_name("FILE").index(1)),
        );
    let args = app.clone().get_matches();
    let stdin = io::stdin();

    match args.subcommand() {
        ("dump", Some(args)) => command_dump(input(args, &stdin))?,
        ("head", Some(args)) => command_head(input(args, &stdin))?,
        _ => app.print_help()?,
    };
    Ok(())
}
