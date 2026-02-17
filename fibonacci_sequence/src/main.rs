use std::io;

fn main() {

    loop {                                                      // Цикл для непрерывного запроса данных
        println!("Enter the final value, please");              // со стороны пользователя

        let mut counter = 1;                                    // Счетчик для рассчета индекса чмсла в
                                                                // последовательности
        let mut st_first = 0;                                   // Начальное значение для первого числа
        let mut st_second = 1;                                  // Начальное значение для второго числа

        let mut fin_value = String::new();                      // Переменная с начальным типом данных 
                                                                // для конечного индекса последовательности
        io::stdin().read_line(&mut fin_value)                   // Принимается ввод значения конечного
            .expect("Filed to read the line");                  // индекса пользователем, вместе с тем,
                                                                // присутствует обработка ошибок
        let fin_value : i32 = fin_value.trim().parse()          // Приведение строки к числу
            .expect("Failed to convert");                       // определенного типа

        while counter <= fin_value {
            
            if counter % 2 == 1 {
                println!("{}", st_first);
                st_first = st_first + st_second;
            }

            else {
                println!("{}", st_second);
                st_second = st_second + st_first;
            }
            counter += 1;
        }
    }

}
