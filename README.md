# alloca shenanigans

an on-going experiment... prepare your eyes... for the worst `alloca` implementation you've ever seen...

rust really does not want you messing with the stack pointer

the goal is pure rust `alloca` with lots of unsafe but I am <font color="red">s</font><font color="orange">t</font><font color="yellow">u</font><font color="green">p</font><font color="blue">i</font><font color="purple">d</font>! so it's really hard :(

I started off using [cc](https://docs.rs/cc/latest/cc/) and calling the C implementation of alloca but I had lots of weird bugs because rust does not like the fact that alloca messes with the stack pointer, also if the compiler decided to not inline some functions it would break. now it's using inline asm and my own alloca implementation