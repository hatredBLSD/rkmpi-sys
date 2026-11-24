# rkmpi-sys

Raw FFI bindings to the **Rockchip Rockit MPI SDK** for **RV1103 / RV1106**
SoCs, as shipped on [Luckfox Pico](https://github.com/LuckfoxTECH/luckfox-pico)
boards (Pico, Pico Mini, Pico Plus, Pico Pro/Max, etc.).

Covers the full MPI API: VI, VO, VPSS, VENC, VDEC, AI/AO, AENC/ADEC,
RGN, IVS, AVS, AF, TDE, SYS, MB (media buffer) and CAL modules.

## Target platform

* **SoC:** Rockchip RV1103 / RV1106 (Cortex-A7, 32-bit ARM, hard-float, uClibc)
* **Rust target:** `armv7-unknown-linux-uclibceabihf` (requires nightly `-Zbuild-std`)
* Bindings are pregenerated for this target — no libclang or SDK headers
  needed at build time.
* **Not** suitable for aarch64 SoCs (RK356x/RK3588) — layouts differ.

## Prerequisites

`rkmpi-sys` does **not** vendor `librockit.so`. Get it from the Luckfox SDK
(`media/rockit/rockit/mpi/sdk/lib/librockit.so`) or from a sysroot of a board
running the Luckfox firmware.

You also need the SDK toolchain `arm-rockchip830-linux-uclibcgnueabihf` for
linking (ships with the Luckfox SDK).

## Usage

`.cargo/config.toml` in your project:

```toml
[target.armv7-unknown-linux-uclibceabihf]
linker = "arm-rockchip830-linux-uclibcgnueabihf-gcc"
```

Build:

```bash
export ROCKIT_LIB=/path/to/sdk/lib
cargo +nightly build --target armv7-unknown-linux-uclibceabihf \
    -Zbuild-std=core,alloc,std,panic_abort --release
```

On the board:

```bash
export LD_LIBRARY_PATH=/path/to/dir/with/librockit.so
```

## Example

```rust,no_run
use rkmpi_sys::*;

fn main() {
    unsafe {
        assert_eq!(RK_MPI_SYS_Init(), 0);
        RK_MPI_SYS_Exit();
    }
}
```

## Regenerating the bindings

Generated with bindgen 0.71 from the headers in `media/rockit/rockit/mpi/sdk/include`
of the Luckfox Pico repository:

```bash
bindgen wrapper.h -o src/bindings.rs -- \
    -I/path/to/sdk/include \
    --target=arm-linux-gnueabihf \
    --sysroot=&lt;toolchain&gt;/arm-rockchip830-linux-uclibcgnueabihf/sysroot \
    --no-size_t-is-usize --no-layout-tests
```

(Note: upstream `rk_mpi_af.h` contains stray `#include &lt;vector&gt;` / `&lt;string&gt;`
that must be removed before generation.)

## License

Apache-2.0, matching the dual GPL-2.0/Apache-2.0 license of the upstream
Rockchip headers. `librockit.so` is a proprietary Rockchip binary and is
**not** distributed with this crate.