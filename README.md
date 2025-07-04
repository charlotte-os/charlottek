# CharlotteOS - charlottek

`charlottek` is an operating system kernel developed as key component of the CharlotteOS project but it is designed to be flexible enough that we hope it can also find use in many other places. It seeks to be a monolithic kernel with low-level system call interfaces that borrows ideas from exokernels and modern systems like Fuchsia. Its design allows for almost any higher level interface to be layered on top and also includes a typesafe system namespace (akin to the namespaces found in Fuschsia and Plan9 but more flexible and typesafe) with URIs as paths which has the added benefit of allowing access to the namespace of another host over a network without having to mount anything all while being secured by granular capability based access control.

charlottek is still in early development, and core subsystems are actively being built. We welcome contributions—feel free to grab an issue from the tracker, suggest features, or participate in discussions on our repository or Matrix server.


[Find us on Matrix](https://matrix.to/#/#charlotteos:matrix.org)

## Programming Languages
- `charlottek` is written in Rust and ISA specific assembly languages
- x86_64 assembly should use Intel syntax exclusively in this project

## External Dependencies:
- C dependencies are allowed if vetted by maintainers.
- Any dependencies in languages other than Rust, C, and assembly are strictly forbidden.
- Always prefer a high-quality Rust equivalent over an external C library unless there is good
  reason to do otherwise

## Target System Requirements
- Processor:
  - x86_64 (Primary ISA)
    - x2APIC LAPIC operating mode
- Firmware:
  - Unified Extensible Firmware Interface (UEFI)
  - Advanced Configuration and Power Interface (ACPI)
- Memory:
  - Recommended: >= 1 GiB
  - Required: 128 MiB
- Storage:
  - Recommended: >= 64 GiB
  - Required: 4 GiB
  - Device Types:
    - Non-Volatile Memory Express (NVMe)
    - USB Mass Storage Device Class
- Output:
  - Display Adapter: Any adapter capable of providing framebuffers via the UEFI Graphics Output Protocol
  - Serial
    - NS16550 compatible UART
    - USB CDC ACM (Virtual UART)
- Input:
  - Keyboard
    - PS/2
    - USB HID
  - Serial
    - NS16550 compatible UART
    - USB CDC ACM (Virtual UART)
- Networking:
  - USB CDC Network Control Model

## Contributing

Please reach out to us on Matrix if you are interested in contributing.

## Licensing
This kernel is licensed under the GNU General Public License version 3.0 (or at your option, any later version).
By contributing to this project you agree to license your contributions under these license terms exclusively.
