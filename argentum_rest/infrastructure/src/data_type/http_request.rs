use crate::data_type::DeserializableSchemaRaw;
use serde::Deserialize;
use serde_valid::json::FromJsonSlice;
use serde_valid::Validate;

pub trait HttpRequest {
    type Body: for<'a> DeserializableSchemaRaw<'a>;

    type Params: HttpParams;

    fn new(body: Self::Body, params: Self::Params) -> Self;

    fn body(&self) -> &Self::Body;

    fn params(&self) -> &Self::Params;
}

pub trait HttpParams {
    type Headers: HttpHeaderParams;

    type Path: HttpPathParams;

    type Query: HttpQueryParams;

    fn new(path: Self::Path, query: Self::Query, headers: Self::Headers) -> Self;

    fn query(&self) -> &Self::Query;

    fn path(&self) -> &Self::Path;

    fn headers(&self) -> &Self::Headers;
}

pub trait HttpPathParams: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a> {}

//TODO: think about alternative for serialize and deserialize
pub trait HttpQueryParams: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a> {}

pub trait HttpHeaderParams: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a> {}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyPathParams {}

impl HttpPathParams for EmptyPathParams {}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyQueryParams {}

impl HttpQueryParams for EmptyQueryParams {}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyHeaderParams {}

impl HttpHeaderParams for EmptyHeaderParams {}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthHeaderParams {
    pub authorization: String,
}

impl AuthHeaderParams {
    pub fn new(authorization: String) -> Self {
        Self { authorization }
    }
}

impl HttpHeaderParams for AuthHeaderParams {}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyRequestBody {}
