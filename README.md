# P2Panda Tauri Chat

A peer-to-peer chat application built with Tauri, Svelte, and P2Panda.

## Tech Stack

- **Frontend**: Svelte 5 with TypeScript
- **Backend**: Rust with Tauri
- **P2P**: P2Panda for peer-to-peer communication
- **Build Tool**: Vite

## Development

### Prerequisites

- Node.js (with pnpm)
- Rust
- Tauri CLI

### Getting Started

1. Install dependencies:
   ```bash
   pnpm install
   ```

2. Start the development server:
   ```bash
   pnpm tauri dev
   ```

3. Build for production:
   ```bash
   pnpm tauri build
   ```

## Project Structure

- `src/` - Svelte frontend application
- `src-tauri/` - Rust backend with Tauri commands
- `src-tauri/src/` - Rust source code for P2Panda integration

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
