pub struct LiningProps {
    pub thickness: f64,
    pub conductivity: f64,
}
pub struct TilesProps {
    pub thickness: f64,
    pub conductivity: f64,
}

pub struct TemperaturePropsRoof {
    pub internal_temperature: f64,
    pub external_temperature: f64,
}

pub struct RoofCalcProps {
    pub temperature: TemperaturePropsRoof,
    pub floor_area: f64,
    pub lining: LiningProps,
    pub tile: TilesProps,
}

pub fn roof(props: RoofCalcProps) -> f64 {
    let tile = props.tile;
    let lining = props.lining;
    let temperature = props.temperature;

    let resistance = 1.00
        / (0.17
            + ((tile.thickness / tile.conductivity)
                + (lining.thickness / lining.conductivity)
                + 0.61)
            + 0.04);

    props.floor_area
        * resistance
        * ((temperature.external_temperature - temperature.internal_temperature) * 2.00)
}
