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

    type Headers: HttpHeaderParams;

    fn new(path: Self::Path, headers: Self::Headers) -> Self;
    // fn query(&self) -> Self::Query;

    fn path(&self) -> &Self::Path;

    fn headers(&self) -> &Self::Headers;
}

pub trait HttpPathParams: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a> {}

pub trait HttpHeaderParams: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a> {
    // fn new() -> Self;
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyPathParams {}

impl HttpPathParams for EmptyPathParams {}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyHeaderParams {}

//TODO: remove impl
// impl EmptyHeaderParams {
//     pub fn new() -> Self {
//         Self {}
//     }
//
//     pub fn new_tr() -> impl HttpHeaderParams {
//         Self {}
//     }
// }

impl HttpHeaderParams for EmptyHeaderParams {
    // fn new() -> Self {
    //     Self {}
    // }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthHeaderParams {
    pub authorization: String,
}

impl AuthHeaderParams {
    pub fn new(authorization: String) -> Self {
        Self { authorization }
    }
}

impl HttpHeaderParams for AuthHeaderParams {
    // fn new() -> Self {
    //     Self {}
    // }
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmptyRequestBody {}

// impl HttpPathParams for EmptyRequestBody {}
