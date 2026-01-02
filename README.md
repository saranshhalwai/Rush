# rush

`rush` is a **Unix shell written in Rust**.

This started as a learning project, but it’s intentionally being built like a *real shell*: minimal abstractions, explicit OS interaction, and a clear separation between shell logic and kernel responsibilities.

The goal is not POSIX compliance (yet), but **understanding how shells actually work**.

---

## Current Features

### Builtins
- `cd`
  - Absolute paths
  - Relative paths
  - `~` and `~/path` expansion
- `exit`

### External Commands
- Execute commands via `PATH`
- Argument passing
- Correct working directory inheritance

Examples:
```sh
ls
ls -l
echo hello
pwd
````

### Prompt

* Displays the current working directory
* Updates correctly after `cd`

### Signal Handling

* `Ctrl-C` (`SIGINT`) **does not kill the shell**
* `Ctrl-C` correctly interrupts the running foreground command

---

## Non-Features (by design, for now)

These are **not implemented yet**:

* Quoted strings (`"hello world"`)
* Environment variable expansion (`$VAR`)
* Pipes (`|`)
* Redirection (`>`, `<`)
* Job control (`&`, `fg`, `bg`)
* Scripting / non-interactive mode
* POSIX compliance

The project is being built incrementally in clearly defined stages.

---

## Design Philosophy

* The **OS is the source of truth**

  * No fake `pwd` tracking
  * Real `chdir`, real processes
* The shell is a **text-to-syscall translator**
* Minimal global state
* Explicit mutation
* Fail loudly, not silently

This project prioritizes **clarity and correctness over completeness**.

---

## Why Rust?

Rust provides:

* Memory safety without garbage collection
* Explicit handling of mutation and ownership
* A clear boundary between safe code and OS-level danger
* Excellent tooling (`cargo`, `clippy`, compiler diagnostics)

Low-level OS features (signals, process control) are handled via well-established ecosystem crates rather than `std`.

---

## Requirements

* Linux / Unix-like OS
* Rust (via `rustup`)
* A C toolchain (for some dependencies)

---

## Building & Running

```sh
cargo build
cargo run
```

---

## Project Status

`rush` is **actively evolving**.

The current focus is:

* solid command execution
* correct signal semantics
* incremental parsing improvements

Expect breaking changes as the internal architecture matures.

---

## Disclaimer

This is **not** intended to replace your system shell.

It *is* intended to replace hand-wavy understanding with real knowledge.

---

## License

Apache License 2.0

