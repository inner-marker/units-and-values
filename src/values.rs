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

/// LengthValue struct
/// 
/// The value is stored in Meters by default.
pub struct LengthValue {
    /// value of length
    value:f64,
}

/// Implementing Value trait for LengthValue
impl Value for LengthValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Meters,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> LengthValue {
        // use the conversion function of the Unit
        LengthValue { 
            value: match unit {
                UnitEnum::Length(length_unit) => length_unit.convert(
                    self.value,
                    &LengthUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for LengthValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Meters before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Meters and store it
        self.value = match unit {
            UnitEnum::Length(length_unit) => length_unit.convert(
                value,
                unit,
                &LengthUnit::default()
            ),
            _ => panic!("Invalid unit for LengthValue"),
        };
    }

}

// ---------------------------------------------------------

/// MassValue struct
pub struct MassValue {
    /// value of mass
    value:f64,
}

/// Implementing Value trait for MassValue
/// 
/// The value is stored in Kilograms by default.
impl Value for MassValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Kilograms,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> MassValue {
        // use the conversion function of the Unit
        MassValue { 
            value: match unit {
                UnitEnum::Mass(mass_unit) => mass_unit.convert(
                    self.value,
                    &MassUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for MassValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Kilograms before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Kilograms and store it
        self.value = match unit {
            UnitEnum::Mass(mass_unit) => mass_unit.convert(
                value,
                unit,
                &MassUnit::default()
            ),
            _ => panic!("Invalid unit for MassValue"),
        };
    }

}

// ---------------------------------------------------------

/// TimeValue struct
/// 
/// The value is stored in Seconds by default.
pub struct TimeValue {
    /// value of time
    value:f64,
}

/// Implementing Value trait for TimeValue
/// 
/// The value is stored in Seconds by default.
impl Value for TimeValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Seconds,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> TimeValue {
        // use the conversion function of the Unit
        TimeValue { 
            value: match unit {
                UnitEnum::Time(time_unit) => time_unit.convert(
                    self.value,
                    &TimeUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for TimeValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Seconds before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Seconds and store it
        self.value = match unit {
            UnitEnum::Time(time_unit) => time_unit.convert(
                value,
                unit,
                &TimeUnit::default()
            ),
            _ => panic!("Invalid unit for TimeValue"),
        };
    }
    
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

/// VelocityValue struct
pub struct VelocityValue {
    /// value of velocity, stored in Knots
    value:f64,

}

/// Implementing Value trait for VelocityValue
impl Value for VelocityValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Meters Per Second,
    /// so the value is converted from Meters Per Second to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> Self {
        VelocityValue {
            value: match unit {
                UnitEnum::Velocity(velocity_unit) => velocity_unit.convert(
                    self.value,
                    &VelocityUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for VelocityValue"),
            }
        }
    }

    /// Set self to the given value in the given unit.
    /// Behind the scenes, the value is stored in Meters Per Second,
    /// so the value is converted to Meters Per Second before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        self.value = match unit {
            UnitEnum::Velocity(velocity_unit) => velocity_unit.convert(
                value,
                unit,
                &VelocityUnit::default()
            ),
            _ => panic!("Invalid unit for VelocityValue"),
        };
    }
}

// ---------------------------------------------------------

/// ForceValue struct
/// 
/// The value is stored in Newtons by default.
pub struct ForceValue {
    /// value of force
    value:f64,
}

/// Implementing Value trait for ForceValue
impl Value for ForceValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Newtons,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> ForceValue {
        // use the conversion function of the Unit
        ForceValue { 
            value: match unit {
                UnitEnum::Force(force_unit) => force_unit.convert(
                    self.value,
                    &ForceUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for ForceValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Newtons before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Newtons and store it
        self.value = match unit {
            UnitEnum::Force(force_unit) => force_unit.convert(
                value,
                unit,
                &ForceUnit::default()
            ),
            _ => panic!("Invalid unit for ForceValue"),
        };
    }

}


// ---------------------------------------------------------

/// PressureValue struct
pub struct PressureValue {
    /// value of pressure, stored in Pascals
    value:f64,
}

/// Implementing Value trait for PressureValue
/// 
/// The value is stored in Pascals by default.
impl Value for PressureValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Pascals,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> PressureValue {
        // use the conversion function of the Unit
        PressureValue { 
            value: match unit {
                UnitEnum::Pressure(pressure_unit) => pressure_unit.convert(
                    self.value,
                    &PressureUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for PressureValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Pascals before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Pascals and store it
        self.value = match unit {
            UnitEnum::Pressure(pressure_unit) => pressure_unit.convert(
                value,
                unit,
                &PressureUnit::default()
            ),
            _ => panic!("Invalid unit for PressureValue"),
        };
    }

}

// ---------------------------------------------------------

/// BearingValue struct
/// 
/// The value is stored in Radians by default.
pub struct BearingValue {
    /// value of bearing
    value:f64,
}

/// Implementing Value trait for BearingValue
/// 
/// The value is stored in Radians by default.
impl Value for BearingValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Radians,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> BearingValue {
        // use the conversion function of the Unit
        BearingValue { 
            value: match unit {
                UnitEnum::Bearing(bearing_unit) => bearing_unit.convert(
                    self.value,
                    &BearingUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for BearingValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Radians before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Radians and store it
        self.value = match unit {
            UnitEnum::Bearing(bearing_unit) => bearing_unit.convert(
                value,
                unit,
                &BearingUnit::default()
            ),
            _ => panic!("Invalid unit for BearingValue"),
        };
    }

}

// ---------------------------------------------------------

/// AccelerationValue struct
/// 
/// The value is stored in Meters Per Second Squared by default.
pub struct AccelerationValue {
    /// value of acceleration
    value:f64,
}

/// Implementing Value trait for AccelerationValue
/// 
/// The value is stored in Meters Per Second Squared by default.
impl Value for AccelerationValue {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in Meters Per Second Squared,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: &UnitEnum) -> AccelerationValue {
        // use the conversion function of the Unit
        AccelerationValue { 
            value: match unit {
                UnitEnum::Acceleration(acceleration_unit) => acceleration_unit.convert(
                    self.value,
                    &AccelerationUnit::default(),
                    unit
                ),
                _ => panic!("Invalid unit for AccelerationValue"),
            }
        }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Meters Per Second Squared before storing.
    fn set(&mut self, value: f64, unit: &UnitEnum) {
        // conver the value to Meters Per Second Squared and store it
        self.value = match unit {
            UnitEnum::Acceleration(acceleration_unit) => acceleration_unit.convert(
                value,
                unit,
                &AccelerationUnit::default()
            ),
            _ => panic!("Invalid unit for AccelerationValue"),
        };
    }

}