use objc2_app_kit::{NSColor, NSColorSpace};

/// Reads the user's system accent colour. Apple documents that this colour
/// carries no guaranteed colour space, so it is converted to sRGB before any
/// component is read.
pub fn accent_color() -> Option<String> {
    let accent = NSColor::controlAccentColor();
    let space = NSColorSpace::sRGBColorSpace();
    let srgb = accent.colorUsingColorSpace(&space)?;

    let components = (
        srgb.redComponent(),
        srgb.greenComponent(),
        srgb.blueComponent(),
    );
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        channel(components.0),
        channel(components.1),
        channel(components.2)
    ))
}

fn channel(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}
