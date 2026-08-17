You are an expert Rust developer specializing in desktop application architecture using Tauri, Askama (for type-safe HTML templates), and htmx. 

Your objective is to build the foundational core of a frameless, transparent desktop application. We are strictly following a Feature-Centric Development methodology. You must structure the Rust codebase so that future features (Audio I/O, LLM Controller, State Machine) can be added as isolated, self-contained modules later.

### TECH STACK
* Backend: Rust, Tauri
* Frontend HTML Generation: Askama (or a minimal Axum/Actix local server if necessary for htmx routing)
* Client-Server Interactivity: htmx

### ARCHITECTURE & FEATURE-CENTRIC DESIGN
* Do not group files by technical role (e.g., do not create global `views` or `controllers` folders).
* Group files by feature. For this initial scaffold, create a `src-tauri/src/features/core` module that will house the window initialization, base layout, and initial mascot placeholder logic.
* The frontend `.html` templates must be strongly typed using Askama and cleanly integrated with the Rust logic that controls them.

### CURRENT SCOPE: THE "CORE" FEATURE
Do not implement any audio APIs, LLM integrations, or complex state machines yet. Build only this foundational scaffold:
1. Window Configuration: The Tauri application must be configured in `tauri.conf.json` and the Rust entry point to be frameless, have a transparent background, and remain always-on-top.
2. UI Scaffolding: The frontend must render a basic placeholder div representing the mascot ("Itachi").
3. Askama & htmx Integration: The UI must be constructed using Askama templates. Since htmx expects HTTP requests, you must implement a reliable bridge for Tauri (e.g., running a lightweight local Axum server inside Tauri to handle the htmx `hx-get`/`hx-post` requests, or using a custom Tauri IPC bridge for htmx).
4. Interactivity: Clicking the mascot placeholder must trigger an htmx request to the Rust backend in the `core` module. The backend method should log the click and return a minimal HTML fragment updating a counter or text state to prove end-to-end connectivity.

### STRICT DEVELOPMENT GUIDELINES
* Do not use React, Vue, or any heavy frontend frameworks.
* You must provide complete, non-fragmented source files for every file required to run this core setup (e.g., `main.rs`, `Cargo.toml`, `tauri.conf.json`, `index.html`, Askama templates, and the bridge implementation). 
* UNDER NO CIRCUMSTANCES should you provide partial snippets, omit imports, or leave placeholders (like "insert logic here") in the code. Always output the full, runnable file scripts.
