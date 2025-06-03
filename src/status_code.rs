use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct StatusCode {
    pub code: u16,
    pub phrase: &'static str,
}

impl StatusCode {
    pub const fn new(code: u16, phrase: &'static str) -> Self {
        StatusCode { code, phrase }
    }

    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    pub fn phrase(mut self, phrase: &'static str) -> Self {
        self.phrase = phrase;
        self
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code, self.phrase)
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::OK
    }
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
            $(
                $(#[$docs])*
                pub const $konst: StatusCode = StatusCode::new($num, $phrase);
            )+
        }

        impl From<u16> for StatusCode {
            fn from(code: u16) -> Self {
                match code {
                    $(
                        $num => StatusCode::$konst,
                    )+
                    _ => StatusCode::new(code, "Other"),
                }
            }
        }
    };
}

status_codes! {
    /// **100 Continue**: Indicates that the client should continue the request.
    (100, CONTINUE, "Continue");
    /// **101 Switching Protocols**: Sent in response to an Upgrade header to indicate protocol switch.
    (101, SWITCHING_PROTOCOLS, "Switching Protocols");
    /// **102 Processing (WebDAV)**: The server has received and is processing the request.
    (102, PROCESSING, "Processing");
    /// **103 Early Hints**: Allows preloading resources while the server prepares a response.
    (103, EARLY_HINTS, "Early Hints");

    /// **200 OK**: The request succeeded.
    (200, OK, "OK");
    /// **201 Created**: A new resource was successfully created.
    (201, CREATED, "Created");
    /// **202 Accepted**: The request was accepted but not yet processed.
    (202, ACCEPTED, "Accepted");
    /// **203 Non-Authoritative Information**: Metadata from a third-party source.
    (203, NON_AUTHORITATIVE_INFORMATION, "Non-Authoritative Information");
    /// **204 No Content**: No content to send, but headers may be useful.
    (204, NO_CONTENT, "No Content");
    /// **205 Reset Content**: Tells the client to reset the document view.
    (205, RESET_CONTENT, "Reset Content");
    /// **206 Partial Content**: Partial resource delivery (used with Range headers).
    (206, PARTIAL_CONTENT, "Partial Content");
    /// **207 Multi-Status (WebDAV)**: Provides multiple status codes for different resources.
    (207, MULTI_STATUS, "Multi-Status");
    /// **208 Already Reported (WebDAV)**: Avoids repeated enumeration of internal members.
    (208, ALREADY_REPORTED, "Already Reported");
    /// **226 IM Used**: Result of instance-manipulations applied to the resource.
    (226, IM_USED, "IM Used");

    /// **300 Multiple Choices**: Multiple options for the resource are available.
    (300, MULTIPLE_CHOICES, "Multiple Choices");
    /// **301 Moved Permanently**: Resource permanently moved to a new URI.
    (301, MOVED_PERMANENTLY, "Moved Permanently");
    /// **302 Found**: Temporary redirection to another URI.
    (302, FOUND, "Found");
    /// **303 See Other**: Redirect to another URI using a GET request.
    (303, SEE_OTHER, "See Other");
    /// **304 Not Modified**: Cached version is still valid.
    (304, NOT_MODIFIED, "Not Modified");
    /// **305 Use Proxy (Deprecated)**: Resource must be accessed via a proxy.
    (305, USE_PROXY_DEPRECATED, "Use Proxy (Deprecated)");
    /// **307 Temporary Redirect**: Same method must be used for redirected request.
    (307, TEMPORARY_REDIRECT, "Temporary Redirect");
    /// **308 Permanent Redirect**: Same as 301, but method must not change.
    (308, PERMANENT_REDIRECT, "Permanent Redirect");

    /// **400 Bad Request**: Malformed request syntax or invalid parameters.
    (400, BAD_REQUEST, "Bad Request");
    /// **401 Unauthorized**: Authentication is required.
    (401, UNAUTHORIZED, "Unauthorized");
    /// **402 Payment Required (Experimental)**: Reserved for future use.
    (402, PAYMENT_REQUIRED_EXPERIMENTAL, "Payment Required (Experimental)");
    /// **403 Forbidden**: Client is authenticated but not authorized.
    (403, FORBIDDEN, "Forbidden");
    /// **404 Not Found**: The resource could not be found.
    (404, NOT_FOUND, "Not Found");
    /// **405 Method Not Allowed**: Method is not allowed on the target resource.
    (405, METHOD_NOT_ALLOWED, "Method Not Allowed");
    /// **406 Not Acceptable**: No acceptable representation found for the request.
    (406, NOT_ACCEPTABLE, "Not Acceptable");
    /// **407 Proxy Authentication Required**: Authentication with proxy required.
    (407, PROXY_AUTHENTICATION_REQUIRED, "Proxy Authentication Required");
    /// **408 Request Timeout**: The server timed out waiting for the request.
    (408, REQUEST_TIMEOUT, "Request Timeout");
    /// **409 Conflict**: Request conflicts with current server state.
    (409, CONFLICT, "Conflict");
    /// **410 Gone**: Resource has been permanently removed.
    (410, GONE, "Gone");
    /// **411 Length Required**: Content-Length header is missing.
    (411, LENGTH_REQUIRED, "Length Required");
    /// **412 Precondition Failed**: Server does not meet request preconditions.
    (412, PRECONDITION_FAILED, "Precondition Failed");
    /// **413 Payload Too Large**: Request entity is too large.
    (413, PAYLOAD_TOO_LARGE, "Payload Too Large");
    /// **414 URI Too Long**: Request URI is too long for the server to process.
    (414, URI_TOO_LONG, "URI Too Long");
    /// **415 Unsupported Media Type**: Media type is not supported.
    (415, UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type");
    /// **416 Range Not Satisfiable**: Requested range cannot be fulfilled.
    (416, RANGE_NOT_SATISFIABLE, "Range Not Satisfiable");
    /// **417 Expectation Failed**: Expect header cannot be fulfilled.
    (417, EXPECTATION_FAILED, "Expectation Failed");
    /// **421 Misdirected Request**: Server cannot produce a response.
    (421, MISDIRECTED_REQUEST, "Misdirected Request");
    /// **422 Unprocessable Content (WebDAV)**: Well-formed but semantically invalid request.
    (422, UNPROCESSABLE_CONTENT, "Unprocessable Content");
    /// **423 Locked (WebDAV)**: Resource is currently locked.
    (423, LOCKED, "Locked");
    /// **424 Failed Dependency (WebDAV)**: Previous request failed.
    (424, FAILED_DEPENDENCY, "Failed Dependency");
    /// **425 Too Early (Experimental)**: Request might be replayed.
    (425, TOO_EARLY_EXPERIMENTAL, "Too Early (Experimental)");
    /// **426 Upgrade Required**: Client must switch protocols.
    (426, UPGRADE_REQUIRED, "Upgrade Required");
    /// **428 Precondition Required**: Request must be conditional to prevent conflicts.
    (428, PRECONDITION_REQUIRED, "Precondition Required");
    /// **429 Too Many Requests**: Rate limit exceeded.
    (429, TOO_MANY_REQUESTS, "Too Many Requests");
    /// **431 Request Header Fields Too Large**: Header fields too large for the server to process.
    (431, REQUEST_HEADER_FIELDS_TOO_LARGE, "Request Header Fields Too Large");
    /// **451 Unavailable For Legal Reasons**: Resource is unavailable due to legal demands.
    (451, UNAVAILABLE_FOR_LEGAL_REASONS, "Unavailable For Legal Reasons");
    
    /// **500 Internal Server Error**: Unexpected server error.
    (500, INTERNAL_SERVER_ERROR, "Internal Server Error");
    /// **501 Not Implemented**: Server doesn't support the request method.
    (501, NOT_IMPLEMENTED, "Not Implemented");
    /// **502 Bad Gateway**: Invalid response from upstream server.
    (502, BAD_GATEWAY, "Bad Gateway");
    /// **503 Service Unavailable**: Server is down or overloaded.
    (503, SERVICE_UNAVAILABLE, "Service Unavailable");
    /// **504 Gateway Timeout**: Timeout from upstream server.
    (504, GATEWAY_TIMEOUT, "Gateway Timeout");
    /// **505 HTTP Version Not Supported**: HTTP version not supported by server.
    (505, HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported");
    /// **506 Variant Also Negotiates**: Transparent content negotiation error.
    (506, VARIANT_ALSO_NEGOTIATES, "Variant Also Negotiates");
    /// **507 Insufficient Storage (WebDAV)**: Server cannot store the representation.
    (507, INSUFFICIENT_STORAGE, "Insufficient Storage");
    /// **508 Loop Detected (WebDAV)**: Infinite loop detected while processing.
    (508, LOOP_DETECTED, "Loop Detected");
    /// **510 Not Extended**: Additional extensions required to fulfill the request.
    (510, NOT_EXTENDED, "Not Extended");
    /// **511 Network Authentication Required**: Client must authenticate to access the network.
    (511, NETWORK_AUTHENTICATION_REQUIRED, "Network Authentication Required");
}