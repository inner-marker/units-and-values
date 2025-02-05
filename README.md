# Overview

This package defines enums and structs for units of measure, values using those units, and conversions between units.

Generally, default units reflect the SI unit.

Conversions are handled by the `convert` function in the respective unit's module.

## Units

Implimentation of units enums and value structs is in progress. The following table shows the current implimentation status.

|Unit | Value | Unit | SI Unit | Abbr. |
|:---:|:---:|:---|:---| :---: |
| ✓ | ✓ | Length | Meter | m |
| ✓ | ✓ | Mass | Kilogram | kg |
| ✓ | ✓ | Time | Second | s |
| ✓ | ✓ | Temperature | Kelvin | K |
| ✓ | ✓ | Velocity | Meters per Second | m/s |
| ✓ | ✓ | Force | Newton | N |
| ✓ | ✓ | Pressure | Pascal | Pa |
| ✓ | ✓ | Bearing | Radian | rad |
| ✓ | ✓ | Acceleration | Meters per Second Squared | m/s^2 |
| | | Mixing Ratio | Kilogram per Kilogram | kg/kg |
| | | Angular Velocity | Radians per Second | rad/s |
| | | Energy | Joule | J |
| | | Luminous Intensity | Candela | cd |
| | | Amount of Substance | Mole | mol |
| | | Electric Current | Ampere | A |
