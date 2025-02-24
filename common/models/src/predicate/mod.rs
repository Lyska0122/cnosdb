use serde::{Deserialize, Serialize};

use self::domain::{ColumnDomains, PredicateRef, TimeRange};
use crate::schema::{ColumnType, TskvTableSchemaRef};

pub mod domain;
pub mod transformation;
pub mod utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Split {
    // partition id
    id: usize,
    time_range: TimeRange,
    tags_filter: ColumnDomains<String>,
    fields_filter: ColumnDomains<String>,
    limit: Option<usize>,
}

impl Split {
    pub fn new(
        id: usize,
        table: TskvTableSchemaRef,
        time_range: TimeRange,
        predicate: PredicateRef,
    ) -> Self {
        let domains_filter = predicate
            .filter()
            .translate_column(|c| table.column(&c.name).cloned());

        let tags_filter = domains_filter.translate_column(|e| match e.column_type {
            ColumnType::Tag => Some(e.name.clone()),
            _ => None,
        });

        let fields_filter = domains_filter.translate_column(|e| match e.column_type {
            ColumnType::Field(_) => Some(e.name.clone()),
            _ => None,
        });

        let limit = predicate.limit();

        Self {
            id,
            time_range,
            tags_filter,
            fields_filter,
            limit,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn time_range(&self) -> &TimeRange {
        &self.time_range
    }

    pub fn tags_filter(&self) -> &ColumnDomains<String> {
        &self.tags_filter
    }

    pub fn fields_filter(&self) -> &ColumnDomains<String> {
        &self.fields_filter
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }
}
