use std::io::Write;

use diesel::{deserialize, serialize};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql, WriteTuple};
use diesel::sql_types::{Record, Text};

#[derive(SqlType)]
#[postgres(type_name = "nav_post")]
pub struct NavPostType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize, QueryableByName)]
#[sql_type = "NavPostType"]
pub struct NavPost {
    #[sql_type = "Text"]
    pub title: String,
    #[sql_type = "Text"]
    pub slug: String,
}

impl ToSql<NavPostType, Pg> for NavPost {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Text)>::write_tuple(&(self.title.as_str(), self.slug.as_str()), out)
    }
}

impl FromSql<NavPostType, Pg> for NavPost {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (title, slug) = FromSql::<Record<(Text, Text)>, Pg>::from_sql(bytes)?;
        Ok(NavPost { title, slug })
    }
}
