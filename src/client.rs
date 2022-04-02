use tonic::{Request, Response};

use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let req = Request::new(HelloRequest {
        name: "otegal".into(),
    });
    let res: Response<HelloReply> = client.say_hello(req).await?;

    println!("Response: {:?}", &res);
    println!("Reply: {:?}", &res.into_inner().message);

    Ok(())
}
