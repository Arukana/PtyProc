# PtyShellMode

[![Build Status](https://travis-ci.org/adjivas/pty-shell-mode.svg?branch=master)](https://travis-ci.org/adjivas/pty-shell-mode)

#### How to use
```shell
git clone https://github.com/adjivas/pty-shell-mode.git pty-shell-mode && cd pty-shell-mode
cargo run
```

#### Knowledge
* [modes](https://en.wikipedia.org/wiki/Computer_terminal#Modes)
* [speudotty](https://en.wikipedia.org/wiki/Pseudoterminal)

#### Module
```shell
pty-shell-mode : crate
 ├── fork : private
 ├── prelude : public
 ├── shell : public
 │   ├── device : public
 │   │   ├── control : public
 │   │   │   └── operate : public
 │   │   │       ├── key : public
 │   │   │       └── mouse : public
 │   │   └── state : private
 │   ├── display : private
 │   │   ├── err : private
 │   │   └── winsz : private
 │   │       └── err : private
 │   ├── err : private
 │   ├── mode : public
 │   └── state : private
 └── terminal : public
     ├── linux : private @ #[cfg(any(target_os = "linux", target_os = "android"))]
     └── macos : private @ #[cfg(target_os = "macos")]
```

#### Dependency
Many thanks goes to:

![Dependency](https://adjivas.github.io/pty-shell-mode/images/dependency.svg)
