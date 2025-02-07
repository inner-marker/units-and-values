#![allow(dead_code)]
use crate::units::*;
use std::fmt::{Debug, Display};


/// The Trait ValueWithUnit defines a common interface for values with units.
/// It provides methods to get the value and unit of the measurement.
/// The ValueWithUnit trait is generic over the type of unit of measure.
/// The ValueWithUnit trait is implemented for any type that implements the UnitOfMeasure trait.
pub trait ValueWithUnit<T: UnitOfMeasure>: Debug 
    + Copy
    + Clone
    + Display
    + PartialEq
{
    /// Get the value of the measurement.
    fn value (&self) -> f64;

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> T;

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The new unit of measure is stored in the `unit` field.
    fn convert (&self, to_unit: &T) -> Self;

    /// Create a new ValueWithUnit instance with the specified value and unit.
    fn new(value: f64, unit: T) -> Self;
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------


/// Define a struct for a length measurement.
/// The Length struct implements the ValueWithUnit trait for the LengthUnit enum.
/// It stores a value and a unit of measure for a length measurement.
/// The Length struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LengthValue {
    pub value: f64,
    pub unit: LengthUnit,
}

/// Implement the Display trait for the LengthValue struct.
impl ValueWithUnit<LengthUnit> for LengthValue {
    /// Get the value of the measurement.
    fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> LengthUnit {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new LengthValue struct with the converted value and unit.
    fn convert (&self, to_unit: &LengthUnit) -> LengthValue {
        LengthValue {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new LengthValue instance with the specified value and unit.
    fn new(value: f64, unit: LengthUnit) -> LengthValue {
        LengthValue {
            value,
            unit,
        }
    }
}

impl Display for LengthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value: {} {}", self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define a struct for a mass measurement
/// The Mass struct implements the ValueWithUnit trait for the MassUnit enum.
/// 
/// It stores a value and a unit of measure for a mass measurement.
/// 
/// The Mass struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MassValue {
    pub value: f64,
    pub unit: MassUnit,
}

/// Implementation of the ValueWithUnit trait for MassValue.
///
/// This implementation provides the following operations:
///
/// - `value()`: Retrieves the numeric value of the mass measurement.
/// - `unit()`: Retrieves the associated MassUnit of the measurement.
/// - `convert()`: Converts the measurement to a desired target unit.
///
/// The `convert()` method performs conversion by:
/// 1. Using the current unit's conversion logic to change the value
///    to a base unit (for example, kilograms).
/// 2. Converting the base unit value to the target unit, returning a new
///    MassValue instance with the converted value and the new unit.
impl ValueWithUnit<MassUnit> for MassValue {
    /// Get the value of the measurement.
    fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> MassUnit {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new MassValue struct with the converted value and unit.
    fn convert (&self, to_unit: &MassUnit) -> MassValue {
        MassValue {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new MassValue instance with the specified value and unit.
    fn new(value: f64, unit: MassUnit) -> MassValue {
        MassValue {
            value,
            unit,
        }
    }
}

/// Impliment the Display trait for the MassValue struct.
impl Display for MassValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value: {} {}", self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define a struct for a time measurement.
/// The Time struct implements the ValueWithUnit trait for the TimeUnit enum.
/// It stores a value and a unit of measure for a time measurement.
/// 
/// The Time struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
/// 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimeValue {
    pub value: f64,
    pub unit: TimeUnit,
}

/// Implement the Display trait for the TimeValue struct.
impl ValueWithUnit<TimeUnit> for TimeValue {
    /// Get the value of the measurement.
    fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> TimeUnit {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new TimeValue struct with the converted value and unit.
    fn convert (&self, to_unit: &TimeUnit) -> TimeValue {
        TimeValue {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new TimeValue instance with the specified value and unit.
    fn new(value: f64, unit: TimeUnit) -> TimeValue {
        TimeValue {
            value,
            unit,
        }
    }
}

impl Display for TimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value: {} {}", self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define a struct for a temperature measurement.
/// The Temperature struct implements the ValueWithUnit trait for the TemperatureUnit enum.
/// It stores a value and a unit of measure for a temperature measurement.
///
/// The Temperature struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TemperatureValue {
    pub value: f64,
    pub unit: TemperatureUnit,
}

/// Implement the Display trait for the TemperatureValue struct.
impl ValueWithUnit<TemperatureUnit> for TemperatureValue {
    /// Get the value of the measurement.
    fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> TemperatureUnit {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new TemperatureValue struct with the converted value and unit.
    fn convert (&self, to_unit: &TemperatureUnit) -> TemperatureValue {
        TemperatureValue {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new TemperatureValue instance with the specified value and unit.
    fn new(value: f64, unit: TemperatureUnit) -> TemperatureValue {
        TemperatureValue {
            value,
            unit,
        }
    }
}

impl Display for TemperatureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value: {} {}", self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define a struct for a velocity measurement.
/// The Velocity struct implements the ValueWithUnit trait for the VelocityUnit enum.
/// It stores a value and a unit of measure for a velocity measurement.
/// 
/// The Velocity struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VelocityValue {
    pub value: f64,
    pub unit: VelocityUnit,
}

/// Implement the Display trait for the VelocityValue struct.
impl ValueWithUnit<VelocityUnit> for VelocityValue {
    /// Get the value of the measurement.
    fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> VelocityUnit {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new VelocityValue struct with the converted value and unit.
    fn convert (&self, to_unit: &VelocityUnit) -> VelocityValue {
        VelocityValue {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new VelocityValue instance with the specified value and unit.
    fn new(value: f64, unit: VelocityUnit) -> VelocityValue {
        VelocityValue {
            value,
            unit,
        }
    }
}

impl Display for VelocityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value: {} {}", self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define a struct for an force measurement.
/// The Force struct implements the ValueWithUnit trait for the ForceUnit enum.
/// It stores a value and a unit of measure for a force measurement.
///
/// The Force struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ForceValue {
    pub value: f64,
    pub unit: ForceUnit,
}

/// Implement the Display trait for the ForceValue struct.
impl ValueWithUnit<ForceUnit> for ForceValue {
    /// Get the value of the measurement.
    fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> ForceUnit {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new ForceValue struct with the converted value and unit.
    fn convert (&self, to_unit: &ForceUnit) -> ForceValue {
        ForceValue {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new ForceValue instance with the specified value and unit.
    fn new(value: f64, unit: ForceUnit) -> ForceValue {
        ForceValue {
            value,
            unit,
        }
    }
}

impl Display for ForceValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value: {} {}", self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Define a struct for an pressure measurement.
/// 
/// The Pressure struct implements the ValueWithUnit trait for the PressureUnit enum.
/// It stores a value and a unit of measure for a pressure measurement.
/// 
/// The Pressure struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PressureValue {
    pub value: f64,
    pub unit: PressureUnit,
}

/// Implement the Display trait for the PressureValue struct.
impl ValueWithUnit<PressureUnit> for PressureValue {
    /// Get the value of the measurement.
    fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    fn unit (&self) -> PressureUnit {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new PressureValue struct with the converted value and unit.
    fn convert (&self, to_unit: &PressureUnit) -> PressureValue {
        PressureValue {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new PressureValue instance with the specified value and unit.
    fn new(value: f64, unit: PressureUnit) -> PressureValue {
        PressureValue {
            value,
            unit,
        }
    }
}

impl Display for PressureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value: {} {}", self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------


/// Define a struct for a measurement.
/// 
/// The struct implements the ValueWithUnit trait for the any enum that impliments UnitOfMeasure trait.
/// It stores a value and a unit of measure for the measurement.
/// 
/// The struct provides methods to get the value and unit of the measurement,
/// as well as to convert the value to a different unit of measure.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GenericValueWithUnit <U: UnitOfMeasure> {
    pub value: f64,
    pub unit: U,
}

/// Implement the Display trait for the GenericValueWithUnit struct.
impl <U: UnitOfMeasure> GenericValueWithUnit<U>{
    /// Get the value of the measurement.
    pub fn value (&self) -> f64 {
        self.value
    }

    /// Get the unit of measure of the measurement.
    pub fn unit (&self) -> U {
        self.unit
    }

    /// Convert the value of the measurement to a different unit of measure.
    /// The value is converted to the base unit of measure (e.g., meters or kilograms),
    /// then converted to the desired unit of measure.
    /// The function returns a new GenericValueWithUnit struct with the converted value and unit.
    pub fn convert (&self, to_unit: &U) -> GenericValueWithUnit<U> {
        GenericValueWithUnit {
            value: self.unit.convert(self.value, to_unit),
            unit: to_unit.clone(),
        }
    }

    /// Create a new GenericValueWithUnit instance with the specified value and unit.
    pub fn new(value: f64, unit: U) -> GenericValueWithUnit<U> {
        GenericValueWithUnit {
            value,
            unit,
        }
    }
}

impl<U: UnitOfMeasure> Display for GenericValueWithUnit<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} Value: {:.2} {}", U::generic_name(), self.value(), self.unit().name_and_abbr())
    }
}

// --------------------------------------------------------------------------------------------------

// Test the GenericValue struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_value() {
        let value = GenericValueWithUnit::new(10.0, LengthUnit::Meters);
        assert_eq!(value.value(), 10.0);
        assert_eq!(value.unit(), LengthUnit::Meters);
        assert_eq!(value.convert(&LengthUnit::Kilometers).value(), 0.01);

        // Test whether the Display trait is implemented correctly
        assert_eq!(format!("{}", value), "Length Value: 10.00 Meters (m)");
    }
}