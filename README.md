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
```