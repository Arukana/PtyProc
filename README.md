# PtyProc

[![Build Status](https://travis-ci.org/adjivas/pty-proc.svg?style=flat-square&branch=master)](https://travis-ci.org/adjivas/pty-proc)

#### How to build
```shell
git clone https://github.com/adjivas/pty-proc.git pty-proc && cd pty-proc
cargo build
```
With clippy:
```shell
cargo build --features clippy
```


#### Knowledge
* [modes](https://en.wikipedia.org/wiki/Computer_terminal#Modes)
* [speudotty](https://en.wikipedia.org/wiki/Pseudoterminal)
* [control](http://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-Application-Program-Control-functions)

#### Module
```shell
pty-proc : crate
 ├── fork : private
 ├── macros : private
 ├── prelude : public
 └── shell : public
     ├── device : public
     │   ├── control : public
     │   │   └── operate : public
     │   │       ├── err : private
     │   │       ├── key : public
     │   │       └── mouse : public
     │   │           └── err : private
     │   └── state : private
     ├── display : public
     │   ├── control : public
     │   │   └── operate : public
     │   │       ├── background : public
     │   │       ├── blink : public
     │   │       ├── bold : public
     │   │       ├── dim : public
     │   │       ├── foreground : public
     │   │       ├── hidden : public
     │   │       ├── reverse : public
     │   │       └── underlined : public
     │   ├── cursor : public
     │   ├── err : private
     │   └── winsz : private
     │       └── err : private
     ├── err : private
     ├── mode : public
     ├── state : private
     │   └── clone : public
     └── termios : private
         ├── linux : private @ #[cfg(any(target_os = "linux", target_os = "android"))]
         └── macos : private @ #[cfg(target_os = "macos")]
```

#### Dependency
Many thanks goes to:

![Dependency](https://adjivas.github.io/pty-proc/images/dependency.svg)
