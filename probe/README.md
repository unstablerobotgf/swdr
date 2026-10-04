# probe-rs target for STM32WL3x

`STM32WL3_Series.pack.yaml` is `target-gen pack` output from Keil.STM32WL3x_DFP 1.2.0, unmodified.
`STM32WL3_Series.yaml` is the one we use, with two edits:

- Flash algorithm `load_address` 0x20000008 -> 0x20001008.
- SRAM region starts at 0x20000100 instead of 0x20000000.

Both keep probe-rs out of 0x20000004..0x2000003F, the ROM bootloader's RAM_VR
(ResetReason, AppBase, bootloader_vr*). With the pack's placement, a reset after
flashing boots with algorithm code in RAM_VR and the ROM HardFaults.
