# X Programming Language

Language X is similar to the C language, 
but is simpler and slightly different.

## Run

To run the program, you need a valid `format.csv` in the current directory telling the style.

To print result in cli, do

```sh
cargo run parser example1.x
```

Or you can store the result in a file and view in any web browser

```sh
cargo run parser example1.x > example1.xhtml
```

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── example1.x
├── example1.xhtml
├── format.csv: format to generate the html
├── LICENSE
├── project02.pdf
├── README.md
├── src
   ├── character_stream.rs: get characters from source file
   ├── main.rs: init the parser with the input source file path
   ├── parser.rs: parser based on EBNF
   ├── prettifier.rs: turn Tokens to html based on `format.csv`
   ├── scanner.rs: lexer that turns characters into tokens
   └── token.rs: definitions of TokenTypes and Tokens
```
