# API Gateway

An API gateway serves as a single point of entry to the clients of an
application by sitting between the clients and backend services used by the
application.

The process of an API gateway is as follows:

1. A client makes a HTTP request
2. HTTP request gets validated (parameter validation)
3. Caller's IP address and HTTP headers get checked (allow-list/deny-list)
4. Rate limiting checks on attributes such as IP address and HTTP headers
5. HTTP request is sent to an identity provider for authentication and
   authorization
6. High-level rate limiting checks on authenticated session
7. Dynamic routing and service discover to locate backend services to handle
    the request by path matching
8. HTTP request gets transformed into the appropriate protocol (e.g., gRPC)
9. Transformed request is sent to the backend service
10. Retrieve response and transform it to the public-facing protocol
11. Transformed response is sent to the client
