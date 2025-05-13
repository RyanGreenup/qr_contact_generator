# QR vCard Generator

A desktop application for creating business contact QR codes. Built with Rust using the [egui](https://github.com/emilk/egui) framework.

## Features

- GUI form to input contact details and gerate [Vcard](https://en.wikipedia.org/wiki/VCard)
- Generate QR codes from vCard data
- Save QR codes as PNG files
- User-friendly interface with form validation

## Quick Start

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/qr_vcard_generator.git
   cd qr_vcard_generator
   ```

2. Build and run the application:
   ```bash
   cargo run --release
   ```

### Usage

1. Fill in the contact information fields
2. Click "Generate vCard" (or press Ctrl+G)
3. Your QR code will be displayed
4. Save the QR code using the "Download QR Code" button (or press Ctrl+S)
5. Share your QR code on business cards, email signatures, or websites

## Keyboard Shortcuts

- `Ctrl+G`: Generate vCard and QR code
- `Ctrl+C`: Copy vCard text to clipboard
- `Ctrl+S`: Save QR code as PNG file

## License

This project is GPL-licensed.

