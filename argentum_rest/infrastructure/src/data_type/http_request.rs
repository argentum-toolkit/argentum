use serde::Deserialize;
use serde_valid::json::FromJsonSlice;
use serde_valid::Validate;

pub trait HttpRequest {
    type Body: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a>;
    type Params: HttpParams;

    fn new(body: Self::Body, params: Self::Params) -> Self;

    fn body(&self) -> &Self::Body;

    fn params(&self) -> &Self::Params;
}

pub trait HttpParams {
    //TODO: implement Query
    // type Query;
    type Path: HttpPathParams;

    fn new(path: Self::Path) -> Self;
    // fn query(&self) -> Self::Query;

    fn path(&self) -> &Self::Path;
}

pub trait HttpPathParams: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a> {}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyPathParams {}

impl HttpPathParams for EmptyPathParams {}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyRequestBody {}

impl HttpPathParams for EmptyRequestBody {}
