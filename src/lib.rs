pub fn run(config: &Config) -> Result<(), String> {
    let (width, height) = parse_aspect_ratio_string(&config.ratio_text)?;

    let output_fov = match config.output_fov_type {
        FovType::VERTICAL => vfov,
        FovType::HORIZONTAL => hfov,
    }(width, height, config.fov.to_radians());

    // TODO: check return value

    let output_fov = output_fov.to_degrees();

    println!("{}", output_fov);

    Ok(())
}

fn parse_aspect_ratio_string(ratio_str: &str) -> Result<(f64, f64), String> {
    // TODO: check infinity, NaN, etc.
    let ratio_str_parts : Vec<&str> = ratio_str.split(':').collect();
    if ratio_str_parts.len() != 2 {
        return Err("No separator in aspect ratio string".to_string());
    }

    let width: f64 = match ratio_str_parts[0].parse() {
        Ok(f) => f,
        Err(_) => return Err("Unable to parse width in aspect ratio string".to_string()),
    };
    let height: f64 = match ratio_str_parts[1].parse() {
        Ok(f) => f,
        Err(_) => return Err("Unable to parse height in aspect ratio string".to_string()),
    };
    
    Ok((width, height))
}

fn vfov(width: f64, height: f64, hfov: f64) -> f64 {
    // TODO: check for division by zero / arithmetic problems
    ((hfov / 2.0).tan() * height / width).atan() * 2.0
}

fn hfov(width: f64, height: f64, vfov: f64) -> f64 {
    // TODO: check for division by zero / arithmetic problems
    ((vfov / 2.0).tan() * width / height).atan() * 2.0
}

#[derive(Debug)]
pub enum FovType {
    VERTICAL,
    HORIZONTAL,
}

#[derive(Debug)]
pub struct Config {
    pub output_fov_type: FovType,
    pub ratio_text: String,
    pub fov: f64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            output_fov_type: FovType::VERTICAL,
            ratio_text: "4:3".to_string(),
            fov: 90.0,
        }
    }
}
