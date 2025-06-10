# FF-A: Firmware Framework for ARMv8-A Profile
[![check](https://github.com/OpenDevicePartnership/ffa/actions/workflows/check.yml/badge.svg)](https://github.com/OpenDevicePartnership/ffa/actions/workflows/check.yml)
[![no-std](https://github.com/OpenDevicePartnership/ffa/actions/workflows/nostd.yml/badge.svg)](https://github.com/OpenDevicePartnership/ffa/actions/workflows/nostd.yml)
## Overview
This repository is a crate for RUST services running under Hafnium that implements the FF-A protocol as described in DEN0077A "Firmware Framework Arm A-profile". This crate is implements the standard features that every service needs for communicating with Hafnium through FF-A.

This crate is a library and as such must be included into a secure partition image. 

## Feature Status
The following components are available within this crate and implemented.
```
lib.rs      - Defines FfaFunctionId, FfaError, utility functions and high level ffa interface
console     - Implements FFA_CONSOLE_LOG64 to allow debug prints to serial port via println and panic functions
features    - Implements FFA_FEATURES to allow supported features to be querried
indirect    - Implements indirect messaging format through shared memory with non-secure world
memory      - Implements FFA_MEM_RETRIEVE_REQ to setup shared memory
msg         - Implements FFA_MSG_SEND_DIRECT_REQ2 for sending and receiving messages
notify      - Implements FFA_NOTIFICATION_SET for sending notifications to non-secure world
rxtx        - Implements FFA_RXTX_MAP and FFA_RXTX_UNMAP to setup RXTX buffers
version     - Implements FFA_VERSION current returns version 1.2
yld         - Implements FFA_YIELD which allows control to be yielded back to caller for specified amount of time
```

## Building
The ffa crate can be built separately using cargo. You will need rustup and cargo installed

https://rustup.rs

https://doc.rust-lang.org/cargo/commands/cargo-install.html

```
cargo build
```
