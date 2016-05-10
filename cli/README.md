## How to run each demo

> NOTE: in the future `cargo run --release --` will become `roboime-next-cli` or something like that.

Checkout the help for more options (pass them after `--`):

    cargo run --release -- --help

#### C demo, requires make and a C compiler.

    make -C demos/c/
    cargo run --release -- demos/c/demo

> maxes at ~29k steps/second

#### C++ demo, requires make and a C++ compiler.

    make -C demos/cpp/
    cargo run --release -- demos/cpp/demo

> maxes at ~15k steps/second

#### Java demo, requires Java.

    javac demos/java/Demo.java
    cargo run --release -- java -cp demos/java Demo

> maxes at ~16k steps/second

#### Lua demo, requires a lua interpreter.

    cargo run --release -- lua demos/lua/demo.lua

> maxes at ~23k steps/second

Also works with luajit (really fast BTW).

    cargo run --release -- luajit demos/lua/demo.lua

> maxes at ~28k steps/second

#### Python 2 demo, requires a Python 2.7 interpreter

    cargo run --release -- python demos/python2/demo.py

> maxes at ~17k steps/second

    cargo run --release -- pypy demos/python2/demo.py

> maxes at ~23k steps/second

#### Python 3 demo, requires a Python 2.7 interpreter

    cargo run --release -- python3 demos/python3/demo.py

> maxes at ~14k steps/second

Also works with pypy3 (faster than CPython, not as awesome as luajit or pypy though).

    cargo run --release -- pypy3 demos/python3/demo.py

> maxes at ~17k steps/second

### Rust demo, you already need rust for this anyway

This was the fastest on my machine, C and LuaJit were really close.

    cargo run --release -- cargo run -q --release --manifest-path demos/rust/Cargo.toml

> maxes at ~31k steps/second

> NOTE: The speed of each demo is mostly due to parsing speed, which is something, but not nearly everything. Also note that we work at a
> rate of 60 steps/second, thus for a simple one robot control loop every one of the above demos makes it very easily.
