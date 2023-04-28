<div align="center">
  <h1><code>tsv-converter</code></h1>
  <p><strong>TinyCircuits TinyScreen Video (TSV) converter</strong></p>
</div>

# Features

- Video preview (source resolution, TinyScreen aspect ratio)

issues:

- webkit2gtk video issue: https://github.com/tauri-apps/tauri/issues/3725
- `asset://` protocol does not release memory:
  - https://github.com/tauri-apps/tauri/issues/2952
  - https://github.com/MicrosoftEdge/WebView2Feedback/issues/2171

todo:

- Crop cover (zoom) gravity (and extent?)
- Crop contain (letterbox) background color
- Drag and drop file or copy paste file
- ffmpeg log
- keyboard shortcuts
- cli interface

# Development

1. Follow the [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) to
   install Rust, then install node LTS gallium (v16) and enable yarn with corepack:

   ```sh
   nvm install --lts=gallium # use your favorite node version manager, or install manually
   nvm use --lts=gallium

   corepack enable yarn
   ```

2. Install dependencies:

   ```sh
   yarn
   ```

3. Run the svelte frontend and tauri app in dev mode:

   ```sh
   yarn dev
   ```

# Other helpful scripts:

* `cargo clean` - Remove artifacts from the target directory that Cargo has generated in the past. 
* `yarn build --verbose` - Gives more error description when building the application. 
* `rustc --version` - Check your version of rust.
