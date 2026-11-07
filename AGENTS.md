# AGENTS.md

Rust ADS-B / Mode S decoder: one binary (`src/main.rs`) over the library in `src/lib.rs`. Reads hex squitters from a file or TCP feed and renders a live terminal table.

Data flow: `main` -> `spawn_reader_thread` (`src/reader.rs`) -> `get_message` -> `get_downlink_format` -> `DF::from_message` -> `Planes::update_aircraft` -> `Plane` update/display.

## Commands

- `make prep` — `cargo test`, then `cargo fmt` + `cargo clippy`. CI runs `make lint` then `make test` (`.github/workflows/CI.yaml`, non-`main` branches only).
- `cargo test <name>` — tests are inline `#[cfg(test)] mod tests`; there is no `tests/` dir. Doctests in `decoder/downlink.rs` and `decoder/adsb/icao.rs` are part of the suite.
- `make run` — release build against the default source `rec/squitters.txt`; run from the repo root.
- `make install` copies the stripped binary to `~/.local/bin`; do not run it unless asked.
- `make lint` runs `cargo fmt` (it writes) before `cargo clippy`.

## Toolchain / gotchas

- Edition 2024 with let-chains (`if let Some(x) = ... && cond`); requires Rust >= 1.85.
- `Cargo.lock` is gitignored but present locally; `make build` / `make release` run `cargo update` first.
- Logging is off unless `-l/--error-log FILE` is passed. Logs are written to that file, not stderr; `RUST_LOG` alone has no effect, and `-M` only works together with `-l`.
- `main` switches to the terminal alternate screen and clears it on every refresh. Use `-i Q` for headless runs. File mode exits at EOF; `-t host:port` reconnects forever every 5s.
- `-F/--format` is declared in `src/arguments.rs` but never read; don't assume it filters input.

## Input format

- `clean_squitter` (`src/decoder/utils.rs`) keeps only hex digits and accepts 14/28-char raw frames or 26/40-char lines whose leading 12 hex chars are stripped (covered in `utils/format.rs` tests). Everything else returns `None`.
- CRC is only checked for DF11/17/18 (pure parity). DF0/4/5/16/20/21/24 carry address/parity, so their syndrome is non-zero; do not add a blanket CRC rejection.
- `rec/` contains sample captures (raw, SBS-style) plus `ruler.txt`, a bit-position ruler.

## Message representation

- A frame is `Vec<u32>` of **hex nibbles** (0..=15), not bytes. Bit positions are 1-based, MSB-first (`bit_location` in `decoder/utils/calc.rs`).
- Use `range_value`, `flag_and_range_value`, `status_flag_and_range_value` (`decoder/utils/calc.rs`). They cross nibble boundaries but panic on out-of-range positions.
- Other core extractors: `get_downlink_format`, `get_icao`, `get_message_type`, `get_capability` in `decoder/downlink.rs` / `decoder/utils.rs`.

## Decode paths — keep both in sync

- `Plane::update` (`plane/from_squitter.rs`) decodes directly from the frame: for existing aircraft with DF>=20 or with `-U/--use-update-method`.
- `Plane::update_from_downlink` (`plane/from_downlink.rs`) decodes from `DF::{Srt,Ext,Mds}` (`decoder/downlink/`): for existing aircraft with DF<20, and for every newly seen aircraft.
- Adding a displayed field usually means changing: the bit decoder (`adsb/`, `ehs/`, `bds/`, `meteo.rs`), both plane update modules, the relevant downlink struct(s), `plane/simple_display.rs`, and `plane/header.rs`. Header tests assert exact spacing.
- DF20/21 parity is AP, so the BDS register is inferred: `bds()` followed by `is_bds_1_7/4_0/5_0/6_0/4_4/4_5` in `plane/from_squitter/from_mode_s.rs`. BDS 4.0/5.0/6.0 are gated on capability bits unless `-R/--relaxed`; preserve that gate.
- `decoder/country/country_icao_mask.rs` is a hand-maintained ICAO prefix table; add ranges there for registration lookups.
