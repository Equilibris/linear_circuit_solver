pub fn scientific_float_renderer(value: f64, unit: &str) -> String {
    let l10 = value.log10();
    let l10 = l10 + 2.0 * ((l10 > 0.0) as i8 as f64 - 0.5);
    let magnitude = (l10 / 3.0) as i64;

    let transformed_number =
        (value * 10f64.powi(-magnitude as i32 * 3) * 100f64).round() as f64 / 100f64;

    let fallback = format!("e^{}", 3 * magnitude);

    let prefix = match magnitude {
        2 => "M",
        1 => "k",
        0 => "",
        -1 => "m",
        -2 => "µ",

        _ => fallback.as_str(),
    };

    format!("{:.02}{}{}", transformed_number, prefix, unit)
}

#[cfg(test)]
mod tests {
    use super::scientific_float_renderer;

    #[test]
    fn milis() {
        println!("{}", scientific_float_renderer(0.01, "A"));
        println!("{}", scientific_float_renderer(0.1, "A"))
    }
    #[test]
    fn kilos() {
        println!("{}", scientific_float_renderer(1000.01, "A"))
    }
    #[test]
    fn micros() {
        println!("{}", scientific_float_renderer(0.000001, "A"))
    }
}
