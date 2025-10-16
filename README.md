# calc

simple terminal calculator. made it because I was bored

## Installation

ensure you have rust installed. clone the repository and build:

```bash
git clone https://github.com/Hashino/calc
cd calc
cargo build --release
```

## Usage

### Interactive Mode

run the calculator without arguments for interactive mode:

```bash
./target/release/calc
```

enter expressions like:
- `2 + 3 * 4`
- `sin(1.57)`
- `sqrt(16)`
- `5!`
- `log(100, 10)` (log base 10 of 100)
- ` + 10` (add 10 to the last result)

type `help/h` for more info or `quit/q` to exit.

### Command-Line Mode

evaluate a single expression:

```bash
calc --input "2 + 3"
```

## Examples

```bash
$ calc --input "(2 + 3) * 4"
20.0

$ calc
> 10 / 2
5.0
> sin(0)
0.0
> quit
```
