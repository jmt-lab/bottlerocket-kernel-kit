# v1.1.4 (2025-02-25)

## OS Changes
 * Update kernel-5.10 from 5.10.233 to 5.10.234  ([#57])
 * Update kernel-5.15 from 5.15.176 to 5.15.178  ([#57])

[#57]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/57

# v1.1.3 (2025-02-24)

## OS Changes
 * Update kernel-6.1 from 6.1.127 to 6.1.128 ([#52])

[#52]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/52

# v1.1.2 (2025-02-18)

## OS Changes
 * Use NVIDIA open gpu drivers for L4 and L40S cards ([#48])
 * Remove NVIDIA Multi-Instance GPU (MIG) and Fabric Manager Interoperability code ([#49])

[#48]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/48
[#49]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/49

# v1.1.1 (2025-02-10)

## Build Changes
 * Fix the kernel-5.15 spec file to apply patches extracted from the SRPM ([#43])
 * Fail kernel builds on mismatches between the applied patches and patches found in the SRPM ([#43])
 * Update twoliter to 0.7.2 ([#36])

[#36]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/36
[#43]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/43

# v1.1.0 (2025-02-06)

## OS Changes
 * Update to kernel 6.1.127 ([#37])
 * Add support for Nvidia MIG ([#35])

## Build Changes
 * Find upstream kernel patches via the upstream source's spec file ([#40])

[#37]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/37
[#40]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/40
[#35]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/35

# v1.0.7 (2025-02-04)

## OS Changes
 * Update to kernel 5.10.233-224.894 and 5.15.176-118.178 ([#30])

[#30]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/30

# v1.0.6 (2025-01-24)

## OS Changes
 * Update to kernel 5.10.233, 5.15.176, and 6.1.124 ([#25])

[#25]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/25

# v1.0.5 (2025-01-24)

## OS Changes
 * Use the version of the driver for `kmod-*-nvidia` packages. ([#22])

## Build Changes
 * Updates the Bottlerocket SDK to v0.50.1. ([#18])

[#18]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/18
[#22]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/22

# v1.0.4 (2025-01-16)

## OS Changes
* Update neruon dkms for kernel-5.10, kernel-5.15 and kernel-6.1 ([#16], ([#17]))
* Update to drivers for kmod-5.10-nvidia, kmod-5.15-nvidia and kmod-6.1-nvidia ([#21])

[#16]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/16
[#17]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/17
[#21]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/21

# v1.0.2 (2024-12-20)

## Build Changes
* Update CHANGELOG.md to match format expected by release automation ([#12])

[#12]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/12

# v1.0.1 (2024-12-20)

## OS Changes
* Update to kernel 5.10.230 and 5.15.173 ([#10])

## Build Changes
* Add GPG verification where possible ([#5])

[#5]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/5
[#10]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/10

# v1.0.0 (2024-12-11)

## Build Changes
* Create the new kernel kit from the following core-kit packages: ([#1])
  * grub
  * kernel-5.10
  * kernel-5.15
  * kernel-6.1
  * kmod-5.10-nvidia
  * kmod-5.15-nvidia
  * kmod-6.1-nvidia
  * libkcapi
  * linux-firmware
  * microcode
  * shim
* Update bottlerocket-sdk to v0.50.0

[#1]: https://github.com/bottlerocket-os/bottlerocket-kernel-kit/pull/1
