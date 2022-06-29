use diesel::pg::Pg;
use diesel::{deserialize, serialize};
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql, WriteTuple};
use diesel::sql_types::{Record, Text};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "nav_post")]
pub struct NavPostType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[sql_type = "NavPostType"]
pub struct NavPost(String, String);

impl ToSql<NavPostType, Pg> for NavPost {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Text)>::write_tuple(&(self.0.as_str(), self.1.as_str()), out)
    }
}

impl FromSql<NavPostType, Pg> for NavPost {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (title, slug) = FromSql::<Record<(Text, Text)>, Pg>::from_sql(bytes)?;
        Ok(NavPost(title, slug))
    }
}
