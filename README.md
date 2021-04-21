# `stm32l4xx-hal`

_formerly [MabezDev/stm32l4xx-hal](https://github.com/mabezdev/stm32l4xx-hal)_

> [HAL] for the STM32L4xx family of microcontrollers

- *Note: this HAL is a work in progress, contributions are appreciated :). If you have a L4 device that is currently unsupported see [#29](https://github.com/stm32-rs/stm32l4xx-hal/issues/29)*

[HAL]: https://crates.io/crates/embedded-hal

## [Documentation](https://docs.rs/stm32l4xx-hal/latest/stm32l4xx_hal/)

## [Changelog](https://github.com/mabezdev/stm32l4xx-hal/blob/master/CHANGELOG.md)

## Product Matrix

The L4 product matrix:

| Product Line | Flash (kB) | RAM (kB) | OP-amp | CAN | ΣΔ  | ADC  | DAC  | SAI  | USB  | LCD  | AES                |
| ------------ | ------     | -------- | ---    | --- | --- | ---- | ---- | ---- | ---  | ---  | ---                |
| `stm32l4x6`  |            |          |        |     |     |      |      |      |      |      |                    |
| 4A6xx        | 1024       | 320      | 2      | 2   | 8   | 3    | 2    | 2    | OTG  | 8x40 | :heavy_check_mark: |
| 496xx        | 512-1024   | 320      | 2      | 2   | 8   | 3    | 2    | 2    | OTG  | 8x40 |                    |
| 486xx        | 1024       | 128      | 2      | 1   | 8   | 3    | 2    | 2    | OTG  | 8x40 | :heavy_check_mark: |
| 476xx        | 256-1024   | 128      | 2      | 1   | 8   | 3    | 2    | 2    | OTG  | 8x40 |                    |
| `stm32l4x5`  |            |          |        |     |     |      |      |      |      |      |                    |
| 475xx        | 256-1024   | 128      | 2      | 1   | 8   | 3    | 2    | 2    | OTG  |      |                    |
| `stm32l4x3`  |            |          |        |     |     |      |      |      |      |      |                    |
| 443xx        | 128-256    | 64       | 1      | 1   |     | 1    | 2    | 1    | USBD | 8x40 | :heavy_check_mark: |
| 433xx        | 128-256    | 64       | 1      | 1   |     | 1    | 2    | 1    | USBD | 8x40 |                    |
| `stm32l4x2`  |            |          |        |     |     |      |      |      |      |      |                    |
| 462xx        | 512        | 160      | 1      | 1   | 4   | 1    | 1    | 1    | USBD |      | :heavy_check_mark: |
| 452xx        | 256-512    | 160      | 1      | 1   | 4   | 1    | 1    | 1    | USBD |      |                    |
| 442xx        | 256        | 64       | 1      | 1   |     | 1    | 2    | 1    | USBD |      | :heavy_check_mark: |
| 432xx        | 128-256    | 64       | 1      | 1   |     | 1    | 2    | 1    | USBD |      |                    |
| 422xx        | 128        | 40       | 2      |     |     | 2    |      |      | USBD |      | :heavy_check_mark: |
| 412xx        | 64-128     | 40       | 1      |     |     | 2    |      |      | USBD |      |                    |
| `stm32l4x1`  |            |          |        |     |     |      |      |      |      |      |                    |
| 471xx        | 512-1024   | 128      | 2      | 1   | 8   | 3    | 2    | 2    |      |      |                    |
| 451xx        | 256- 512   | 160      | 1      | 1   | 4   | 1    | 1    | 1    |      |      |                    |
| 431xx        | 128- 256   | 64       | 1      | 1   |     | 1    | 2    | 1    |      |      |                    |

- *Note*: FSMC is supported on all STM32L47xx/48xx/49xx/4Axx products.

Manuals:
- [RM0351](https://www.st.com/resource/en/reference_manual/dm00083560-stm32l47xxx-stm32l48xxx-stm32l49xxx-and-stm32l4axxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf): Covers STM32L47xxx/48xxx/49xxx/4Axxx.
- [RM0394](https://www.st.com/resource/en/reference_manual/dm00151940-stm32l41xxx42xxx43xxx44xxx45xxx46xxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf): Covers STM32L41xxx/42xxx/43xxx/44xxx/45xxx/46xxx.

The L4+ product matrix:

| Product Line   | Flash (MB) | RAM (kB) | OP-AMP | CMP  | ΣΔ   | ADC  | TFT                | CGRC               | MIPI               | AES                |
| ------------   | ---------- | -------- | ------ | ---- | ---- | ---- | ----               | ----               | ----               | ---                |
| `stm32l4r9/s9` |            |          |        |      |      |      |                    |                    |                    |                    |
| 4R9xx          | 1, 2       | 640      | 2      | 2    | 8    | 1    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                    |
| 4S9xx          | 2          | 640      | 2      | 2    | 8    | 1    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| `stm32l4r7/s7` |            |          |        |      |      |      |                    |                    |                    |                    |
| 4R7xx          | 1, 2       | 640      | 2      | 2    | 8    | 1    | :heavy_check_mark: | :heavy_check_mark: |                    |                    |
| 4S7xx          | 2          | 640      | 2      | 2    | 8    | 1    | :heavy_check_mark: | :heavy_check_mark: |                    | :heavy_check_mark: |
| `stm32l4r5/s5` |            |          |        |      |      |      |                    |                    |                    |                    |
| 4R5xx          | 1, 2       | 640      | 2      | 2    | 8    | 1    |                    |                    |                    |                    |
| 4S5xx          | 2          | 640      | 2      | 2    | 8    | 1    |                    |                    |                    | :heavy_check_mark: |
| `stm32l4p5/q5` |            |          |        |      |      |      |                    |                    |                    |                    |
| 4P5xx          | 0.5, 1     | 320      | 2      | 2    | 4    | 2    | :heavy_check_mark: |                    |                    |                    |
| 4Q5xx          | 1          | 320      | 2      | 2    | 4    | 2    | :heavy_check_mark: |                    |                    | :heavy_check_mark: |

All products support FSMC and USB OTG.

Manuals:
- [RM0432](https://www.st.com/resource/en/reference_manual/dm00310109-stm32l4-series-advanced-armbased-32bit-mcus-stmicroelectronics.pdf): All L4+ products.

## About

    - Minimum rustc version 1.31

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
