use std::any::Any;
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

    // Пишем запрос в формат ютф_8
    let x = String::from_utf8_lossy(&buffer[..]);
    let mut filename = "404.html";
    let mut status_line = "HTTP/1.1 404 NOT FOUND";

    // Итерируем запрос построчно
    for sp in x.split("\n") {
        println!("Пришел запрос: {}", sp.trim());

        // Смотрим какой запрос к нам пришел
        match sp.trim() {
            // Заход на главную страницу
            "GET / HTTP/1.1" => {
                status_line = "HTTP/1.1 200 OK";
                filename = "index.html";
            },

            // Нажатие кнопки "Регистрация"
            "POST /register HTTP/1.1" => {
                status_line = "HTTP/1.1 200 OK";
                filename = "register.html";
            },

            // Нажатие кнопки "Авторизация"
            "POST /auth HTTP/1.1" => {
                status_line = "HTTP/1.1 200 OK";
                filename = "auth.html";
            },

            // Если не удалось найти подходящего запроса
            _ => {
                status_line = "HTTP/1.1 404 NOT FOUND";
                filename = "404.html";
            }
        }
        break
    }

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


