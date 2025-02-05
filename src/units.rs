/// Enum containing all possible unit types
#[derive(Debug)]
pub enum UnitEnum {
    Length(LengthUnit),
    Mass(MassUnit),
    Time(TimeUnit),
    Temperature(TemperatureUnit),
    Speed(SpeedUnit),
    Force(ForceUnit),
    Pressure(PressureUnit),
    Bearing(BearingUnit),
}

/// Trait for a Unit of Measurement
pub trait Unit: std::fmt::Debug + std::fmt::Display {
    /// Create a new instance of the Unit with the default value
    fn new() -> UnitEnum;
    
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
    fn default() -> UnitEnum;

    /// Convert a value from one unit to another.
    /// The value is first converted to the default unit,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64;

    /// Create a new instance of the Unit from a &str input.
    /// The function supports name_full, name_short, and abbreviation.
    fn from_str(&self, input: &str) -> UnitEnum;

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
    /// Create a new instance of the Unit with the default value
    fn new() -> UnitEnum {
        UnitEnum::Length(LengthUnit::Meters)
    }


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
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_meters = match from_unit {
            UnitEnum::Length(unit) => {
                match unit.abbr().as_str() {
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
                }
            },
            _ => panic!("Invalid unit type for LengthUnit conversion"),
        };
        match to_unit {
            UnitEnum::Length(unit) => {
                match unit.abbr().as_str() {
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
            },
            _ => panic!("Invalid unit type for LengthUnit conversion"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Meters
    fn default() -> UnitEnum {
        UnitEnum::Length(LengthUnit::Meters)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Meters" | "m" | "Meters (m)"
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Meters" | "m" | "Meters (m)" => UnitEnum::Length(LengthUnit::Meters),
            "Kilometers" | "km" | "Kilometers (km)" => UnitEnum::Length(LengthUnit::Kilometers),
            "Centimeters" | "cm" | "Centimeters (cm)" => UnitEnum::Length(LengthUnit::Centimeters),
            "Millimeters" | "mm" | "Millimeters (mm)" => UnitEnum::Length(LengthUnit::Millimeters),
            "Inches" | "in" | "Inches (in)" => UnitEnum::Length(LengthUnit::Inches),
            "Feet" | "ft" | "Feet (ft)" => UnitEnum::Length(LengthUnit::Feet),
            "Yards" | "yd" | "Yards (yd)" => UnitEnum::Length(LengthUnit::Yards),
            "Miles" | "mi" | "Miles (mi)" => UnitEnum::Length(LengthUnit::Miles),
            "Nautical Miles" | "Nmi" | "Nautical Miles (Nmi)" => UnitEnum::Length(LengthUnit::NauticalMiles),
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
    /// Create a new instance of the Unit with the default value
    fn new() -> UnitEnum {
        UnitEnum::Mass(MassUnit::Kilograms)
    }

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
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_kilograms = match from_unit {
            UnitEnum::Mass(unit) => {
                match unit.abbr().as_str() {
                    "kg" => value,
                    "g" => value * 0.001,
                    "mg" => value * 0.000001,
                    "lbm" => value * 0.453592,
                    "oz" => value * 0.0283495,
                    "slugs" => value * 14.5939,
                    _ => panic!("Invalid unit"),
                }
            },
            _ => panic!("Invalid unit type for MassUnit conversion"),
        };
        match to_unit {
            UnitEnum::Mass(unit) => {
                match unit.abbr().as_str() {
                    "kg" => value_kilograms,
                    "g" => value_kilograms / 0.001,
                    "mg" => value_kilograms / 0.000001,
                    "lbm" => value_kilograms / 0.453592,
                    "oz" => value_kilograms / 0.0283495,
                    "slugs" => value_kilograms / 14.5939,
                    _ => panic!("Invalid unit"),
                }
            },
            _ => panic!("Invalid unit type for MassUnit conversion"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Kilograms
    /// Example: "Kilograms" | "kg" | "Kilograms (kg)"
    fn default() -> UnitEnum {
        UnitEnum::Mass(MassUnit::Kilograms)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Kilograms" | "kg" | "Kilograms (kg)"
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Kilograms" | "kg" | "Kilograms (kg)" => UnitEnum::Mass(MassUnit::Kilograms),
            "Grams" | "g" | "Grams (g)" => UnitEnum::Mass(MassUnit::Grams),
            "Milligrams" | "mg" | "Milligrams (mg)" => UnitEnum::Mass(MassUnit::Milligrams),
            "Pounds Mass" | "lbm" | "Pounds Mass (lbm)" => UnitEnum::Mass(MassUnit::PoundsMass),
            "Ounces" | "oz" | "Ounces (oz)" => UnitEnum::Mass(MassUnit::Ounces),
            "Slugs" | "slugs" | "Slugs (slugs)" => UnitEnum::Mass(MassUnit::Slugs),
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
    /// Create a new instance of the Unit with the default value
    /// Default unit is Seconds
    fn new() -> UnitEnum {
        UnitEnum::Time(TimeUnit::Seconds)
    }
    
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
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_seconds = match from_unit {
            UnitEnum::Time(unit) => {
                match unit.abbr().as_str() {
                    "s" => value,
                    "ms" => value * 0.001,
                    "µs" => value * 0.000001,
                    "ns" => value * 0.000000001,
                    "min" => value * 60.0,
                    "hr" => value * 3600.0,
                    "days" => value * 86400.0,
                    "weeks" => value * 604800.0,
                    _ => panic!("Invalid unit"),
                }
            },
            _ => panic!("Invalid unit type for TimeUnit conversion"),
        };
        match to_unit {
            UnitEnum::Time(unit) => {
                match unit.abbr().as_str() {
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
            },
            _ => panic!("Invalid unit type for TimeUnit conversion"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Seconds
    fn default() -> UnitEnum {
        UnitEnum::Time(TimeUnit::Seconds)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Seconds" | "s" | "Seconds (s)"
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Seconds" | "s" | "Seconds (s)" => UnitEnum::Time(TimeUnit::Seconds),
            "Milliseconds" | "ms" | "Milliseconds (ms)" => UnitEnum::Time(TimeUnit::Milliseconds),
            "Microseconds" | "µs" | "Microseconds (µs)" => UnitEnum::Time(TimeUnit::Microseconds),
            "Nanoseconds" | "ns" | "Nanoseconds (ns)" => UnitEnum::Time(TimeUnit::Nanoseconds),
            "Minutes" | "min" | "Minutes (min)" => UnitEnum::Time(TimeUnit::Minutes),
            "Hours" | "hr" | "Hours (hr)" => UnitEnum::Time(TimeUnit::Hours),
            "Days" | "days" | "Days (days)" => UnitEnum::Time(TimeUnit::Days),
            "Weeks" | "weeks" | "Weeks (weeks)" => UnitEnum::Time(TimeUnit::Weeks),
            _ => panic!("Invalid unit"),
        }
    }
}

/// Implementing Display trait for TimeUnit
impl std::fmt::Display for TimeUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Seconds => write!(f, "Seconds"),
            Self::Milliseconds => write!(f, "Milliseconds"),
            Self::Microseconds => write!(f, "Microseconds"),
            Self::Nanoseconds => write!(f, "Nanoseconds"),
            Self::Minutes => write!(f, "Minutes"),
            Self::Hours => write!(f, "Hours"),
            Self::Days => write!(f, "Days"),
            Self::Weeks => write!(f, "Weeks"),
        }
    }
}

/// Implementing Debug trait for TimeUnit
impl std::fmt::Debug for TimeUnit {
    /// Implementing Debug trait for TimeUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
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
    /// Create a new instance of the Unit with the default value
    fn new() -> UnitEnum {
        UnitEnum::Temperature(TemperatureUnit::Kelvin)
    }

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
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_kelvin = match from_unit {
            UnitEnum::Temperature(unit) => {
                match unit.abbr().as_str() {
                    "°F" => (value + 459.67) * 5.0/9.0,
                    "°C" => value + 273.15,
                    "K" => value,
                    "°R" => value * 5.0/9.0,
                    _ => panic!("Invalid unit"),
                }
            },
            _ => panic!("Invalid unit type for Temperature conversion"),
        };
        match to_unit {
            UnitEnum::Temperature(unit) => {
                match unit.abbr().as_str() {
                    "°F" => value_kelvin * 9.0/5.0 - 459.67,
                    "°C" => value_kelvin - 273.15,
                    "K" => value_kelvin,
                    "°R" => value_kelvin * 9.0/5.0,
                    _ => panic!("Invalid unit"),
                }
            },
            _ => panic!("Invalid unit type for Temperature conversion"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Kelvin
    fn default() -> UnitEnum {
        UnitEnum::Temperature(TemperatureUnit::Kelvin)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Fehrenheit" | "°F" | "Degrees Fehrenheit (°F)"=> UnitEnum::Temperature(TemperatureUnit::Fehrenheit),
            "Celcius" | "°C" | "Degrees Celcius (°C)" => UnitEnum::Temperature(TemperatureUnit::Celcius),
            "Kelvin" | "K" | "Degrees Kelvin (K)" => UnitEnum::Temperature(TemperatureUnit::Kelvin),
            "Rankine" | "°R" | "Degrees Rankine (°R)" => UnitEnum::Temperature(TemperatureUnit::Rankine),
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
    /// Create a new instance of the Unit with the default value
    /// Default unit is Meters Per Second
    fn new() -> UnitEnum {
        UnitEnum::Speed(SpeedUnit::MetersPerSecond)
    }

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
    fn default() -> UnitEnum {
        UnitEnum::Speed(SpeedUnit::MetersPerSecond)
    }


    /// Convert a value from one unit to another.
    /// The value is first converted to Meters Per Second,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_mps = match from_unit {
            UnitEnum::Speed(unit) => {
                match unit.abbr().as_str() {
                    "m/s" => value,
                    "ft/s" => value * 0.3048,
                    "km/h" => value * 0.277778,
                    "mph" => value * 0.44704,
                    "Kts" => value * 0.514444,
                    _ => panic!("Invalid unit"),
                }
            },
            _ => panic!("Invalid unit type for SpeedUnit conversion"),
        };
        match to_unit {
            UnitEnum::Speed(unit) => {
                match unit.abbr().as_str() {
                    "m/s" => value_mps,
                    "ft/s" => value_mps * 3.28084,
                    "km/h" => value_mps * 3.6,
                    "mph" => value_mps * 2.23694,
                    "Kts" => value_mps * 1.94384,
                    _ => panic!("Invalid unit"),
                }
            },
            _ => panic!("Invalid unit type for SpeedUnit conversion"),
        }
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Knots" | "Kts" | "Knots (Kts)"
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Knots" | "Kts" | "Knots (Kts)" => UnitEnum::Speed(SpeedUnit::Knots),
            "Miles Per Hour" | "mph" | "Miles Per Hour (mph)" => UnitEnum::Speed(SpeedUnit::MilesPerHour),
            "Kilometers Per Hour" | "km/h" | "Kilometers Per Hour (km/h)" => UnitEnum::Speed(SpeedUnit::KilometersPerHour),
            "Feet Per Second" | "ft/s" | "Feet Per Second (ft/s)" => UnitEnum::Speed(SpeedUnit::FeetPerSecond),
            "Meters Per Second" | "m/s" | "Meters Per Second (m/s)" => UnitEnum::Speed(SpeedUnit::MetersPerSecond),
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

// ------------------------------------------------------------

/// Enum for ForceUnit
pub enum ForceUnit {
    Newtons,
    PoundsForce,
    KilogramsForce,
}

/// Implementing Unit trait for ForceUnit
impl Unit for ForceUnit {
    /// Create a new instance of the Unit with the default value
    fn new() -> UnitEnum {
        UnitEnum::Force(ForceUnit::Newtons)
    }

    /// Display the full name with abbreviation
    /// Example: "Newtons (N)"
    fn name_full(&self) -> String {
        String::from("{&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name without abbreviation.
    /// Example: "Newtons"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit.
    /// Example: "N"
    fn abbr(&self) -> String {
        match self {
            Self::Newtons => String::from("N"),
            Self::PoundsForce => String::from("lbf"),
            Self::KilogramsForce => String::from("kgf"),
        }
    }

    /// Convert a value from one unit to another.
    /// The value is first converted to Newtons,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_newtons = match from_unit {
            UnitEnum::Force(unit) => match unit.abbr().as_str() {
                "N" => value,
                "lbf" => value * 4.44822,
                "kgf" => value * 9.80665,
                _ => panic!("Invalid unit"),
            },
            _ => panic!("Invalid unit type for ForceUnit conversion"),
        };
        match to_unit {
            UnitEnum::Force(unit) => match unit.abbr().as_str() {
                "N" => value_newtons,
                "lbf" => value_newtons / 4.44822,
                "kgf" => value_newtons / 9.80665,
                _ => panic!("Invalid unit"),
            },
            _ => panic!("Invalid unit type for ForceUnit conversion"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Newtons
    fn default() -> UnitEnum {
        UnitEnum::Force(ForceUnit::Newtons)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Newtons" | "N" | "Newtons (N)"
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Newtons" | "N" | "Newtons (N)" => UnitEnum::Force(ForceUnit::Newtons),
            "Pounds Force" | "lbf" | "Pounds Force (lbf)" => UnitEnum::Force(ForceUnit::PoundsForce),
            "Kilograms Force" | "kgf" | "Kilograms Force (kgf)" => UnitEnum::Force(ForceUnit::KilogramsForce),
            _ => panic!("Invalid unit"),
        }
    }
}

/// Implementing Display trait for ForceUnit
impl std::fmt::Display for ForceUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Newtons => write!(f, "Newtons"),
            Self::PoundsForce => write!(f, "Pounds Force"),
            Self::KilogramsForce => write!(f, "Kilograms Force"),
        }
    }
}

/// Implementing Debug trait for ForceUnit
impl std::fmt::Debug for ForceUnit {
    /// Implementing Debug trait for ForceUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

// ------------------------------------------------------------

/// Enum for PressureUnit
pub enum PressureUnit {
    Pascals,
    Kilopascals,
    Megapascals,
    Hectopascals,
    InchesOfMercury,
    MillimetersOfMercury,
    Milibars,
    Atmospheres,
    PoundsPerSquareInch,
}

/// Implementing Unit trait for PressureUnit
impl Unit for PressureUnit {
    /// Create a new instance of the Unit with the default value
    fn new() -> UnitEnum {
        UnitEnum::Pressure(PressureUnit::Pascals)
    }

    /// Display the full name with abbreviation
    /// Example: "Pascals (Pa)"
    fn name_full(&self) -> String {
        String::from("{&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name without abbreviation.
    /// Example: "Pascals"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit.
    /// Example: "Pa"
    fn abbr(&self) -> String {
        match self {
            Self::Pascals => String::from("Pa"),
            Self::Kilopascals => String::from("kPa"),
            Self::Megapascals => String::from("MPa"),
            Self::Hectopascals => String::from("hPa"),
            Self::InchesOfMercury => String::from("inHg"),
            Self::MillimetersOfMercury => String::from("mmHg"),
            Self::Milibars => String::from("mbar"),
            Self::Atmospheres => String::from("atm"),
            Self::PoundsPerSquareInch => String::from("psi"),
        }
    }

    /// Convert a value from one unit to another.
    /// The value is first converted to Pascals,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_pascals = match from_unit {
            UnitEnum::Pressure(unit) => match unit.abbr().as_str() {
                "Pa" => value,
                "kPa" => value * 1000.0,
                "MPa" => value * 1000000.0,
                "hPa" => value * 100.0,
                "inHg" => value * 3386.39,
                "mmHg" => value * 133.322,
                "mbar" => value,
                "atm" => value * 101325.0,
                "psi" => value * 6894.76,
                _ => panic!("Invalid unit"),
            },
            _ => panic!("Invalid unit type for Pressure conversion"),
        };
        match to_unit {
            UnitEnum::Pressure(unit) => match unit.abbr().as_str() {
                "Pa" => value_pascals,
                "kPa" => value_pascals / 1000.0,
                "MPa" => value_pascals / 1000000.0,
                "hPa" => value_pascals / 100.0,
                "inHg" => value_pascals / 3386.39,
                "mmHg" => value_pascals / 133.322,
                "mbar" => value_pascals,
                "atm" => value_pascals / 101325.0,
                "psi" => value_pascals / 6894.76,
                _ => panic!("Invalid unit"),
            },
            _ => panic!("Invalid unit type for Pressure conversion"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Pascals
    fn default() -> UnitEnum {
        UnitEnum::Pressure(PressureUnit::Pascals)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Pascals" | "Pa" | "Pascals (Pa)"
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Pascals" | "Pa" | "Pascals (Pa)" => UnitEnum::Pressure(PressureUnit::Pascals),
            "Kilopascals" | "kPa" | "Kilopascals (kPa)" => UnitEnum::Pressure(PressureUnit::Kilopascals),
            "Megapascals" | "MPa" | "Megapascals (MPa)" => UnitEnum::Pressure(PressureUnit::Megapascals),
            "Hectopascals" | "hPa" | "Hectopascals (hPa)" => UnitEnum::Pressure(PressureUnit::Hectopascals),
            "Inches Of Mercury" | "inHg" | "Inches Of Mercury (inHg)" => UnitEnum::Pressure(PressureUnit::InchesOfMercury),
            "Millimeters Of Mercury" | "mmHg" | "Millimeters Of Mercury (mmHg)" => UnitEnum::Pressure(PressureUnit::MillimetersOfMercury),
            "Milibars" | "mbar" | "Milibars (mbar)" => UnitEnum::Pressure(PressureUnit::Milibars),
            "Atmospheres" | "atm" | "Atmospheres (atm)" => UnitEnum::Pressure(PressureUnit::Atmospheres),
            "Pounds Per Square Inch" | "psi" | "Pounds Per Square Inch (psi)" => UnitEnum::Pressure(PressureUnit::PoundsPerSquareInch),
            _ => panic!("Invalid unit"),
        }
    }

}

/// Implementing Display trait for PressureUnit
impl std::fmt::Display for PressureUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pascals => write!(f, "Pascals"),
            Self::Kilopascals => write!(f, "Kilopascals"),
            Self::Megapascals => write!(f, "Megapascals"),
            Self::Hectopascals => write!(f, "Hectopascals"),
            Self::InchesOfMercury => write!(f, "Inches Of Mercury"),
            Self::MillimetersOfMercury => write!(f, "Millimeters Of Mercury"),
            Self::Milibars => write!(f, "Milibars"),
            Self::Atmospheres => write!(f, "Atmospheres"),
            Self::PoundsPerSquareInch => write!(f, "Pounds Per Square Inch"),
        }
    }
}

/// Implementing Debug trait for PressureUnit
/// Display the unit name
impl std::fmt::Debug for PressureUnit {
    /// Implementing Debug trait for PressureUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

// ------------------------------------------------------------

/// Enum for BearingUnit
pub enum BearingUnit {
    Degrees,
    Radians,
    Gradians,
    Mils,
}

/// Implementing Unit trait for BearingUnit
impl Unit for BearingUnit {
    /// Create a new instance of the Unit with the default value
    fn new() -> UnitEnum {
        UnitEnum::Bearing(BearingUnit::Degrees)
    }
    
    /// Display the full name with abbreviation
    /// Example: "Degrees (°)"
    fn name_full(&self) -> String {
        String::from("{&self.fmt(f)} ({&self.abbr()})")
    }

    /// Display the short name without abbreviation.
    /// Example: "Degrees"
    fn name_short(&self) -> String {
        String::from("{&self.fmt(f)}")
    }

    /// Get the abbreviation of the unit.
    /// Example: "°"
    fn abbr(&self) -> String {
        match self {
            Self::Degrees => String::from("°"),
            Self::Radians => String::from("rad"),
            Self::Gradians => String::from("grad"),
            Self::Mils => String::from("mil"),
        }
    }

    /// Convert a value from one unit to another.
    /// The value is first converted to Degrees,
    /// then converted to the target unit.
    fn convert(&self, value: f64, from_unit: &UnitEnum, to_unit: &UnitEnum) -> f64 {
        let value_degrees = match from_unit {
            UnitEnum::Bearing(unit) => match unit.abbr().as_str() {
                "°" => value,
                "rad" => value * 57.2958,
                "grad" => value * 0.9,
                "mil" => value * 0.05625,
                _ => panic!("Invalid unit"),
            },
            _ => panic!("Invalid unit type for BearingUnit conversion"),
        };
        match to_unit {
            UnitEnum::Bearing(unit) => match unit.abbr().as_str() {
                "°" => value_degrees,
                "rad" => value_degrees / 57.2958,
                "grad" => value_degrees / 0.9,
                "mil" => value_degrees / 0.05625,
                _ => panic!("Invalid unit"),
            },
            _ => panic!("Invalid unit type for BearingUnit conversion"),
        }
    }

    /// Get the default unit for the given unit type
    /// Default unit is Degrees
    fn default() -> UnitEnum {
        UnitEnum::Bearing(BearingUnit::Degrees)
    }

    /// Create a new instance of the Unit from a &str input.
    /// The function supports full name, short name, and abbreviation.
    /// Example: "Degrees" | "°" | "Degrees (°)"
    fn from_str(&self, input: &str) -> UnitEnum {
        match input {
            "Degrees" | "°" | "Degrees (°)" => UnitEnum::Bearing(BearingUnit::Degrees),
            "Radians" | "rad" | "Radians (rad)" => UnitEnum::Bearing(BearingUnit::Radians),
            "Gradians" | "grad" | "Gradians (grad)" => UnitEnum::Bearing(BearingUnit::Gradians),
            "Mils" | "mil" | "Mils (mil)" => UnitEnum::Bearing(BearingUnit::Mils),
            _ => panic!("Invalid unit"),
        }
    }
}

/// Implementing Display trait for BearingUnit
/// Display the unit name
/// Example: "Degrees"
impl std::fmt::Display for BearingUnit {
    /// Display the unit name
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Degrees => write!(f, "Degrees"),
            Self::Radians => write!(f, "Radians"),
            Self::Gradians => write!(f, "Gradians"),
            Self::Mils => write!(f, "Mils"),
        }
    }
}

/// Implementing Debug trait for BearingUnit
impl std::fmt::Debug for BearingUnit {
    /// Implementing Debug trait for BearingUnit
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

// ------------------------------------------------------------