#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::process;
use std::thread;
use std::io::{self, Read, Write, Error};
use std::net::TcpStream;
use std::net::TcpListener;

type Port = u16;

struct Program {
    name: String
}

impl Program {
    fn new(name: String) -> Program {
        Program { name: name }
    }

    fn usage(&self) {
        println!("usage: {} HOST PORT",self.name);
    }

    fn print_error(&self,mesg: String) {
        writeln!(io::stderr(),"{}: error: {}",self.name,mesg);
    }

    fn print_fail(&self,mesg: String) -> ! {
        self.print_error(mesg);
        self.fail();
    }

    fn exit(&self,status: i32) -> ! { process::exit(status); }
    fn fail(&self) -> ! { self.exit(-1); }
}

fn main() {
    let mut args = env::args();
    let program = Program::new(
        args.next().unwrap_or("test".to_string())
    );

    let host = args.next().unwrap_or_else(|| {
        program.usage();
        program.fail();
    });

    let port = args.next().unwrap_or_else(|| {
        program.usage();
        program.fail();
    }).parse::<Port>().unwrap_or_else(|error| {
        program.print_error(format!("invalid port number: {}",error));
        program.usage();
        program.fail();
    });

    let mut stream = TcpStream::connect(
        (host.as_str(), port)
    ).unwrap_or_else(|error|
        program.print_fail(error.to_string())
    );
    let mut input_stream = stream.try_clone().unwrap();

    let handler = thread::spawn(move || {
        let mut client_buffer = [0u8; 1024*10];

        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        program.exit(0);
                    }
                    else
                    {
                        io::stdout().write(&client_buffer).unwrap();
                        io::stdout().flush().unwrap();
                    }
                },
                Err(error) => program.print_fail(error.to_string()),
            }
        }
    });

    let output_stream = &mut stream;
    let mut user_buffer = String::new();

    loop {
        io::stdin().read_line(&mut user_buffer).unwrap();

        output_stream.write(user_buffer.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}
