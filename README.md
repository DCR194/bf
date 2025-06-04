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
You can also run some of the example programs like so:
`./bf ex/hallo.bf`
`./bf ex/fib.bf`


## Acknowledgements
Most of the example programs (if not all) were kindly licenced under creative commons by Daniel B Cristofani from brainfuck.org
brainfuck.org was also a really helpful source of truth (from which this program may stray)
Thanks to my roommate for implying I wouldn't actually do this, and to my ego for forcing me to write a bf interpreter at 2 am.
You are more than welcome to use this prgram and its files for whatever you may like.

Cheers
