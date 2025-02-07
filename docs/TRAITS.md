
# Description of the Traits

## Description of the `UnitOfMeasure` Trait

Implimentaitons of the trait should include the following functions, plus the `Debug`, `Copy`, and `Clone` traits.

```rust
/// Define the units of measure for length.
/// The base unit of measure is meters.
#[derive(Debug, Copy, Clone)]
pub enum LengthUnit {
    Unit1,
    Unit2,
}

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
```

## Description of the `Value` Trait

Implimentaitons of the trait should include the following functions, plus the `Debug`, `Copy`, and `Clone` traits.

```rust
#[derive(Debug, Copy, Clone)]
```

```rust
/// Get the value of the measurement.
fn value (&self) -> f64;

/// Get the unit of measure of the measurement.
fn unit (&self) -> T;

/// Convert the value of the measurement to a different unit of measure.
/// The value is converted to the base unit of measure (e.g., meters or kilograms),
/// then converted to the desired unit of measure.
/// The new unit of measure is stored in the `unit` field.
fn convert (&self, to_unit: &T) -> Self;
```

Then, impliment Display.

```rust
impl Display for Value<T>
```