use nu_protocol::{PipelineData, Span};

/// The only real reason I have this is so that I can implement foreign traits on it. (Classic.)
#[derive(Clone, Debug)]
pub struct Value(pub nu_protocol::Value);

/// A table for displaying. I hereby pinky promise that `header_row`, and every row in `rows`, has
/// the same number of elements. It sucks to have to promise this, but it's so much easier to
/// display when it's a vector of rows, rather than columns.
#[derive(Clone, Debug)]
pub struct Table {
    pub header_row: Vec<String>,
    pub rows: Vec<Vec<Value>>,
    /// So you can't construct your own; to ensure `header_row` and `rows` really do have the same
    /// number of values.
    _secret: (),
}

/// The output of a shell command, in a format ready for displaying in the view.
#[derive(Clone, Debug)]
pub enum Output {
    Empty,
    Value(Value),
    List(Vec<Value>),
    /// Header row, then a vector of rows.
    Table(Table),
    Raw(Result<Vec<u8>, nu_protocol::ShellError>),
}

impl From<PipelineData> for Output {
    fn from(value: PipelineData) -> Output {
        match value {
            PipelineData::Empty => Output::Empty,
            PipelineData::Value(nu_protocol::Value::List { vals, .. }, _)
                => Output::from(vals.into_iter().map(Value).collect::<Vec<_>>()),
            PipelineData::Value(value, _) => Output::Value(Value(value)), // :(
            PipelineData::ListStream(stream, _)
                => Output::from(stream.stream.map(Value).collect::<Vec<_>>()),
            PipelineData::ExternalStream { stdout: None, .. } => Output::Raw(Ok(vec![])),
            PipelineData::ExternalStream { stdout: Some(stdout), .. } => {
                let result: Result<Vec<Vec<u8>>, _> = stdout.stream.collect();
                Output::Raw(result.map(|vecs| vecs.into_iter().flatten().collect()))
            }
        }
    }
}

impl From<Vec<Value>> for Output {
    /// Implemented for the special case that a list of records is to be treaded as a table.
    fn from(values: Vec<Value>) -> Output {
        /// Makes records easier to work with.
        struct Record {
            cols: Vec<String>,
            vals: Vec<nu_protocol::Value>,
        }

        if !values.iter().all(|value| matches!(value, Value(nu_protocol::Value::Record { .. }))) {
            return Output::List(values);
        }

        let records: Vec<_> = values.into_iter()
            .map(|value| {
                match value {
                    Value(nu_protocol::Value::Record { cols, vals, .. }) => {
                        debug_assert_eq!(cols.len(), vals.len()); // I'm assuming this is how Nushell works.
                        Record { cols, vals }
                    },
                    _ => unreachable!("Already checked"),
                }
            })
            .collect();

        let mut header_row: Vec<String> = vec![];

        for record in records.iter().flat_map(|record| record.cols.iter()) {
            if !header_row.contains(record) {
                header_row.push(record.clone());
            }
        }

        let mut table: Table = Table {
            header_row,
            rows: Vec::new(),
            _secret: ()
        };

        for record in records {
            let mut new_row = vec![
                Value(nu_protocol::Value::Nothing { span: Span::unknown() });
                table.header_row.len()
            ];

            for (index, (rec_col, rec_val)) in record.cols.into_iter()
                .zip(record.vals.into_iter())
                .enumerate() {
                // If a column is in the table, but not the record, the value is left as empty.
                if table.header_row.contains(&rec_col) {
                    new_row[index] = Value(rec_val);
                }
            }

            table.rows.push(new_row);
        }

        debug_assert!(table.rows.iter().all(|row| row.len() == table.header_row.len()));

        Output::Table(table)
    }
}