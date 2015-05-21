IRONTRAY
========

Irontray is a simple HTTP server written in Rust. This is meant to be a learning project, and could only end up being useful accidentally.


Current goal
------------
The basic implementation goal is simply to have a TCP listener that spawns threads when a request arrives. It then parses the HTTP header transmitted, fetches the requested resource and serves it. Only HTTP 1.1 is meant to be supported (and partially).

It'll also log requests in a very simple way (append to a file).


Future goals
------------
A few non-ambitious goals for the project:

- Full HTTP 1.1 support
- Have configuration files
- Support fastcgi
- Support wsgi

Very future goals
-----------------
Let's get wild:

- GZip compression
- TLS support
- URL rewriting support
- Reverse proxy
- HTTP/2 support
- Who knows...