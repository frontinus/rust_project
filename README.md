# 📸 Screen Grabber - Advanced Multi-Platform Screenshot Utility

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)

*A powerful, feature-rich screenshot application built with Rust and Druid*

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Keyboard Shortcuts](#%EF%B8%8F-keyboard-shortcuts) • [Documentation](#-documentation)

</div>

---

## 🎯 Overview

Screen Grabber is a modern, cross-platform screenshot utility that goes beyond simple screen capturing. With advanced annotation tools, real-time editing, and multi-monitor support, it's designed for professionals who need precision and flexibility in their workflow.

### ✨ Why Screen Grabber?

- **🎨 Rich Annotation Tools**: Add circles, arrows, text, and highlights directly to your screenshots
- **🖥️ Multi-Monitor Ready**: Seamlessly capture from any connected display
- **⚡ Lightning Fast**: Built in Rust for maximum performance
- **🎨 Post-Capture Editing**: Crop and annotate after taking the screenshot
- **⌨️ Customizable Shortcuts**: Define your own hotkey combinations
- **📋 Clipboard Integration**: Instant copy-to-clipboard with Ctrl+C
- **🎯 Pixel-Perfect Selection**: Precise region selection with visual guides

---

## 🚀 Features

### Core Functionality

#### 📷 **Flexible Capture Modes**
- **Full Screen**: Capture entire monitor with one click
- **Custom Region**: Click-and-drag to select any area
- **Delay Timer**: 0-10 second countdown for timed captures
- **Multi-Monitor**: Choose which screen to capture from dropdown

#### 🎨 **Annotation Suite**
- **Shapes**: Circles, triangles, arrows, and rectangles
- **Highlighter**: Semi-transparent overlay for emphasis
- **Text Tool**: Add custom text with font support
- **Color Picker**: 17 preset colors with adjustable transparency
- **Alpha Slider**: Control annotation opacity (1-100%)

#### ✂️ **Post-Processing**
- **Intelligent Crop**: Refine captured area after screenshot
- **Real-time Preview**: See changes before saving
- **Resizable Annotations**: Drag and resize overlays
- **Layer Management**: Non-destructive editing workflow

#### 💾 **Export Options**
- **Multiple Formats**: PNG, JPG, GIF
- **Smart Naming**: Auto-generate unique filenames or use custom names
- **Custom Save Paths**: Choose where to save your screenshots
- **Clipboard Support**: Copy directly to clipboard for quick sharing

#### ⚙️ **Advanced Settings**
- **Customizable Hotkeys**: Set your preferred keyboard shortcuts
- **Persistent Configuration**: Settings saved between sessions
- **Multi-Monitor Selection**: Easy screen switcher in UI
- **DPI Aware**: Handles high-DPI displays correctly (125%, 150%, 200%)

---

## 📋 Requirements

### System Requirements

- **OS**: Windows 10/11, macOS 10.14+, or Linux (Ubuntu 18.04+, Fedora 30+)
- **RAM**: 256 MB minimum
- **Disk Space**: ~50 MB

### Development Requirements

- **Rust**: 1.70.0 or higher
- **Cargo**: Latest stable version

### Platform-Specific Dependencies

#### Linux

```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libx11-dev libxcb1-dev

# Fedora
sudo dnf install gtk3-devel libX11-devel libxcb-devel

# Arch
sudo pacman -S gtk3 libx11 libxcb
```

#### macOS

```bash
# Xcode Command Line Tools required
xcode-select --install
```

#### Windows

No additional dependencies required.

---

## 🔧 Installation

### Option 1: Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/frontinus/rust_project.git
cd rust_project/application

# Build in release mode
cargo build --release

# Run the application
cargo run --release
```

### Option 2: Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/yourusername/screen-grabber/releases) page.

---

## 🎮 Usage

### Quick Start

1. **Launch the application**
```bash
   cargo run --release
```

2. **Take a screenshot**
   - Press your configured hotkey (default: `KeyB` + `KeyA`)
   - Or click "Take Screenshot" button
   - Select region by clicking and dragging

3. **Annotate (Optional)**
   - Click annotation tools (⭕, △, →, etc.)
   - Drag to position, resize as needed
   - Adjust color and transparency

4. **Save**
   - Click "Save" or let auto-save handle it
   - Find your screenshot in `./src/screenshots/`

### Screenshot Workflow
```
┌─────────────┐
│   Capture   │ ──┐
└─────────────┘   │
                  ▼
┌─────────────────────┐
│  Review & Annotate  │
│  • Add shapes       │
│  • Add text         │
│  • Adjust colors    │
└─────────────────────┘
                  │
                  ▼
┌─────────────────────┐
│   Crop (Optional)   │
└─────────────────────┘
                  │
                  ▼
┌─────────────────────┐
│  Save/Copy/Export   │
└─────────────────────┘
```

---

## ⌨️ Keyboard Shortcuts

### Global Shortcuts

| Shortcut | Action |
|----------|--------|
| **Custom Hotkey** | Open screenshot overlay (configurable in settings) |
| **Ctrl + C** | Copy current screenshot to clipboard |
| **Esc** | Close screenshot overlay / Cancel current operation |
| **Ctrl + W** | Close main window |

### Screenshot Overlay

| Shortcut | Action |
|----------|--------|
| **Click + Drag** | Select capture region |
| **Arrow Keys** | Fine-tune selection (1px increments) |
| **Shift + Arrow Keys** | Fine-tune selection (10px increments) |
| **Enter** | Confirm selection and capture |
| **Esc** | Cancel and close overlay |

### Reserved Combinations

The following shortcuts are reserved and cannot be customized:
- `Ctrl + C` - System copy function
- `Ctrl + W` - Window close
- `Esc` - Cancel operations

---

## 📖 Documentation

### Project Structure
```
screen-grabber/
├── application/
│   ├── src/
│   │   ├── custom_widget/      # Custom UI components
│   │   │   ├── alert.rs
│   │   │   ├── colored_button.rs
│   │   │   ├── custom_slider.rs
│   │   │   ├── custom_zstack.rs    # Layer management
│   │   │   ├── resizable_box.rs    # Annotation container
│   │   │   ├── screenshot_image.rs # Image handling
│   │   │   ├── selected_rect.rs    # Selection rectangle
│   │   │   ├── shortcut_keys.rs    # Hotkey management
│   │   │   └── take_screenshot_button.rs
│   │   ├── images/
│   │   │   └── icons/          # Annotation overlays
│   │   ├── screenshots/        # Default save location
│   │   ├── shortcut/           # Hotkey settings
│   │   └── main.rs             # Application entry point
│   ├── Cargo.toml
│   └── Cargo.lock
└── README.md
```

### Key Components

#### Custom Widgets

- **`CustomZStack`**: Manages layered images and annotations
- **`ScreenshotImage`**: Handles image display, cropping, and transformations
- **`SelectedRect`**: Interactive selection rectangle with 8-point resizing
- **`ResizableBox`**: Container for movable/resizable annotation overlays
- **`CustomSlider`**: Transparency control with real-time preview
- **`ShortcutKeys`**: Keyboard shortcut configuration and persistence

#### Image Processing Pipeline
```rust
Screen Capture (screenshots crate)
    ↓
DPI Scaling Correction
    ↓
Region Selection
    ↓
Annotation Layer Composition
    ↓
Optional Cropping
    ↓
Format Conversion (PNG/JPG/GIF)
    ↓
Save to Disk / Copy to Clipboard
```

---

## 🛠️ Configuration

### Settings File

Settings are automatically saved to:
- **Windows**: `./src/shortcut/shortcut_settings.json`
- **macOS/Linux**: `./src/shortcut/shortcut_settings.json`

### Customizing Hotkeys

1. Click **Settings** → **Shortcut Keys**
2. Click **"Change Shortcut"**
3. Press your desired key combination
4. Settings auto-save on confirmation

### Default Save Location

Screenshots are saved to `./src/screenshots/` by default. To change:
1. Click **Settings** → **Set Path**
2. Select your preferred directory
3. All future screenshots will save there

---

## 🐛 Troubleshooting

### Common Issues

#### ❌ "Window too small" / "Cropped screenshot"
**Cause**: Windows display scaling (125%, 150%, etc.)  
**Solution**: Already handled in v1.0+. If issues persist, check DPI settings.

#### ❌ "Hotkey not working"
**Cause**: Conflicting system shortcut  
**Solution**: Choose a different key combination in Settings → Shortcut Keys

#### ❌ "Screenshot captures the overlay window"
**Cause**: Timing issue with window hiding  
**Solution**: Increase delay timer to 1-2 seconds

#### ❌ "Cannot save screenshot"
**Cause**: Invalid save path or permissions  
**Solution**: Check write permissions in selected directory

### Debug Mode

Run with verbose logging:
```bash
RUST_LOG=debug cargo run --release
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit your changes**: `git commit -m 'Add amazing feature'`
4. **Push to branch**: `git push origin feature/amazing-feature`
5. **Open a Pull Request**

### Development Setup
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/frontinus/rust_project.git
cd rust_project/application
cargo build

# Run tests
cargo test

# Run with hot reload (using cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

### Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new features

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👥 Authors

**Original Development Team**

- **Pietro Bertorelle**
- **Francesco Abate**
- **Elio Magliari**

---

## 🙏 Acknowledgments

- **[Druid](https://github.com/linebender/druid)** - Rust-native UI toolkit
- **[image-rs](https://github.com/image-rs/image)** - Image processing library
- **[screenshots](https://github.com/nashaofu/screenshots-rs)** - Cross-platform screen capture
- **[arboard](https://github.com/1Password/arboard)** - Clipboard integration

---

## 📊 Project Status

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-1.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

**Current Version**: 1.1.0  
**Status**: Active Development  
**Last Updated**: November 2025

---

## 🗺️ Roadmap

### Planned Features

- [ ] **Video Recording**: Capture screen video with audio
- [ ] **GIF Animation**: Create animated GIFs from screen recordings
- [ ] **Cloud Sync**: Auto-upload to Google Drive/Dropbox
- [ ] **OCR Integration**: Extract text from screenshots
- [ ] **Drawing Tools**: Freehand pen, brush, eraser
- [ ] **Blur/Pixelate**: Privacy-focused redaction tools
- [ ] **Annotations Presets**: Save favorite annotation styles
- [ ] **Batch Processing**: Apply edits to multiple screenshots
- [ ] **Screen Recording History**: Timeline view of all captures
- [ ] **Collaboration**: Share screenshots with annotations

---

<div align="center">

**Made with ❤️ and Rust**

⭐ Star us on GitHub if you find this useful!

[Report Bug](https://github.com/yourusername/screen-grabber/issues) • [Request Feature](https://github.com/yourusername/screen-grabber/issues)

</div>
