/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;
use crate::models::common::*;

/// Struct representing a row in table `relays`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=relays, primary_key(id))]
pub struct Relays {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `url`
    pub url: String,
    /// Field representing column `info`
    pub info: serde_json::Value,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `relays` for [`Relays`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=relays)]
pub struct CreateRelays {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `url`
    pub url: String,
    /// Field representing column `info`
    pub info: serde_json::Value,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `relays` for [`Relays`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=relays)]
pub struct UpdateRelays {
    /// Field representing column `url`
    pub url: Option<String>,
    /// Field representing column `info`
    pub info: Option<serde_json::Value>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Relays {
    /// Insert a new row into `relays` with a given [`CreateRelays`]
    pub fn create(db: &mut ConnectionType, item: &CreateRelays) -> diesel::QueryResult<Self> {
        use crate::schema::relays::dsl::*;

        diesel::insert_into(relays).values(item).get_result::<Self>(db)
    }

    /// Get a row from `relays`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::relays::dsl::*;

        relays.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64, filter: RelaysFilter) -> diesel::QueryResult<PaginationResult<Self>> {
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
        filter: RelaysFilter,
    ) -> crate::schema::relays::BoxedQuery<'a, diesel::pg::Pg> {
        let mut query = crate::schema::relays::table.into_boxed();
        
        if let Some(filter_id) = filter.id {
            query = query.filter(crate::schema::relays::id.eq(filter_id));
        }
        if let Some(filter_url) = filter.url {
            query = query.filter(crate::schema::relays::url.eq(filter_url));
        }
        if let Some(filter_info) = filter.info {
            query = query.filter(crate::schema::relays::info.eq(filter_info));
        }
        if let Some(filter_created_at) = filter.created_at {
            query = query.filter(crate::schema::relays::created_at.eq(filter_created_at));
        }
        
        query
    }

    /// Update a row in `relays`, identified by the primary key with [`UpdateRelays`]
    pub fn update(db: &mut ConnectionType, param_id: String, item: &UpdateRelays) -> diesel::QueryResult<Self> {
        use crate::schema::relays::dsl::*;

        diesel::update(relays.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `relays`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::relays::dsl::*;

        diesel::delete(relays.filter(id.eq(param_id))).execute(db)
    }
}
#[derive(Debug, Default, Clone)]
pub struct RelaysFilter {
    pub id: Option<String>,
    pub url: Option<String>,
    pub info: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
