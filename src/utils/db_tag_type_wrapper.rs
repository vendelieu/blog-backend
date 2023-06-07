use std::io::Write;

use diesel::{deserialize, serialize};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql, WriteTuple};
use diesel::sql_types::{Integer, Record, Text};

#[derive(SqlType)]
#[postgres(type_name = "tag")]
pub struct TagType;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Deserialize, Serialize)]
#[sql_type = "TagType"]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl ToSql<TagType, Pg> for Tag {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Integer, Text, Text)>::write_tuple(&(self.id, self.name.as_str(), self.slug.as_str()), out)
    }
}

impl FromSql<TagType, Pg> for Tag {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (id, name, slug) = FromSql::<Record<(Integer, Text, Text)>, Pg>::from_sql(bytes)?;
        Ok(Tag { id, name, slug })
    }
}