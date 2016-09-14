## How to run each demo

After installing `roboime-next`, the `roboime-next-cli` command will be available.

To test if you have it:

    roboime-next-cli --help

#### C demo, requires make and a C compiler.

    make -C ./c/
    roboime-next-cli ./c/demo

#### C++ demo, requires make and a C++ compiler.

    make -C ./cpp/
    roboime-next-cli ./cpp/demo

#### Java demo, requires Java.

    javac ./java/Demo.java
    roboime-next-cli java -cp ./java Demo

#### Lua demo, requires a lua interpreter.

    roboime-next-cli lua ./lua/demo.lua

Also works with luajit (really fast BTW).

    roboime-next-cli luajit ./lua/demo.lua

#### Python 2 demo, requires a Python 2.7 interpreter

    roboime-next-cli python ./python2/demo.py

Also works with pypy:

    roboime-next-cli pypy ./python2/demo.py

#### Python 3 demo, requires a Python 2.7 interpreter

    roboime-next-cli python3 ./python3/demo.py

Also works with pypy3: (faster than CPython, not as awesome as luajit or pypy though)

    roboime-next-cli pypy3 ./python3/demo.py

### Rust demo, you already need rust for this anyway

This was the fastest on my machine, C and LuaJit were really close.

    roboime-next-cli cargo run -q --release --manifest-path ./rust/Cargo.toml

