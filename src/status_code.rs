use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
/// HTTP status codes as defined in the HTTP/1.1 and WebDAV specifications.
///
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
pub enum StatusCode {
    // --- Informational responses (100–199) ---
    /// 100 Continue: Indicates that the client should continue the request.
    Continue,
    /// 101 Switching Protocols: Sent in response to an Upgrade header to indicate protocol switch.
    SwitchingProtocols,
    /// 102 Processing (WebDAV): The server has received and is processing the request.
    Processing,
    /// 103 Early Hints: Allows preloading resources while the server prepares a response.
    EarlyHints,

    // --- Successful responses (200–299) ---
    /// 200 OK: The request succeeded.
    Ok,
    /// 201 Created: A new resource was successfully created.
    Created,
    /// 202 Accepted: The request was accepted but not yet processed.
    Accepted,
    /// 203 Non-Authoritative Information: Metadata from a third-party source.
    NonAuthoritativeInformation,
    /// 204 No Content: No content to send, but headers may be useful.
    NoContent,
    /// 205 Reset Content: Tells the client to reset the document view.
    ResetContent,
    /// 206 Partial Content: Partial resource delivery (used with Range headers).
    PartialContent,
    /// 207 Multi-Status (WebDAV): Provides multiple status codes for different resources.
    MultiStatus,
    /// 208 Already Reported (WebDAV): Avoids repeated enumeration of internal members.
    AlreadyReported,
    /// 226 IM Used: Result of instance-manipulations applied to the resource.
    IMUsed,

    // --- Redirection messages (300–399) ---
    /// 300 Multiple Choices: Multiple options for the resource are available.
    MultipleChoices,
    /// 301 Moved Permanently: Resource permanently moved to a new URI.
    MovedPermanently,
    /// 302 Found: Temporary redirection to another URI.
    Found,
    /// 303 See Other: Redirect to another URI using a GET request.
    SeeOther,
    /// 304 Not Modified: Cached version is still valid.
    NotModified,
    /// 305 Use Proxy (Deprecated): Resource must be accessed via a proxy.
    UseProxyDeprecated,
    /// 307 Temporary Redirect: Same method must be used for redirected request.
    TemporaryRedirect,
    /// 308 Permanent Redirect: Same as 301, but method must not change.
    PermanentRedirect,

    // --- Client error responses (400–499) ---
    /// 400 Bad Request: Malformed request syntax or invalid parameters.
    BadRequest,
    /// 401 Unauthorized: Authentication is required.
    Unauthorized,
    /// 402 Payment Required (Experimental): Reserved for future use.
    PaymentRequiredExperimental,
    /// 403 Forbidden: Client is authenticated but not authorized.
    Forbidden,
    /// 404 Not Found: The resource could not be found.
    NotFound,
    /// 405 Method Not Allowed: Method is not allowed on the target resource.
    MethodNotAllowed,
    /// 406 Not Acceptable: No acceptable representation found for the request.
    NotAcceptable,
    /// 407 Proxy Authentication Required: Authentication with proxy required.
    ProxyAuthenticationRequired,
    /// 408 Request Timeout: The server timed out waiting for the request.
    RequestTimeout,
    /// 409 Conflict: Request conflicts with current server state.
    Conflict,
    /// 410 Gone: Resource has been permanently removed.
    Gone,
    /// 411 Length Required: Content-Length header is missing.
    LengthRequired,
    /// 412 Precondition Failed: Server does not meet request preconditions.
    PreconditionFailed,
    /// 413 Payload Too Large: Request entity is too large.
    PayloadTooLarge,
    /// 414 URI Too Long: Request URI is too long for the server to process.
    URITooLong,
    /// 415 Unsupported Media Type: Media type is not supported.
    UnsupportedMediaType,
    /// 416 Range Not Satisfiable: Requested range cannot be fulfilled.
    RangeNotSatisfiable,
    /// 417 Expectation Failed: Expect header cannot be fulfilled.
    ExpectationFailed,
    /// 421 Misdirected Request: Server cannot produce a response.
    MisdirectedRequest,
    /// 422 Unprocessable Content (WebDAV): Well-formed but semantically invalid request.
    UnprocessableContent,
    /// 423 Locked (WebDAV): Resource is currently locked.
    Locked,
    /// 424 Failed Dependency (WebDAV): Previous request failed.
    FailedDependency,
    /// 425 Too Early (Experimental): Request might be replayed.
    TooEarlyExperimental,
    /// 426 Upgrade Required: Client must switch protocols.
    UpgradeRequired,
    /// 428 Precondition Required: Request must be conditional to prevent conflicts.
    PreconditionRequired,
    /// 429 Too Many Requests: Rate limit exceeded.
    TooManyRequests,
    /// 431 Request Header Fields Too Large: Header fields too large for the server to process.
    RequestHeaderFieldsTooLarge,
    /// 451 Unavailable For Legal Reasons: Resource is unavailable due to legal demands.
    UnavailableForLegalReasons,

    // --- Server error responses (500–599) ---
    /// 500 Internal Server Error: Unexpected server error.
    InternalServerError,
    /// 501 Not Implemented: Server doesn't support the request method.
    NotImplemented,
    /// 502 Bad Gateway: Invalid response from upstream server.
    BadGateway,
    /// 503 Service Unavailable: Server is down or overloaded.
    ServiceUnavailable,
    /// 504 Gateway Timeout: Timeout from upstream server.
    GatewayTimeout,
    /// 505 HTTP Version Not Supported: HTTP version not supported by server.
    HTTPVersionNotSupported,
    /// 506 Variant Also Negotiates: Transparent content negotiation error.
    VariantAlsoNegotiates,
    /// 507 Insufficient Storage (WebDAV): Server cannot store the representation.
    InsufficientStorage,
    /// 508 Loop Detected (WebDAV): Infinite loop detected while processing.
    LoopDetected,
    /// 510 Not Extended: Additional extensions required to fulfill the request.
    NotExtended,
    /// 511 Network Authentication Required: Client must authenticate to access the network.
    NetworkAuthenticationRequired,

    Other(u16),
}

impl StatusCode {
    pub fn as_tuple(&self) -> (usize, &str) {
        return match self {
            // Information responses
            StatusCode::Continue => (100, "Continue"),
            StatusCode::SwitchingProtocols => (101, "SwitchingProtocols"),
            StatusCode::Processing => (102, "Processing"),
            StatusCode::EarlyHints => (103, "EarlyHints"),

            // Successful responses
            StatusCode::Ok => (200, "Ok"),
            StatusCode::Created => (201, "Created"),
            StatusCode::Accepted => (202, "Accepted"),
            StatusCode::NonAuthoritativeInformation => (203, "NonAuthoritativeInformation"),
            StatusCode::NoContent => (204, "NoContent"),
            StatusCode::ResetContent => (205, "ResetContent"),
            StatusCode::PartialContent => (206, "PartialContent"),
            StatusCode::MultiStatus => (207, "MultiStatus"),
            StatusCode::AlreadyReported => (208, "AlreadyReported"),
            StatusCode::IMUsed => (226, "IMUsed"),

            // Redirection messages
            StatusCode::MultipleChoices => (300, "MultipleChoices"),
            StatusCode::MovedPermanently => (301, "MovedPermanently"),
            StatusCode::Found => (302, "Found"),
            StatusCode::SeeOther => (303, "SeeOther"),
            StatusCode::NotModified => (304, "NotModified"),
            StatusCode::UseProxyDeprecated => (305, "UseProxyDeprecated"),
            StatusCode::TemporaryRedirect => (307, "TemporaryRedirect"),
            StatusCode::PermanentRedirect => (308, "PermanentRedirect"),

            // Client error responses
            StatusCode::BadRequest => (400, "BadRequest"),
            StatusCode::Unauthorized => (401, "Unauthorized"),
            StatusCode::PaymentRequiredExperimental => (402, "PaymentRequiredExperimental"),
            StatusCode::Forbidden => (403, "Forbidden"),
            StatusCode::NotFound => (404, "NotFound"),
            StatusCode::MethodNotAllowed => (405, "MethodNotAllowed"),
            StatusCode::NotAcceptable => (406, "NotAcceptable"),
            StatusCode::ProxyAuthenticationRequired => (407, "ProxyAuthenticationRequired"),
            StatusCode::RequestTimeout => (408, "RequestTimeout"),
            StatusCode::Conflict => (409, "Conflict"),
            StatusCode::Gone => (410, "Gone"),
            StatusCode::LengthRequired => (411, "LengthRequired"),
            StatusCode::PreconditionFailed => (412, "PreconditionFailed"),
            StatusCode::PayloadTooLarge => (413, "PayloadTooLarge"),
            StatusCode::URITooLong => (414, "URITooLong"),
            StatusCode::UnsupportedMediaType => (415, "UnsupportedMediaType"),
            StatusCode::RangeNotSatisfiable => (416, "RangeNotSatisfiable"),
            StatusCode::ExpectationFailed => (417, "ExpectationFailed"),
            StatusCode::MisdirectedRequest => (421, "MisdirectedRequest"),
            StatusCode::UnprocessableContent => (422, "UnprocessableContent"),
            StatusCode::Locked => (423, "Locked"),
            StatusCode::FailedDependency => (424, "FailedDependency"),
            StatusCode::TooEarlyExperimental => (425, "TooEarlyExperimental"),
            StatusCode::UpgradeRequired => (426, "UpgradeRequired"),
            StatusCode::PreconditionRequired => (428, "PreconditionRequired"),
            StatusCode::TooManyRequests => (429, "TooManyRequests"),
            StatusCode::RequestHeaderFieldsTooLarge => (431, "RequestHeaderFieldsTooLarge"),
            StatusCode::UnavailableForLegalReasons => (451, "UnavailableForLegalReasons"),

            // Server error responses
            StatusCode::InternalServerError => (500, "InternalServerError"),
            StatusCode::NotImplemented => (501, "NotImplemented"),
            StatusCode::BadGateway => (502, "BadGateway"),
            StatusCode::ServiceUnavailable => (503, "ServiceUnavailable"),
            StatusCode::GatewayTimeout => (504, "GatewayTimeout"),
            StatusCode::HTTPVersionNotSupported => (505, "HTTPVersionNotSupported"),
            StatusCode::VariantAlsoNegotiates => (506, "VariantAlsoNegotiates"),
            StatusCode::InsufficientStorage => (507, "InsufficientStorage"),
            StatusCode::LoopDetected => (508, "LoopDetected"),
            StatusCode::NotExtended => (510, "NotExtended"),
            StatusCode::NetworkAuthenticationRequired => (511, "NetworkAuthenticationRequired"),

            StatusCode::Other(code) => (code.clone().into(), "Unknown"),
        };
    }

    pub fn message(&self) -> &str {
        self.as_tuple().1
    }

    pub fn code(&self) -> usize {
        self.as_tuple().0
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (code, status) = self.as_tuple();
        write!(f, "{code} {status}")
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        match code {
            100 => StatusCode::Continue,
            101 => StatusCode::SwitchingProtocols,
            102 => StatusCode::Processing,
            103 => StatusCode::EarlyHints,

            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            203 => StatusCode::NonAuthoritativeInformation,
            204 => StatusCode::NoContent,
            205 => StatusCode::ResetContent,
            206 => StatusCode::PartialContent,
            207 => StatusCode::MultiStatus,
            208 => StatusCode::AlreadyReported,
            226 => StatusCode::IMUsed,

            300 => StatusCode::MultipleChoices,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::Found,
            303 => StatusCode::SeeOther,
            304 => StatusCode::NotModified,
            305 => StatusCode::UseProxyDeprecated,
            307 => StatusCode::TemporaryRedirect,
            308 => StatusCode::PermanentRedirect,

            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            402 => StatusCode::PaymentRequiredExperimental,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            406 => StatusCode::NotAcceptable,
            407 => StatusCode::ProxyAuthenticationRequired,
            408 => StatusCode::RequestTimeout,
            409 => StatusCode::Conflict,
            410 => StatusCode::Gone,
            411 => StatusCode::LengthRequired,
            412 => StatusCode::PreconditionFailed,
            413 => StatusCode::PayloadTooLarge,
            414 => StatusCode::URITooLong,
            415 => StatusCode::UnsupportedMediaType,
            416 => StatusCode::RangeNotSatisfiable,
            417 => StatusCode::ExpectationFailed,
            421 => StatusCode::MisdirectedRequest,
            422 => StatusCode::UnprocessableContent,
            423 => StatusCode::Locked,
            424 => StatusCode::FailedDependency,
            425 => StatusCode::TooEarlyExperimental,
            426 => StatusCode::UpgradeRequired,
            428 => StatusCode::PreconditionRequired,
            429 => StatusCode::TooManyRequests,
            431 => StatusCode::RequestHeaderFieldsTooLarge,
            451 => StatusCode::UnavailableForLegalReasons,

            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HTTPVersionNotSupported,
            506 => StatusCode::VariantAlsoNegotiates,
            507 => StatusCode::InsufficientStorage,
            508 => StatusCode::LoopDetected,
            510 => StatusCode::NotExtended,
            511 => StatusCode::NetworkAuthenticationRequired,

            code => StatusCode::Other(code),
        }
    }
}

impl TryFrom<&str> for StatusCode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let code = value.parse::<u16>().or(Err(value.to_string()))?;
        let status: Self = code.into();
        if matches!(status, StatusCode::Other(_)) {
            return Err(value.to_string());
        }
        Ok(status)
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Ok
    }
}

impl std::error::Error for StatusCode {}
