# Packaging the VSCode Extension (.vsix)

This guide explains how to build the Rust Analyzer VSCode extension and package it into a `.vsix` file.

## Steps

0. **Set environment variable**
  - To prevent 'overly long loop turn took' messages, run the following command in your terminal (see also https://github.com/rust-lang/rust-analyzer/issues/19754):
    ```cmd
    SET CFG_RELEASE=1
    ```
  - Note when installing the official extension version from the marketplace, make sure to open vscode in a clean environment (without possibly conflicting environment variables set when building a custom extension test build, so possibly have to restart AHK).

1. **Run the VSCode task: Build Server (Release) and Extension**

   This task builds:
   - The Rust backend (`cargo build --release`)
   - The VSCode extension (`npm run build` in `editors/code`)

   The task is defined in `.vscode/tasks.json` like this:

   ```json
   {
     "label": "Build Server (Release) and Extension",
     "dependsOn": [
       "Build Server (Release)",
       "Build Extension"
     ]
   }
   ```

   ### How to run it:

   - Open the Command Palette:
     `Ctrl+Shift+P` or `Cmd+Shift+P`
   - Type and select:
     `Tasks: Run Task`
   - Select:
     `Build Server (Release) and Extension`

2. **Change to the extension directory**

   The extension lives in `editors/code`, not the project root.

   ```bash
   cd editors/code
   ```

3. **Package the extension**

   This creates a `.vsix` file in the current folder:

   ```bash
   npx vsce package
   ```

   Example output:

   ```
   rust-analyzer-<version>.vsix
   ```

4. **Locate and install the generated `.vsix`**

   After running `npx vsce package` inside `editors/code`, the `.vsix` file will appear in the `editors/code` folder in your VSCode file explorer.

   To install the extension:

   - Right-click the `.vsix` file in VSCode’s explorer
   - Select **Install Extension VSIX**

   This installs the packaged extension locally for testing or use.
