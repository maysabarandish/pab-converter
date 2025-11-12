use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, FileReader, HtmlInputElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ConvertArgs {
    content: String,
}

#[derive(Clone, Default)]
struct FileInfo {
    name: String,
    size: f64,
}

#[component]
pub fn App() -> impl IntoView {
    let (is_dragging, set_is_dragging) = signal(false);
    let (selected_file_info, set_selected_file_info) = signal::<Option<FileInfo>>(None);
    let (upload_status, set_upload_status) = signal::<Option<String>>(None);
    let (is_converting, set_is_converting) = signal(false);
    let (converted_content, set_converted_content) = signal(String::new());

    let file_input_ref = NodeRef::<leptos::html::Input>::new();

    let handle_files = move |files: web_sys::FileList| {
        if files.length() > 0 {
            if let Some(file) = files.item(0) {
                let file_name = file.name();
                if !file_name.ends_with(".ohh")
                    && !file_name.ends_with(".txt")
                    && !file_name.ends_with(".json")
                {
                    set_upload_status.set(Some(
                        "Error: Please upload an OHH, TXT, or JSON file".to_string(),
                    ));
                    set_selected_file_info.set(None);
                    return;
                }

                let file_info = FileInfo {
                    name: file_name.clone(),
                    size: file.size(),
                };

                set_selected_file_info.set(Some(file_info));
                set_upload_status.set(Some(format!("Selected: {}", file_name)));

                // Start reading the file
                let file_reader = FileReader::new().unwrap();
                let fr = file_reader.clone();

                let onload = Closure::wrap(Box::new(move |_: Event| {
                    if let Ok(result) = fr.result() {
                        if let Some(text) = result.as_string() {
                            set_upload_status.set(Some("File ready to convert".to_string()));
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();

                let _ = file_reader.read_as_text(&file);
            }
        }
    };

    let on_file_input = move |ev: Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                if let Some(files) = input.files() {
                    handle_files(files);
                }
            }
        }
    };

    let convert_file = move |_| {
        if let Some(input_element) = file_input_ref.get() {
            let input_el: &HtmlInputElement = input_element.as_ref();
            if let Some(files) = input_el.files() {
                if let Some(file) = files.item(0) {
                    let file_name = file.name().clone();
                    set_is_converting.set(true);
                    set_upload_status.set(Some("Converting...".to_string()));

                    let file_reader = FileReader::new().unwrap();
                    let fr = file_reader.clone();

                    // Clone signals for use in the closure
                    let set_is_converting_clone = set_is_converting;
                    let set_upload_status_clone = set_upload_status;
                    let set_converted_content_clone = set_converted_content;
                    let set_selected_file_info_clone = set_selected_file_info;

                    let onload = Closure::wrap(Box::new(move |_: Event| {
                        if let Ok(result) = fr.result() {
                            if let Some(content) = result.as_string() {
                                // Now we have the file content, invoke the backend
                                let file_name_copy = file_name.clone();
                                spawn_local(async move {
                                    let args =
                                        serde_wasm_bindgen::to_value(&ConvertArgs { content })
                                            .unwrap();
                                    let result = invoke("convert_ohh_content", args).await;

                                    set_is_converting_clone.set(false);

                                    if let Some(response) = result.as_string() {
                                        if response.starts_with("Error")
                                            || response.starts_with("Failed")
                                        {
                                            set_upload_status_clone
                                                .set(Some(format!("[ERR] {}", response)));
                                            set_converted_content_clone.set(String::new());
                                        } else {
                                            set_converted_content_clone.set(response);
                                            set_upload_status_clone.set(Some(format!(
                                                "[OK] Successfully converted: {}",
                                                file_name_copy
                                            )));
                                            set_selected_file_info_clone.set(None);
                                        }
                                    }
                                });
                            }
                        }
                    }) as Box<dyn FnMut(_)>);

                    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    onload.forget();

                    let _ = file_reader.read_as_text(&file);
                }
            }
        }
    };

    let download_file = move |_| {
        let content = converted_content.get_untracked();
        if content.is_empty() {
            return;
        }

        // Create a blob and download link
        spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let array = js_sys::Array::new();
            array.push(&JsValue::from_str(&content));
            let blob = web_sys::Blob::new_with_str_sequence(&array).unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", "converted_hands.txt").unwrap();

            let html_element = a.dyn_into::<web_sys::HtmlElement>().unwrap();
            html_element.click();

            web_sys::Url::revoke_object_url(&url).unwrap();
        });
    };

    let copy_to_clipboard = move |_| {
        let content = converted_content.get_untracked();
        if content.is_empty() {
            return;
        }

        spawn_local(async move {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            let promise = clipboard.write_text(&content);
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            set_upload_status.set(Some("Copied to clipboard".to_string()));
        });
    };

    view! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-4xl mx-auto">
                <div class="text-center mb-12">
                    <h1 class="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-4">
                        "OHH to PokerStars Converter"
                    </h1>
                    <p class="text-xl text-gray-600 dark:text-gray-300">
                        "Convert .ohh poker hand history files to PokerStars format for GTO Wizard"
                    </p>
                </div>

                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 mb-8">
                    <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
                        "Upload and Convert"
                    </h2>

                    <div
                        class="border-2 border-dashed rounded-lg p-12 text-center transition-all duration-200 cursor-pointer border-gray-300 dark:border-gray-600 hover:border-blue-400 dark:hover:border-blue-500"
                    >
                        <div class="space-y-4">
                            <div class="flex justify-center">
                                <svg class="w-16 h-16 text-gray-400 dark:text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                                </svg>
                            </div>
                            <div>
                                <p class="text-lg text-gray-700 dark:text-gray-300 mb-2">
                                    "Drag and drop your file here"
                                </p>
                                <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
                                    "or"
                                </p>
                                <button
                                    class="inline-block bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded-lg cursor-pointer transition-colors"
                                    on:click=move |_| {
                                        if let Some(input_element) = file_input_ref.get() {
                                            let input_el: &HtmlInputElement = input_element.as_ref();
                                            let _ = input_el.click();
                                        }
                                    }
                                >
                                    "Browse Files"
                                </button>
                                <input
                                    node_ref=file_input_ref
                                    type="file"
                                    accept=".ohh,.txt,.json"
                                    class="hidden"
                                    on:change=on_file_input
                                />
                            </div>
                            <p class="text-xs text-gray-500 dark:text-gray-400">
                                "Only .ohh, .txt, or .json files are accepted"
                            </p>
                        </div>
                    </div>

                    {move || selected_file_info.get().map(|file_info| {
                        view! {
                            <div class="mt-6 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center space-x-3">
                                        <svg class="w-8 h-8 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path>
                                        </svg>
                                        <div>
                                            <p class="font-medium text-gray-900 dark:text-white">{file_info.name.clone()}</p>
                                            <p class="text-sm text-gray-500 dark:text-gray-400">
                                                {format!("{:.2} KB", file_info.size / 1024.0)}
                                            </p>
                                        </div>
                                    </div>
                                    <button
                                        class="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-semibold py-2 px-6 rounded-lg transition-colors"
                                        on:click=convert_file
                                        disabled=move || is_converting.get()
                                    >
                                        {move || if is_converting.get() { "Converting..." } else { "Convert" }}
                                    </button>
                                </div>
                            </div>
                        }
                    })}

                    {move || upload_status.get().map(|status| {
                        let is_error = status.starts_with("Error") || status.starts_with("[ERR]");
                        let is_success = status.starts_with("[OK]");
                        view! {
                            <div class=move || {
                                let base = "mt-4 p-4 rounded-lg";
                                if is_error {
                                    format!("{} bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-400", base)
                                } else if is_success {
                                    format!("{} bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400", base)
                                } else {
                                    format!("{} bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400", base)
                                }
                            }>
                                {status}
                            </div>
                        }
                    })}
                </div>

                {move || (!converted_content.get().is_empty()).then(|| {
                    view! {
                        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
                            <div class="flex justify-between items-center mb-4">
                                <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
                                    "Converted Output"
                                </h2>
                                <div class="flex gap-2">
                                    <button class="bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-900 dark:text-white font-semibold py-2 px-4 rounded-lg transition-colors" on:click=copy_to_clipboard>
                                        "Copy"
                                    </button>
                                    <button class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors" on:click=download_file>
                                        "Download"
                                    </button>
                                </div>
                            </div>
                            <pre class="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 overflow-auto max-h-96 text-sm text-gray-800 dark:text-gray-200">
                                { move || {
                                    let content = converted_content.get();
                                    if content.len() > 1000 {
                                        format!("{}...\n\n[{} total characters - download to see full output]",
                                                &content[..1000], content.len())
                                    } else {
                                        content
                                    }
                                }}
                            </pre>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
