// Определяем структуру Example, которая хранит целое число
struct Example(i32);

// Реализуем трейт Drop для структуры Example
impl Drop for Example {
    fn drop(&mut self) {
        // При уничтожении экземпляра Example выводим его значение
        println!("{}", self.0);
    }
}

// Определяем структуру ExampleWrap, которая содержит экземпляр Example
struct ExampleWrap(Example);

// Реализуем трейт Drop для структуры ExampleWrap
impl Drop for ExampleWrap {
    fn drop(&mut self) {
        // Заменяем внутренний Example на новый (с значением 0)
        let e = std::mem::replace(&mut self.0, Example(0));
        // Выводим значение старого экземпляра Example
        println!("wrap {}", e.0);
    }
}

fn main() {
    // Создаем экземпляр Example(700), который будет уничтожен при выходе из main
    Example(700);
    
    // Создаем экземпляр Example(67) и сохраняем в переменной _e2
    let _e2 = Example(67);
    
    // Создаем экземпляр Example(3) и сохраняем в переменной _e3
    let _e3 = Example(3);
    
    // Создаем экземпляр Example(4), который не сохраняется
    let _ = Example(4);
    
    // Создаем опциональный экземпляр Example(5) и затем присваиваем None
    let mut _e5;
    _e5 = Some(Example(5)); // drop не будет вызван, так как мы его не освобождаем
    _e5 = None; // В этом случае drop для Example(5) не вызывается
    
    // Создаем экземпляр Example(6) и вызываем drop() явно
    let e6 = Example(6);
    drop(e6); // Выводит 6 при уничтожении
    
    // Создаем экземпляр Example(7) и предотвращаем его уничтожение
    let e7 = Example(7);
    std::mem::forget(e7); // drop не будет вызван для e7, вывод не будет
    
    // Создаем экземпляр ExampleWrap, который содержит Example(8)
    ExampleWrap(Example(8)); // При выходе из main будет вызван drop для ExampleWrap
    // Это вызовет:
    // 1. Замена внутреннего Example(8) на Example(0), выводя "wrap 8"
    // 2. Уничтожение Example(8), выводя "8"
    // 3. Уничтожение Example(0), выводя "0"
}

// Ожидаемый вывод программы будет:
// 1
// 4
// 6
// wrap 8
// 8
// 0
// 3
// 2
