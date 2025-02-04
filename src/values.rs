
trait Value {
    /// Get the value in the given unit.
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to the given unit before returning.
    fn get(&self, unit: Box<dyn Unit>) -> Self;
    /// Set self to the given value in the given unit.
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to the default unit before storing.
    fn set(&mut self, value: f64, unit: Box<dyn Unit>);
}

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
    fn get(&self, unit: Box<dyn Unit>) -> TemperatureValue {
        let value = match unit.abbr().as_str() {
            "°F" => self.value * 9.0/5.0 - 459.67,
            "°C" => self.value - 273.15,
            "K" => self.value,
            "°R" => self.value * 9.0/5.0,
            _ => panic!("Invalid unit"),
        };
        TemperatureValue { value }
    }

    /// set self to the given value in the given unit
    /// Behind the scenes, the value is stored in the default unit,
    /// so the value is converted to Kelvin before storing.
    fn set(&mut self, value: f64, unit: Box<dyn Unit>) {
        self.value = match unit.abbr().as_str() {
            "°F" => (value + 459.67) * 5.0/9.0,
            "°C" => value + 273.15,
            "K" => value,
            "°R" => value * 5.0/9.0,
            _ => panic!("Invalid unit"),
        };
    }

}

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
    fn get(&self, unit: Box<dyn Unit>) -> Self {
        let value = match unit.abbr().as_str() {
            "m/s" => self.value,
            "ft/s" => self.value * 0.3048,
            "km/h" => self.value * 0.277778,
            "mph" => self.value * 0.44704,
            "Kts" => self.value * 0.514444,
            _ => panic!("Invalid unit"),
        };
        SpeedValue { value }
    }

    /// Set self to the given value in the given unit.
    /// Behind the scenes, the value is stored in Meters Per Second,
    /// so the value is converted to Meters Per Second before storing.
    fn set(&mut self, value: f64, unit: Box<dyn Unit>) {
        self.value = match unit.abbr().as_str() {
            "m/s" => value,
            "ft/s" => value / 03.28084,
            "km/h" => value / 3.6,
            "mph" => value / 2.23694,
            "Kts" => value / 1.94384,
            _ => panic!("Invalid unit"),
        };
    }
}