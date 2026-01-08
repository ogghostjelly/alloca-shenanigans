# alloca shenanigans

> [!NOTE]
> If you're looking for an actually useable alloca library, use [alloca](https://docs.rs/crate/alloca/latest/) instead.

an on-going experiment... prepare your eyes... for the worst `alloca` implementation you've ever seen... only works on x86-64 AFAIK because of 16 byte stack alignment

pure rust with lots of unsafe and inline asm but I am <font color="red">s</font><font color="orange">t</font><font color="yellow">u</font><font color="green">p</font><font color="blue">i</font><font color="purple">d</font>! so it's not very good.