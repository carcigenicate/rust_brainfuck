# Rust Brainfuck and Ezfuck Interpreters

This is the start of my attempt to write interpreters for Brainfuck and the variant of Brainfuck I created called "Ezfuck"
which adds some convenience operators.

# Running

`helloWorld.txt`

```brainfuck
+8[>+4[>+2>+3>+3>+<4-]>+>+>->2+[<]<-]>2.>-3.+7..+3.>2.<-.<.+3.-6.-8.>2+.>+2.
```

With cargo:

```powershell
PS path> cargo run -- --path helloWorld.txt
Hello World!
```

Running the executable directly after compiling it:

```powershell
PS path> .\brainfuck.exe --path helloWorld.txt
Hello World!
```

This will interpret the code as Ezfuck. The option to specify Brainfuck is not available yet.

Any comment-less Brainfuck should be valid Ezfuck. If you currently use `^`, `V`, `*`, or `/` in comments though, those
will need to be removed first.

# Ezfuck

This is a re-implementation of a [project I did years ago](https://github.com/carcigenicate/ezfuck). The only differences
between Brainfuck and Ezfuck are Ezfuck:

 - Adds `*` and `/` operators so you can do multiplication and division.
 - Adds the ability to give numeric "arguments" to most commands. `+5` adds 5 instead of 1 (effectively the same as `+++++`),
   `*5` multiplies the current cell by 5, and `>5` moves 5 cells to the right. If an argument is omitted, it defaults to 1.
 - Adds the `^` operator that sets the current cell value, regardless of what it was before. `^` sets the current cell to 1,
   and `^25` sets the current cell to 25.
 - Adds a special `V` value that allows using the current cell as an argument. If the current cell has a value of 5, `+V` will
   have the same effect as `+5` or `+++++`.

Unlike my previous implementation though, I omitted the `{}` operators that allow directly manipulating the instruction pointer.
I'm not sure if those were even a useful feature to begin with.

# Future Plans

 - A REPL
 - The ability to compile Brainfuck/Ezfuck to machine code