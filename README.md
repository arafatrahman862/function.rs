A [RPC](https://en.wikipedia.org/wiki/Remote_procedure_call) server for rust programing language.

# Motavition

An application protocol defines a way how data should be exchanged between a client and server. For example, Web browser use http protocol to transfer it's data.

But if client know how to communicate to the server then there is no need for any application protocols. 

For example: Chat based web application use websocket connection. Client side exectly know how the data should exchanged. 

We can use a tool that automatically generates client side code from server defined functions.
This has many advantages!

- No need for [fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) like api, you can call functions as if it were a normal (local) procedure call, without the programmer explicitly coding the details for the remote interaction. 
- No codes no bugs. Because the code is generated, no need for api testing tools. such as [postman](https://www.postman.com/)
- Automatically generated typesafe APIs.
- Because the application knows the type of complex data structures at compile time, it does not need to encode the type information, and efficiently decode data from the encoded bytes.
- Better use of system resources by using a single (tpc) connection.


[A Lightweight and High Performance Remote Procedure Call Framework for Cross Platform Communication](https://www.scitepress.org/papers/2016/59312/59312.pdf)
[JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
[Implementing Remote Procedure Calls](http://birrell.org/andrew/papers/ImplementingRPC.pdf)