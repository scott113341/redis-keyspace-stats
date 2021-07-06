use crate::config::Config;
use crate::data::keys;
use crate::data::other::example_keys;
use crate::data::Data;
use crate::data::{memory, ttl, types};
use crate::metadata::Metadata;
use crate::stats::Stats;

pub fn stdout_table(config: &Config, metadata: &Metadata, data: &Data) {
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
        Cell::new("Keys"),
        Cell::new("Example keys"),
    ]));

    if config.has_stat(&Stats::Memory) {
        let row = table.get_mut_row(0).unwrap();
        row.add_cell(Cell::new("Memory"));
    }

    if config.has_stat(&Stats::TTL) {
        let row = table.get_mut_row(0).unwrap();
        row.add_cell(Cell::new("TTL"));
    }

    if config.has_stat(&Stats::Type) {
        let row = table.get_mut_row(0).unwrap();
        row.add_cell(Cell::new("Type"));
    }

    /************/
    /* ADD ROWS */
    /************/

    for pattern in data.patterns() {
        let bin = data.bins().get(pattern).unwrap();

        let mut row = Row::new(vec![
            Cell::new(pattern.as_str()),
            Cell::new(
                &vec![
                    format!("{} counted", bin.len(),),
                    format!("{} est. total", keys::total_estimate(metadata, data, bin)),
                ]
                .join("\n"),
            ),
            Cell::new(&example_keys(&bin).join("\n")),
        ]);

        if config.has_stat(&Stats::Memory) {
            row.add_cell(Cell::new(
                &vec![
                    format!("{} (sum)", convert(memory::total(data, bin) as f64)),
                    format!(
                        "{} (est. total)",
                        convert(memory::total_estimate(metadata, data, bin) as f64),
                    ),
                    format!("{} (p50)", convert(memory::percentile(data, bin, 50_f64))),
                    format!("{} (p90)", convert(memory::percentile(data, bin, 90_f64))),
                    format!("{} (p99)", convert(memory::percentile(data, bin, 99_f64))),
                ]
                .join("\n"),
            ));
        }

        if config.has_stat(&Stats::TTL) {
            row.add_cell(Cell::new(
                &vec![
                    format!("{:.2}% have TTL", ttl::pct_with_ttl(data, bin)),
                    format!(
                        "{} (p50)",
                        format_duration(Duration::from_secs_f64(ttl::percentile(
                            data, bin, 50_f64
                        )))
                        .to_string()
                    ),
                    format!(
                        "{} (p90)",
                        format_duration(Duration::from_secs_f64(ttl::percentile(
                            data, bin, 90_f64
                        )))
                        .to_string()
                    ),
                    format!(
                        "{} (p99)",
                        format_duration(Duration::from_secs_f64(ttl::percentile(
                            data, bin, 99_f64
                        )))
                        .to_string()
                    ),
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
