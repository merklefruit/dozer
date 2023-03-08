use crate::cache::{expression::QueryExpression, RecordWithId, RoCache};

use super::cache::expression::FilterExpression;
use crate::errors::CacheError;
use dozer_types::{
    serde,
    types::{IndexDefinition, Record, Schema},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]

/// This filter gets dynamically added to the query.
pub struct AccessFilter {
    /// FilterExpression to evaluate access
    pub filter: Option<FilterExpression>,

    /// Fields to be restricted
    pub fields: Vec<String>,
}

#[derive(Debug)]
/// CacheReader dynamically attaches permissions on top of queries
pub struct CacheReader {
    cache: Box<dyn RoCache>,
}

impl CacheReader {
    pub fn new(cache: Box<dyn RoCache>) -> Self {
        Self { cache }
    }

    // TODO: Implement check_access
    fn check_access(&self, _rec: &Record, _access_filter: &AccessFilter) -> Result<(), CacheError> {
        Ok(())
    }

    pub fn get_schema(&self) -> Result<&(Schema, Vec<IndexDefinition>), CacheError> {
        self.cache.get_schema()
    }

    pub fn get(
        &self,
        key: &[u8],
        access_filter: &AccessFilter,
    ) -> Result<RecordWithId, CacheError> {
        let record = self.cache.get(key)?;
        match self.check_access(&record.record, access_filter) {
            Ok(_) => Ok(record),
            Err(e) => Err(e),
        }
    }

    pub fn query(
        &self,
        query: &mut QueryExpression,
        access_filter: AccessFilter,
    ) -> Result<Vec<RecordWithId>, CacheError> {
        self.apply_access_filter(query, access_filter);
        self.cache.query(query)
    }

    pub fn count(
        &self,
        query: &mut QueryExpression,
        access_filter: AccessFilter,
    ) -> Result<usize, CacheError> {
        self.apply_access_filter(query, access_filter);
        self.cache.count(query)
    }

    // Apply filter if specified in access
    fn apply_access_filter(&self, query: &mut QueryExpression, access_filter: AccessFilter) {
        // TODO: Use `fields` in `access_filter`.
        if let Some(access_filter) = access_filter.filter {
            let filter = match query.filter.take() {
                Some(query_filter) => FilterExpression::And(vec![access_filter, query_filter]),
                None => access_filter,
            };

            query.filter = Some(filter);
        }
    }
}
