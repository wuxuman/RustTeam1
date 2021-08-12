use std ::net::{TcpListener,TcpStream};
use std::thread;
use std::time;
use std::io::{self,Read, Write};
use std::str;
 
 
fn handle_client(mut stream: TcpStream) -> io::Result<()>{
 
    //define a array to store the inputed bytes from client scream
    let mut buf = [0;512];


    //get the bytes from stream
    for _ in 0..1000{
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0{
            return Ok(());
        }
 
        //put the bytes into varible buf and print out into terminal
        stream.write(&buf[..bytes_read])?;
        println!("read form client:{}",str::from_utf8(&buf).unwrap());

        //wait for next connect from client
        thread::sleep(time::Duration::from_secs(1));
 
    }
    Ok(())
}
 
fn main() -> io::Result<()>{
    //listen to the localhost 8080 port
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    //define a vector varible to store the thread handle to control the thread
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    
    for stream in listener.incoming() {

        //a stream to get the input from client, if any exception return failed
        let stream = stream.expect("failed");
 
        //invoke handle_client method to handle the input stream from client
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });

        //put the handle into the vector
        thread_vec.push(handle);
    }
 

    //wait for the finish of the thread
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())

}
