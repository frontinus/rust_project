# Rust Screen Grabber & Annotation Tool

A multi-platform screen grabbing utility built with Rust and the Druid GUI framework. This was created for a university programming project and implements all core and bonus features.

## 🌟 Features

This tool fulfills all project requirements, including:

- Cross-Platform: Built to be compatible with Windows, macOS, and Linux.

- Advanced Selection: Capture the full screen or select a custom area with a click-and-drag motion.

- Multi-Monitor Support: Recognizes and captures from any connected display.

- Customizable Hotkeys: Set your own global hotkey for instant captures.

- Delay Timer: Set a delay (in seconds) before the capture begins.

- Multiple Output Formats:

  - Save files as .png, .jpg, or .gif.

  - Copy directly to the clipboard.

- Full Annotation Suite:

  - Shapes: Add resizable circles and triangles.

  - Arrows: Add resizable arrows to point things out.

  - Text: Add text directly onto the image.

  - Highlighter: Add a semi-transparent highlighter.

  - Color Picker: Choose the color and transparency for all your annotations.

- Image Editing:

  - Crop: Crop your screenshot after taking it.

  - File Naming: Set a custom file name and save location.

## 🚀 How to Use

- Go to the Releases page.

- Download the .zip file for your operating system.

- Unzip the file and run the application executable.

## 🛠️ Tech Stack

- Language: Rust

- GUI Framework: druid

- Core Libraries:

  - image (For image processing and annotation)

  - arboard (For cross-platform clipboard access)

  - screenshots (For multi-monitor screen capture)

  - serde (For saving and loading hotkey configurations)
  