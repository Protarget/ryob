// A type-safe newtype wrapper around a u64 id

use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::serialize::ToSql;
use diesel::sql_types::BigInt;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::marker::PhantomData;

#[derive(PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id<T>(i64, #[serde(skip)] PhantomData<T>);

impl<T> Id<T> {
    pub fn new(id: i64) -> Id<T> {
        Id(id, PhantomData)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id::new(self.0)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "<ID: {}>", self.0)
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "<ID: {}>", self.0)
    }
}

impl<T> AsExpression<BigInt> for Id<T>
where
    i64: AsExpression<BigInt>,
{
    type Expression = <i64 as AsExpression<BigInt>>::Expression;
    fn as_expression(self) -> Self::Expression {
        <i64 as AsExpression<BigInt>>::as_expression(self.0)
    }
}

impl<T> AsExpression<BigInt> for &Id<T>
where
    i64: AsExpression<BigInt>,
{
    type Expression = <i64 as AsExpression<BigInt>>::Expression;
    fn as_expression(self) -> Self::Expression {
        <i64 as AsExpression<BigInt>>::as_expression(self.0)
    }
}

impl<T, DB> ToSql<BigInt, DB> for Id<T>
where
    DB: diesel::backend::Backend,
    i64: ToSql<BigInt, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut diesel::serialize::Output<W, DB>) -> diesel::serialize::Result {
        (self.0).to_sql(out)
    }
}

impl<T, DB> FromSqlRow<BigInt, DB> for Id<T>
where
    DB: diesel::backend::Backend,
    i64: FromSqlRow<BigInt, DB>,
{
    fn build_from_row<R: diesel::row::Row<DB>>(row: &mut R) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Id::new(i64::build_from_row(row)?))
    }
}

impl<T, DB> Queryable<BigInt, DB> for Id<T>
where
    DB: diesel::backend::Backend,
    i64: FromSqlRow<BigInt, DB>,
{
    type Row = Self;
    fn build(row: Self) -> Self {
        row
    }
}
