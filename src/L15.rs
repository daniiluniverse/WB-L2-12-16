fn main() {
    // Создаем строковый срез (строку) с помощью строкового литерала
    let s1 = "hello";

    // Создаем объект типа String из строки "hello"
    let s2 = String::from("hello");

    // Получаем строковый срез из объекта String
    let s3 = s2.as_str();

    // Получаем размер s1 (строкового литерала)
    let size_of_s1 = std::mem::size_of_val(s1);

    // Получаем размер объекта String s2
    let size_of_s2 = std::mem::size_of_val(&s2);

    // Получаем размер строкового среза s3
    let size_of_s3 = std::mem::size_of_val(&s3);

    // Выводим размеры
    println!("{:?}", size_of_s1);
    println!("{:?}", size_of_s2);
    println!("{:?}", size_of_s3);
}
