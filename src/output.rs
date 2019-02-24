use prettytable::Table;

use super::device;

fn print_status_table(data: &[device::Device]) {
    let mut table = Table::new();

    table.add_row(row![
        "ref",
        "name",
        "location2",
        "location",
        "value",
        "status",
        "device_api",
        "device_type",
        "control_pairs\nLabel | Value | Type"
    ]);

    for d in data.iter() {
        let mut t = Table::new();
        d.control_pairs.iter().for_each(|x| {
            t.add_row(row![
                x.label,
                x.control_value.to_string(),
                x.control_type.to_str()
            ]);
        });

        table.add_row(row![
            d.id,
            d.name,
            d.location2,
            d.location,
            d.value,
            d.status,
            d.device_api.to_str(),
            d.device_type.to_str(),
            t //control_pair_str
        ]);
    }

    table.printstd();
}

fn print_status_json(data: &[device::Device]) {
    println!("{}", serde_json::to_string(data).unwrap());
}

pub fn print_status(output_type: &str, data: &[device::Device]) {
    match output_type {
        "table" => print_status_table(&data),
        "json" => print_status_json(&data),
        _ => println!("Error, unknown output type: {}", output_type),
    }
}
