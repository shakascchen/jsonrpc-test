# jsonrpc-test
Test jsonrpc server by Rust

# Usage
In project folder
```bash
jsonrpc-test $ cargo run
```

# Client
Use [simeple-jsonrpc-client](https://github.com/shakascchen/simple-jsonrpc-client) to test
```lisp
CL-USER> (simple-jsonrpc-client:call "http://127.0.0.1:3030" 
                                     :|method| "say_hello") 
"{\"jsonrpc\":\"2.0\",\"result\":\"hello\",\"id\":1}
"
200
#<HASH-TABLE :TEST EQUAL :COUNT 4 {1005FCD3D3}>
#<QURI.URI.HTTP:URI-HTTP http://127.0.0.1:3030>
#<SB-SYS:FD-STREAM for "socket 127.0.0.1:63697, peer: 127.0.0.1:3030" {1005FA0593}>

CL-USER> (simple-jsonrpc-client:call "http://127.0.0.1:3030" 
                                     :|method| "sum"
                                     :|params| '(5 11))
"{\"jsonrpc\":\"2.0\",\"result\":\"16\",\"id\":1}
"
200
#<HASH-TABLE :TEST EQUAL :COUNT 4 {1005FED463}>
#<QURI.URI.HTTP:URI-HTTP http://127.0.0.1:3030>
#<SB-SYS:FD-STREAM for "socket 127.0.0.1:63697, peer: 127.0.0.1:3030" {1005FA0593}>
```
