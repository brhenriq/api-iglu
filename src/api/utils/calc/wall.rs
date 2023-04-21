pub struct BlockProps {
    width: f64,
    height: f64,
    length: f64,
    conductivity: f64,
}

pub struct PlasterProps {
    internal_thickness: f64,
    external_thickness: f64,
    conductivity: f64,
}

pub struct TemperatureProps {
    internal_temperature: f64,
    external_temperature: f64,
}

pub struct SettlementProps {
    conductivity: f64,
}

pub struct WallCalcProps {
    block: BlockProps,
    plaster: PlasterProps,
    settlement: SettlementProps,
    temperature: TemperatureProps,
    wall_area: f64,
}

pub fn wall(props: WallCalcProps) -> f64 {
    let block = props.block;
    let plaster = props.plaster;
    let settlement = props.settlement;
    let temperature = props.temperature;

    let area_a = (block.length * 0.01) * (block.height * 0.01);
    let resistance_a = (plaster.internal_thickness / plaster.conductivity)
        + (block.width / settlement.conductivity)
        + (plaster.external_thickness / plaster.conductivity);

    let area_b = block.height * block.length;
    let resistance_b = (plaster.internal_thickness / plaster.conductivity)
        + (block.width / block.conductivity)
        + (plaster.external_thickness / plaster.conductivity);

    let resistance_wall = (area_a + area_b) / ((area_a / resistance_a) + (area_b + resistance_b));

    let u = 1.00 / (0.13 + resistance_wall + 0.04);

    let delta = temperature.external_temperature - temperature.internal_temperature;

    props.wall_area * u * delta
}
