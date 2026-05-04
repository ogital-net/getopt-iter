# getopt-iter

A POSIX-compliant command-line option parser for Rust.

This library provides a `getopt` implementation that closely follows POSIX conventions while also supporting GNU-style long options. It's designed to be flexible, ergonomic, and suitable for both `std` and `no_std` environments.

## Features

- **POSIX-compliant** option parsing
- **GNU long options** with `--option` syntax
- **Short option aggregation** (`-abc` equivalent to `-a -b -c`)
- **Flexible argument handling** (optional, required, attached, or separate)
- **Iterator-based API** for ergonomic usage
- **`no_std` support** with optional `std` feature
- **Zero dependencies**
- **Type-safe option parsing** with compile-time checks
- **Works with multiple string types** (`&str`, `String`, `OsString`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
getopt-iter = "1.0.1"
```

For `no_std` environments, disable default features:

```toml
[dependencies]
getopt-iter = { version = "1.0.1", default-features = false }
```

## Quick Start

```rust
use getopt_iter::Getopt;

fn main() {
    let args = &["myapp", "-a", "-b", "value", "file1", "file2"];
    let mut opts = Getopt::new(args, "ab:");
    opts.set_opterr(false); // Suppress error messages

    while let Some(opt) = opts.next() {
        match opt.val() {
            'a' => println!("Option -a provided"),
            'b' => println!("Option -b with arg: {}", opt.arg().unwrap()),
            '?' => eprintln!("Unknown option: {}", opt.erropt().unwrap()),
            _ => {}
        }
    }

    // Get remaining non-option arguments
    for arg in opts.remaining() {
        println!("Positional argument: {}", arg);
    }
}
```

## Usage Examples

### Basic Option Parsing

```rust
use getopt_iter::Getopt;

let args = &["prog", "-a", "-b", "-c"];
let mut getopt = Getopt::new(args, "abc");

while let Some(opt) = getopt.next() {
    match opt.val() {
        'a' => println!("Got -a"),
        'b' => println!("Got -b"),
        'c' => println!("Got -c"),
        _ => {}
    }
}
```

### Options with Required Arguments

```rust
use getopt_iter::Getopt;

let args = &["prog", "-o", "output.txt", "-v"];
let mut getopt = Getopt::new(args, "o:v");

let mut output = None;
let mut verbose = false;

while let Some(opt) = getopt.next() {
    match opt.val() {
        'o' => output = opt.into_arg(),
        'v' => verbose = true,
        _ => {}
    }
}

println!("Output: {:?}, Verbose: {}", output, verbose);
```

### Aggregated Short Options

```rust
use getopt_iter::Getopt;

// -abc is equivalent to -a -b -c
let args = &["prog", "-abc"];
let mut getopt = Getopt::new(args, "abc");

assert_eq!(getopt.next().map(|o| o.val()), Some('a'));
assert_eq!(getopt.next().map(|o| o.val()), Some('b'));
assert_eq!(getopt.next().map(|o| o.val()), Some('c'));
```

### Long Options

```rust
use getopt_iter::Getopt;

let args = &["prog", "--help", "--output=file.txt", "--verbose"];
let mut getopt = Getopt::new(
    args,
    "h(help)o:(output)v(verbose)"
);

while let Some(opt) = getopt.next() {
    match opt.val() {
        'h' => println!("Help requested"),
        'o' => println!("Output: {}", opt.arg().unwrap()),
        'v' => println!("Verbose mode"),
        _ => {}
    }
}
```

### Working with `std::env::args() or std::env::args_os()`

```rust
use getopt_iter::Getopt;

fn main() {
    let mut getopt = Getopt::new(std::env::args_os(), "abc:d");
    
    while let Some(opt) = getopt.next() {
        match opt.val() {
            'a' => { /* handle -a */ },
            'b' => { /* handle -b */ },
            'c' => {
                if let Some(arg) = opt.arg() {
                    println!("Got -c with arg: {}", arg);
                }
            },
            'd' => { /* handle -d */ },
            '?' => {
                eprintln!("Unknown option: -{}", opt.erropt().unwrap());
                std::process::exit(1);
            },
            ':' => {
                eprintln!("Missing argument for option -{}", opt.erropt().unwrap());
                std::process::exit(1);
            },
            _ => {}
        }
    }
    
    // Process remaining positional arguments
    for arg in getopt.remaining() {
        println!("File: {}", arg);
    }
}
```

### Error Handling

```rust
use getopt_iter::Getopt;

// Leading ':' in optstring changes error behavior
let args = &["prog", "-x", "-a"];
let mut getopt = Getopt::new(args, ":a:");
getopt.set_opterr(false); // Suppress error messages

while let Some(opt) = getopt.next() {
    match opt.val() {
        'a' => println!("Got -a with arg: {}", opt.arg().unwrap()),
        '?' => {
            // Unknown option
            eprintln!("Unknown option: -{}", opt.erropt().unwrap());
        },
        ':' => {
            // Missing required argument
            eprintln!("Missing argument for option -{}", opt.erropt().unwrap());
        },
        _ => {}
    }
}
```

## Option String Syntax

The option string (`optstring`) parameter follows POSIX conventions with GNU extensions:

### Basic Syntax

- **Single character**: Defines a flag option
  ```rust
  "a"       // Accepts -a
  "abc"     // Accepts -a, -b, -c
  ```

- **Character followed by `:`**: Requires an argument
  ```rust
  "a:"      // -a requires an argument: -a value or -avalue
  "a:b"     // -a takes arg, -b is a flag
  ```

### Long Options

Use parentheses to define long option names:

```rust
"h(help)"           // -h or --help
"v(verbose)"        // -v or --verbose
"o:(output)"        // -o value or --output=value
```

### Special Prefixes

- **Leading `:`**: Suppress error messages, return ':' for missing arguments
  ```rust
  ":abc:"   // Silent errors, returns ':' instead of '?'
  ```

### Examples

```rust
// Complex option string
let optstring = "hv(verbose)o:(output)c:(config)d(debug)";

// This accepts:
// -h, --help          (no argument)
// -v, --verbose       (no argument)
// -o value, --output=value  (required argument)
// -c value, --config=value  (required argument)
// -d, --debug         (no argument)
```

## Special Return Values

The `Opt::val()` method returns:

- **Option character**: When a valid option is parsed
- **`'?'`**: When an unknown option is encountered (or any error if optstring doesn't start with `:`)
- **`':'`**: When a required argument is missing (only if optstring starts with `:`)

Use `Opt::erropt()` to get the problematic option character when an error occurs.

## Program Name Handling

The first argument (argv\[0\]) is consumed and used as the program name:

```rust
use getopt_iter::Getopt;

let args = &["/usr/bin/myapp", "-a"];
let getopt = Getopt::new(args, "a");

assert_eq!(getopt.prog_name(), "myapp");  // Basename extracted
```

## `no_std` Support

The library supports `no_std` environments. Disable the default `std` feature:

```toml
[dependencies]
getopt-iter = { version = "1.0.1", default-features = false }
```

In `no_std` mode:
- Error messages are not printed (no stderr)
- `OsString` support is unavailable
- Core functionality remains the same

## API Overview

### `Opt` Structure

Represents a parsed option. Fields are private; use the accessors below.

Methods:
- `val(&self) -> char` — the option character, or `'?'` / `':'` for errors
- `erropt(&self) -> Option<char>` — the problematic option character when an error occurred
- `arg(&self) -> Option<&str>` — the option argument, borrowed
- `into_arg(self) -> Option<Cow<'static, str>>` — the option argument, owned (zero-copy when the
  argument was a borrowed `'static` input that didn't need to be sliced)

### `Getopt` Structure

Iterator-based option parser:

```rust
pub struct Getopt<'a, V, I: Iterator<Item = V>> { /* ... */ }
```

Methods:
- `new(args, optstring) -> Self` — create a new parser. `args` is any `IntoIterator` whose items
  implement [`ArgV`](#argv-trait); the first item is consumed as `argv[0]`.
- `set_opterr(&mut self, opterr: bool)` — enable or disable POSIX error messages on stderr
  (default: enabled; only effective with the `std` feature)
- `next(&mut self) -> Option<Opt>` — parse the next option (also available via the `Iterator` impl)
- `remaining(self) -> I` — consume the parser and return the underlying iterator at its current
  position to access positional arguments
- `prog_name(&self) -> &str` — the basename of `argv[0]`

### `ArgV` Trait

A sealed trait implemented for types that can be used as command-line arguments. Borrowed
implementations are bounded by `'static` so they can flow through the parser as `Cow::Borrowed`
without allocation — a good fit for sources like the [`argv`](https://crates.io/crates/argv)
crate, which yields `&'static OsStr`.

| Type | Notes |
|---|---|
| `&'static str` | Zero-copy |
| `String` | Zero-copy (takes ownership) |
| `&'static CStr` | Zero-copy when valid UTF-8; lossy + allocation otherwise |
| `OsString` | (requires `std`) Zero-copy when valid UTF-8 |
| `&'static OsStr` | (requires `std`) Zero-copy when valid UTF-8 |

Attached arguments (`-ofile.txt`, `--name=value`) always allocate, since the slice is tied to the
lifetime of the surrounding argument.

## Differences from GNU `getopt_long`

This implementation deliberately diverges from GNU `getopt_long` in a few places:

- **Long-option syntax** is the Solaris-style parenthesis form (`"o:(output)"`) embedded in the
  optstring, not a separate `struct option` array.
- **`::` (optional argument) is treated as `:` (required argument).** True optional-argument
  semantics are not implemented.
- **No `-W foo` → `--foo` translation.**
- **No `-` prefix mode** (returning non-options as option code `'1'`).
- **No long-option abbreviation.** Long options must match exactly.
- **No argv permutation.** Parsing stops at the first non-option, matching POSIX (and the GNU
  `+` prefix mode) rather than GNU's default permuting behavior. The leading `+` prefix is
  accepted for compatibility but has no effect since this is already the default.
- **Option characters must be ASCII.** Non-ASCII bytes in argv are never matched as option
  characters; they fall through as unknown options or remain part of an attached argument.

## Testing

The library includes comprehensive tests covering:
- POSIX compliance scenarios
- GNU extensions
- Error handling
- Edge cases

Run tests with:
```bash
cargo test
```

For `no_std` testing:
```bash
cargo test --no-default-features
```

### Fuzzing

This library includes fuzzing targets to ensure robustness and catch potential panics. The fuzzers use `cargo-fuzz` and libFuzzer.

Install fuzzing tools:
```bash
cargo install cargo-fuzz
rustup install nightly
```

Run all fuzzers for 60 seconds each:
```bash
./fuzz/run-all-fuzzers.sh 60
```

Or run a specific fuzzer:
```bash
cargo +nightly fuzz run fuzz_getopt -- -max_total_time=60
```

See the [fuzz/README.md](fuzz/README.md) for detailed fuzzing documentation.

## License

BSD 2-Clause License

## Contributing

Contributions are welcome! Please ensure all tests pass and add new tests for any new features.
