use std::sync::mpsc; // Импортируем модуль для работы с каналами

// Функция, которая создает канал и отправляет значения из вектора
fn as_chan(vs: &[i32]) -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(); // Создаем канал

    let handle = std::thread::spawn(move || { // Запускаем новый поток
        let vs = vs.to_owned(); // Клонируем вектор

        for v in vs {
            tx.send(v).unwrap(); // Отправляем значения через канал
            std::thread::sleep(std::time::Duration::from_secs(1)); // Задержка в 1 секунду
        }

        drop(tx); // Закрываем отправитель
    });

    handle.join().unwrap(); // Ждем завершения потока

    rx // Возвращаем получатель
}

// Функция, которая объединяет два канала
fn merge(a: mpsc::Receiver<i32>, b: mpsc::Receiver<i32>) -> mpsc::Receiver<i32> {
    let (tx, rx) = mpsc::channel(); // Создаем новый канал

    let mut a_done = false;
    let mut b_done = false;

    loop {
        // Проверяем канал a
        match a.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap(); // Отправляем значение в новый канал
            }
            Err(_) => {
                a_done = true; // Если ошибки, значит, канал a завершен
            }
        }

        // Проверяем канал b
        match b.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap(); // Отправляем значение в новый канал
            }
            Err(_) => {
                b_done = true; // Если ошибки, значит, канал b завершен
            }
        }

        // Если оба канала завершены, выходим из цикла
        if a_done && b_done {
            break;
        }
    }

    rx // Возвращаем получатель
}

fn main() {
    let a = as_chan(&vec![1, 3, 5, 7]); // Создаем канал из первого вектора
    let b = as_chan(&vec![2, 4, 6, 8]); // Создаем канал из второго вектора
    let c = merge(a, b); // Объединяем два канала

    for v in c.iter() { // Итерируемся по получателю c
        println!("{v:?}"); // Выводим значения
    }
}
