/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `record`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=record, primary_key(id))]
pub struct Record {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `event_id`
    pub event_id: String,
    /// Field representing column `relay`
    pub relay: String,
    /// Field representing column `message_id`
    pub message_id: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `info`
    pub info: serde_json::Value,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `record` for [`Record`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=record)]
pub struct CreateRecord {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `event_id`
    pub event_id: String,
    /// Field representing column `relay`
    pub relay: String,
    /// Field representing column `message_id`
    pub message_id: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `info`
    pub info: serde_json::Value,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `record` for [`Record`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=record)]
pub struct UpdateRecord {
    /// Field representing column `event_id`
    pub event_id: Option<String>,
    /// Field representing column `relay`
    pub relay: Option<String>,
    /// Field representing column `message_id`
    pub message_id: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `info`
    pub info: Option<serde_json::Value>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Record {
    /// Insert a new row into `record` with a given [`CreateRecord`]
    pub fn create(db: &mut ConnectionType, item: &CreateRecord) -> diesel::QueryResult<Self> {
        use crate::schema::record::dsl::*;

        diesel::insert_into(record).values(item).get_result::<Self>(db)
    }

    /// Get a row from `record`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::record::dsl::*;

        record.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: RecordFilter) -> diesel::QueryResult<PaginationResult<Self>> {
        let page = page.max(0);
        let page_size = page_size.max(1);
        let total_items = Self::filter(filter.clone()).count().get_result(db)?;
        let items = Self::filter(filter).limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// A utility function to help build custom search queries
    /// 
    /// Example:
    /// 
    /// ```
    /// // create a filter for completed todos
    /// let query = Todo::filter(TodoFilter {
    ///     completed: Some(true),
    ///     ..Default::default()
    /// });
    /// 
    /// // delete completed todos
    /// diesel::delete(query).execute(db)?;
    /// ```
    pub fn filter<'a>(
        filter: RecordFilter,
    ) -> crate::schema::record::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::record::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::record::id.eq(filter_id));
        }
        if let Some(filter_event_id) = filter.event_id {
            query = query.filter(crate::schema::record::event_id.eq(filter_event_id));
        }
        if let Some(filter_relay) = filter.relay {
            query = query.filter(crate::schema::record::relay.eq(filter_relay));
        }
        if let Some(filter_message_id) = filter.message_id {
            query = query.filter(crate::schema::record::message_id.eq(filter_message_id));
        }
        if let Some(filter_status) = filter.status {
            query = query.filter(crate::schema::record::status.eq(filter_status));
        }
        if let Some(filter_info) = filter.info {
            query = query.filter(crate::schema::record::info.eq(filter_info));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::record::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `record`, identified by the primary key with [`UpdateRecord`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateRecord) -> diesel::QueryResult<Self> {
        use crate::schema::record::dsl::*;

        diesel::update(record.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `record`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::record::dsl::*;

        diesel::delete(record.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct RecordFilter {
    pub id: Option<String>,
    pub event_id: Option<String>,
    pub relay: Option<String>,
    pub message_id: Option<String>,
    pub status: Option<String>,
    pub info: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
