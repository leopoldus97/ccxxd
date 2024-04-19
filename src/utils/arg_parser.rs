use clap::{
    builder::{ArgPredicate, OsStr},
    Arg, ArgAction, Command,
};

#[derive(Debug)]
pub struct ArgumentMatches {
    pub input: Option<String>,
    pub output: Option<String>,
    pub cols: u16,
    pub little_endian: bool,
    pub group_size: u8,
    pub len: u32,
    pub reverse: bool,
    pub seek: u32,
}

fn setup_parser() -> Command {
    Command::new("ccxxd")
        .about("A simple implementation of the command line tool `xxd` in Rust.")
        .arg(
            Arg::new("input")
                .help("Sets the input file to use")
                .index(1)
                .num_args(1)
        )
        .arg(
            Arg::new("output")
                .help("Sets the output file to use")
                .index(2)
                .num_args(1)
        )
        .arg(
            Arg::new("cols")
                .short('c')
                .value_name("cols")
                .help("Format <cols> octets per line. Default 16. Max 256.")
                .value_parser(clap::value_parser!(u16).range(1..=256))
                .default_value("16")
        )
        .arg(
            Arg::new("little_endian")
                .short('e')
                .help("Switch to little-endian hexdump.")
                .long_help("Switch to little-endian hexdump. This option treats byte groups as words in little-endian byte order. The default grouping of 4 bytes may be changed using -g. This option only applies to hexdump, leaving the ASCII (or EBCDIC) representation unchanged. The command line switch -r does not work with this mode.")
                .action(ArgAction::Count)
        )
        .arg(
            Arg::new("group_size")
                .short('g')
                .value_name("bytes")
                .help("Separate the output of every <bytes> bytes (two hex characters or eight bit-digits each) by a whitespace.")
                .long_help("Separate the output of every <bytes> bytes (two hex characters or eight bit-digits each) by a whitespace. Specify -g 0 to suppress grouping. <Bytes> defaults to 2 in normal mode, 4 in little-endian mode and 1 in bits mode. Grouping does not apply to postscript or include style.")
                .value_parser(clap::value_parser!(u8))
                .default_value_if("little_endian", ArgPredicate::Equals(OsStr::from("1")), Some("4"))
                .default_value("2")
        )
        .arg(
            Arg::new("len")
                .short('l')
                .value_name("len")
                .help("Stop after writing <len> octets.")
                .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .help("Reverse operation: convert (or patch) hexdump into binary.")
                .long_help("Reverse operation: convert (or patch) hexdump into binary. If not writing to stdout, xxd writes into its output file without truncating it. Use the combination -r -p to read plain hexadecimal dumps without line number information and  without a particular column layout. Additional Whitespace and line-breaks are allowed anywhere.")
                .action(ArgAction::Count)
        )
        .arg(
            Arg::new("seek")
                .short('s')
                .value_name("bytes")
                .help("Start at <seek> bytes abs. (or rel.) infile offset.")
                .value_parser(clap::value_parser!(u32))
                .default_value("0")
        )
}

pub fn get_matches() -> ArgumentMatches {
    let command = setup_parser();
    let matches = command.get_matches();

    let input = matches.get_one::<String>("input").cloned();
    let output = matches.get_one::<String>("output").cloned();
    let cols = matches.get_one::<u16>("cols").copied().unwrap_or_default();
    let little_endian = matches.get_count("little_endian") > 0;
    let group_size = matches
        .get_one::<u8>("group_size")
        .copied()
        .unwrap_or_default();
    let len = matches
        .get_one::<u32>("len")
        .copied()
        .unwrap_or(std::u32::MAX);
    let reverse = matches.get_count("reverse") > 0;
    let seek = matches.get_one::<u32>("seek").copied().unwrap_or_default();

    ArgumentMatches {
        input,
        output,
        cols,
        little_endian,
        group_size,
        len,
        reverse,
        seek,
    }
}
