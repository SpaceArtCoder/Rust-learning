use std::io;

fn main() {

    loop {
        println!("Enter the temperature value, please");
                                                                
        let mut temp_val = String::new();                       // Переменная, которая будет хранить 
                                                                // пользовательский ввод в дальнейшем

        io::stdin().read_line(&mut temp_val)                    // Обработка ввода пользователем значения
            .expect("Filed to read the line");                  // температуры, а также обработка некорр-
                                                                // ектных значений
        
        let temp_val : i32 = temp_val.trim().parse()            // Приведение строки к числу определенного
            .expect("Failed to convert");                       // типа

        let mut conv_result : i32 = (temp_val * 9 / 5) + 32;    // Переменная хранящая итоговый результат
                                                                // конвертации температур
        println!("value: {}", conv_result);
    }
}