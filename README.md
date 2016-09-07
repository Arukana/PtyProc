# PtyShellMode

[![Build Status](https://travis-ci.org/adjivas/pty-shell-mode.svg?branch=master)](https://travis-ci.org/adjivas/pty-shell-mode)

```
pty-shell-mode : crate
 ├── ffi : private
 ├── fork : private
 │   └── child : private
 ├── prelude : public
 ├── shell : public
 │   ├── device : private
 │   │   └── state : private
 │   ├── display : private
 │   ├── err : private
 │   └── state : public
 ├── terminal : public
 └── winsize : private
```

### How to use
```shell
git clone https://github.com/limaconoob/Neko.git pty-shell-mode && cd pty-shell-mode
cargo run
```

### Knowledge
* [modes](https://en.wikipedia.org/wiki/Computer_terminal#Modes)
* [speudotty](https://en.wikipedia.org/wiki/Pseudoterminal)
