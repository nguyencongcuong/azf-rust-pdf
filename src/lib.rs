use std::io::{Cursor, Read};

use jpeg_decoder;
use printpdf::{BuiltinFont, Color, Image, ImageTransform, IndirectFontRef, Mm, PdfDocument, PdfDocumentReference, Rect, Rgb, TextRenderingMode};
use serde::Deserialize;

mod util;

#[derive(Deserialize)]
pub struct Options {
    pub title: String,
    pub template: String,
}

pub fn init_fonts(doc: &PdfDocumentReference, language: &str) -> IndirectFontRef {
    let font;

    match language {
        "HK" => font = doc.add_external_font(Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansHK-Regular.ttf").as_ref())).unwrap(),
        "JP" => font = doc.add_external_font(Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansJP-Regular.ttf").as_ref())).unwrap(),
        "KR" => font = doc.add_external_font(Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansKR-Regular.ttf").as_ref())).unwrap(),
        "MY" => font = doc.add_external_font(Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansMY-Regular.ttf").as_ref())).unwrap(),
        "SC" => font = doc.add_external_font(Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansSC-Regular.ttf").as_ref())).unwrap(),
        "TC" => font = doc.add_external_font(Cursor::new(include_bytes!("../src/fonts/NotoSans/NotoSansTC-Regular.ttf").as_ref())).unwrap(),
        _ => font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap()
    }

    font
}

pub fn print_template_1(options: &Options) -> Result<Vec<u8>, printpdf::Error> {
    let Options { title, .. } = options;
    const DOC_WIDTH: f32 = 174.0;
    const DOC_LENGTH: f32 = 100.0;
    const DOC_LANGUAGE: &str = "SC";
    const FONT_SIZE_MED: f32 = 8.0;

    // Initialize
    let (doc, page1, layer1) = PdfDocument::new(title, Mm(DOC_WIDTH), Mm(DOC_LENGTH), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = init_fonts(&doc, DOC_LANGUAGE);
    let default_font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    // Generate contents
    let rect = Rect::new(Mm(174.0), Mm(174.0), Mm(0.0), Mm(90.0));
    layer.add_rect(rect);

    let mut white: Color = Color::Rgb(Rgb::new(100., 100., 100., None));
    let mut black: Color = Color::Rgb(Rgb::new(0., 0., 0., None));

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&font, 14.0);
    layer.set_text_cursor(Mm(5.0), Mm(93.0));
    layer.set_outline_thickness(1.);
    layer.set_outline_color(white.clone());
    layer.set_fill_color(white.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("OVERNIGHT", &font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&font, 14.0);
    layer.set_text_cursor(Mm(100.0), Mm(93.0));
    layer.set_outline_thickness(1.);
    layer.set_outline_color(white.clone());
    layer.set_fill_color(white.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("5KG", &font);
    layer.end_text_section();

    // Add Image: Logo
    let logo = util::read_image("assets/images/logo.png");
    Image::from_dynamic_image(&logo).add_to_layer(
        layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(5.0)),
            translate_y: Some(Mm(74.0)),
            rotate: None,
            scale_x: None,
            scale_y: None,
            dpi: None,
        },
    );

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&default_font, FONT_SIZE_MED);
    layer.set_text_cursor(Mm(50.0), Mm(85.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.1);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("Cust Ref: PXLUS0000121GNJ1G1", &default_font);
    layer.add_line_break();
    layer.write_text("Intl CourierPost Overnight", &default_font);
    layer.end_text_section();

    // Add: QR Code
    let qr_code = util::read_image("assets/images/qr_code.png");
    Image::from_dynamic_image(&qr_code).add_to_layer(
        layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(50.0)),
            translate_y: Some(Mm(35.0)),
            rotate: None,
            scale_x: None,
            scale_y: None,
            dpi: None,
        },
    );

    // Add: Barcode
    let barcode = util::read_image("assets/images/barcode.png");
    Image::from_dynamic_image(&barcode).add_to_layer(
        layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(25.0)),
            translate_y: Some(Mm(-5.0)),
            rotate: None,
            scale_x: None,
            scale_y: None,
            dpi: None,
        },
    );

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&default_font, FONT_SIZE_MED + 2.0);
    layer.set_text_cursor(Mm(31.0), Mm(4.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.5);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("(00) 79420 00251 00317 892", &default_font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&default_font, FONT_SIZE_MED + 2.0);
    layer.set_text_cursor(Mm(3.0), Mm(68.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.1);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("Non Signature:", &default_font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&default_font, FONT_SIZE_MED);
    layer.set_text_cursor(Mm(3.0), Mm(25.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.1);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("Generated:", &default_font);
    layer.add_line_break();
    layer.write_text("27/01/2024", &default_font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&default_font, FONT_SIZE_MED);
    layer.set_text_cursor(Mm(30.0), Mm(25.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.1);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("Expires:", &default_font);
    layer.add_line_break();
    layer.write_text("20/05/2024", &default_font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&font, FONT_SIZE_MED);
    layer.set_text_cursor(Mm(100.0), Mm(75.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.1);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("To:", &font);
    layer.add_line_break();
    layer.write_text("北京市,", &font);
    layer.add_line_break();
    layer.write_text("朝阳区,", &font);
    layer.add_line_break();
    layer.write_text("建国路,", &font);
    layer.add_line_break();
    layer.write_text("甲1号", &font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&font, FONT_SIZE_MED);
    layer.set_text_cursor(Mm(150.0), Mm(75.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.3);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("Ph: 6596521951", &font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&default_font, FONT_SIZE_MED + 8.0);
    layer.set_text_cursor(Mm(100.0), Mm(45.0));
    layer.set_line_height(16.0);
    layer.set_outline_thickness(0.3);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("Flat Bush", &default_font);
    layer.add_line_break();
    layer.write_text("Auckland", &default_font);
    layer.end_text_section();

    // Add: Text
    layer.begin_text_section();
    layer.set_font(&font, FONT_SIZE_MED);
    layer.set_text_cursor(Mm(100.0), Mm(20.0));
    layer.set_line_height(12.0);
    layer.set_outline_thickness(0.3);
    layer.set_outline_color(black.clone());
    layer.set_fill_color(black.clone());
    layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
    layer.write_text("From: Nz Post IFS", &font);
    layer.add_line_break();
    layer.write_text("107 Ransom Smyth Drive", &font);
    layer.add_line_break();
    layer.write_text("Goodwood Heights", &font);
    layer.add_line_break();
    layer.write_text("Auckland", &font);
    layer.add_line_break();
    layer.write_text("2100", &font);
    layer.end_text_section();

    // Return PDF
    doc.save_to_bytes()
}