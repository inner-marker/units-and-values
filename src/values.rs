#![allow(dead_code)]
use crate::units::*;

trait Value {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to the given unit before returning.
    /// The conversion uses the `convert` function of the Unit.
    fn get(&self, unit: &UnitEnum) -> Self;
    /// Set self to the given value in the given unit.
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to the default unit before storing.
    /// The conversion uses the `convert` function of the Unit.
    fn set(&mut self, value: f64, unit: &UnitEnum);
}

// ---------------------------------------------------------

/// TemperatureValue struct
pub struct TemperatureValue {
    /// value of temperature. 
    /// stored in Kelvin by default.
    value:f64,
}

/// Implementing Value trait for TemperatureValue
impl Value for TemperatureValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Kelvin,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> TemperatureValue {
        // use the conversion function of the Unit
        TemperatureValue { 
            value: match unit {
                UnitEnum::Temperature(temp_unit) => temp_unit.convert(
                    self.value,
                    &TemperatureUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for TemperatureValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Kelvin before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Kelvin and store it
        self.value = match unit {
            UnitEnum::Temperature(temp_unit) => temp_unit.convert(
                value,
                unit,
                &TemperatureUnit::default()
            ),
            _ => panic!("Invalid unit for TemperatureValue"),
        };
    }

}

// ---------------------------------------------------------

/// SpeedValue struct
pub struct SpeedValue {
    /// value of speed, stored in Knots
    value:f64,

}

/// Implementing Value trait for SpeedValue
impl Value for SpeedValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Meters Per Second,
    /// so the value is converted from Meters Per Second to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> Self {
        SpeedValue {
            value: match unit {
                UnitEnum::Speed(speed_unit) => speed_unit.convert(
                    self.value,
                    &SpeedUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for SpeedValue"),
            }
        }
    }

    /// Set self to the given value in the given unit.
    /// Behind the scenes, the value is stored in Meters Per Second,
    /// so the value is converted to Meters Per Second before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        self.value = match unit {
            UnitEnum::Speed(speed_unit) => speed_unit.convert(
                value,
                unit,
                &SpeedUnit::default()
            ),
            _ => panic!("Invalid unit for SpeedValue"),
        };
    }
}