use std::fmt::format;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    // Создаем слушатель
    let  listener= TcpListener::bind("127.0.0.1:8080").unwrap();

    // Каждый пришедший запрос кидаем в функцию для обработки
    for stream in listener.incoming() {

        // Переменная содержащая запрос
        let stream = stream.unwrap();

        // Кидаем это в функцию
        handler_connection(stream);
    }

}

fn handler_connection(mut stream: TcpStream) {
    // Создаем буфер ????
    let mut buffer = [0; 1024];

    // Читаем пришедший запрос
    stream.read(&mut buffer).unwrap();

    // Можем работать с запросом тут
    /*println!(
        "Запрос: {}",
        String::from_utf8_lossy(&buffer[..])
    )*/

    // ???
    let get = b"GET / HTTP/1.1\r\n";

    // Начало тернарного оператора. Определяем статус запроса и подставляем ответ и файл для ответа
    let (status_line, filename) =

    if buffer.starts_with(get) {
        // Если всё ок, возвращаем ок и главную страницу
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        // Если не ок, возвращаем 404
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Указываем путь к контенту
    let contents = fs::read_to_string(filename).unwrap();

    // Формируем ответ
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Отправляем ответ пользователю
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


