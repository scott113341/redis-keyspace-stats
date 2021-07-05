use crate::config::Config;
use crate::data::other::example_keys;
use crate::data::Data;
use crate::data::{memory, ttl, types};
use crate::stats::Stats;

pub fn stdout_table(config: &Config, data: &Data) {
    use humantime::format_duration;
    use pretty_bytes::converter::convert;
    use prettytable::{Cell, Row, Table};
    use std::time::Duration;

    /***************/
    /* ADD HEADERS */
    /***************/

    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);

    table.add_row(Row::new(vec![
        Cell::new("Pattern"),
        Cell::new("Key count"),
        Cell::new("Example keys"),
    ]));

    if config.has_stat(&Stats::Memory) {
        let row = table.get_mut_row(0).unwrap();
        row.add_cell(Cell::new("Memory (sum)"));
        row.add_cell(Cell::new("Memory (p50/90/99)"));
    }

    if config.has_stat(&Stats::TTL) {
        let row = table.get_mut_row(0).unwrap();
        row.add_cell(Cell::new("TTL (% with)"));
        row.add_cell(Cell::new("TTL (p50/90/99)"));
    }

    if config.has_stat(&Stats::Type) {
        let row = table.get_mut_row(0).unwrap();
        row.add_cell(Cell::new("Types"));
    }

    /************/
    /* ADD ROWS */
    /************/

    for pattern in data.patterns() {
        let bin = data.bins().get(pattern).unwrap();

        let mut row = Row::new(vec![
            Cell::new(pattern.as_str()),
            Cell::new(&format!("{}", bin.len())),
            Cell::new(&example_keys(&bin).join("\n")),
        ]);

        if config.has_stat(&Stats::Memory) {
            row.add_cell(Cell::new(&convert(memory::total(data, bin) as f64)));
            row.add_cell(Cell::new(
                &vec![
                    convert(memory::percentile(data, bin, 50_f64)),
                    convert(memory::percentile(data, bin, 90_f64)),
                    convert(memory::percentile(data, bin, 99_f64)),
                ]
                .join("\n"),
            ));
        }

        if config.has_stat(&Stats::TTL) {
            row.add_cell(Cell::new(&format!("{:.2}%", ttl::pct_with_ttl(data, bin))));
            row.add_cell(Cell::new(
                &vec![
                    format_duration(Duration::from_secs_f64(ttl::percentile(data, bin, 50_f64)))
                        .to_string(),
                    format_duration(Duration::from_secs_f64(ttl::percentile(data, bin, 90_f64)))
                        .to_string(),
                    format_duration(Duration::from_secs_f64(ttl::percentile(data, bin, 99_f64)))
                        .to_string(),
                ]
                .join("\n"),
            ));
        }

        if config.has_stat(&Stats::Type) {
            let mut type_lines = Vec::new();
            for (type_, pct) in types::type_pcts(data, bin) {
                type_lines.push(format!("{:.2}% {}", pct, type_));
            }
            row.add_cell(Cell::new(&type_lines.join("\n")));
        }

        table.add_row(row);
    }

    // Print the table to stdout
    table.printstd();
}
