#![allow(dead_code)]
use crate::units::*;

trait Value: Sized{
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to the given unit before returning.
    /// The conversion uses the `convert` function of the Unit.
    /// 
    /// # Example
    /// ```rust
    /// let length = LengthValue::new(100.0, &UnitEnum::Length(LengthUnit::Meters));
    /// assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Meters)).value, 100.0);
    /// ```
    fn get(&self, unit: &UnitEnum) -> Self;
    /// Set self to the given value in the given unit.
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to the default unit before storing.
    /// The conversion uses the `convert` function of the Unit.
    /// 
    /// # Example
    /// ```rust
    /// let mut length = LengthValue::new(100.0, &UnitEnum::Length(LengthUnit::Meters));
    /// length.set(200.0, &UnitEnum::Length(LengthUnit::Meters));
    /// assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Meters)).value, 200.0);
    /// ```
    fn set(&mut self, value: f64, unit: &UnitEnum);

    /// Create a new Value with the given value and unit.
    /// The value is stored in the default unit.
    /// The value is converted to the default unit before storing.
    /// 
    /// # Example
    /// ```rust
    /// let length = LengthValue::new(100.0, &UnitEnum::Length(LengthUnit::Meters));
    /// assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Meters)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> Self;
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

    /// Create a new LengthValue with the given value and unit.
    /// The value is stored in Meters by default.
    /// The value is converted to Meters before storing.
    /// 
    /// # Example
    /// ```rust
    /// let length = LengthValue::new(100.0, &UnitEnum::Length(LengthUnit::Meters));
    /// assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Meters)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> LengthValue {
        LengthValue {
            value: match unit {
                UnitEnum::Length(length_unit) => length_unit.convert(
                    value,
                    unit,
                    &LengthUnit::default()
                ),
                _ => panic!("Invalid unit for LengthValue"),
            }
        }
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

    /// Create a new MassValue with the given value and unit.
    /// The value is stored in Kilograms by default.
    /// The value is converted to Kilograms before storing.
    /// 
    /// # Example
    /// ```rust
    /// let mass = MassValue::new(100.0, &UnitEnum::Mass(MassUnit::Kilograms));
    /// assert_eq!(mass.get(&UnitEnum::Mass(MassUnit::Kilograms)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> MassValue {
        MassValue {
            value: match unit {
                UnitEnum::Mass(mass_unit) => mass_unit.convert(
                    value,
                    unit,
                    &MassUnit::default()
                ),
                _ => panic!("Invalid unit for MassValue"),
            }
        }
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

    /// Create a new TimeValue with the given value and unit.
    /// The value is stored in Seconds by default.
    /// The value is converted to Seconds before storing.
    /// 
    /// # Example
    /// ```rust
    /// let time = TimeValue::new(100.0, &UnitEnum::Time(TimeUnit::Seconds));
    /// assert_eq!(time.get(&UnitEnum::Time(TimeUnit::Seconds)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> TimeValue {
        TimeValue {
            value: match unit {
                UnitEnum::Time(time_unit) => time_unit.convert(
                    value,
                    unit,
                    &TimeUnit::default()
                ),
                _ => panic!("Invalid unit for TimeValue"),
            }
        }
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

    /// Create a new TemperatureValue with the given value and unit.
    /// The value is stored in Kelvin by default.
    /// The value is converted to Kelvin before storing.
    /// 
    /// # Example
    /// ```rust
    /// let temperature = TemperatureValue::new(100.0, &UnitEnum::Temperature(TemperatureUnit::Kelvin));
    /// assert_eq!(temperature.get(&UnitEnum::Temperature(TemperatureUnit::Kelvin)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> TemperatureValue {
        TemperatureValue {
            value: match unit {
                UnitEnum::Temperature(temp_unit) => temp_unit.convert(
                    value,
                    unit,
                    &TemperatureUnit::default()
                ),
                _ => panic!("Invalid unit for TemperatureValue"),
            }
        }
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

    /// Create a new VelocityValue with the given value and unit.
    /// The value is stored in Meters Per Second by default.
    /// The value is converted to Meters Per Second before storing.
    /// 
    /// # Example
    /// ```rust
    /// let velocity = VelocityValue::new(100.0, &UnitEnum::Velocity(VelocityUnit::MetersPerSecond));
    /// assert_eq!(velocity.get(&UnitEnum::Velocity(VelocityUnit::MetersPerSecond)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> Self {
        VelocityValue {
            value: match unit {
                UnitEnum::Velocity(velocity_unit) => velocity_unit.convert(
                    value,
                    unit,
                    &VelocityUnit::default()
                ),
                _ => panic!("Invalid unit for VelocityValue"),
            }
        }
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

    /// Create a new ForceValue with the given value and unit.
    /// The value is stored in Newtons by default.
    /// The value is converted to Newtons before storing.
    /// 
    /// # Example
    /// ```rust
    /// let force = ForceValue::new(100.0, &UnitEnum::Force(ForceUnit::Newtons));
    /// assert_eq!(force.get(&UnitEnum::Force(ForceUnit::Newtons)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> ForceValue {
        ForceValue {
            value: match unit {
                UnitEnum::Force(force_unit) => force_unit.convert(
                    value,
                    unit,
                    &ForceUnit::default()
                ),
                _ => panic!("Invalid unit for ForceValue"),
            }
        }
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

    /// Create a new PressureValue with the given value and unit.
    /// The value is stored in Pascals by default.
    /// The value is converted to Pascals before storing.
    /// 
    /// # Example
    /// ```rust
    /// let pressure = PressureValue::new(100.0, &UnitEnum::Pressure(PressureUnit::Pascals));
    /// assert_eq!(pressure.get(&UnitEnum::Pressure(PressureUnit::Pascals)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> PressureValue {
        PressureValue {
            value: match unit {
                UnitEnum::Pressure(pressure_unit) => pressure_unit.convert(
                    value,
                    unit,
                    &PressureUnit::default()
                ),
                _ => panic!("Invalid unit for PressureValue"),
            }
        }
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

    /// Create a new BearingValue with the given value and unit.
    /// The value is stored in Radians by default.
    /// The value is converted to Radians before storing.
    /// 
    /// # Example
    /// ```rust
    /// let bearing = BearingValue::new(1.0, &UnitEnum::Bearing(BearingUnit::Radians));
    /// assert_eq!(bearing.get(&UnitEnum::Bearing(BearingUnit::Radians)).value, 1.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> BearingValue {
        BearingValue {
            value: match unit {
                UnitEnum::Bearing(bearing_unit) => bearing_unit.convert(
                    value,
                    unit,
                    &BearingUnit::default()
                ),
                _ => panic!("Invalid unit for BearingValue"),
            }
        }
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

    /// Create a new AccelerationValue with the given value and unit.
    /// The value is stored in Meters Per Second Squared by default.
    /// The value is converted to Meters Per Second Squared before storing.
    /// 
    /// # Example
    /// ```rust
    /// let acceleration = AccelerationValue::new(100.0, &UnitEnum::Acceleration(AccelerationUnit::MetersPerSecondSquared));
    /// assert_eq!(acceleration.get(&UnitEnum::Acceleration(AccelerationUnit::MetersPerSecondSquared)).value, 100.0);
    /// ```
    fn new(value: f64, unit: &UnitEnum) -> AccelerationValue {
        AccelerationValue {
            value: match unit {
                UnitEnum::Acceleration(acceleration_unit) => acceleration_unit.convert(
                    value,
                    unit,
                    &AccelerationUnit::default()
                ),
                _ => panic!("Invalid unit for AccelerationValue"),
            }
        }
    }

}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::*;

    #[test]
    fn test_length_value() {
        let length = LengthValue::new(100.0, &UnitEnum::Length(LengthUnit::Meters));
        assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Meters)).value, 100.0);
        assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Kilometers)).value, 0.1);
        assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Feet)).value, 328.084);
        assert_eq!(length.get(&UnitEnum::Length(LengthUnit::Inches)).value, 3937.007874015748);
    }

    #[test]
    fn test_mass_value() {
        let mass = MassValue::new(100.0, &UnitEnum::Mass(MassUnit::Kilograms));
        assert_eq!(mass.get(&UnitEnum::Mass(MassUnit::Kilograms)).value, 100.0);
        assert_eq!(mass.get(&UnitEnum::Mass(MassUnit::Grams)).value, 100000.0);
        assert_eq!(mass.get(&UnitEnum::Mass(MassUnit::PoundsMass)).value, 220.46226218488);
    }
}