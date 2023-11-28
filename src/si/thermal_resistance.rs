//! Thermal resistance (base unit kelvin per watt, kg⁻¹ · m⁻² · s³ · K).
//!
//! Thermal resistance has the same kind as [temperature interval][ti], as this quantity relates
//! to change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic
//! temperature][tt]. See [thermodynamic temperature][tt] for a full explanation.
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html

quantity! {
    /// Thermal resistance (base unit kelvin per watt, kg⁻¹ · m⁻² · s³ · K).
    quantity: ThermalResistance; "thermal resistance";
    /// Dimension of thermal resistance, L⁻²M⁻¹T³Th (base unit kelvin per watt, kg⁻¹ · m⁻² · s³ · K).
    dimension: ISQ<
        N2,     // length
        N1,     // mass
        P3,     // time
        Z0,     // electric current
        P1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kelvin_second_cubed_per_yottagram_meter_squared: prefix!(kilo) / prefix!(yotta);
            "K · s³/(Yg · m²)", "kelvin second cubed per yottagram meter squared",
            "kelvin second cubed per yottagrams meter squared";
        @kelvin_second_cubed_per_zettagram_meter_squared: prefix!(kilo) / prefix!(zetta);
            "K · s³/(Zg · m²)", "kelvin second cubed per zettagram meter squared",
            "kelvin second cubed per zettagrams meter squared";
        @kelvin_second_cubed_per_exagram_meter_squared: prefix!(kilo) / prefix!(exa); "K · s³/(Eg · m²)",
            "kelvin second cubed per exagram meter squared",
            "kelvin second cubed per exagrams meter squared";
        @kelvin_second_cubed_per_petagram_meter_squared: prefix!(kilo) / prefix!(peta); "K · s³/(Pg · m²)",
            "kelvin second cubed per petagram meter squared",
            "kelvin second cubed per petagrams meter squared";
        @kelvin_second_cubed_per_teragram_meter_squared: prefix!(kilo) / prefix!(tera); "K · s³/(Tg · m²)",
            "kelvin second cubed per teragram meter squared",
            "kelvin second cubed per teragrams meter squared";
        @kelvin_second_cubed_per_gigagram_meter_squared: prefix!(kilo) / prefix!(giga); "K · s³/(Gg · m²)",
            "kelvin second cubed per gigagram meter squared",
            "kelvin second cubed per gigagrams meter squared";
        @kelvin_second_cubed_per_megagram_meter_squared: prefix!(kilo) / prefix!(mega); "K · s³/(Mg · m²)",
            "kelvin second cubed per megagram meter squared",
            "kelvin second cubed per megagrams meter squared";
        /// Derived unit of thermal resistance in base units. Equivalent to K/W.
        @kelvin_second_cubed_per_kilogram_meter_squared: prefix!(kilo) / prefix!(kilo); "K · s³/(kg · m²)",
            "kelvin second cubed per kilogram meter squared",
            "kelvin second cubed per kilograms meter squared";
        @kelvin_second_cubed_per_hectogram_meter_squared: prefix!(kilo) / prefix!(hecto);
            "K · s³/(hg · m²)", "kelvin second cubed per hectogram meter squared",
            "kelvin second cubed per hectograms meter squared";
        @kelvin_second_cubed_per_decagram_meter_squared: prefix!(kilo) / prefix!(deca); "K · s³/(dag · m²)",
            "kelvin second cubed per decagram meter squared",
            "kelvin second cubed per decagrams meter squared";
        @kelvin_second_cubed_per_gram_meter_squared: prefix!(kilo) / prefix!(none); "K · s³/(g · m²)",
            "kelvin second cubed per gram meter squared", "kelvin second cubed per grams meter squared";
        @kelvin_second_cubed_per_decigram_meter_squared: prefix!(kilo) / prefix!(deci); "K · s³/(dg · m²)",
            "kelvin second cubed per decigram meter squared",
            "kelvin second cubed per decigrams meter squared";
        @kelvin_second_cubed_per_centigram_meter_squared: prefix!(kilo) / prefix!(centi);
            "K · s³/(cg · m²)", "kelvin second cubed per centigram meter squared",
            "kelvin second cubed per centigrams meter squared";
        @kelvin_second_cubed_per_milligram_meter_squared: prefix!(kilo) / prefix!(milli);
            "K · s³/(mg · m²)", "kelvin second cubed per milligram meter squared",
            "kelvin second cubed per milligrams meter squared";
        @kelvin_second_cubed_per_microgram_meter_squared: prefix!(kilo) / prefix!(micro); "K · s³/µg · m",
            "kelvin second K · s³cubed per microgram meter squared",
            "kelvin second cubed per micrograms meter squared";
        @kelvin_second_cubed_per_nanogram_meter_squared: prefix!(kilo) / prefix!(nano); "K · s³/(ng · m²)",
            "kelvin second cubed per nanogram meter squared",
            "kelvin second cubed per nanograms meter squared";
        @kelvin_second_cubed_per_picogram_meter_squared: prefix!(kilo) / prefix!(pico); "K · s³/(pg · m²)",
            "kelvin second cubed per picogram meter squared",
            "kelvin second cubed per picograms meter squared";
        @kelvin_second_cubed_per_femtogram_meter_squared: prefix!(kilo) / prefix!(femto);
            "K · s³/(fg · m²)", "kelvin second cubed per femtogram meter squared",
            "kelvin second cubed per femtograms meter squared";
        @kelvin_second_cubed_per_attogram_meter_squared: prefix!(kilo) / prefix!(atto); "K · s³/(ag · m²)",
            "kelvin second cubed per attogram meter squared",
            "kelvin second cubed per attograms meter squared";
        @kelvin_second_cubed_per_zeptogram_meter_squared: prefix!(kilo) / prefix!(zepto);
            "K · s³/(zg · m²)", "kelvin second cubed per zeptogram meter squared",
            "kelvin second cubed per zeptograms meter squared";
        @kelvin_second_cubed_per_yoctogram_meter_squared: prefix!(kilo) / prefix!(yocto);
            "K · s³/(yg · m²)", "kelvin second cubed per yoctogram meter squared",
            "kelvin second cubed per yoctograms meter squared";

        // Thermal resistance is much more commonly expressed in terms of temperature / power.
        @kelvin_per_yottawatt: prefix!(none) / prefix!(yotta) ; "K/YW", "kelvin per yottawatt",
            "kelvin per yottawatts";
        @kelvin_per_zettawatt: prefix!(none) / prefix!(zetta); "K/ZW", "kelvin per zettawatt",
            "kelvin per zettawatts";
        @kelvin_per_exawatt: prefix!(none) / prefix!(exa); "K/EW", "kelvin per exawatt", "kelvin per exawatts";
        @kelvin_per_petawatt: prefix!(none) / prefix!(peta); "K/PW", "kelvin per petawatt", "kelvin per petawatts";
        @kelvin_per_terawatt: prefix!(none) / prefix!(tera); "K/TW", "kelvin per terawatt", "kelvin per terawatts";
        @kelvin_per_gigawatt: prefix!(none) / prefix!(giga); "K/GW", "kelvin per gigawatt", "kelvin per gigawatts";
        @kelvin_per_megawatt: prefix!(none) / prefix!(mega); "K/MW", "kelvin per megawatt", "kelvin per megawatts";
        @kelvin_per_kilowatt: prefix!(none) / prefix!(kilo); "K/kW", "kelvin per kilowatt", "kelvin per kilowatts";
        @kelvin_per_hectowatt: prefix!(none) / prefix!(hecto); "K/hW", "kelvin per hectowatt",
            "kelvin per hectowatts";
        @kelvin_per_decawatt: prefix!(none) / prefix!(deca); "K/daW", "kelvin per decawatt", "kelvin per decawatts";
        /// Derived unit of thermal resistance in derived units. Equivalent to K · s³/(kg · m²).
        @kelvin_per_watt: prefix!(none); "K/W", "kelvin per watt", "kelvin per watts";
        @kelvin_per_deciwatt: prefix!(none) / prefix!(deci); "K/dW", "kelvin per deciwatt", "kelvin per deciwatts";
        @kelvin_per_centiwatt: prefix!(none) / prefix!(centi); "K/cW", "kelvin per centiwatt",
            "kelvin per centiwatts";
        @kelvin_per_milliwatt: prefix!(none) / prefix!(milli); "K/mW", "kelvin per milliwatt",
            "kelvin per milliwatts";
        @kelvin_per_microwatt: prefix!(none) / prefix!(micro); "K/µW", "kelvin per microwatt",
            "kelvin per microwatts";
        @kelvin_per_nanowatt: prefix!(none) / prefix!(nano); "K/nW", "kelvin per nanowatt", "kelvin per nanowatts";
        @kelvin_per_picowatt: prefix!(none) / prefix!(pico); "K/pW", "kelvin per picowatt", "kelvin per picowatts";
        @kelvin_per_femtowatt: prefix!(none) / prefix!(femto); "K/fW", "kelvin per femtowatt",
            "kelvin per femtowatts";
        @kelvin_per_attowatt: prefix!(none) / prefix!(atto); "K/aW", "kelvin per attowatt", "kelvin per attowatts";
        @kelvin_per_zeptowatt: prefix!(none) / prefix!(zepto); "K/zW", "kelvin per zeptowatt",
            "kelvin per zeptowatts";
        @kelvin_per_yoctowatt: prefix!(none) / prefix!(yocto); "K/yW", "kelvin per yoctowatt",
            "kelvin per yoctowatts";

        // Celsius for convenience.
        @degree_celsius_second_cubed_per_kilogram_meter_squared: prefix!(kilo) / prefix!(kilo);
            "(s³ · °C)/(kg · m²)", "degree Celsius second cubed per kilogram meter squared",
            "sdegree Celsius econd cubed per kilograms meter squared";
        @degree_celsius_per_kilowatt: prefix!(none) / prefix!(kilo); "°C/kW", "degree Celsius per kilowatt",
            "degree Celsius per kilowatts";
        /// Derived unit of thermal resistance in derived units. Equivalent to K · s³/(kg · m²).
        @degree_celsius_per_watt: prefix!(none); "°C/W", "degree Celsius per watt",
            "degree Celsius per watts";
        @degree_celsius_per_milliwatt: prefix!(none) / prefix!(milli); "°C/mW", "degree Celsius per milliwatt",
            "degree Celsius per milliwatts";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::length as l;
        use crate::si::mass as m;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::temperature_interval as ti;
        use crate::si::thermal_resistance as tr;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ThermalResistance<V> = TemperatureInterval::new::<ti::kelvin>(V::one())
                * Time::new::<t::second>(V::one())
                * Time::new::<t::second>(V::one()) 
                * Time::new::<t::second>(V::one())
                / (Mass::new::<m::kilogram>(V::one()) 
                    * Length::new::<l::meter>(V::one()) 
                    * Length::new::<l::meter>(V::one()));
        }

        #[test]
        fn check_base_units() {
            test::<ti::kelvin, m::yottagram, tr::kelvin_second_cubed_per_yottagram_meter_squared>();
            test::<ti::kelvin, m::zettagram, tr::kelvin_second_cubed_per_zettagram_meter_squared>();
            test::<ti::kelvin, m::exagram, tr::kelvin_second_cubed_per_exagram_meter_squared>();
            test::<ti::kelvin, m::petagram, tr::kelvin_second_cubed_per_petagram_meter_squared>();
            test::<ti::kelvin, m::teragram, tr::kelvin_second_cubed_per_teragram_meter_squared>();
            test::<ti::kelvin, m::gigagram, tr::kelvin_second_cubed_per_gigagram_meter_squared>();
            test::<ti::kelvin, m::megagram, tr::kelvin_second_cubed_per_megagram_meter_squared>();
            test::<ti::kelvin, m::kilogram, tr::kelvin_second_cubed_per_kilogram_meter_squared>();
            test::<ti::kelvin, m::hectogram, tr::kelvin_second_cubed_per_hectogram_meter_squared>();
            test::<ti::kelvin, m::decagram, tr::kelvin_second_cubed_per_decagram_meter_squared>();
            test::<ti::kelvin, m::gram, tr::kelvin_second_cubed_per_gram_meter_squared>();
            test::<ti::kelvin, m::decigram, tr::kelvin_second_cubed_per_decigram_meter_squared>();
            test::<ti::kelvin, m::centigram, tr::kelvin_second_cubed_per_centigram_meter_squared>();
            test::<ti::kelvin, m::milligram, tr::kelvin_second_cubed_per_milligram_meter_squared>();
            test::<ti::kelvin, m::microgram, tr::kelvin_second_cubed_per_microgram_meter_squared>();
            test::<ti::kelvin, m::nanogram, tr::kelvin_second_cubed_per_nanogram_meter_squared>();
            test::<ti::kelvin, m::picogram, tr::kelvin_second_cubed_per_picogram_meter_squared>();
            test::<ti::kelvin, m::femtogram, tr::kelvin_second_cubed_per_femtogram_meter_squared>();
            test::<ti::kelvin, m::attogram, tr::kelvin_second_cubed_per_attogram_meter_squared>();
            test::<ti::kelvin, m::zeptogram, tr::kelvin_second_cubed_per_zeptogram_meter_squared>();
            test::<ti::kelvin, m::yoctogram, tr::kelvin_second_cubed_per_yoctogram_meter_squared>();

            test::<ti::degree_celsius, m::kilogram, 
                tr::degree_celsius_second_cubed_per_kilogram_meter_squared>();

            fn test<
                TI: ti::Conversion<V>,
                M: m::Conversion<V>,
                TR: tr::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalResistance::new::<TR>(V::one()),
                    &(Time::new::<t::second>(V::one())
                        * Time::new::<t::second>(V::one())
                        * Time::new::<t::second>(V::one())
                        * TemperatureInterval::new::<TI>(V::one()) 
                        / (Mass::new::<M>(V::one()) 
                            * Length::new::<l::meter>(V::one()) 
                            * Length::new::<l::meter>(V::one()))));
            }
        }

        #[test]
        fn check_ti_power_units() {
            test::<ti::kelvin, p::yottawatt, tr::kelvin_per_yottawatt>();
            test::<ti::kelvin, p::zettawatt, tr::kelvin_per_zettawatt>();
            test::<ti::kelvin, p::exawatt, tr::kelvin_per_exawatt>();
            test::<ti::kelvin, p::petawatt, tr::kelvin_per_petawatt>();
            test::<ti::kelvin, p::terawatt, tr::kelvin_per_terawatt>();
            test::<ti::kelvin, p::gigawatt, tr::kelvin_per_gigawatt>();
            test::<ti::kelvin, p::megawatt, tr::kelvin_per_megawatt>();
            test::<ti::kelvin, p::kilowatt, tr::kelvin_per_kilowatt>();
            test::<ti::kelvin, p::hectowatt, tr::kelvin_per_hectowatt>();
            test::<ti::kelvin, p::decawatt, tr::kelvin_per_decawatt>();
            test::<ti::kelvin, p::watt, tr::kelvin_per_watt>();
            test::<ti::kelvin, p::deciwatt, tr::kelvin_per_deciwatt>();
            test::<ti::kelvin, p::centiwatt, tr::kelvin_per_centiwatt>();
            test::<ti::kelvin, p::milliwatt, tr::kelvin_per_milliwatt>();
            test::<ti::kelvin, p::microwatt, tr::kelvin_per_microwatt>();
            test::<ti::kelvin, p::nanowatt, tr::kelvin_per_nanowatt>();
            test::<ti::kelvin, p::picowatt, tr::kelvin_per_picowatt>();
            test::<ti::kelvin, p::femtowatt, tr::kelvin_per_femtowatt>();
            test::<ti::kelvin, p::attowatt, tr::kelvin_per_attowatt>();
            test::<ti::kelvin, p::zeptowatt, tr::kelvin_per_zeptowatt>();
            test::<ti::kelvin, p::yoctowatt, tr::kelvin_per_yoctowatt>();

            test::<ti::degree_celsius, p::kilowatt, tr::degree_celsius_per_kilowatt>();
            test::<ti::degree_celsius, p::watt, tr::degree_celsius_per_watt>();
            test::<ti::degree_celsius, p::milliwatt, tr::degree_celsius_per_milliwatt>();

            fn test<
                TI: ti::Conversion<V>,
                P: p::Conversion<V>,
                TR: tr::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalResistance::new::<TR>(V::one()),
                    &(TemperatureInterval::new::<TI>(V::one()) 
                    / Power::new::<P>(V::one())));
            }
        }
    }
}
