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
