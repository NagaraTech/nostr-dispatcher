/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `message`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=message, primary_key(id))]
pub struct Message {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `from`
    pub from: String,
    /// Field representing column `to`
    pub to: String,
    /// Field representing column `action`
    pub action: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `info`
    pub info: serde_json::Value,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `message` for [`Message`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=message)]
pub struct CreateMessage {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `from`
    pub from: String,
    /// Field representing column `to`
    pub to: String,
    /// Field representing column `action`
    pub action: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `info`
    pub info: serde_json::Value,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `message` for [`Message`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=message)]
pub struct UpdateMessage {
    /// Field representing column `from`
    pub from: Option<String>,
    /// Field representing column `to`
    pub to: Option<String>,
    /// Field representing column `action`
    pub action: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `info`
    pub info: Option<serde_json::Value>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Message {
    /// Insert a new row into `message` with a given [`CreateMessage`]
    pub fn create(db: &mut ConnectionType, item: &CreateMessage) -> diesel::QueryResult<Self> {
        use crate::schema::message::dsl::*;

        diesel::insert_into(message).values(item).get_result::<Self>(db)
    }

    /// Get a row from `message`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::message::dsl::*;

        message.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: MessageFilter) -> diesel::QueryResult<PaginationResult<Self>> {
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
        filter: MessageFilter,
    ) -> crate::schema::message::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::message::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::message::id.eq(filter_id));
        }
        if let Some(filter_from) = filter.from {
            query = query.filter(crate::schema::message::from.eq(filter_from));
        }
        if let Some(filter_to) = filter.to {
            query = query.filter(crate::schema::message::to.eq(filter_to));
        }
        if let Some(filter_action) = filter.action {
            query = query.filter(crate::schema::message::action.eq(filter_action));
        }
        if let Some(filter_status) = filter.status {
            query = query.filter(crate::schema::message::status.eq(filter_status));
        }
        if let Some(filter_info) = filter.info {
            query = query.filter(crate::schema::message::info.eq(filter_info));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::message::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `message`, identified by the primary key with [`UpdateMessage`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateMessage) -> diesel::QueryResult<Self> {
        use crate::schema::message::dsl::*;

        diesel::update(message.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `message`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::message::dsl::*;

        diesel::delete(message.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct MessageFilter {
    pub id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub action: Option<String>,
    pub status: Option<String>,
    pub info: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
