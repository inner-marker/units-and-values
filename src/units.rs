/// Trait for a Unit of Measurement
pub trait Unit: std::fmt::Debug + std::fmt::Display {
    /// Display the full name with abbreviation
    /// Example: "Degrees Fehrenheit (°F)"
    fn name_full(&self) -> String {
        String::from("{&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name with abbreviation
    /// Example: "Fehrenheit"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit
    /// Example: "°F"
    fn abbr(&self) -> String;

    /// Get the default unit for the given unit type
    fn default(&self) -> Box<dyn Unit>;

    /// Convert a value from one unit to another.
    /// The value is first converted to the default unit,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: Box<dyn Unit>, to_unit: Box<dyn Unit>) -> f64;

    /// Create a new instance of the Unit from a &str input.
    /// The function supports name_full, name_short, and abbreviation.
    fn from_str(&self, input: &str) -> Box<dyn Unit>;

}

// ------------------------------------------------------------

/// Enum for LengthUnit
pub enum LengthUnit {
    Meters,
    Kilometers,
    Centimeters,
    Millimeters,
    Inches,
    Feet,
    Yards,
    Miles,
    NauticalMiles,
}

/// Implementing Unit trait for LengthUnit
impl Unit for LengthUnit {
    /// Display the full name with abbreviation
    /// Example: "Meters (m)"
    fn name_full(&self) -> String {
        String::from("{&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name without abbreviation.
    /// Example: "Meters"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit.
    /// Example: "m"
    fn abbr(&self) -> String {
        match self {
            Self::Meters => String::from("m"),
            Self::Kilometers => String::from("km"),
            Self::Centimeters => String::from("cm"),
            Self::Millimeters => String::from("mm"),
            Self::Inches => String::from("in"),
            Self::Feet => String::from("ft"),
            Self::Yards => String::from("yd"),
            Self::Miles => String::from("mi"),
            Self::NauticalMiles => String::from("Nmi"),
        }
    }

    /// Convert a value from one unit to another.
    /// The value is first converted to Meters,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: Box<dyn Unit>, to_unit: Box<dyn Unit>) -> f64 {
        let value_meters = match from_unit.abbr().as_str() {
            "m" => value,
            "km" => value * 1000.0,
            "cm" => value * 0.01,
            "mm" => value * 0.001,
            "in" => value * 0.0254,
            "ft" => value * 0.3048,
            "yd" => value * 0.9144,
            "mi" => value * 1609.34,
            "Nmi" => value * 1852.0,
            _ => panic!("Invalid unit"),
        };
        match to_unit.abbr().as_str() {
            "m" => value_meters,
            "km" => value_meters / 1000.0,
            "cm" => value_meters / 0.01,
            "mm" => value_meters / 0.001,
            "in" => value_meters / 0.0254,
            "ft" => value_meters / 0.3048,
            "yd" => value_meters / 0.9144,
            "mi" => value_meters / 1609.34,
            "Nmi" => value_meters / 1852.0,
            _ => panic!("Invalid unit"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Meters
    fn default(&self) -> Box<dyn Unit> {
        Box::new(LengthUnit::Meters)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Meters" | "m" | "Meters (m)"
    fn from_str(&self, input: &str) -> Box<dyn Unit> {
        match input {
            "Meters" | "m" | "Meters (m)" => Box::new(LengthUnit::Meters),
            "Kilometers" | "km" | "Kilometers (km)" => Box::new(LengthUnit::Kilometers),
            "Centimeters" | "cm" | "Centimeters (cm)" => Box::new(LengthUnit::Centimeters),
            "Millimeters" | "mm" | "Millimeters (mm)" => Box::new(LengthUnit::Millimeters),
            "Inches" | "in" | "Inches (in)" => Box::new(LengthUnit::Inches),
            "Feet" | "ft" | "Feet (ft)" => Box::new(LengthUnit::Feet),
            "Yards" | "yd" | "Yards (yd)" => Box::new(LengthUnit::Yards),
            "Miles" | "mi" | "Miles (mi)" => Box::new(LengthUnit::Miles),
            "Nautical Miles" | "Nmi" | "Nautical Miles (Nmi)" => Box::new(LengthUnit::NauticalMiles),
            _ => panic!("Invalid unit"),
        }
    }
}

/// Implementing Display trait for LengthUnit
impl std::fmt::Display for LengthUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Meters => write!(f, "Meters"),
            Self::Kilometers => write!(f, "Kilometers"),
            Self::Centimeters => write!(f, "Centimeters"),
            Self::Millimeters => write!(f, "Millimeters"),
            Self::Inches => write!(f, "Inches"),
            Self::Feet => write!(f, "Feet"),
            Self::Yards => write!(f, "Yards"),
            Self::Miles => write!(f, "Miles"),
            Self::NauticalMiles => write!(f, "Nautical Miles"),
        }
    }
}

/// Implementing Debug trait for LengthUnit
impl std::fmt::Debug for LengthUnit {
    /// Implementing Debug trait for LengthUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

// ------------------------------------------------------------

/// Enum for MassUnit
pub enum MassUnit {
    Kilograms,
    Grams,
    Milligrams,
    PoundsMass,
    Ounces,
    Slugs,
}

/// Implementing Unit trait for MassUnit
impl Unit for MassUnit {
    /// Display the full name with abbreviation
    /// Example: "Kilograms (kg)"
    fn name_full(&self) -> String {
        String::from("{&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name without abbreviation.
    /// Example: "Kilograms"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit.
    /// Example: "kg"
    fn abbr(&self) -> String {
        match self {
            Self::Kilograms => String::from("kg"),
            Self::Grams => String::from("g"),
            Self::Milligrams => String::from("mg"),
            Self::PoundsMass => String::from("lbm"),
            Self::Ounces => String::from("oz"),
            Self::Slugs => String::from("slugs"),
        }
    }

    /// Convert a value from one unit to another.
    /// The value is first converted to Kilograms,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: Box<dyn Unit>, to_unit: Box<dyn Unit>) -> f64 {
        let value_kilograms = match from_unit.abbr().as_str() {
            "kg" => value,
            "g" => value * 0.001,
            "mg" => value * 0.000001,
            "lbm" => value * 0.453592,
            "oz" => value * 0.0283495,
            "slugs" => value * 14.5939,
            _ => panic!("Invalid unit"),
        };
        match to_unit.abbr().as_str() {
            "kg" => value_kilograms,
            "g" => value_kilograms / 0.001,
            "mg" => value_kilograms / 0.000001,
            "lbm" => value_kilograms / 0.453592,
            "oz" => value_kilograms / 0.0283495,
            "slugs" => value_kilograms / 14.5939,
            _ => panic!("Invalid unit"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Kilograms
    /// Example: "Kilograms" | "kg" | "Kilograms (kg)"
    fn default(&self) -> Box<dyn Unit> {
        Box::new(MassUnit::Kilograms)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Kilograms" | "kg" | "Kilograms (kg)"
    fn from_str(&self, input: &str) -> Box<dyn Unit> {
        match input {
            "Kilograms" | "kg" | "Kilograms (kg)" => Box::new(MassUnit::Kilograms),
            "Grams" | "g" | "Grams (g)" => Box::new(MassUnit::Grams),
            "Milligrams" | "mg" | "Milligrams (mg)" => Box::new(MassUnit::Milligrams),
            "Pounds Mass" | "lbm" | "Pounds Mass (lbm)" => Box::new(MassUnit::PoundsMass),
            "Ounces" | "oz" | "Ounces (oz)" => Box::new(MassUnit::Ounces),
            "Slugs" | "slugs" | "Slugs (slugs)" => Box::new(MassUnit::Slugs),
            _ => panic!("Invalid unit"),
        }
    }
}

/// Implementing Display trait for MassUnit
/// Display the unit name
/// Example: "Kilograms"
impl std::fmt::Display for MassUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Kilograms => write!(f, "Kilograms"),
            Self::Grams => write!(f, "Grams"),
            Self::Milligrams => write!(f, "Milligrams"),
            Self::PoundsMass => write!(f, "Pounds Mass"),
            Self::Ounces => write!(f, "Ounces"),
            Self::Slugs => write!(f, "Slugs"),
        }
    }
}

/// Implementing Debug trait for MassUnit
impl std::fmt::Debug for MassUnit {
    /// Implementing Debug trait for MassUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

// ------------------------------------------------------------

/// Enum for TimeUnit
pub enum TimeUnit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}

/// Implementing Unit trait for TimeUnit
impl Unit for TimeUnit {
    /// Display the full name with abbreviation
    /// Example: "Seconds (s)"
    fn name_full(&self) -> String {
        String::from("{&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name without abbreviation.
    /// Example: "Seconds"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit.
    /// Example: "s"
    fn abbr(&self) -> String {
        match self {
            Self::Seconds => String::from("s"),
            Self::Milliseconds => String::from("ms"),
            Self::Microseconds => String::from("µs"),
            Self::Nanoseconds => String::from("ns"),
            Self::Minutes => String::from("min"),
            Self::Hours => String::from("hr"),
            Self::Days => String::from("days"),
            Self::Weeks => String::from("weeks"),
        }
    }

    /// Convert a value from one unit to another.
    /// The value is first converted to Seconds,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: Box<dyn Unit>, to_unit: Box<dyn Unit>) -> f64 {
        let value_seconds = match from_unit.abbr().as_str() {
            "s" => value,
            "ms" => value * 0.001,
            "µs" => value * 0.000001,
            "ns" => value * 0.000000001,
            "min" => value * 60.0,
            "hr" => value * 3600.0,
            "days" => value * 86400.0,
            "weeks" => value * 604800.0,
            _ => panic!("Invalid unit"),
        };
        match to_unit.abbr().as_str() {
            "s" => value_seconds,
            "ms" => value_seconds / 0.001,
            "µs" => value_seconds / 0.000001,
            "ns" => value_seconds / 0.000000001,
            "min" => value_seconds / 60.0,
            "hr" => value_seconds / 3600.0,
            "days" => value_seconds / 86400.0,
            "weeks" => value_seconds / 604800.0,
            _ => panic!("Invalid unit"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Seconds
    fn default(&self) -> Box<dyn Unit> {
        Box::new(TimeUnit::Seconds)
    }

// ------------------------------------------------------------


/// Enum for TemperatureUnit
pub enum TemperatureUnit {
    Kelvin,
    Celcius,
    Rankine,
    Fehrenheit,
}

/// Implementing Unit trait for TemperatureUnit
impl Unit for TemperatureUnit {
    /// Display the full name with abbreviation
    /// Example: "Degrees Fehrenheit (°F)"
    fn name_full(&self) -> String {
        String::from("Degrees {&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name without abbreviation.
    /// Example: "Fehrenheit"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit.
    /// Example: "°F"
    fn abbr(&self) -> String {
        match self {
            Self::Fehrenheit => String::from("°F"),
            Self::Celcius => String::from("°C"),
            Self::Kelvin => String::from("K"),
            Self::Rankine => String::from("°R"),
        }
    }

    /// Convert a value from one unit to another.
    /// The value is first converted to Kelvin,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: Box<dyn Unit>, to_unit: Box<dyn Unit>) -> f64 {
        let value_kelvin = match from_unit.abbr().as_str() {
            "°F" => (value + 459.67) * 5.0/9.0,
            "°C" => value + 273.15,
            "K" => value,
            "°R" => value * 5.0/9.0,
            _ => panic!("Invalid unit"),
        };
        match to_unit.abbr().as_str() {
            "°F" => value_kelvin * 9.0/5.0 - 459.67,
            "°C" => value_kelvin - 273.15,
            "K" => value_kelvin,
            "°R" => value_kelvin * 9.0/5.0,
            _ => panic!("Invalid unit"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Kelvin
    fn default(&self) -> Box<dyn Unit> {
        Box::new(TemperatureUnit::Kelvin)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    fn from_str(&self, input: &str) -> Box<dyn Unit> {
        match input {
            "Fehrenheit" | "°F" | "Degrees Fehrenheit (°F)"=> Box::new(TemperatureUnit::Fehrenheit),
            "Celcius" | "°C" | "Degrees Celcius (°C)" => Box::new(TemperatureUnit::Celcius),
            "Kelvin" | "K" | "Degrees Kelvin (K)" => Box::new(TemperatureUnit::Kelvin),
            "Rankine" | "°R" | "Degrees Rankine (°R)" => Box::new(TemperatureUnit::Rankine),
            _ => panic!("Invalid unit"),
        }
    }
}

/// Implementing Display trait for TemperatureUnit
/// Display the unit name
impl std::fmt::Display for TemperatureUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Fehrenheit => write!(f, "Fehrenheit"),
            Self::Celcius => write!(f, "Celcius"),
            Self::Kelvin => write!(f, "Kelvin"),
            Self::Rankine => write!(f, "Rankine"),
        }
    }
}

/// Implementing Debug trait for TemperatureUnit
impl std::fmt::Debug for TemperatureUnit {
    /// Implementing Debug trait for TemperatureUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

// ------------------------------------------------------------

/// Enum for SpeedUnit
#[allow(dead_code)]
pub enum SpeedUnit {
    Knots,
    MilesPerHour,
    KilometersPerHour,
    FeetPerSecond,
    MetersPerSecond,
}

impl Unit for SpeedUnit {
    /// Get the abbreviation of the unit
    fn abbr(&self) -> String {
        match self {
            Self::Knots => String::from("Kts"),
            Self::MilesPerHour => String::from("mph"),
            Self::KilometersPerHour => String::from("km/h"),
            Self::FeetPerSecond => String::from("ft/s"),
            Self::MetersPerSecond => String::from("m/s"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Meters Per Second
    fn default(&self) -> Box<dyn Unit> {
        Box::new(SpeedUnit::MetersPerSecond)
    }


    /// Convert a value from one unit to another.
    /// The value is first converted to Meters Per Second,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: Box<dyn Unit>, to_unit: Box<dyn Unit>) -> f64 {
        let value_mps = match from_unit.abbr().as_str() {
            "m/s" => value,
            "ft/s" => value * 0.3048,
            "km/h" => value * 0.277778,
            "mph" => value * 0.44704,
            "Kts" => value * 0.514444,
            _ => panic!("Invalid unit"),
        };
        match to_unit.abbr().as_str() {
            "m/s" => value_mps,
            "ft/s" => value_mps * 3.28084,
            "km/h" => value_mps * 3.6,
            "mph" => value_mps * 2.23694,
            "Kts" => value_mps * 1.94384,
            _ => panic!("Invalid unit"),
        }
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Knots" | "Kts" | "Knots (Kts)"
    fn from_str(&self, input: &str) -> Box<dyn Unit> {
        match input {
            "Knots" | "Kts" | "Knots (Kts)" => Box::new(SpeedUnit::Knots),
            "Miles Per Hour" | "mph" | "Miles Per Hour (mph)" => Box::new(SpeedUnit::MilesPerHour),
            "Kilometers Per Hour" | "km/h" | "Kilometers Per Hour (km/h)" => Box::new(SpeedUnit::KilometersPerHour),
            "Feet Per Second" | "ft/s" | "Feet Per Second (ft/s)" => Box::new(SpeedUnit::FeetPerSecond),
            "Meters Per Second" | "m/s" | "Meters Per Second (m/s)" => Box::new(SpeedUnit::MetersPerSecond),
            _ => panic!("Invalid unit"),
        }
    }
}

impl std::fmt::Display for SpeedUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Knots => write!(f, "Knots"),
            Self::MilesPerHour => write!(f, "Miles Per Hour"),
            Self::KilometersPerHour => write!(f, "Kilometers Per Hour"),
            Self::FeetPerSecond => write!(f, "Feet Per Second"),
            Self::MetersPerSecond => write!(f, "Meters Per Second"),
        }
    }
}

/// Implementing Debug trait for SpeedUnit
impl std::fmt::Debug for SpeedUnit {
    /// Implementing Debug trait for SpeedUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
