use base64::Engine;
use qrcode::QrCode;

pub fn generate_qr_code(url: &str) -> Result<String, String> {
    let code = QrCode::new(url).map_err(|e| format!("Failed to create QR code: {}", e))?;

    // 获取二维码矩阵
    let modules = code.to_colors();
    let size = code.width();

    // 模块大小（像素）
    let module_size = 4u32;
    // 边距（模块数）
    let quiet_zone = 2u32;
    // 总像素尺寸
    let pixel_size = (size as u32 + 2 * quiet_zone) * module_size;

    // 创建像素数据（RGBA格式）
    let mut pixels: Vec<u8> = Vec::with_capacity((pixel_size * pixel_size * 4) as usize);

    for y in 0..pixel_size {
        for x in 0..pixel_size {
            // 计算当前像素对应的模块位置
            let module_x = (x / module_size) as i32 - quiet_zone as i32;
            let module_y = (y / module_size) as i32 - quiet_zone as i32;

            // 判断是否在二维码区域内
            let is_dark = if module_x >= 0
                && module_x < size as i32
                && module_y >= 0
                && module_y < size as i32
            {
                // 获取该模块的颜色
                let idx = module_y as usize * size + module_x as usize;
                matches!(modules[idx], qrcode::Color::Dark)
            } else {
                false // 边距区域为白色
            };

            // RGBA像素数据（白色或黑色）
            if is_dark {
                pixels.extend_from_slice(&[0, 0, 0, 255]); // 黑色
            } else {
                pixels.extend_from_slice(&[255, 255, 255, 255]); // 白色
            }
        }
    }

    // 将像素数据编码为PNG
    let mut png_data: Vec<u8> = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_data, pixel_size, pixel_size);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| format!("PNG encode error: {}", e))?;
        writer
            .write_image_data(&pixels)
            .map_err(|e| format!("PNG write error: {}", e))?;
    }

    Ok(base64::engine::general_purpose::STANDARD.encode(&png_data))
}

pub fn qr_to_data_url(url: &str) -> Result<String, String> {
    let base64 = generate_qr_code(url)?;
    Ok(format!("data:image/png;base64,{}", base64))
}
