# apple-utils

A Rust library providing safe wrappers around Apple platform features using [objc2](https://crates.io/crates/objc2) bindings.

## Features

### iOS

- **File Picker**: Native iOS document picker implementation via `FilePicker`
- **File Type Handling**: Work with different file type specifications using `FileType`

## Usage Example

```rust
use apple_utils::file_type::FileType;
use apple_utils::ios::FilePicker;

async fn pick_images() -> Vec<std::path::PathBuf> {
    let picker = FilePicker {
        filters: vec![FileType::UniformTypeIdentifier("public.image".to_string())],
        multiple_selection: true,
        ..Default::default()
    };

    picker.open().await
}
```

## Safety

The underlying objc2 bindings are unsafe. While this crate aims to provide safe wrappers,
these wrappers have not been rigorously tested.

## Requirements

- iOS target platform
- Must be run on the main thread
- Requires an active UIViewController

## License

apple-utils is distributed under the terms of both the MIT license and the
Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for details. Opening a pull request is
assumed to signal agreement with these licensing terms.