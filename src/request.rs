//! RTSP request types.
//!
//! This module contains structs related to RTSP requests, notably the
//! `Request` type itself as well as a builder to create requests. Typically
//! you'll import the `RTSP::Request` type rather than reaching into this
//! module itself.
//!
//! # Examples
//!
//! Creating a `Request` to send
//!
//! ```no_run
//! use http::{Request, Response};
//!
//! let mut request = Request::builder()
//!     .uri("rtsps://www.rust-lang.org/")
//!     .header("User-Agent", "my-awesome-agent/1.0");
//!
//! if needs_awesome_header() {
//!     request = request.header("Awesome", "yes");
//! }
//!
//! let response = send(request.body(()).unwrap());
//!
//! # fn needs_awesome_header() -> bool {
//! #     true
//! # }
//! #
//! fn send(req: Request<()>) -> Response<()> {
//!     // ...
//! # panic!()
//! }
//! ```
//!
//! Inspecting a request to see what was sent.
//!
//! ```
//! use http::{Request, Response, StatusCode};
//!
//! fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
//!     if req.uri() != "/awesome-url" {
//!         return Response::builder()
//!             .status(StatusCode::NOT_FOUND)
//!             .body(())
//!     }
//!
//!     let has_awesome_header = req.headers().contains_key("Awesome");
//!     let body = req.body();
//!
//!     // ...
//! # panic!()
//! }
//! ```

use std::any::Any;
use std::convert::{TryFrom};
use std::fmt;

use crate::header::{HeaderMap, HeaderName, HeaderValue};
use crate::method::Method;
use crate::version::Version;
use crate::{Extensions, Result, Uri};

/// Represents an RTSP request.
///
/// An RTSP request consists of a head and a potentially optional body. The body
/// component is generic, enabling arbitrary types to represent the RTSP body.
/// For example, the body could be `Vec<u8>`, a `Stream` of byte chunks, or a
/// value that has been deserialized.
///
/// # Examples
///
/// Creating a `Request` to send
///
/// ```no_run
/// use http::{Request, Response};
///
/// let mut request = Request::builder()
///     .uri("rtsps://www.rust-lang.org/")
///     .header("User-Agent", "my-awesome-agent/1.0");
///
/// if needs_awesome_header() {
///     request = request.header("Awesome", "yes");
/// }
///
/// let response = send(request.body(()).unwrap());
///
/// # fn needs_awesome_header() -> bool {
/// #     true
/// # }
/// #
/// fn send(req: Request<()>) -> Response<()> {
///     // ...
/// # panic!()
/// }
/// ```
///
/// Inspecting a request to see what was sent.
///
/// ```
/// use http::{Request, Response, StatusCode};
///
/// fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
///     if req.uri() != "/awesome-url" {
///         return Response::builder()
///             .status(StatusCode::NOT_FOUND)
///             .body(())
///     }
///
///     let has_awesome_header = req.headers().contains_key("Awesome");
///     let body = req.body();
///
///     // ...
/// # panic!()
/// }
/// ```
///
/// Deserialize a request of bytes via json:
///
/// ```
/// # extern crate serde;
/// # extern crate serde_json;
/// # extern crate http;
/// use http::Request;
/// use serde::de;
///
/// fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
///     where for<'de> T: de::Deserialize<'de>,
/// {
///     let (parts, body) = req.into_parts();
///     let body = serde_json::from_slice(&body)?;
///     Ok(Request::from_parts(parts, body))
/// }
/// #
/// # fn main() {}
/// ```
///
/// Or alternatively, serialize the body of a request to json
///
/// ```
/// # extern crate serde;
/// # extern crate serde_json;
/// # extern crate http;
/// use http::Request;
/// use serde::ser;
///
/// fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
///     where T: ser::Serialize,
/// {
///     let (parts, body) = req.into_parts();
///     let body = serde_json::to_vec(&body)?;
///     Ok(Request::from_parts(parts, body))
/// }
/// #
/// # fn main() {}
/// ```
pub struct Request<T> {
    head: Parts,
    body: T,
}

/// Component parts of an RTSP `Request`
///
/// The RTSP request head consists of a method, uri, version, and a set of
/// header fields.
pub struct Parts {
    /// The request's method
    pub method: Method,

    /// The request's URI
    pub uri: Uri,

    /// The request's version
    pub version: Version,

    /// The request's headers
    pub headers: HeaderMap<HeaderValue>,

    /// The request's extensions
    pub extensions: Extensions,

    _priv: (),
}

/// An RTSP request builder
///
/// This type can be used to construct an instance or `Request`
/// through a builder-like pattern.
#[derive(Debug)]
pub struct Builder {
    inner: Result<Parts>,
}

impl Request<()> {
    /// Creates a new builder-style object to manufacture a `Request`
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request = Request::builder()
    ///     .method("GET")
    ///     .uri("rtsps://www.rust-lang.org/")
    ///     .header("X-Custom-Foo", "Bar")
    ///     .body(())
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Creates a new `Builder` initialized with a DESCRIBE method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::describe("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn describe<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,

    {
        Builder::new().method(Method::DESCRIBE).uri(uri)
    }

    /// Creates a new `Builder` initialized with a ANNOUNCE method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::announce("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn announce<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,

    {
        Builder::new().method(Method::ANNOUNCE).uri(uri)
    }

    /// Creates a new `Builder` initialized with a GET_PARAMETER method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::get_parameter("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn get_parameter<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,

    {
        Builder::new().method(Method::GET_PARAMETER).uri(uri)
    }

    /// Creates a new `Builder` initialized with an OPTIONS method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::options("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// # assert_eq!(*request.method(), Method::OPTIONS);
    /// ```
    pub fn options<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,

    {
        Builder::new().method(Method::OPTIONS).uri(uri)
    }

    /// Creates a new `Builder` initialized with a PAUSE method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::pause("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn pause<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,

    {
        Builder::new().method(Method::PAUSE).uri(uri)
    }

    /// Creates a new `Builder` initialized with a PLAY method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::play("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn play<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,

    {
        Builder::new().method(Method::PLAY).uri(uri)
    }

    /// Creates a new `Builder` initialized with a RECORD method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::record("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn record<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,

    {
        Builder::new().method(Method::RECORD).uri(uri)
    }

    /// Creates a new `Builder` initialized with a REDIRECT method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::redirect("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn redirect<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::REDIRECT).uri(uri)
    }

    /// Creates a new `Builder` initialized with a SETUP method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::setup("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn setup<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::SETUP).uri(uri)
    }

    /// Creates a new `Builder` initialized with a SET_PARAMETER method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::set_parameter("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn set_parameter<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::SET_PARAMETER).uri(uri)
    }

        /// Creates a new `Builder` initialized with a TEARDOWN method and the given URI.
    ///
    /// This method returns an instance of `Builder` which can be used to
    /// create a `Request`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::teardown("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn teardown<T>(uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        Builder::new().method(Method::TEARDOWN).uri(uri)
    }
}

impl<T> Request<T> {
    /// Creates a new blank `Request` with the body
    ///
    /// The component parts of this request will be set to their default, e.g.
    /// the GET method, no headers, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request = Request::new("hello world");
    ///
    /// assert_eq!(*request.method(), Method::DESCRIBE);
    /// assert_eq!(*request.body(), "hello world");
    /// ```
    #[inline]
    pub fn new(body: T) -> Request<T> {
        Request {
            head: Parts::new(),
            body: body,
        }
    }

    /// Creates a new `Request` with the given components parts and body.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request = Request::new("hello world");
    /// let (mut parts, body) = request.into_parts();
    /// parts.method = Method::TEARDOWN;
    ///
    /// let request = Request::from_parts(parts, body);
    /// ```
    #[inline]
    pub fn from_parts(parts: Parts, body: T) -> Request<T> {
        Request {
            head: parts,
            body: body,
        }
    }

    /// Returns a reference to the associated RTSP method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request: Request<()> = Request::default();
    /// assert_eq!(*request.method(), Method::DESCRIBE);
    /// ```
    #[inline]
    pub fn method(&self) -> &Method {
        &self.head.method
    }

    /// Returns a mutable reference to the associated RTSP method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let mut request: Request<()> = Request::default();
    /// *request.method_mut() = Method::SET_PARAMETER;
    /// assert_eq!(*request.method(), Method::SET_PARAMETER);
    /// ```
    #[inline]
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.head.method
    }

    /// Returns a reference to the associated URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request: Request<()> = Request::default();
    /// assert_eq!(*request.uri(), *"/");
    /// ```
    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.head.uri
    }

    /// Returns a mutable reference to the associated URI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let mut request: Request<()> = Request::default();
    /// *request.uri_mut() = "/hello".parse().unwrap();
    /// assert_eq!(*request.uri(), *"/hello");
    /// ```
    #[inline]
    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.head.uri
    }

    /// Returns the associated version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request: Request<()> = Request::default();
    /// assert_eq!(request.version(), Version::RTSP_10);
    /// ```
    #[inline]
    pub fn version(&self) -> Version {
        self.head.version
    }

    /// Returns a mutable reference to the associated version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let mut request: Request<()> = Request::default();
    /// *request.version_mut() = Version::RTSP_20;
    /// assert_eq!(request.version(), Version::RTSP_20);
    /// ```
    #[inline]
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.head.version
    }

    /// Returns a reference to the associated header field map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request: Request<()> = Request::default();
    /// assert!(request.headers().is_empty());
    /// ```
    #[inline]
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.head.headers
    }

    /// Returns a mutable reference to the associated header field map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// # use http::header::*;
    /// let mut request: Request<()> = Request::default();
    /// request.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    /// assert!(!request.headers().is_empty());
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.head.headers
    }

    /// Returns a reference to the associated extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request: Request<()> = Request::default();
    /// assert!(request.extensions().get::<i32>().is_none());
    /// ```
    #[inline]
    pub fn extensions(&self) -> &Extensions {
        &self.head.extensions
    }

    /// Returns a mutable reference to the associated extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// # use http::header::*;
    /// let mut request: Request<()> = Request::default();
    /// request.extensions_mut().insert("hello");
    /// assert_eq!(request.extensions().get(), Some(&"hello"));
    /// ```
    #[inline]
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.head.extensions
    }

    /// Returns a reference to the associated RTSP body.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request: Request<String> = Request::default();
    /// assert!(request.body().is_empty());
    /// ```
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Returns a mutable reference to the associated RTSP body.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let mut request: Request<String> = Request::default();
    /// request.body_mut().push_str("hello world");
    /// assert!(!request.body().is_empty());
    /// ```
    #[inline]
    pub fn body_mut(&mut self) -> &mut T {
        &mut self.body
    }

    /// Consumes the request, returning just the body.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::Request;
    /// let request = Request::new(10);
    /// let body = request.into_body();
    /// assert_eq!(body, 10);
    /// ```
    #[inline]
    pub fn into_body(self) -> T {
        self.body
    }

    /// Consumes the request returning the head and body parts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request = Request::new(());
    /// let (parts, body) = request.into_parts();
    /// assert_eq!(parts.method, Method::DESCRIBE);
    /// ```
    #[inline]
    pub fn into_parts(self) -> (Parts, T) {
        (self.head, self.body)
    }

    /// Consumes the request returning a new request with body mapped to the
    /// return type of the passed in function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// let request = Request::builder().body("some string").unwrap();
    /// let mapped_request: Request<&[u8]> = request.map(|b| {
    ///   assert_eq!(b, "some string");
    ///   b.as_bytes()
    /// });
    /// assert_eq!(mapped_request.body(), &"some string".as_bytes());
    /// ```
    #[inline]
    pub fn map<F, U>(self, f: F) -> Request<U>
    where
        F: FnOnce(T) -> U,
    {
        Request {
            body: f(self.body),
            head: self.head,
        }
    }
}

impl<T: Default> Default for Request<T> {
    fn default() -> Request<T> {
        Request::new(T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for Request<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("method", self.method())
            .field("uri", self.uri())
            .field("version", &self.version())
            .field("headers", self.headers())
            // omits Extensions because not useful
            .field("body", self.body())
            .finish()
    }
}

impl Parts {
    /// Creates a new default instance of `Parts`
    fn new() -> Parts {
        Parts {
            method: Method::default(),
            uri: Uri::default(),
            version: Version::default(),
            headers: HeaderMap::default(),
            extensions: Extensions::default(),
            _priv: (),
        }
    }
}

impl fmt::Debug for Parts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parts")
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("version", &self.version)
            .field("headers", &self.headers)
            // omits Extensions because not useful
            // omits _priv because not useful
            .finish()
    }
}

impl Builder {
    /// Creates a new default instance of `Builder` to construct a `Request`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let req = request::Builder::new()
    ///     .method("DESCRIBE")
    ///     .body(())
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }

    /// Set the RTSP method for this request.
    ///
    /// This function will configure the RTSP method of the `Request` that will
    /// be returned from `Builder::build`.
    ///
    /// By default this is `GET`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let req = Request::builder()
    ///     .method("DESCRIBE")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn method<T>(self, method: T) -> Builder
    where
        Method: TryFrom<T>,
        <Method as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let method = TryFrom::try_from(method).map_err(Into::into)?;
            head.method = method;
            Ok(head)
        })
    }

    /// Get the RTSP Method for this request.
    ///
    /// By default this is `GET`. If builder has error, returns None.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let mut req = Request::builder();
    /// assert_eq!(req.method_ref(),Some(&Method::DESCRIBE));
    ///
    /// req = req.method("OPTIONS");
    /// assert_eq!(req.method_ref(),Some(&Method::OPTIONS));
    /// ```
    pub fn method_ref(&self) -> Option<&Method> {
        self.inner.as_ref().ok().map(|h| &h.method)
    }

    /// Set the URI for this request.
    ///
    /// This function will configure the URI of the `Request` that will
    /// be returned from `Builder::build`.
    ///
    /// By default this is `/`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let req = Request::builder()
    ///     .uri("rtsps://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn uri<T>(self, uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            head.uri = TryFrom::try_from(uri).map_err(Into::into)?;
            Ok(head)
        })
    }

    /// Get the URI for this request
    ///
    /// By default this is `/`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let mut req = Request::builder();
    /// assert_eq!(req.uri_ref().unwrap(), "/" );
    ///
    /// req = req.uri("rtsps://www.rust-lang.org/");
    /// assert_eq!(req.uri_ref().unwrap(), "rtsps://www.rust-lang.org/" );
    /// ```
    pub fn uri_ref(&self) -> Option<&Uri> {
        self.inner.as_ref().ok().map(|h| &h.uri)
    }

    /// Set the RTSP version for this request.
    ///
    /// This function will configure the RTSP version of the `Request` that
    /// will be returned from `Builder::build`.
    ///
    /// By default this is RTSP/1.1
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let req = Request::builder()
    ///     .version(Version::RTSP_20)
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn version(self, version: Version) -> Builder {
        self.and_then(move |mut head| {
            head.version = version;
            Ok(head)
        })
    }

    /// Appends a header to this request builder.
    ///
    /// This function will append the provided key/value as a header to the
    /// internal `HeaderMap` being constructed. Essentially this is equivalent
    /// to calling `HeaderMap::append`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    /// # use http::header::HeaderValue;
    ///
    /// let req = Request::builder()
    ///     .header("Accept", "text/html")
    ///     .header("X-Custom-Foo", "bar")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn header<K, V>(self, key: K, value: V) -> Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<crate::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let name = <HeaderName as TryFrom<K>>::try_from(key).map_err(Into::into)?;
            let value = <HeaderValue as TryFrom<V>>::try_from(value).map_err(Into::into)?;
            head.headers.append(name, value);
            Ok(head)
        })
    }

    /// Get header on this request builder.
    /// when builder has error returns None
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Request;
    /// let req = Request::builder()
    ///     .header("Accept", "text/html")
    ///     .header("X-Custom-Foo", "bar");
    /// let headers = req.headers_ref().unwrap();
    /// assert_eq!( headers["Accept"], "text/html" );
    /// assert_eq!( headers["X-Custom-Foo"], "bar" );
    /// ```
    pub fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {
        self.inner.as_ref().ok().map(|h| &h.headers)
    }

    /// Get headers on this request builder.
    ///
    /// When builder has error returns None.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::{header::HeaderValue, Request};
    /// let mut req = Request::builder();
    /// {
    ///   let headers = req.headers_mut().unwrap();
    ///   headers.insert("Accept", HeaderValue::from_static("text/html"));
    ///   headers.insert("X-Custom-Foo", HeaderValue::from_static("bar"));
    /// }
    /// let headers = req.headers_ref().unwrap();
    /// assert_eq!( headers["Accept"], "text/html" );
    /// assert_eq!( headers["X-Custom-Foo"], "bar" );
    /// ```
    pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
        self.inner.as_mut().ok().map(|h| &mut h.headers)
    }

    /// Adds an extension to this builder
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let req = Request::builder()
    ///     .extension("My Extension")
    ///     .body(())
    ///     .unwrap();
    ///
    /// assert_eq!(req.extensions().get::<&'static str>(),
    ///            Some(&"My Extension"));
    /// ```
    pub fn extension<T>(self, extension: T) -> Builder
    where
        T: Any + Send + Sync + 'static,
    {
        self.and_then(move |mut head| {
            head.extensions.insert(extension);
            Ok(head)
        })
    }

    /// Get a reference to the extensions for this request builder.
    ///
    /// If the builder has an error, this returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Request;
    /// let req = Request::builder().extension("My Extension").extension(5u32);
    /// let extensions = req.extensions_ref().unwrap();
    /// assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    /// assert_eq!(extensions.get::<u32>(), Some(&5u32));
    /// ```
    pub fn extensions_ref(&self) -> Option<&Extensions> {
        self.inner.as_ref().ok().map(|h| &h.extensions)
    }

    /// Get a mutable reference to the extensions for this request builder.
    ///
    /// If the builder has an error, this returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::Request;
    /// let mut req = Request::builder().extension("My Extension");
    /// let mut extensions = req.extensions_mut().unwrap();
    /// assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    /// extensions.insert(5u32);
    /// assert_eq!(extensions.get::<u32>(), Some(&5u32));
    /// ```
    pub fn extensions_mut(&mut self) -> Option<&mut Extensions> {
        self.inner.as_mut().ok().map(|h| &mut h.extensions)
    }

    /// "Consumes" this builder, using the provided `body` to return a
    /// constructed `Request`.
    ///
    /// # Errors
    ///
    /// This function may return an error if any previously configured argument
    /// failed to parse or get converted to the internal representation. For
    /// example if an invalid `head` was specified via `header("Foo",
    /// "Bar\r\n")` the error will be returned when this function is called
    /// rather than when `header` was called.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::*;
    ///
    /// let request = Request::builder()
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn body<T>(self, body: T) -> Result<Request<T>> {
        self.inner.map(move |head| {
            Request {
                head,
                body,
            }
        })
    }

    // private

    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts>
    {
        Builder {
            inner: self.inner.and_then(func),
        }
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Builder {
        Builder {
            inner: Ok(Parts::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_map_a_body_from_one_type_to_another() {
        let request = Request::builder().body("some string").unwrap();
        let mapped_request = request.map(|s| {
            assert_eq!(s, "some string");
            123u32
        });
        assert_eq!(mapped_request.body(), &123u32);
    }
}
