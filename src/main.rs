extern crate jsonrpc_minihttp_server;

use jsonrpc_minihttp_server::{cors, ServerBuilder, DomainsValidation};
use jsonrpc_minihttp_server::jsonrpc_core::*;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("say_hello", |_params| {
        println!("got messages");
        futures::finished(Value::String("hello".to_owned()))
    });

    io.add_method("sum", |params: Params| {
        println!("got messages");
        println!("{:?}", params);

        let args: [i32; 2] = params.parse().unwrap();
        println!("{:?}", args);
       
        futures::finished(Value::String(format!("{}", args[0] + args[1]).to_owned()))
    });

    let server = ServerBuilder::new(io)
	.cors(DomainsValidation::AllowOnly(vec![cors::AccessControlAllowOrigin::Null]))
	.start_http(&"127.0.0.1:3030".parse().unwrap())
	.expect("Unable to start RPC server");

    println!("started");
    
    server.wait().unwrap();
}
