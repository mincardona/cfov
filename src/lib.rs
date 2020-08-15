use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

pub fn run(config: &Config) -> Result<(), String> {
    let output_fov = match config.output_fov_type {
        FovType::VERTICAL => vfov,
        FovType::HORIZONTAL => hfov,
    }(config.aspect_ratio, config.fov);

    println!("{}", output_fov);

    Ok(())
}

fn vfov(aspect_ratio: AspectRatio, hfov: Fov) -> Fov {
    let f = ((hfov.value() / 2.0).to_radians().tan() * aspect_ratio.value().recip()).atan().to_degrees() * 2.0;
    f.try_into().unwrap()
}

fn hfov(aspect_ratio: AspectRatio, vfov: Fov) -> Fov {
    let f = ((vfov.value() / 2.0).to_radians().tan() * aspect_ratio.value()).atan().to_degrees() * 2.0;
    f.try_into().unwrap()
}

// Fov

#[derive(Debug, Copy, Clone)]
pub struct Fov {
    value: f64
}

impl Fov {
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl TryFrom<f64> for Fov {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || value <= 0.0 || value > 180.0 {
            Err("Floating-point value out of Fov range")
        } else {
            Ok(Fov { value })
        }
    }
}

impl TryFrom<&str> for Fov {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<f64>() {
            Ok(f) => f.try_into(),
            Err(_) => Err("Couldn't parse string as floating-point Fov"),
        }
    }
}

impl From<Fov> for f64 {
    fn from(value: Fov) -> f64 {
        value.value
    }
}

impl fmt::Display for Fov {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// AspectRatio

#[derive(Debug, Copy, Clone)]
pub struct AspectRatio {
    // width/height
    value: f64
}

impl AspectRatio {
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl TryFrom<f64> for AspectRatio {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() || value <= 0.0 {
            Err("Floating-point value out of AspectRatio range")
        } else {
            Ok(AspectRatio { value })
        }
    }
}

impl TryFrom<&str> for AspectRatio {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ratio_str_parts : Vec<&str> = value.split(':').collect();

        match ratio_str_parts.len() {
            // single value, like "1.33"
            1 => {
                match ratio_str_parts[0].parse::<f64>() {
                    Ok(f) => f.try_into(),
                    Err(_) => Err("Unable to parse AspectRatio string as floating-point number")
                }
            },
            // pair of values, like "4:3"
            2 => {
                let width: f64 = match ratio_str_parts[0].parse() {
                    Ok(f) => f,
                    Err(_) => return Err("Unable to parse width in AspectRatio string")
                };
                let height: f64 = match ratio_str_parts[1].parse() {
                    Ok(f) => f,
                    Err(_) => return Err("Unable to parse height in AspectRatio string")
                };
                if height == 0.0 {
                    Err("AspectRatio cannot have zero height")
                } else {
                    (width / height).try_into()
                }
            },
            _ => {
                Err("Unable to parse AspectRatio string")
            }
        }
    }
}

impl From<AspectRatio> for f64 {
    fn from(value: AspectRatio) -> f64 {
        value.value
    }
}

// Config

#[derive(Debug, Clone)]
pub enum FovType {
    VERTICAL,
    HORIZONTAL,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub output_fov_type: FovType,
    pub aspect_ratio: AspectRatio,
    pub fov: Fov,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            output_fov_type: FovType::VERTICAL,
            aspect_ratio: (4.0f64 / 3.0f64).try_into().unwrap(),
            fov: 90.0f64.try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hfov_typical() {
        let ratio = AspectRatio::try_from("4:3").unwrap();
        let fov = Fov::try_from(55.4).unwrap();
        assert_about_eq(70.0, hfov(ratio, fov).into());
    }

    #[test]
    fn test_vfov_typical() {
        let ratio = AspectRatio::try_from(4.0 / 3.0).unwrap();
        let fov = Fov::try_from(69.5).unwrap();
        assert_about_eq(55.0, vfov(ratio, fov).into());
    }

    fn assert_about_eq(expected: f64, actual: f64) {
        assert!((expected - actual).abs() < 0.5, "assert_about_eq failed: expected {}, actual {}", expected, actual);
    }
}
