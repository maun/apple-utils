//! Example using [dioxus](https://github.com/dioxuslabs/dioxus)
//! [Dioxus Setup](https://dioxuslabs.com/learn/0.6/getting_started/#install-the-dioxus-cli)
//! 
//! Make sure to the dioxus cli and start the iOS Simulator,
//! then launch with `dx serve --example dioxus_file_picker --platform ios`

use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;",
            button {
                style: "font-size: 30px;",
                onclick: move |_| async {
                    #[cfg(target_os = "ios")]
                    {
                        use apple_utils::ios::FilePicker;
                        use apple_utils::file_type::FileType;

                        let picker = FilePicker {
                            filters: vec![FileType::MimeType("text/plain".to_string())],
                            multiple_selection: true,
                            ..Default::default()
                        };

                        let paths = picker.open().await;
                        for path in paths {
                            println!("Selected file: {:?}", path);
                            match std::fs::read_to_string(&path) {
                                Ok(content) => println!("File content: {:?}", content),
                                Err(err) => println!("Error reading file: {}", err),
                            }
                        }
                    }
                },
                "Pick files"
            }
        }
    }
}
