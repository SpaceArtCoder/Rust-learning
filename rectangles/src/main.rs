fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "Площадь проямоугольника равна {} квадратным пикселям.",
    //     area(width1, height1)
    // );

//     let rect1 = (30, 50);                                                               // Указание ширины и высоты прямоугольника с 
//                                                                                         // помощью кортежа 

//     println!(
//         "Площадь проямоугольника равна {} квадратным пикселям.",
//         area(rect1)
//     );
// }

// // fn area(width1: u32, height1: u32) -> u32 {                                             // Функция для расчета площади прямоугольника
// //     width1 * height1
// // }

// fn area(dimensions: (u32, u32)) -> u32 {                                                   // В данном способе существуют свои недостатки,
//     dimensions.0 * dimensions.1                                                            // так как каждый элемент кортежа не будет иметь
//                                                                                            // собственное название, что будет менее явно и 
// }                                                                                          // читаемо

//     let rect1 = Rectangle {width: 30, height: 50};

//     println!(
//         "Площадь прямоугольника равна {} квадратным пикселям",
//         area(&rect1)
//     );

// }

// struct Rectangle {
//         width: u32,
//         height: u32,
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {width: 30, height: 50};

    println!("rect1 равен {}", rect1);

}