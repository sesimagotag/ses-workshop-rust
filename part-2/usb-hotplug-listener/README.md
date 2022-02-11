# Build a USB hotplug events listener

Find the [rusb](https://docs.rs/rusb/0.9.0/rusb/) crate. This crate provides a safe wrapper around the low-level
library `libusb`. Make sure your system is providing this library.

## The Challenge

Make use of the [rusb](https://docs.rs/rusb/0.9.0/rusb/) crate and build a loop to listen for any USB hotplug events and
print basic USB device descriptor information.
