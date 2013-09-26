//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::str;
use std::{io, path};

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count: uint =0;

unsafe fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    //visitor_count+=1;
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                //visitor_count+=1;
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        let aaaa: ~str = ~""+ request_str.slice(5,request_str.find_str("HTTP/1.1").unwrap()-1);
                        //println(fmt!("AAAAAAA %s",aaaa));
                        println(fmt!("Request received:\n%s", request_str));
                        //println(fmt!("%s",request_str.slice(0,(uint)(request_str.find_str("HTTP/1.1")))));
                        
                        if aaaa==~"favicon.ico"{
                            println("TUE");
                        }
                        else if aaaa!=~"" {
                            let response: ~str= ~""+load_file(aaaa)[0];
                            println(fmt!("Request Count: %u",visitor_count));
                            net_tcp::write(&sock, response.as_bytes_with_null_consume());
                        }
                        else{
                            visitor_count+=1;
                            println(fmt!("Request Count: %u \n",visitor_count));
                            let response: ~str = ~
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                 <doctype !html><html><head><title>Hello, Rust!</title>
                                 <style>body { background-color: #111; color: #FFEEAA }
                                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                 </style></head>
                                 <body>
                                 <h1>Greetings, Rusty!</h1>
                                 </body></html>\r\n";
                            net_tcp::write(&sock, response.as_bytes_with_null_consume());
                        }
                        
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
fn load_file(pathname : ~str) -> ~[~str] {
    let filereader : Result<@Reader, ~str> = io::file_reader(~path::Path(pathname));
    match filereader {
        Ok(reader) => reader.read_lines(),
        Err(msg) => fail!("Cannot open file: " + msg),
    }
}