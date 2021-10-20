# CLI math expression with addition and subtraction

The program parses the last argument as a math expression (0-9, +, -) and evaluates it. If evaluation was successful it
will print the result to STDOUT, otherwise it will print an error message to STDERR.

Note that the math expression can only contain positive numbers with base 10 (decimal), any non-valid char will be
skipped.

## How to use

```shell
cargo run -- "1+1"
```

## Testing

```shell
cargo test
```
