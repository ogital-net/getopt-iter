//! Benchmarks comparing `getopt-iter` against `getargs` and `getopts`.
//!
//! Run with: `cargo bench --bench versus`
//!
//! The input set is inspired by the `getargs` crate's own `versus` bench
//! (<https://github.com/j-tai/getargs/tree/master/bench>)
//!
//! Two scenarios are exercised:
//!   * `short_only` — only short options (`-a`, `-b value`, ...).
//!   * `mixed`      — short and long options together. `getopt-iter` supports
//!     long options via the parenthesized optstring syntax (e.g.
//!     `"1(present1)4:(val1)"`), so all three parsers participate.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------

/// Short-option-only input (compatible with POSIX `getopt`).
///
/// Mix of present flags and short options that take values, both as
/// separate args (`-4 value1`) and clustered (`-5value2`).
const SHORT_ARGS: &[&str] = &[
    "-1",       // short flag 1
    "-3",       // short flag 3
    "-4",       // short value 1 (separate)
    "value1",   //
    "-5value2", // short value 2 (clustered)
    "-6",       // short value 3 (separate)
    "value3",   //
    "-2",       // short flag 2
    "file.txt", // positional
];

/// Mixed short + long input (compatible with `getopts` / `getargs`).
const MIXED_ARGS: &[&str] = &[
    "-1",
    "--present1",
    "-3",
    "--present3",
    "-4",
    "value1",
    "--val1",
    "value1",
    "-5value2",
    "--val2=value2",
    "-6",
    "value3",
    "--val3",
    "value3",
    "file.txt",
];

#[derive(Default)]
struct Settings {
    short_present1: bool,
    short_present2: bool,
    short_present3: bool,
    long_present1: bool,
    long_present2: bool,
    long_present3: bool,
    short_value1: Option<String>,
    short_value2: Option<String>,
    short_value3: Option<String>,
    long_value1: Option<String>,
    long_value2: Option<String>,
    long_value3: Option<String>,
}

// ---------------------------------------------------------------------------
// Short-only parsers
// ---------------------------------------------------------------------------

fn parse_short_getopt_iter(args: &[&'static str]) -> Settings {
    use getopt_iter::Getopt;

    let mut settings = Settings::default();
    // `Getopt::new` consumes argv[0] as the program name.
    let argv = std::iter::once("bench").chain(args.iter().copied());
    let mut opts = Getopt::new(argv, ":1234:5:6:");
    opts.set_opterr(false);

    for opt in opts {
        match opt.val() {
            '1' => settings.short_present1 = true,
            '2' => settings.short_present2 = true,
            '3' => settings.short_present3 = true,
            '4' => settings.short_value1 = opt.into_arg().map(|c| c.into_owned()),
            '5' => settings.short_value2 = opt.into_arg().map(|c| c.into_owned()),
            '6' => settings.short_value3 = opt.into_arg().map(|c| c.into_owned()),
            _ => {}
        }
    }
    settings
}

fn parse_short_getargs(args: &[&'static str]) -> Settings {
    use getargs::{Opt, Options};

    let mut settings = Settings::default();
    let mut opts = Options::new(args.iter().copied());

    while let Some(opt) = opts.next_opt().unwrap() {
        match opt {
            Opt::Short('1') => settings.short_present1 = true,
            Opt::Short('2') => settings.short_present2 = true,
            Opt::Short('3') => settings.short_present3 = true,
            Opt::Short('4') => {
                settings.short_value1 = Some(opts.value().unwrap().to_string());
            }
            Opt::Short('5') => {
                settings.short_value2 = Some(opts.value().unwrap().to_string());
            }
            Opt::Short('6') => {
                settings.short_value3 = Some(opts.value().unwrap().to_string());
            }
            _ => {}
        }
    }
    settings
}

fn parse_short_getopts(args: &[&'static str]) -> Settings {
    use getopts::{HasArg, Occur, Options};

    let mut settings = Settings::default();
    let mut opts = Options::new();
    opts.optflag("1", "", "");
    opts.optflag("2", "", "");
    opts.optflag("3", "", "");
    opts.opt("4", "", "", "", HasArg::Yes, Occur::Optional);
    opts.opt("5", "", "", "", HasArg::Yes, Occur::Optional);
    opts.opt("6", "", "", "", HasArg::Yes, Occur::Optional);

    let matches = opts.parse(args).unwrap();
    settings.short_present1 = matches.opt_present("1");
    settings.short_present2 = matches.opt_present("2");
    settings.short_present3 = matches.opt_present("3");
    settings.short_value1 = matches.opt_str("4");
    settings.short_value2 = matches.opt_str("5");
    settings.short_value3 = matches.opt_str("6");
    settings
}

// ---------------------------------------------------------------------------
// Mixed (short + long) parsers
// ---------------------------------------------------------------------------

fn parse_mixed_getargs(args: &[&'static str]) -> Settings {
    use getargs::{Opt, Options};

    let mut settings = Settings::default();
    let mut opts = Options::new(args.iter().copied());

    while let Some(opt) = opts.next_opt().unwrap() {
        match opt {
            Opt::Short('1') => settings.short_present1 = true,
            Opt::Short('2') => settings.short_present2 = true,
            Opt::Short('3') => settings.short_present3 = true,
            Opt::Long("present1") => settings.long_present1 = true,
            Opt::Long("present2") => settings.long_present2 = true,
            Opt::Long("present3") => settings.long_present3 = true,
            Opt::Short('4') => {
                settings.short_value1 = Some(opts.value().unwrap().to_string());
            }
            Opt::Short('5') => {
                settings.short_value2 = Some(opts.value().unwrap().to_string());
            }
            Opt::Short('6') => {
                settings.short_value3 = Some(opts.value().unwrap().to_string());
            }
            Opt::Long("val1") => {
                settings.long_value1 = Some(opts.value().unwrap().to_string());
            }
            Opt::Long("val2") => {
                settings.long_value2 = Some(opts.value().unwrap().to_string());
            }
            Opt::Long("val3") => {
                settings.long_value3 = Some(opts.value().unwrap().to_string());
            }
            _ => {}
        }
    }
    settings
}

fn parse_mixed_getopt_iter(args: &[&'static str]) -> Settings {
    use getopt_iter::Getopt;

    let mut settings = Settings::default();
    let argv = std::iter::once("bench").chain(args.iter().copied());
    // Parenthesized names bind long options to their short-option character.
    let mut opts = Getopt::new(
        argv,
        ":1(present1)2(present2)3(present3)4:(val1)5:(val2)6:(val3)",
    );
    opts.set_opterr(false);

    for opt in opts {
        match opt.val() {
            '1' => settings.short_present1 = true,
            '2' => settings.short_present2 = true,
            '3' => settings.short_present3 = true,
            '4' => settings.short_value1 = opt.into_arg().map(|c| c.into_owned()),
            '5' => settings.short_value2 = opt.into_arg().map(|c| c.into_owned()),
            '6' => settings.short_value3 = opt.into_arg().map(|c| c.into_owned()),
            _ => {}
        }
    }
    // NOTE: `getopt-iter` reports the bound short character for long options,
    // so the `long_*` Settings fields stay default. The parser still does the
    // full long-option matching work, which is what we are timing.
    settings
}

fn parse_mixed_getopts(args: &[&'static str]) -> Settings {
    use getopts::{HasArg, Occur, Options};

    let mut settings = Settings::default();
    let mut opts = Options::new();
    opts.optflag("1", "", "");
    opts.optflag("2", "", "");
    opts.optflag("3", "", "");
    opts.optflag("", "present1", "");
    opts.optflag("", "present2", "");
    opts.optflag("", "present3", "");
    opts.opt("4", "", "", "", HasArg::Yes, Occur::Optional);
    opts.opt("5", "", "", "", HasArg::Yes, Occur::Optional);
    opts.opt("6", "", "", "", HasArg::Yes, Occur::Optional);
    opts.opt("", "val1", "", "", HasArg::Yes, Occur::Optional);
    opts.opt("", "val2", "", "", HasArg::Yes, Occur::Optional);
    opts.opt("", "val3", "", "", HasArg::Yes, Occur::Optional);

    let matches = opts.parse(args).unwrap();
    settings.short_present1 = matches.opt_present("1");
    settings.short_present2 = matches.opt_present("2");
    settings.short_present3 = matches.opt_present("3");
    settings.long_present1 = matches.opt_present("present1");
    settings.long_present2 = matches.opt_present("present2");
    settings.long_present3 = matches.opt_present("present3");
    settings.short_value1 = matches.opt_str("4");
    settings.short_value2 = matches.opt_str("5");
    settings.short_value3 = matches.opt_str("6");
    settings.long_value1 = matches.opt_str("val1");
    settings.long_value2 = matches.opt_str("val2");
    settings.long_value3 = matches.opt_str("val3");
    settings
}

// ---------------------------------------------------------------------------
// Criterion groups
// ---------------------------------------------------------------------------

fn bench_short_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("short_only");
    group.throughput(Throughput::Elements(SHORT_ARGS.len() as u64));

    group.bench_function(BenchmarkId::new("getopt_iter", SHORT_ARGS.len()), |b| {
        b.iter(|| parse_short_getopt_iter(black_box(SHORT_ARGS)))
    });
    group.bench_function(BenchmarkId::new("getargs", SHORT_ARGS.len()), |b| {
        b.iter(|| parse_short_getargs(black_box(SHORT_ARGS)))
    });
    group.bench_function(BenchmarkId::new("getopts", SHORT_ARGS.len()), |b| {
        b.iter(|| parse_short_getopts(black_box(SHORT_ARGS)))
    });

    group.finish();
}

fn bench_mixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed");
    group.throughput(Throughput::Elements(MIXED_ARGS.len() as u64));

    group.bench_function(BenchmarkId::new("getopt_iter", MIXED_ARGS.len()), |b| {
        b.iter(|| parse_mixed_getopt_iter(black_box(MIXED_ARGS)))
    });
    group.bench_function(BenchmarkId::new("getargs", MIXED_ARGS.len()), |b| {
        b.iter(|| parse_mixed_getargs(black_box(MIXED_ARGS)))
    });
    group.bench_function(BenchmarkId::new("getopts", MIXED_ARGS.len()), |b| {
        b.iter(|| parse_mixed_getopts(black_box(MIXED_ARGS)))
    });

    group.finish();
}

criterion_group!(benches, bench_short_only, bench_mixed);
criterion_main!(benches);
