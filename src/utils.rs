use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};

pub fn set_table_format(table: &mut prettytable::Table) {
    table.set_format(
        FormatBuilder::new()
            .column_separator('│')
            .separator(LinePosition::Top, LineSeparator::new('─', '┬', ' ', ' '))
            .separator(LinePosition::Title, LineSeparator::new('─', '┼', ' ', ' '))
            .separator(LinePosition::Bottom, LineSeparator::new('─', '┴', ' ', ' '))
            .padding(1, 1)
            .build(),
    );
}

pub mod deque;