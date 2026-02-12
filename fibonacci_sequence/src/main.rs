fn main() {
/*
    fn fib(end : i32) {                                     // Определяется тип параметра

        let mut first_num = 0;                              // Начальное значение для первого числа 
                                                            // последовательности
        let mut second_num = 1;                             // Начальное значение для второго числа

        let mut third_num = 0;                              // Начальное значение для третьего числа

        let mut counter = 1;                                // Счетчик для рассчета индекса чмсла в
                                                            // последовательности
        while counter <= end {                              // Цикл будет выполняться до достижения
                                                            // конечного индекса, переданного в 
            if counter % 3 == 1 {                           // качестве аргумента вызову функции fib
                if counter != 1 {                           // Первые два значения будут выведены 
                    first_num = second_num + third_num;     // сразу, далее - производиться вычисление
                }
                println!("{}", first_num);
            }

            else if counter % 3 == 2 {
                if counter != 2 {
                    second_num = first_num + third_num;
                }
                println!("{}", second_num);
            }

            else {
                third_num = first_num + second_num;
                println!("{}", third_num);
            }

            counter += 1;                                   // После каждой итерации с проверкой значение
                                                            // счетчика инкрементируется единицей
        }
    }

    fib(19);
*/

    fn fib(end : i32) {

        let mut counter = 1;

        let mut a = 0;
        let mut b = 1;

        while counter <= end {
            
            if counter % 2 == 1 {
                println!("{}", a);
                a = a + b;
            }

            else {
                println!("{}", b);
                b = b + a;
            }
            counter += 1;
        }
    }

    fib(18);
}
