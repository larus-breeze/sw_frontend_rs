Firmware for the Larus Frontend 57
===

This section contains the sources for the hardware adaptation to the Larus Frontend 57 display. If you want to develop or compile the sources yourself, you will need the following:
- You must first [install the Rust environment](https://www.rust-lang.org/tools/install). 
- Then you will [enable support for the embedded hardware](https://docs.rust-embedded.org/book/intro/install.html). You will need the Cortex M4F and Cortex M7F architecture ($ rustup target add thumbv7em-none-eabihf).
- It is recommended to use [Visual Studio Code](https://code.visualstudio.com/) as a development environment. You will also need the plugins Cargo, CodeLLDB, Cortex-Debug, rust-analyzer.
