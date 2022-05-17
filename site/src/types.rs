use axum::{
    body::{Bytes, Full},
    http::{header, HeaderValue},
    response::{IntoResponse, Response},
};

/// A CSS response.
///
/// Will automatically get `Content-Type: text/css`.
#[derive(Clone, Copy, Debug)]
pub struct Css<T>(pub T);

impl<T> IntoResponse for Css<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        (
            [(header::CONTENT_TYPE, HeaderValue::from_static("text/css"))],
            self.0.into(),
        )
            .into_response()
    }
}

impl<T> From<T> for Css<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

/// A JavaScript response.
///
/// Will automatically get `Content-Type: application/javascript`.
#[derive(Clone, Copy, Debug)]
pub struct JavaScript<T>(pub T);

impl<T> IntoResponse for JavaScript<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/javascript"),
            )],
            self.0.into(),
        )
            .into_response()
    }
}

impl<T> From<T> for JavaScript<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

/// A PNG response.
///
/// Will automatically get `Content-Type: image/png`.
#[derive(Clone, Copy, Debug)]
pub struct Png<T>(pub T);

impl<T> IntoResponse for Png<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        (
            [(header::CONTENT_TYPE, HeaderValue::from_static("image/png"))],
            self.0.into(),
        )
            .into_response()
    }
}

impl<T> From<T> for Png<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

/// An icon response.
///
/// Will automatically get `Content-Type: image/x-icon`.
#[derive(Clone, Copy, Debug)]
pub struct Ico<T>(pub T);

impl<T> IntoResponse for Ico<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("image/x-icon"),
            )],
            self.0.into(),
        )
            .into_response()
    }
}

impl<T> From<T> for Ico<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
