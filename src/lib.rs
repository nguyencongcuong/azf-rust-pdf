use std::io::Cursor;
use std::time::Instant;

use printpdf::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PdfOptions {
    pub title: String,
    pub language: String,
    pub width: f32,
    pub height: f32,
}

pub fn calculate_time<F, R>(operation_name: &str, function: F) -> R where F: FnOnce() -> R {
    let start = Instant::now();
    println!("[Start] {}", operation_name);

    let result = function();

    let elapsed = start.elapsed();
    println!("[End] {} {:?}", operation_name, elapsed);

    return result;
}

pub fn generate_pdf(options: &PdfOptions) -> Result<Vec<u8>, Error> {
    // PDF Initialization
    let (doc, page1, layer1) = PdfDocument::new(&options.title, Mm(options.width), Mm(options.height), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let language: &str = &options.language;

    // Create a PdfLayerReference and embed the font
    // AWESOME: This make the pdf size extremely tiny!!!
    let mut font_reader: Cursor<&[u8]>;
    match language {
        "HK" => font_reader = std::io::Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansHK-Regular.ttf").as_ref()),
        "JP" => font_reader = std::io::Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansJP-Regular.ttf").as_ref()),
        "KR" => font_reader = std::io::Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansKR-Regular.ttf").as_ref()),
        "MY" => font_reader = std::io::Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansMalayalam-Regular.ttf").as_ref()),
        "SC" => font_reader = std::io::Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansSC-Regular.ttf").as_ref()),
        "TC" => font_reader = std::io::Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansTC-Regular.ttf").as_ref()),
        _ => font_reader = std::io::Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSans-Regular.ttf").as_ref())
    }
    let font = doc.add_external_font(&mut font_reader).unwrap();

    // Create PDF content
    current_layer.begin_text_section();

    // Set up the general fonts.
    current_layer.set_font(&font, 5.0);
    current_layer.set_text_cursor(Mm(5.0), Mm(95.0));
    current_layer.set_line_height(6.0);

    // write two lines (one line break)
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();
    current_layer.write_text("HK:飲茶去邊 - JP:今日いい - KR:저는 7년 동 - MY: സ്വയരക്ഷാബോധ - SC:鉴于对人 - TC:鑑於人", &font);
    current_layer.add_line_break();

    current_layer.end_text_section();

    return doc.save_to_bytes();
}