# vectorcade-games

Game logic crates for VectorCade.

This repo should stay **renderer-agnostic**: games emit `DrawCmd` only.

Depends on:
- `vectorcade-shared`
- `vectorcade-fonts` (for distinct vector label styles)

## Build

```bash
cargo test
```
