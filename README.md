# EPIK17 Launcher

Hey, this is a simple Rust app that helps you launch games on EPIK17. It keeps itself and the game client updated, sets up a way to open games from links, and gets you playing quickly.

## What It Does

- **Stays Up to Date**: Downloads new versions of itself if there's an update.
- **Updates the Game**: Grabs the latest EPIK17 client (the player app) when needed.
- **Launches Games**: Starts the game with the right settings from those links.
- Works on Windows (uses the registry for link stuff).

## What You Need

- Rust installed (get it from rustup.rs).
- Windows computer.

## How to Build It

1. Get the code (clone or download).

2. Open a terminal and go to the folder:
   ```
   cd EPIK17-Launcher
   ```

3. Build it:
   ```
   cargo build --release
   ```

4. Find the app at `target/release/epik17_launcher.exe`.

## How to Use It

- Just run the exe. It'll check for updates and set things up.
- For playing: Click a link like `epik17:play+gameid:123+ticket:abc123` – it opens the game for you.
- No link? It just does the update checks.

## Quick Notes

- Files go in your `%APPDATA%\EPIK17\` folder.
- Game stuff in `%APPDATA%\EPIK17\Client\`.

## If Something Goes Wrong

- Can't register links? Try running as admin.
- Check the console for error messages.

Anything that is needed for the setup is gonna be added here if needed