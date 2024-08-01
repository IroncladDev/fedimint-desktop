use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::ui::Flex;

#[component]
pub fn QRCode(value: String, class: Option<String>, size: Option<usize>) -> Element {
    let qr_image_size = size.unwrap_or(1024);
    let qr_data_url = use_memo(move || {
        let png_bytes = qrcode_generator::to_png_to_vec_from_str(
            value.clone(),
            qrcode_generator::QrCodeEcc::Medium,
            qr_image_size,
        )
        .expect("Failed to generate QR code");
        let png_base64 = base64::display::Base64Display::new(
            &png_bytes,
            &base64::engine::general_purpose::STANDARD,
        );
        format!("data:image/png;base64,{png_base64}")
    });

    let class_name = tw_merge!("max-w-[320px] rounded-lg border", class);

    rsx! {
        Flex { col: true, center: true, p: 2, class: "bg-secondary rounded-lg",
            img { class: class_name, src: qr_data_url }
        }
    }
}
