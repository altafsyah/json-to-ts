# json-to-ts

A fast, zero-config CLI tool that converts JSON into TypeScript interfaces. Pipe any JSON in get clean `.ts` interfaces out.

## Features

- Nested objects become separate named interfaces
- Arrays are inferred as typed arrays (`string[]`, `number[]`, etc.)
- Mixed-type arrays become union types (`(number | string)[]`)
- Null values become optional fields (`key?: null`)
- Empty arrays are typed as `unknown[]`

## Quick Example

```bash
echo '{
  "id": 1,
  "name": "John Doe",
  "address": { "city": "Jakarta", "zip": "12345" },
  "tags": ["rust", "cli"],
  "scores": [90, 85, 100],
  "meta": null
}' | json-to-ts
```

Output:

```typescript
interface Root {
  address: Address;
  id: number;
  meta?: null;
  name: string;
  scores: number[];
  tags: string[];
}

interface Address {
  city: string;
  zip: string;
}
```

## Installation

### From source (requires [Rust](https://www.rust-lang.org/tools/install))

```bash
cargo install --git https://github.com/altafsyah/json-to-ts
```

### Linux dependencies (required for Zed integration)

The Zed task uses `xclip` for clipboard and `notify-send` for notifications:

```bash
sudo apt install xclip libnotify-bin
```

## Usage

```bash
# From a file
cat data.json | json-to-ts

# Inline JSON
echo '{"id": 1, "name": "John Doe"}' | json-to-ts

# Copy output to clipboard (Linux/X11)
cat data.json | json-to-ts | xclip -selection clipboard

# Save to a file
cat data.json | json-to-ts > types.ts
```

## Editor Integration

### Zed

Add a task to your Zed task configuration (`.zed/tasks.json` or global tasks):

```json
[
  {
    "label": "json-to-ts",
    "command": "cat $ZED_FILE | json-to-ts | xclip -selection clipboard && notify-send 'JSON to TS' 'Copied!'",
    "reveal": "never"
  }
]
```

Optionally bind it to a shortcut in `~/.config/zed/keymap.json`:

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "ctrl-j j": ["task::Spawn", { "task_name": "json-to-ts" }]
    }
  }
]
```

Open any `.json` file, press `Ctrl+J J`, then `Ctrl+V` into your `.ts` file.

## Contributing

Contributions are welcome! Here's how to get started:

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (stable)

### Setup

```bash
git clone https://github.com/altafsyah/json-to-ts.git
cd json-to-ts
cargo build
```

### Running locally

```bash
echo '{"id": 1, "name": "John Doe"}' | cargo run
```

### Project structure

```
src/
  main.rs    # Entire CLI — stdin parsing, interface generation, type inference
Cargo.toml   # Single dependency: serde_json
```

This is intentionally a single-file project. The core logic is ~96 lines of Rust.

### Areas for contribution

- **Tests** — there are currently no automated tests; adding test coverage would be very valuable
- **Nested array objects** — improve type merging when array items have different shapes
- **Output formatting** — options like `export` keyword, `type` aliases, or `--indent` flag
- **Clipboard support** — built-in cross-platform clipboard without needing `xclip`
- **Editor plugins** — integrations for VS Code, Neovim, or other editors

### Submitting a PR

1. Fork the repository
2. Create a feature branch (`git checkout -b my-feature`)
3. Make your changes
4. Verify it works: `echo '{"test": true}' | cargo run`
5. Commit and push your branch
6. Open a pull request

## Roadmap

- [x] CLI tool
- [x] Zed task integration
- [x] Keybinding support
- [ ] Installation script (`curl | sh`)
- [ ] Zed extension with slash command (`/json-to-ts`)

## License

MIT
