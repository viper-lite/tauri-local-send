use base64::Engine;

pub fn generate_qr_code(url: &str) -> Result<String, String> {
    let code = qrcode::QrCode::new(url).map_err(|e| format!("Failed to create QR code: {}", e))?;

    let image = code.render::<char>().quiet_zone(true).build();
    let lines: Vec<&str> = image.lines().collect();
    let height = lines.len();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">
            <rect width="100%" height="100%" fill="white"/>
            <text x="50%" y="50%" font-family="monospace" font-size="8" text-anchor="middle" fill="black">{}</text>
        </svg>"#,
        width,
        height,
        lines.join("")
    );

    Ok(base64::engine::general_purpose::STANDARD.encode(svg))
}

pub fn qr_to_data_url(url: &str) -> Result<String, String> {
    let base64 = generate_qr_code(url)?;
    Ok(format!("data:image/svg+xml;base64,{}", base64))
}
