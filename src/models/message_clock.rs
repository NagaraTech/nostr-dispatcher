/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `message_clock`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=message_clock, primary_key(id))]
pub struct MessageClock {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `from`
    pub from: String,
    /// Field representing column `from_clock`
    pub from_clock: i64,
    /// Field representing column `to`
    pub to: String,
    /// Field representing column `to_clock`
    pub to_clock: i64,
    /// Field representing column `action`
    pub action: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `message_clock` for [`MessageClock`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=message_clock)]
pub struct CreateMessageClock {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `from`
    pub from: String,
    /// Field representing column `from_clock`
    pub from_clock: i64,
    /// Field representing column `to`
    pub to: String,
    /// Field representing column `to_clock`
    pub to_clock: i64,
    /// Field representing column `action`
    pub action: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `message_clock` for [`MessageClock`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=message_clock)]
pub struct UpdateMessageClock {
    /// Field representing column `from`
    pub from: Option<String>,
    /// Field representing column `from_clock`
    pub from_clock: Option<i64>,
    /// Field representing column `to`
    pub to: Option<String>,
    /// Field representing column `to_clock`
    pub to_clock: Option<i64>,
    /// Field representing column `action`
    pub action: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl MessageClock {
    /// Insert a new row into `message_clock` with a given [`CreateMessageClock`]
    pub fn create(db: &mut ConnectionType, item: &CreateMessageClock) -> diesel::QueryResult<Self> {
        use crate::schema::message_clock::dsl::*;

        diesel::insert_into(message_clock).values(item).get_result::<Self>(db)
    }

    /// Get a row from `message_clock`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::message_clock::dsl::*;

        message_clock.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: MessageClockFilter) -> diesel::QueryResult<PaginationResult<Self>> {
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
        filter: MessageClockFilter,
    ) -> crate::schema::message_clock::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::message_clock::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::message_clock::id.eq(filter_id));
        }
        if let Some(filter_from) = filter.from {
            query = query.filter(crate::schema::message_clock::from.eq(filter_from));
        }
        if let Some(filter_from_clock) = filter.from_clock {
            query = query.filter(crate::schema::message_clock::from_clock.eq(filter_from_clock));
        }
        if let Some(filter_to) = filter.to {
            query = query.filter(crate::schema::message_clock::to.eq(filter_to));
        }
        if let Some(filter_to_clock) = filter.to_clock {
            query = query.filter(crate::schema::message_clock::to_clock.eq(filter_to_clock));
        }
        if let Some(filter_action) = filter.action {
            query = query.filter(crate::schema::message_clock::action.eq(filter_action));
        }
        if let Some(filter_status) = filter.status {
            query = query.filter(crate::schema::message_clock::status.eq(filter_status));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::message_clock::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `message_clock`, identified by the primary key with [`UpdateMessageClock`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateMessageClock) -> diesel::QueryResult<Self> {
        use crate::schema::message_clock::dsl::*;

        diesel::update(message_clock.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `message_clock`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::message_clock::dsl::*;

        diesel::delete(message_clock.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct MessageClockFilter {
    pub id: Option<String>,
    pub from: Option<String>,
    pub from_clock: Option<i64>,
    pub to: Option<String>,
    pub to_clock: Option<i64>,
    pub action: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
