// Technical Requirements:
// 1. Listen for bare TCP connections on 127.0.0.1:9000.
// 2. For each accepted TCP stream, parse exactly one full HTTP request.
// 3. Extract method, path (including query), headers, and body from the incoming request.
// 4. Construct a new HTTP request that targets 127.0.0.1:3000, preserving:
//    • The HTTP method (GET, POST, etc.)
//    • The path + query string (e.g. “/api/users?page=2”)
//    • Most headers, except hop-by-hop headers (Connection, Transfer-Encoding, etc.).
//    • The request body (for POST/PUT).
// 5. Open a new TCP connection to 127.0.0.1:3000 and send the constructed request.
// 6. Await the backend’s full HTTP response, including status line, headers, and body.
// 7. Strip or rewrite any hop-by-hop headers in the upstream response (Connection,
///   Transfer-Encoding, Keep-Alive, etc.), then copy the remaining status code, headers, and body.
// 8. Send that relayed response back to the original client over the first TCP stream.
// 9. If the backend connection or request fails (timeout, connection refused, parse error), return
//    a 5xx response (e.g. “500 Internal Server Error”) to the client.
// 10. Close the client connection once the response is fully flushed, then loop back to accept new connections.

// User Flow (step-by-step):
// 1. Client performs “curl http://127.0.0.1:9000/some/path?foo=bar”.
// 2. Proxy accepts TCP handshake on port 9000.
// 3. Proxy reads bytes until it has parsed a complete HTTP request (method, path, headers, body).
// 4. Proxy builds an outbound HTTP request with identical method and path “/some/path?foo=bar”, but
//    with the host set to 127.0.0.1:3000, and with original headers (minus hop-by-hop).
// 5. Proxy establishes a new TCP connection to 127.0.0.1:3000:80 (or 3000 if HTTP is nonstandard).
// 6. Proxy sends the freshly constructed request to the backend, streaming the client body as needed.
// 7. Backend (running on 127.0.0.1:3000) receives and processes the request, then sends back its response.
// 8. Proxy reads the complete response from the backend (status, headers, body).
// 9. Proxy removes any hop-by-hop headers (e.g. Connection, Transfer-Encoding) from that response.
// 10. Proxy forwards the cleaned response (status, headers, body) back to the original client on port 9000.
// 11. Client sees the same payload it would get if it had connected directly to port 3000.
// 12. Proxy closes both the backend TCP connection and the client TCP connection, then is ready for the next request.
//
// As you implement, tick off each requirement in order. Once you successfully relay a GET with headers, move on to
// streaming or chunked bodies, then error handling, and finally verify that every edge case (timeouts, malformed input)
// returns an appropriate HTTP error code.
