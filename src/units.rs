#![allow(unused)]

use std::fmt::{Debug, Display};

/// Define a trait for units of measure.
/// The `UnitOfMeasure` trait defines a common interface for managing
/// different types of measurement units. It provides several methods:
///
/// - `name_and_abbr`: Returns a formatted string with both the unit's name and its abbreviation,
///   for example, "Meters (m)".
/// - `abbr`: Retrieves the abbreviation of the unit (e.g., "m").
/// - `name`: Retrieves the full name of the unit (e.g., "Meters").
/// - `convert`: Converts a given value from the current unit to another unit by first converting
///   to a common base unit (e.g., meters or kilograms) and then to the target unit.
/// - `all_names`: Returns a vector containing all unit names.
/// - `all_abbrs`: Returns a vector containing all unit abbreviations.
/// - `all_names_and_abbrs`: Returns a vector with formatted strings of each unit's name and abbreviation.
///
/// This design eliminates the need for deeply nested conditional logic in handling unit conversions,
/// ensuring a simple and maintainable approach to extending measurement units.
pub trait UnitOfMeasure: Debug
    + Copy
    + Clone
{
    /// Get the name of the unit of measure with its abbreviation.
    /// For example, "Meters (m)".
    fn name_and_abbr (&self) -> String {
        String::from(format!("{} ({})", self.name(), self.abbr()))
    }

    /// Get the abbreviation of the unit of measure.
    /// For example, "m" for meters.
    /// 
    /// This function is the only place where hard-coding of the unit abbreviation is allowed.
    fn abbr (&self) -> String;
    
    /// Get the name of the unit of measure.
    /// For example, "Meters".
    /// 
    /// This function is the only place where hard-coding of the unit name is allowed.
    fn name (&self) -> String;

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements, 
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    fn convert (&self, value: f64, to_unit: &Self) -> f64;

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String>;

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    fn all_abbrs () -> Vec<String>;

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    fn all_names_and_abbrs () -> Vec<String>;

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// Example: "m" | "Meters" | "Meters (m)" -> Some(LengthUnit::Meters)
    fn from_str (unit_str: &str) -> Option<Self>;

    /// Return the default unit of measure.
    fn default () -> Self;

}

/// Define the units of measure for length.
/// The base unit of measure is meters.
#[derive(Debug, Copy, Clone)]
pub enum LengthUnit {
    Millimeters,
    Centimeters,
    Meters,
    Kilometers,
    Inches,
    Feet,
    Yards,
    StatuteMiles,
    NauticalMiles,
}

impl UnitOfMeasure for LengthUnit {
    /// Get the abbreviation of the unit of measure.
    /// For example, "m" for meters.
    fn abbr (&self) -> String {
        match self {
            LengthUnit::Millimeters => "mm".to_string(),
            LengthUnit::Centimeters => "cm".to_string(),
            LengthUnit::Meters => "m".to_string(),
            LengthUnit::Kilometers => "km".to_string(),
            LengthUnit::Inches => "in".to_string(),
            LengthUnit::Feet => "ft".to_string(),
            LengthUnit::Yards => "yd".to_string(),
            LengthUnit::StatuteMiles => "mi".to_string(),
            LengthUnit::NauticalMiles => "nmi".to_string(),
        }
    }

    /// Get the name of the unit of measure.
    /// For example, "Meters".
    fn name (&self) -> String {
        match self {
            LengthUnit::Millimeters => "Millimeters".to_string(),
            LengthUnit::Centimeters => "Centimeters".to_string(),
            LengthUnit::Meters => "Meters".to_string(),
            LengthUnit::Kilometers => "Kilometers".to_string(),
            LengthUnit::Inches => "Inches".to_string(),
            LengthUnit::Feet => "Feet".to_string(),
            LengthUnit::Yards => "Yards".to_string(),
            LengthUnit::StatuteMiles => "Statute Miles".to_string(),
            LengthUnit::NauticalMiles => "Nautical Miles".to_string(),
        }
    }

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements,
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    /// 
    /// # Examples
    /// ```rust
    /// let length_value = 5280.0;
    /// let length_output = length_unit_test.convert(length_value, &LengthUnit::Miles);
    /// assert_eq!(length_output, 1.0);
    /// ```
    fn convert (&self, value: f64, to_unit: &Self) -> f64 {
        let length_meters = match self {
            LengthUnit::Millimeters => value / 1000.0,
            LengthUnit::Centimeters => value / 100.0,
            LengthUnit::Meters => value,
            LengthUnit::Kilometers => value * 1000.0,
            LengthUnit::Inches => value / 39.3701,
            LengthUnit::Feet => value / 3.28084,
            LengthUnit::Yards => value / 1.09361,
            LengthUnit::StatuteMiles => value / 0.000621371,
            LengthUnit::NauticalMiles => value / 0.000539957,
        };

        let length_output = match to_unit {
            LengthUnit::Millimeters => length_meters * 1000.0,
            LengthUnit::Centimeters => length_meters * 100.0,
            LengthUnit::Meters => length_meters,
            LengthUnit::Kilometers => length_meters / 1000.0,
            LengthUnit::Inches => length_meters * 39.3701,
            LengthUnit::Feet => length_meters * 3.28084,
            LengthUnit::Yards => length_meters * 1.09361,
            LengthUnit::StatuteMiles => length_meters * 0.000621371,
            LengthUnit::NauticalMiles => length_meters * 0.000539957,
        };
        length_output
    }

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String> {
        vec![
            LengthUnit::Millimeters.name(),
            LengthUnit::Centimeters.name(),
            LengthUnit::Meters.name(),
            LengthUnit::Kilometers.name(),
            LengthUnit::Inches.name(),
            LengthUnit::Feet.name(),
            LengthUnit::Yards.name(),
            LengthUnit::StatuteMiles.name(),
            LengthUnit::NauticalMiles.name(),
        ]
    }

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    fn all_abbrs () -> Vec<String> {
        vec![
            LengthUnit::Millimeters.abbr(),
            LengthUnit::Centimeters.abbr(),
            LengthUnit::Meters.abbr(),
            LengthUnit::Kilometers.abbr(),
            LengthUnit::Inches.abbr(),
            LengthUnit::Feet.abbr(),
            LengthUnit::Yards.abbr(),
            LengthUnit::StatuteMiles.abbr(),
            LengthUnit::NauticalMiles.abbr(),
        ]
    }

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    fn all_names_and_abbrs () -> Vec<String> {
        vec![
            LengthUnit::Millimeters.name_and_abbr(),
            LengthUnit::Centimeters.name_and_abbr(),
            LengthUnit::Meters.name_and_abbr(),
            LengthUnit::Kilometers.name_and_abbr(),
            LengthUnit::Inches.name_and_abbr(),
            LengthUnit::Feet.name_and_abbr(),
            LengthUnit::Yards.name_and_abbr(),
            LengthUnit::StatuteMiles.name_and_abbr(),
            LengthUnit::NauticalMiles.name_and_abbr(),
        ]
    }

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// Example:
    /// ```rust
    /// let unit = LengthUnit::from_string("m");
    /// assert_eq!(unit, Some(LengthUnit::Meters));
    /// ```
    fn from_str (unit_str: &str) -> Option<Self> {
        // abbreviations
        let millimeters_abbr = LengthUnit::Millimeters.abbr();
        let centimeters_abbr = LengthUnit::Centimeters.abbr();
        let meters_abbr = LengthUnit::Meters.abbr();
        let kilometers_abbr = LengthUnit::Kilometers.abbr();
        let inches_abbr = LengthUnit::Inches.abbr();
        let feet_abbr = LengthUnit::Feet.abbr();
        let yards_abbr = LengthUnit::Yards.abbr();
        let statute_miles_abbr = LengthUnit::StatuteMiles.abbr();
        let nautical_miles_abbr = LengthUnit::NauticalMiles.abbr();

        // names
        let millimeters_name = LengthUnit::Millimeters.name();
        let centimeters_name = LengthUnit::Centimeters.name();
        let meters_name = LengthUnit::Meters.name();
        let kilometers_name = LengthUnit::Kilometers.name();
        let inches_name = LengthUnit::Inches.name();
        let feet_name = LengthUnit::Feet.name();
        let yards_name = LengthUnit::Yards.name();
        let statute_miles_name = LengthUnit::StatuteMiles.name();
        let nautical_miles_name = LengthUnit::NauticalMiles.name();

        // full names
        let millimeters_name_and_abbr = LengthUnit::Millimeters.name_and_abbr();
        let centimeters_name_and_abbr = LengthUnit::Centimeters.name_and_abbr();
        let meters_name_and_abbr = LengthUnit::Meters.name_and_abbr();
        let kilometers_name_and_abbr = LengthUnit::Kilometers.name_and_abbr();
        let inches_name_and_abbr = LengthUnit::Inches.name_and_abbr();
        let feet_name_and_abbr = LengthUnit::Feet.name_and_abbr();
        let yards_name_and_abbr = LengthUnit::Yards.name_and_abbr();
        let statute_miles_name_and_abbr = LengthUnit::StatuteMiles.name_and_abbr();
        let nautical_miles_name_and_abbr = LengthUnit::NauticalMiles.name_and_abbr();

        // match
        match unit_str {
            s if s == millimeters_abbr || s == millimeters_name || s == millimeters_name_and_abbr => Some(LengthUnit::Millimeters),
            s if s == centimeters_abbr || s == centimeters_name || s == centimeters_name_and_abbr => Some(LengthUnit::Centimeters),
            s if s == meters_abbr || s == meters_name || s == meters_name_and_abbr => Some(LengthUnit::Meters),
            s if s == kilometers_abbr || s == kilometers_name || s == kilometers_name_and_abbr => Some(LengthUnit::Kilometers),
            s if s == inches_abbr || s == inches_name || s == inches_name_and_abbr => Some(LengthUnit::Inches),
            s if s == feet_abbr || s == feet_name || s == feet_name_and_abbr => Some(LengthUnit::Feet),
            s if s == yards_abbr || s == yards_name || s == yards_name_and_abbr => Some(LengthUnit::Yards),
            s if s == statute_miles_abbr || s == statute_miles_name || s == statute_miles_name_and_abbr => Some(LengthUnit::StatuteMiles),
            s if s == nautical_miles_abbr || s == nautical_miles_name || s == nautical_miles_name_and_abbr => Some(LengthUnit::NauticalMiles),
            _ => None,
        }
    }

    /// Return the default unit of measure.
    /// The default unit of measure is meters.
    fn default () -> Self {
        LengthUnit::Meters
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define the units of measure for mass.
/// The base unit of measure is kilograms.
/// The MassUnit enum implements the UnitOfMeasure trait.
/// It provides methods to get the name and abbreviation of the unit,
/// convert a value to a different unit of measure, and return all unit names and abbreviations.
/// 
/// # Examples
/// ```rust
/// let mass_value = 1.0;
/// let mass_output = mass_unit_test.convert(mass_value, &MassUnit::Pounds);
/// assert_eq!(mass_output, 2.20462);
/// ```
#[derive(Debug, Copy, Clone)]
pub enum MassUnit {
    Kilograms,
    PoundsMass,
}

/// Implement the UnitOfMeasure trait for the MassUnit enum.
impl UnitOfMeasure for MassUnit {
    /// Get the abbreviation of the unit of measure.
    /// For example, "kg" for kilograms.
    fn abbr (&self) -> String {
        match self {
            MassUnit::Kilograms => "kg".to_string(),
            MassUnit::PoundsMass => "lb".to_string(),
        }
    }

    /// Get the name of the unit of measure.
    /// For example, "Kilograms".
    fn name (&self) -> String {
        match self {
            MassUnit::Kilograms => "Kilograms".to_string(),
            MassUnit::PoundsMass => "Pounds".to_string(),
        }
    }

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements,
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    fn convert (&self, value: f64, to_unit: &Self) -> f64 {
        // Conver to kilograms
        let mass_kilograms = match self {
            MassUnit::Kilograms => value,
            MassUnit::PoundsMass => value / 2.20462,
        };

        // Convert to desired unit
        let mass_output = match to_unit {
            MassUnit::Kilograms => mass_kilograms,
            MassUnit::PoundsMass => mass_kilograms * 2.20462,
        };
        mass_output
    }

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String> {
        vec![
            MassUnit::Kilograms.name(),
            MassUnit::PoundsMass.name(),
        ]
    }

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    fn all_abbrs () -> Vec<String> {
        vec![
            MassUnit::Kilograms.abbr(),
            MassUnit::PoundsMass.abbr(),
        ]
    }

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    fn all_names_and_abbrs () -> Vec<String> {
        vec![
            MassUnit::Kilograms.name_and_abbr(),
            MassUnit::PoundsMass.name_and_abbr(),
        ]
    }

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// Example:
    /// ```rust
    /// let unit = MassUnit::from_string("kg");
    /// assert_eq!(unit, Some(MassUnit::Kilograms));
    /// ```
    fn from_str (unit_str: &str) -> Option<Self> {
        // abbreviations
        let kilograms_abbr = MassUnit::Kilograms.abbr();
        let pounds_abbr = MassUnit::PoundsMass.abbr();

        // names
        let kilograms_name = MassUnit::Kilograms.name();
        let pounds_name = MassUnit::PoundsMass.name();

        // full names
        let kilograms_name_and_abbr = MassUnit::Kilograms.name_and_abbr();
        let pounds_name_and_abbr = MassUnit::PoundsMass.name_and_abbr();

        // match
        match unit_str {
            s if s == kilograms_abbr || s == kilograms_name || s == kilograms_name_and_abbr => Some(MassUnit::Kilograms),
            s if s == pounds_abbr || s == pounds_name || s == pounds_name_and_abbr => Some(MassUnit::PoundsMass),
            _ => None,
        }
    }

    /// Return the default unit of measure.
    /// The default unit of measure is kilograms.
    fn default () -> Self {
        MassUnit::Kilograms
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define the units of measure for time.
/// 
/// The TimeUnit enum implements the UnitOfMeasure trait.
/// 
/// It provides methods to get the name and abbreviation of the unit,
/// convert a value to a different unit of measure, and return all unit names and abbreviations.
/// 
/// The default unit of measure is seconds.
#[derive(Debug, Copy, Clone)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Years,
}

/// Implement the UnitOfMeasure trait for the TimeUnit enum.
impl UnitOfMeasure for TimeUnit {
    /// Get the abbreviation of the unit of measure.
    /// For example, "s" for seconds.
    fn abbr (&self) -> String {
        match self {
            TimeUnit::Seconds => "s".to_string(),
            TimeUnit::Minutes => "min".to_string(),
            TimeUnit::Hours => "hr".to_string(),
            TimeUnit::Days => "d".to_string(),
            TimeUnit::Weeks => "wk".to_string(),
            TimeUnit::Years => "yr".to_string(),
        }
    }

    /// Get the name of the unit of measure.
    /// For example, "Seconds".
    fn name (&self) -> String {
        match self {
            TimeUnit::Seconds => "Seconds".to_string(),
            TimeUnit::Minutes => "Minutes".to_string(),
            TimeUnit::Hours => "Hours".to_string(),
            TimeUnit::Days => "Days".to_string(),
            TimeUnit::Weeks => "Weeks".to_string(),
            TimeUnit::Years => "Years".to_string(),
        }
    }

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements,
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    fn convert (&self, value: f64, to_unit: &Self) -> f64 {
        // Convert to seconds
        let time_seconds = match self {
            TimeUnit::Seconds => value,
            TimeUnit::Minutes => value * 60.0,
            TimeUnit::Hours => value * 3600.0,
            TimeUnit::Days => value * 86400.0,
            TimeUnit::Weeks => value * 604800.0,
            TimeUnit::Years => value * 31536000.0,
        };

        // Convert to desired unit
        let time_output = match to_unit {
            TimeUnit::Seconds => time_seconds,
            TimeUnit::Minutes => time_seconds / 60.0,
            TimeUnit::Hours => time_seconds / 3600.0,
            TimeUnit::Days => time_seconds / 86400.0,
            TimeUnit::Weeks => time_seconds / 604800.0,
            TimeUnit::Years => time_seconds / 31536000.0,
        };
        time_output
    }

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String> {
        vec![
            TimeUnit::Seconds.name(),
            TimeUnit::Minutes.name(),
            TimeUnit::Hours.name(),
            TimeUnit::Days.name(),
            TimeUnit::Weeks.name(),
            TimeUnit::Years.name(),
        ]
    }

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    fn all_abbrs () -> Vec<String> {
        vec![
            TimeUnit::Seconds.abbr(),
            TimeUnit::Minutes.abbr(),
            TimeUnit::Hours.abbr(),
            TimeUnit::Days.abbr(),
            TimeUnit::Weeks.abbr(),
            TimeUnit::Years.abbr(),
        ]
    }

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    fn all_names_and_abbrs () -> Vec<String> {
        vec![
            TimeUnit::Seconds.name_and_abbr(),
            TimeUnit::Minutes.name_and_abbr(),
            TimeUnit::Hours.name_and_abbr(),
            TimeUnit::Days.name_and_abbr(),
            TimeUnit::Weeks.name_and_abbr(),
            TimeUnit::Years.name_and_abbr(),
        ]
    }

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// For example: "s" | "Seconds" | "Seconds (s)" -> Some(TimeUnit::Seconds)
    fn from_str (unit_str: &str) -> Option<Self> {
        // abbreviations
        let seconds_abbr = TimeUnit::Seconds.abbr();
        let minutes_abbr = TimeUnit::Minutes.abbr();
        let hours_abbr = TimeUnit::Hours.abbr();
        let days_abbr = TimeUnit::Days.abbr();
        let weeks_abbr = TimeUnit::Weeks.abbr();
        let years_abbr = TimeUnit::Years.abbr();

        // names
        let seconds_name = TimeUnit::Seconds.name();
        let minutes_name = TimeUnit::Minutes.name();
        let hours_name = TimeUnit::Hours.name();
        let days_name = TimeUnit::Days.name();
        let weeks_name = TimeUnit::Weeks.name();
        let years_name = TimeUnit::Years.name();

        // full names
        let seconds_name_and_abbr = TimeUnit::Seconds.name_and_abbr();
        let minutes_name_and_abbr = TimeUnit::Minutes.name_and_abbr();
        let hours_name_and_abbr = TimeUnit::Hours.name_and_abbr();
        let days_name_and_abbr = TimeUnit::Days.name_and_abbr();
        let weeks_name_and_abbr = TimeUnit::Weeks.name_and_abbr();
        let years_name_and_abbr = TimeUnit::Years.name_and_abbr();

        // match
        match unit_str {
            s if s == seconds_abbr || s == seconds_name || s == seconds_name_and_abbr => Some(TimeUnit::Seconds),
            s if s == minutes_abbr || s == minutes_name || s == minutes_name_and_abbr => Some(TimeUnit::Minutes),
            s if s == hours_abbr || s == hours_name || s == hours_name_and_abbr => Some(TimeUnit::Hours),
            s if s == days_abbr || s == days_name || s == days_name_and_abbr => Some(TimeUnit::Days),
            s if s == weeks_abbr || s == weeks_name || s == weeks_name_and_abbr => Some(TimeUnit::Weeks),
            s if s == years_abbr || s == years_name || s == years_name_and_abbr => Some(TimeUnit::Years),
            _ => None,
        }

    }

    /// Return the default unit of measure.
    /// The default unit of measure is seconds.
    fn default () -> Self {
        TimeUnit::Seconds
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define the units of measure for temperature.
/// 
/// The TemperatureUnit enum implements the UnitOfMeasure trait.
/// 
/// It provides methods to get the name and abbreviation of the unit,
/// convert a value to a different unit of measure, and return all unit names and abbreviations.
/// 
/// The default unit of measure is Kelvin.
#[derive(Debug, Copy, Clone)]
pub enum TemperatureUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
    Rankine,   
}

/// Implement the UnitOfMeasure trait for the TemperatureUnit enum.
/// 
/// The TemperatureUnit enum provides methods to get the name and abbreviation of the unit,
/// convert a value to a different unit of measure, and return all unit names and abbreviations.
/// 
/// The default unit of measure is Kelvin.
impl UnitOfMeasure for TemperatureUnit {
    /// Get the abbreviation of the unit of measure.
    /// For example, "K" for Kelvin.
    fn abbr (&self) -> String {
        match self {
            TemperatureUnit::Kelvin => "K".to_string(),
            TemperatureUnit::Celsius => "°C".to_string(),
            TemperatureUnit::Fahrenheit => "°F".to_string(),
            TemperatureUnit::Rankine => "°R".to_string(),
        }
    }

    /// Get the name of the unit of measure.
    /// For example, "Kelvin".
    fn name (&self) -> String {
        match self {
            TemperatureUnit::Kelvin => "Kelvin".to_string(),
            TemperatureUnit::Celsius => "Celsius".to_string(),
            TemperatureUnit::Fahrenheit => "Fahrenheit".to_string(),
            TemperatureUnit::Rankine => "Rankine".to_string(),
        }
    }

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements,
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    fn convert (&self, value: f64, to_unit: &Self) -> f64 {
        // Convert to Kelvin
        let temp_kelvin = match self {
            TemperatureUnit::Kelvin => value,
            TemperatureUnit::Celsius => value + 273.15,
            TemperatureUnit::Fahrenheit => (value + 459.67) * 5.0 / 9.0,
            TemperatureUnit::Rankine => value * 5.0 / 9.0,
        };

        // Convert to desired unit
        let temp_output = match to_unit {
            TemperatureUnit::Kelvin => temp_kelvin,
            TemperatureUnit::Celsius => temp_kelvin - 273.15,
            TemperatureUnit::Fahrenheit => temp_kelvin * 9.0 / 5.0 - 459.67,
            TemperatureUnit::Rankine => temp_kelvin * 9.0 / 5.0,
        };
        temp_output
    }

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String> {
        vec![
            TemperatureUnit::Kelvin.name(),
            TemperatureUnit::Celsius.name(),
            TemperatureUnit::Fahrenheit.name(),
            TemperatureUnit::Rankine.name(),
        ]
    }

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    /// For example, "K" | "°C" | "°F" | "°R"
    fn all_abbrs () -> Vec<String> {
        vec![
            TemperatureUnit::Kelvin.abbr(),
            TemperatureUnit::Celsius.abbr(),
            TemperatureUnit::Fahrenheit.abbr(),
            TemperatureUnit::Rankine.abbr(),
        ]
    }

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    /// For example, "Kelvin (K)" | "Celsius (°C)" | "Fahrenheit (°F)" | "Rankine (°R)"
    fn all_names_and_abbrs () -> Vec<String> {
        vec![
            TemperatureUnit::Kelvin.name_and_abbr(),
            TemperatureUnit::Celsius.name_and_abbr(),
            TemperatureUnit::Fahrenheit.name_and_abbr(),
            TemperatureUnit::Rankine.name_and_abbr(),
        ]
    }

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// For example: "K" | "Kelvin" | "Kelvin (K)" -> Some(TemperatureUnit::Kelvin)
    fn from_str(unit_str: &str) -> Option<Self> {
        // abbreviations
        let kelvin_abbr = TemperatureUnit::Kelvin.abbr();
        let celsius_abbr = TemperatureUnit::Celsius.abbr();
        let fahrenheit_abbr = TemperatureUnit::Fahrenheit.abbr();
        let rankine_abbr = TemperatureUnit::Rankine.abbr();

        // names
        let kelvin_name = TemperatureUnit::Kelvin.name();
        let celsius_name = TemperatureUnit::Celsius.name();
        let fahrenheit_name = TemperatureUnit::Fahrenheit.name();
        let rankine_name = TemperatureUnit::Rankine.name();

        // full names
        let kelvin_name_and_abbr = TemperatureUnit::Kelvin.name_and_abbr();
        let celsius_name_and_abbr = TemperatureUnit::Celsius.name_and_abbr();
        let fahrenheit_name_and_abbr = TemperatureUnit::Fahrenheit.name_and_abbr();
        let rankine_name_and_abbr = TemperatureUnit::Rankine.name_and_abbr();

        // match the unit str to the unit using the abbreviations, names, and full names variables
        match unit_str {
            s if s == kelvin_abbr || s == kelvin_name || s == kelvin_name_and_abbr => Some(TemperatureUnit::Kelvin),
            s if s == celsius_abbr || s == celsius_name || s == celsius_name_and_abbr => Some(TemperatureUnit::Celsius),
            s if s == fahrenheit_abbr || s == fahrenheit_name || s == fahrenheit_name_and_abbr => Some(TemperatureUnit::Fahrenheit),
            s if s == rankine_abbr || s == rankine_name || s == rankine_name_and_abbr => Some(TemperatureUnit::Rankine),
            _ => None,
        }
    }

    /// Return the default unit of measure.
    /// The default unit of measure is Kelvin.
    fn default () -> Self {
        TemperatureUnit::Kelvin
    }   
}


// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define the units of measure for velocity.
/// The base unit of measure is meters per second.
#[derive(Debug, Copy, Clone)]
pub enum VelocityUnit {
    MetersPerSecond,
    KilometersPerHour,
    FeetPerSecond,
    MilesPerHour,
    Knots,
}

/// Implement the UnitOfMeasure trait for the VelocityUnit enum.
impl UnitOfMeasure for VelocityUnit {
    /// Get the abbreviation of the unit of measure.
    /// For example, "m/s" for meters per second.
    fn abbr (&self) -> String {
        match self {
            VelocityUnit::MetersPerSecond => "m/s".to_string(),
            VelocityUnit::KilometersPerHour => "km/h".to_string(),
            VelocityUnit::FeetPerSecond => "ft/s".to_string(),
            VelocityUnit::MilesPerHour => "mph".to_string(),
            VelocityUnit::Knots => "kn".to_string(),
        }
    }

    /// Get the name of the unit of measure.
    /// For example, "Meters per Second".
    fn name (&self) -> String {
        match self {
            VelocityUnit::MetersPerSecond => "Meters per Second".to_string(),
            VelocityUnit::KilometersPerHour => "Kilometers per Hour".to_string(),
            VelocityUnit::FeetPerSecond => "Feet per Second".to_string(),
            VelocityUnit::MilesPerHour => "Miles per Hour".to_string(),
            VelocityUnit::Knots => "Knots".to_string(),
        }
    }

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements,
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    fn convert (&self, value: f64, to_unit: &Self) -> f64 {
        // Convert to meters per second
        let velocity_meters_per_second = match self {
            VelocityUnit::MetersPerSecond => value,
            VelocityUnit::KilometersPerHour => value / 3.6,
            VelocityUnit::FeetPerSecond => value / 3.28084,
            VelocityUnit::MilesPerHour => value / 2.23694,
            VelocityUnit::Knots => value / 1.94384,
        };

        // Convert to desired unit
        let velocity_output = match to_unit {
            VelocityUnit::MetersPerSecond => velocity_meters_per_second,
            VelocityUnit::KilometersPerHour => velocity_meters_per_second * 3.6,
            VelocityUnit::FeetPerSecond => velocity_meters_per_second * 3.28084,
            VelocityUnit::MilesPerHour => velocity_meters_per_second * 2.23694,
            VelocityUnit::Knots => velocity_meters_per_second * 1.94384,
        };
        velocity_output
    }

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String> {
        vec![
            VelocityUnit::MetersPerSecond.name(),
            VelocityUnit::KilometersPerHour.name(),
            VelocityUnit::FeetPerSecond.name(),
            VelocityUnit::MilesPerHour.name(),
            VelocityUnit::Knots.name(),
        ]
    }

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    fn all_abbrs () -> Vec<String> {
        vec![
            VelocityUnit::MetersPerSecond.abbr(),
            VelocityUnit::KilometersPerHour.abbr(),
            VelocityUnit::FeetPerSecond.abbr(),
            VelocityUnit::MilesPerHour.abbr(),
            VelocityUnit::Knots.abbr(),
        ]
    }

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    fn all_names_and_abbrs () -> Vec<String> {
        vec![
            VelocityUnit::MetersPerSecond.name_and_abbr(),
            VelocityUnit::KilometersPerHour.name_and_abbr(),
            VelocityUnit::FeetPerSecond.name_and_abbr(),
            VelocityUnit::MilesPerHour.name_and_abbr(),
            VelocityUnit::Knots.name_and_abbr(),
        ]
    }

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// For example: "m/s" | "Meters per Second" | "Meters per Second (m/s)" -> Some(VelocityUnit::MetersPerSecond)
    fn from_str (unit_str: &str) -> Option<Self> {
        // abbreviations
        let meterspersecond_abbr = VelocityUnit::MetersPerSecond.abbr();
        let kilometersperhour_abbr = VelocityUnit::KilometersPerHour.abbr();
        let feetpersecond_abbr = VelocityUnit::FeetPerSecond.abbr();
        let milesperhour_abbr = VelocityUnit::MilesPerHour.abbr();
        let knots_abbr = VelocityUnit::Knots.abbr();

        // names
        let meterspersecond_name = VelocityUnit::MetersPerSecond.name();
        let kilometersperhour_name = VelocityUnit::KilometersPerHour.name();
        let feetpersecond_name = VelocityUnit::FeetPerSecond.name();
        let milesperhour_name = VelocityUnit::MilesPerHour.name();
        let knots_name = VelocityUnit::Knots.name();

        // full names
        let meterspersecond_name_and_abbr = VelocityUnit::MetersPerSecond.name_and_abbr();
        let kilometersperhour_name_and_abbr = VelocityUnit::KilometersPerHour.name_and_abbr();
        let feetpersecond_name_and_abbr = VelocityUnit::FeetPerSecond.name_and_abbr();
        let milesperhour_name_and_abbr = VelocityUnit::MilesPerHour.name_and_abbr();
        let knots_name_and_abbr = VelocityUnit::Knots.name_and_abbr();

        // match the unit str to the unit using the abbreviations, names, and full names variables
        match unit_str {
            s if s == meterspersecond_abbr || s == meterspersecond_name || s == meterspersecond_name_and_abbr => Some(VelocityUnit::MetersPerSecond),
            s if s == kilometersperhour_abbr || s == kilometersperhour_name || s == kilometersperhour_name_and_abbr => Some(VelocityUnit::KilometersPerHour),
            s if s == feetpersecond_abbr || s == feetpersecond_name || s == feetpersecond_name_and_abbr => Some(VelocityUnit::FeetPerSecond),
            s if s == milesperhour_abbr || s == milesperhour_name || s == milesperhour_name_and_abbr => Some(VelocityUnit::MilesPerHour),
            s if s == knots_abbr || s == knots_name || s == knots_name_and_abbr => Some(VelocityUnit::Knots),
            _ => None,
        }
    }

    /// Return the default unit of measure.
    /// The default unit of measure is meters per second.
    fn default () -> Self {
        VelocityUnit::MetersPerSecond
    }

    
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define the units of measure for force.
/// The base unit of measure is newtons.
#[derive(Debug, Copy, Clone)]
pub enum ForceUnit {
    Newtons,
    PoundsForce,
    KilogramsForce,
}

/// Implement the UnitOfMeasure trait for the ForceUnit enum.
impl UnitOfMeasure for ForceUnit {
    /// Get the abbreviation of the unit of measure.
    /// For example, "N" for newtons.
    fn abbr (&self) -> String {
        match self {
            ForceUnit::Newtons => "N".to_string(),
            ForceUnit::PoundsForce => "lbf".to_string(),
            ForceUnit::KilogramsForce => "kgf".to_string(),
        }
    }

    /// Get the name of the unit of measure.
    /// For example, "Newtons".
    fn name (&self) -> String {
        match self {
            ForceUnit::Newtons => "Newtons".to_string(),
            ForceUnit::PoundsForce => "Pounds Force".to_string(),
            ForceUnit::KilogramsForce => "Kilograms Force".to_string(),
        }
    }

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements,
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    fn convert (&self, value: f64, to_unit: &Self) -> f64 {
        // Convert to newtons
        let force_newtons = match self {
            ForceUnit::Newtons => value,
            ForceUnit::PoundsForce => value * 4.44822,
            ForceUnit::KilogramsForce => value * 9.80665,
        };

        // Convert to desired unit
        let force_output = match to_unit {
            ForceUnit::Newtons => force_newtons,
            ForceUnit::PoundsForce => force_newtons / 4.44822,
            ForceUnit::KilogramsForce => force_newtons / 9.80665,
        };
        force_output
    }

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String> {
        vec![
            ForceUnit::Newtons.name(),
            ForceUnit::PoundsForce.name(),
            ForceUnit::KilogramsForce.name(),
        ]
    }

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    fn all_abbrs () -> Vec<String> {
        vec![
            ForceUnit::Newtons.abbr(),
            ForceUnit::PoundsForce.abbr(),
            ForceUnit::KilogramsForce.abbr(),
        ]
    }

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    fn all_names_and_abbrs () -> Vec<String> {
        vec![
            ForceUnit::Newtons.name_and_abbr(),
            ForceUnit::PoundsForce.name_and_abbr(),
            ForceUnit::KilogramsForce.name_and_abbr(),
        ]
    }

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// For example: "N" | "Newtons" | "Newtons (N)" -> Some(ForceUnit::Newtons)
    fn from_str (unit_str: &str) -> Option<Self> {
        // abbreviations
        let newtons_abbr = ForceUnit::Newtons.abbr();
        let poundsforce_abbr = ForceUnit::PoundsForce.abbr();
        let kilogramsforce_abbr = ForceUnit::KilogramsForce.abbr();

        // names
        let newtons_name = ForceUnit::Newtons.name();
        let poundsforce_name = ForceUnit::PoundsForce.name();
        let kilogramsforce_name = ForceUnit::KilogramsForce.name();

        // full names
        let newtons_name_and_abbr = ForceUnit::Newtons.name_and_abbr();
        let poundsforce_name_and_abbr = ForceUnit::PoundsForce.name_and_abbr();
        let kilogramsforce_name_and_abbr = ForceUnit::KilogramsForce.name_and_abbr();

        // match the unit str to the unit using the abbreviations, names, and full names variables
        match unit_str {
            s if s == newtons_abbr || s == newtons_name || s == newtons_name_and_abbr => Some(ForceUnit::Newtons),
            s if s == poundsforce_abbr || s == poundsforce_name || s == poundsforce_name_and_abbr => Some(ForceUnit::PoundsForce),
            s if s == kilogramsforce_abbr || s == kilogramsforce_name || s == kilogramsforce_name_and_abbr => Some(ForceUnit::KilogramsForce),
            _ => None,
        }
    }

    /// Return the default unit of measure.
    /// The default unit of measure is newtons.
    fn default () -> Self {
        ForceUnit::Newtons
    }
}


// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define the units of measure for pressure.
/// The base unit of measure is pascals.
#[derive(Debug, Copy, Clone)]
pub enum PressureUnit {
    Pascals,
    Kilopascals,
    Megapascals,
    Bars,
    PoundsPerSquareInch,
    Atmospheres,
    Torrs,
}

/// Implement the UnitOfMeasure trait for the PressureUnit enum.
impl UnitOfMeasure for PressureUnit {
    /// Get the abbreviation of the unit of measure.
    /// For example, "Pa" for pascals.
    fn abbr (&self) -> String {
        match self {
            PressureUnit::Pascals => "Pa".to_string(),
            PressureUnit::Kilopascals => "kPa".to_string(),
            PressureUnit::Megapascals => "MPa".to_string(),
            PressureUnit::Bars => "bar".to_string(),
            PressureUnit::PoundsPerSquareInch => "psi".to_string(),
            PressureUnit::Atmospheres => "atm".to_string(),
            PressureUnit::Torrs => "Torr".to_string(),
        }
    }

    /// Get the name of the unit of measure.
    /// For example, "Pascals".
    fn name (&self) -> String {
        match self {
            PressureUnit::Pascals => "Pascals".to_string(),
            PressureUnit::Kilopascals => "Kilopascals".to_string(),
            PressureUnit::Megapascals => "Megapascals".to_string(),
            PressureUnit::Bars => "Bars".to_string(),
            PressureUnit::PoundsPerSquareInch => "Pounds per Square Inch".to_string(),
            PressureUnit::Atmospheres => "Atmospheres".to_string(),
            PressureUnit::Torrs => "Torrs".to_string(),
        }
    }

    /// Convert a value from one unit of measure to another.
    /// To eliminate exponentially growing nested match statements,
    /// each value is converted to the base unit of measure (e.g. meters or kilograms),
    /// then converted to the desired unit of measure.
    fn convert (&self, value: f64, to_unit: &Self) -> f64 {
        // Convert to pascals
        let pressure_pascals = match self {
            PressureUnit::Pascals => value,
            PressureUnit::Kilopascals => value * 1000.0,
            PressureUnit::Megapascals => value * 1_000_000.0,
            PressureUnit::Bars => value * 100_000.0,
            PressureUnit::PoundsPerSquareInch => value * 6894.76,
            PressureUnit::Atmospheres => value * 101_325.0,
            PressureUnit::Torrs => value * 133.322,
        };

        // Convert to desired unit
        let pressure_output = match to_unit {
            PressureUnit::Pascals => pressure_pascals,
            PressureUnit::Kilopascals => pressure_pascals / 1000.0,
            PressureUnit::Megapascals => pressure_pascals / 1_000_000.0,
            PressureUnit::Bars => pressure_pascals / 100_000.0,
            PressureUnit::PoundsPerSquareInch => pressure_pascals / 6894.76,
            PressureUnit::Atmospheres => pressure_pascals / 101_325.0,
            PressureUnit::Torrs => pressure_pascals / 133.322,
        };
        pressure_output
    }

    /// Return a vector Strings of all of the names of the units of measure.
    fn all_names () -> Vec<String> {
        vec![
            PressureUnit::Pascals.name(),
            PressureUnit::Kilopascals.name(),
            PressureUnit::Megapascals.name(),
            PressureUnit::Bars.name(),
            PressureUnit::PoundsPerSquareInch.name(),
            PressureUnit::Atmospheres.name(),
            PressureUnit::Torrs.name(),
        ]
    }

    /// Return a vector Strings of all of the abbreviations of the units of measure.
    fn all_abbrs () -> Vec<String> {
        vec![
            PressureUnit::Pascals.abbr(),
            PressureUnit::Kilopascals.abbr(),
            PressureUnit::Megapascals.abbr(),
            PressureUnit::Bars.abbr(),
            PressureUnit::PoundsPerSquareInch.abbr(),
            PressureUnit::Atmospheres.abbr(),
            PressureUnit::Torrs.abbr(),
        ]
    }

    /// Return a vector Strings of all of the names and abbreviations of the units of measure.
    fn all_names_and_abbrs () -> Vec<String> {
        vec![
            PressureUnit::Pascals.name_and_abbr(),
            PressureUnit::Kilopascals.name_and_abbr(),
            PressureUnit::Megapascals.name_and_abbr(),
            PressureUnit::Bars.name_and_abbr(),
            PressureUnit::PoundsPerSquareInch.name_and_abbr(),
            PressureUnit::Atmospheres.name_and_abbr(),
            PressureUnit::Torrs.name_and_abbr(),
        ]
    }

    /// Create a new instance of the unit of measure from a string.
    /// The function takes either the abbr, name, or name_and_abbr of the unit of measure.
    /// For example: "Pa" | "Pascals" | "Pascals (Pa)" -> Some(PressureUnit::Pascals)
    fn from_str (unit_str: &str) -> Option<Self> {
        // abbreviations
        let pascals_abbr = PressureUnit::Pascals.abbr();
        let kilopascals_abbr = PressureUnit::Kilopascals.abbr();
        let megapascals_abbr = PressureUnit::Megapascals.abbr();
        let bars_abbr = PressureUnit::Bars.abbr();
        let poundspersquareinch_abbr = PressureUnit::PoundsPerSquareInch.abbr();
        let atmospheres_abbr = PressureUnit::Atmospheres.abbr();
        let torrs_abbr = PressureUnit::Torrs.abbr();

        // names
        let pascals_name = PressureUnit::Pascals.name();
        let kilopascals_name = PressureUnit::Kilopascals.name();
        let megapascals_name = PressureUnit::Megapascals.name();
        let bars_name = PressureUnit::Bars.name();
        let poundspersquareinch_name = PressureUnit::PoundsPerSquareInch.name();
        let atmospheres_name = PressureUnit::Atmospheres.name();
        let torrs_name = PressureUnit::Torrs.name();

        // full names
        let pascals_name_and_abbr = PressureUnit::Pascals.name_and_abbr();
        let kilopascals_name_and_abbr = PressureUnit::Kilopascals.name_and_abbr();
        let megapascals_name_and_abbr = PressureUnit::Megapascals.name_and_abbr();
        let bars_name_and_abbr = PressureUnit::Bars.name_and_abbr();
        let poundspersquareinch_name_and_abbr = PressureUnit::PoundsPerSquareInch.name_and_abbr();
        let atmospheres_name_and_abbr = PressureUnit::Atmospheres.name_and_abbr();
        let torrs_name_and_abbr = PressureUnit::Torrs.name_and_abbr();

        // match the unit str to the unit using the abbreviations, names, and full names variables
        match unit_str {
            s if s == pascals_abbr || s == pascals_name || s == pascals_name_and_abbr => Some(PressureUnit::Pascals),
            s if s == kilopascals_abbr || s == kilopascals_name || s == kilopascals_name_and_abbr => Some(PressureUnit::Kilopascals),
            s if s == megapascals_abbr || s == megapascals_name || s == megapascals_name_and_abbr => Some(PressureUnit::Megapascals),
            s if s == bars_abbr || s == bars_name || s == bars_name_and_abbr => Some(PressureUnit::Bars),
            s if s == poundspersquareinch_abbr || s == poundspersquareinch_name || s == poundspersquareinch_name_and_abbr => Some(PressureUnit::PoundsPerSquareInch),
            s if s == atmospheres_abbr || s == atmospheres_name || s == atmospheres_name_and_abbr => Some(PressureUnit::Atmospheres),
            s if s == torrs_abbr || s == torrs_name || s == torrs_name_and_abbr => Some(PressureUnit::Torrs),
            _ => None,
        }
    }

    /// Return the default unit of measure.
    /// The default unit of measure is pascals.
    fn default () -> Self {
        PressureUnit::Pascals
    }
}