Firmware for the Air Avionics Air Control Display 57
===

Install Development Environment
---
This section contains the sources for the hardware adaptation to the Air Avionic AD57 display. If you want to develop or compile the sources yourself, you will need the following:
- You must first [install the Rust environment](https://www.rust-lang.org/tools/install). 
- Then you will [enable support for the embedded hardware](https://docs.rust-embedded.org/book/intro/install.html). You will need the Cortex M4F and Cortex M7F architecture ($ rustup target add thumbv7em-none-eabihf).
- It is recommended to use [Visual Studio Code](https://code.visualstudio.com/) as a development environment. You will also need the plugins CodeLLDB, Cortex-Debug, rust-analyzer.

Install Firmware from SdCard
---
- Find the [*.bin image files from the releases](https://github.com/larus-breeze/sw_frontend_rs/releases) and copy image to an SdCard.
- Switch off the frontend and insert the SdCard.
- Switch on the frontend. The image is installed automatically. This takes a few seconds and is indicated by a message on the display. 
- Do not switch off the frontend during this process.
