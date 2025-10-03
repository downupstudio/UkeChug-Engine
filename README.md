# UkeChug Browser Engine

A browser rendering engine built from scratch in Rust that parses HTML and CSS and renders them to PNG images.

## Features

- **HTML Parser** - Parses HTML into a DOM tree structure
- **CSS Parser** - Parses CSS selectors, properties, and values including hex colors
- **Style Engine** - Matches CSS rules to DOM elements with specificity-based cascade
- **Layout Engine** - Implements CSS box model with margin, padding, and borders
- **Rendering** - Outputs styled, positioned content as PNG images with:
  - Custom text colors and font sizes
  - Background colors
  - Borders with configurable colors and widths
  - Text rendering with TrueType font support

## Installation

### Prerequisites

- Rust (1.70 or later)
- Cargo

### Build

```bash
git clone https://github.com/downupstudio/UkeChug.git
cd UkeChug
cargo build --release
```

## Usage

### Basic Usage

Render default test files:
```bash
cargo run
```

### Specify Files

```bash
cargo run <html_file> <css_file>
```

Example:
```bash
cargo run mypage.html styles.css
```

### Custom Output and Size

```bash
cargo run <html_file> <css_file> -o <output.png> -w <width> -h <height>
```

Example:
```bash
cargo run index.html style.css -o result.png -w 1024 -h 768
```

### Command-Line Options

- `-o, --output <FILE>` - Output PNG file (default: output.png)
- `-w, --width <PIXELS>` - Image width (default: 800)
- `-h, --height <PIXELS>` - Image height (default: 600)
- `--help` - Show help information

## Example

Create `example.html`:
```html
<html>
<body>
<div>
<h1>Hello World</h1>
<p>This is a browser engine.</p>
</div>
</body>
</html>
```

Create `example.css`:
```css
html { display: block; width: 780px; }
body { display: block; width: 700px; background-color: #f0f0f0; }
div { display: block; width: 600px; background-color: white; 
      margin: 20px; padding: 15px; border-width: 2px; border-color: #333; }
h1 { display: block; color: #e74c3c; font-size: 32px; margin: 10px; }
p { display: block; color: #34495e; font-size: 16px; margin: 10px; }
```

Render:
```bash
cargo run example.html example.css
```

Output will be saved as `output.png`.

## Supported CSS Properties

- `display` (block only)
- `width`, `height`
- `margin`, `padding`
- `border-width`, `border-color`
- `background-color`, `background`
- `color`
- `font-size`

Colors support hex format (`#RRGGBB`) and keywords (white, black, red, green, blue, yellow, gray).

## Project Structure

```
UkeChug/
├── src/
│   ├── html/         # HTML parser
│   ├── css/          # CSS parser and stylesheet structures
│   ├── dom/          # DOM tree representation
│   ├── style/        # Style matching and cascade
│   ├── layout/       # Layout engine (box model)
│   ├── render/       # PNG rendering
│   ├── fonts/        # TrueType fonts
│   └── main.rs       # CLI entry point
├── Cargo.toml
└── README.md
```

## Technical Details

- **Language**: Rust
- **Dependencies**: 
  - `image` - Image manipulation
  - `imageproc` - Drawing primitives
  - `ab_glyph` - Font rendering
  - `clap` - Command-line parsing
  - `html5ever` - HTML parsing utilities
  - `cssparser` - CSS parsing utilities

## Limitations

- Only block-level layout (no inline or flex)
- No JavaScript support
- Limited CSS property support
- No network fetching
- Single-threaded rendering

## Future Roadmap

- Inline layout support
- More CSS properties (flexbox, positioning)
- Word wrapping and text flow
- Image element support (`<img>`)
- JavaScript engine integration
- Network request handling

## License

MIT

## Contributing

Contributions welcome. This is an educational project demonstrating browser engine fundamentals.

## Acknowledgments

Built as a learning project to understand browser rendering pipelines. Inspired by browser engines like Servo, WebKit, and Blink.
