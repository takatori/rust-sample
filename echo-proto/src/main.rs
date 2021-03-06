/// Simple line-based echo server using tokio-proto
///
/// tokioを用いるサーバーは以下の3つのパーツから構成される
/// - transport
///   - ソケットへのRustリクエストとレスポンスの型のシリアル化を管理する
/// - protocol specification
///   - 
/// - service
///   - リクエストからレスポンスを生成する
///   - 基本的に非同期関数


extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf, Io, Framed};
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;

pub struct LineCodec;
pub struct LineProto;
pub struct Echo;

impl Codec for LineCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        // position: 与えられた関数(T -> bool)で true を返すような最初の要素のindexをSomeで返す。
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            // draint_to: Splits the buffer into two at the given index.
            // @see https://tokio-rs.github.io/tokio-core/tokio_core/io/struct.EasyBuf.html#method.drain_to
            let line:EasyBuf = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(line.as_slice()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()>{
        buf.extend(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
    
}

impl<T: Io + 'static> ServerProto<T> for LineProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = String;

    /// For this protocol style,  `Response` matches the codec `Out` type
    type Response = String;

    /// A bit of boilerprate to hook in the codec
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}


impl Service for Echo {
    // These types must match the corresonding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing thre resopnse; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immidiate.
        future::ok(req).boxed()
    }
    
}


fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address.
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new connection;
    // here, we just immidiately return a new instance.
    server.serve(|| Ok(Echo));
    
}
