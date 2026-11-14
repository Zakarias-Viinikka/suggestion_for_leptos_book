use std::io::{Cursor, Write};
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub fn execute_download(things: Vec<String>, extension: &str) {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut cursor);
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);

        for (i, content) in things.into_iter().enumerate() {
            let filename = format!("file_{i}.{extension}");
            zip.start_file(&filename, options).unwrap();
            zip.write_all(content.as_bytes()).unwrap();
        }
        zip.finish().unwrap();
    }

    let zip_bytes = cursor.into_inner();
    download_blob("things.zip", &zip_bytes);
}

fn download_blob(filename: &str, bytes: &[u8]) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let uint8 = js_sys::Uint8Array::from(bytes);
    let parts = js_sys::Array::new();
    parts.push(&uint8.buffer());

    let mut opts = BlobPropertyBag::new();
    opts.type_("application/zip");
    let blob = Blob::new_with_buffer_source_sequence_and_options(&parts, &opts).unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
    a.set_href(&url);
    a.set_download(filename);
    a.click();

    Url::revoke_object_url(&url).unwrap();
}
