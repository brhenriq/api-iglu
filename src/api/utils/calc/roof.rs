pub struct LiningProps {
    thickness: f64,
    conductivity: f64,
}
pub struct TilesProps {
    thickness: f64,
    conductivity: f64,
}

pub struct TemperatureProps {
    internal_temperature: f64,
    external_temperature: f64,
}

pub struct RoofCalcProps {
    temperature: TemperatureProps,
    floor_area: f64,
    lining: LiningProps,
    tile: TilesProps,
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
