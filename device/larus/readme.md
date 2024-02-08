Firmware for the Larus Frontend 57
===

Development Environment
---
This section contains the sources for the hardware adaptation to the Larus Frontend 57 display. If you want to develop or compile the sources yourself, you will need the following:
- You must first [install the Rust environment](https://www.rust-lang.org/tools/install). 
- Then you will [enable support for the embedded hardware](https://docs.rust-embedded.org/book/intro/install.html). You will need the Cortex M4F and Cortex M7F architecture ($ rustup target add thumbv7em-none-eabihf).
- It is recommended to use [Visual Studio Code](https://code.visualstudio.com/) as a development environment. You will also need the plugins Cargo, CodeLLDB, Cortex-Debug, rust-analyzer.

Install Software using USB
---
- Find the [*.elf binarys files from the releases](https://github.com/larus-breeze/sw_frontend_rs/releases).
- Flash it using [the tool STM32CUBEPROG](https://www.st.com/en/development-tools/stm32cubeprog.html).
- Keep the button on the backside of the frontend pressed and insert the USB-C cable connected to a PC. The frontend with boot to STs build-in DFU bootloader.
- Press the USB connect button in the STM32CUBEPROG.
- Load and download the *.elf file.