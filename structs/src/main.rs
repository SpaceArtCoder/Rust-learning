fn main() {
    struct User {                                               // Для определения структуры вводится ключевое слово struct
        username: String,                                       // Имя труктуры, как правило, должно выразительно описывать
        email: String,                                          // сгруппированные фрагменты данных. Затем, внутри фигурных
        sign_in_count: u64,                                     // скобок определяются имя и тип для элементов данных, они
        active: bool,                                           // именуются как "поля"
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }
}
