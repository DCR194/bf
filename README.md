# My Brainf*ck Interpreter
## Running guide
To run, first cd into the main working directory,
then compile it with:
`rustc src/main.rs -o bf`
(You will need to have rust installed ;)

## How to use it
### In-Line
You can use this code by running it without commands and doing `ctl+c` `ctl+v` into a pre-existing bf program.
The problem here is that when in in-line mode, it does not support multiple lines (yet) (probably never)

### Interpreted
Alternatively, you can also design your bf program ahead of time and (after compiling the interpreter) run it like so: `./bf <your-file-name>`

Cheers
