use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;
 
 
fn main() -> io::Result<( )> {

    //connect to the server , get the connected stream  
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    //
    for _ in 0..10 {

        //defing a varible to store the customer input
        let mut input = String::new();

        //read the customer input and set it to 'input' varible
        io::stdin().read_line(&mut input).expect("Failed to read");

        //send the inputed bytes into connected stream
        stream.write(input.as_bytes()).expect("failed to write");
 

        //define reader to get message from server
        let mut reader =BufReader::new(&stream);

        //new a varble to stroe the bytes from reader
        let mut buffer: Vec<u8> = Vec::new();

        //get the messages from server and print messages to termial
        reader.read_until(b'\n',&mut buffer)?;
        println!("read form server:{}",str::from_utf8(&buffer).unwrap());

 
    }
    Ok(())
}
