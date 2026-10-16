# Testing

Hard rule: **no test in this repository requires audio hardware or a real
ASIO driver.** The suite must pass on a bare CI runner (including runners
with no sound devices and no ASIO drivers installed).

## How

- The host state machine consumes a raw `IASIO` vtable pointer, not a COM
  object. Tests substitute an in-memory **fake driver**: a Rust-implemented
  vtable behind `extern "C"` thunks, configurable for channels, sample
  types, sample rates, buffer constraints, and injected driver quirks. A
  test-owned thread plays the driver's engine and drives the callback loop.
- Registry access goes through a mockable layer; parsing tests use fixtures,
  never real registry writes.
- Examples in this repo run correctly on a machine with an empty ASIO
  driver registry.

## Manual validation (not part of this repo's tests)

Real-driver validation against FlexASIO configurations and vendor drivers is
run by hand on a development machine. Findings from those sessions are
reproduced as fake-driver scenarios so the automated suite keeps covering
them. Scratch scripts used for those sessions stay out of the repository.
