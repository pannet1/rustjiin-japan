# Rusting Jiin <img src="icons/32x32.png" alt="icon" align="right" width="48"> 

An interactive, transparent desktop mascot and conversational Japanese language learning companion built with Rust and Tauri.

> :snake: **GitHub Pages**: Visit the project landing page at [https://pannet1.github.io/rustjiin-japan/](https://pannet1.github.io/rustjiin-japan/)

## Features
- **Stateless AI Architecture**: Uses `axum` and HTMX to drive the UI.
- **Local Contextual Memory**: Pure-Rust vector database using `rusqlite` with a custom `cosine_similarity` function for recalling past conversations.
- **Native OS Speech**: Leverages the Web Speech API (`SpeechRecognition` & `SpeechSynthesis`) directly inside the Tauri Webview for zero-latency TTS/STT, along with a fallback text input for systems without a microphone.
- **Dynamic Mascot States**: Seamless UI updates rotating between idle, thinking, listening, and talking graphic states.
- **Transparent Desktop Mode**: Frameless, transparent window for native desktop immersion.

## Prerequisites & Installation

### Windows Installation (Recommended)
Tauri natively hooks into Edge WebView2 and the Windows Desktop Window Manager (DWM), making it the optimal platform for transparent window applications.

1. **Install C++ Build Tools**: 
   - Download the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
   - Install the **Desktop development with C++** workload (ensure Windows 10/11 SDK and MSVC v143 are checked).
2. **Install Rust**:
   - Go to [rustup.rs](https://rustup.rs/) and run `rustup-init.exe`.
   - Restart your terminal when finished.
3. **Run the App**:
   - Open PowerShell or Command Prompt in the project folder.
   - Run `cargo tauri dev`. 

### Linux Installation
*Note: Transparent windows combined with WebKitGTK and proprietary NVIDIA drivers may experience GPU compositing artifacts. CSS hardware-acceleration workarounds are included.*

1. **Install Dependencies**:
   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
   ```
2. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. **Run the App**:
   - Run `cargo tauri dev`.

## Pre-requisites (for Windows CPU-Only Setup)
Since this application connects to a local AI server and your Windows machine has 16GB of RAM with no dedicated GPU, follow this exact path to set up your LLM environment to run comfortably on your CPU:

1. **Install LM Studio**: Download and install [LM Studio](https://lmstudio.ai/) for Windows.
2. **Download the Chat Model**: 
   - Open LM Studio and search for `Meta-Llama-3-8B-Instruct`. 
   - Download the **`Q4_K_M`** version (usually uploaded by `Bartowski`). This is a highly compressed 4-bit model that only requires ~5GB of RAM and runs very fast on CPUs.
3. **Download the Embedding Model**:
   - Search for `nomic-embed-text` and download the **`GGUF`** version. This tiny model runs instantly on the CPU to power Itachi's memory database.
4. **Start the Local Server**: 
   - Go to the **Local Server** tab in LM Studio.
   - Load BOTH models into memory simultaneously (LM Studio allows multiple models).
   - Ensure the server is running on port `8080` (the URL should be `http://localhost:8080/v1`).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
