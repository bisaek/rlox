use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use std::io::{BufRead, BufReader, Write};
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::panic;
use std::rc::Rc;

#[derive(Debug)]
pub struct Two;

impl Callable for Two {
    fn arity(&self) -> usize {
        0
    }
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<crate::literal::Literal>,
    ) -> crate::literal::Literal {
        Literal::Number(2.0)
    }
}

#[derive(Debug)]
struct TcpRequest {
    stream: TcpStream,
}
impl Callable for TcpRequest {
    fn arity(&self) -> usize {
        1
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Literal {
        let res = match arguments.get(0).unwrap() {
            Literal::Str(s) => s,
            _ => panic!("its need to be a string"),
        };

        //println!("res: {}", res);

        let mut clone = self.stream.try_clone().unwrap();

        clone
            .write_all(res.replace(r"\r", "\r").replace(r"\n", "\n").as_bytes())
            .unwrap();
        clone.flush().unwrap();
        clone.shutdown(Shutdown::Write).unwrap();

        Literal::None
    }
}

#[derive(Debug)]
pub struct Tcp;

impl Callable for Tcp {
    fn arity(&self) -> usize {
        1
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Literal {
        let req_function = match arguments.get(0) {
            Some(l) => match l {
                Literal::Callable(f) => f,
                _ => panic!("its need to be a callable"),
            },
            None => panic!("need 1 argument"),
        };
        //let res = match arguments.get(1).unwrap() {
        //    Literal::Str(f) => f,
        //    _ => panic!("its need to be a callable"),
        //};

        let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let buf_reader = BufReader::new(stream.try_clone().unwrap());
            let request = buf_reader
                .lines()
                .map(|line| line.unwrap())
                .take_while(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            req_function.call(
                interpreter,
                vec![
                    Literal::Str(request),
                    Literal::Callable(Rc::new(TcpRequest { stream })),
                ],
            );
        }

        Literal::None
    }
}
