fn main() {
    // enum IpAddrKind {                                                   // Здесь представлено перечисление IpAddrKind
    //     V4,                                                             // с возможными вариантами видов IP-адреса V4
    //     V6,                                                             // и V6. Они называются вариантами перечисления
    // }


    // let four = IpAddrKind::v4;                                          // Так происходит создание экземпляров каждого
    // let six = IpAddrKind::v6;                                           // из двух вариантов перечисления.
    //                                                                     // Между тем, представленные варианты перечисления
    //                                                                     // находятся в пространстве имен IpAddrKInd, и для 
    //                                                                     // отделения их друг от друга используется 
    //                                                                     // синтаксис ::
    //                                                                     // Теперь оба значения, IpAddrKind::v4 и 
    //                                                                     // ipAddrKind::v6, имеют один и тот же тип: 
    //                                                                     // ipAddrKind

    // fn route(ip_kind: IpAddrKind) -> IpAddrKInd {
    //     ip_kind
    // }

    // route(IpAddrKind::v4);
    // route(IpAddrKind::v6);


    enum IpAddrKInd {
        V4,
        V6,
    }

    struct IpAddr {                                                                     // Структура используется здесь для 
        kind: IpAddrKInd,                                                               // компоновки значений вида и адреса
        address: String,                                                                // вместе
    }

    let home = IpAddr {
        kind: IpAddrKInd::V4,
        address: String::from("127.0.0.1"),
    };


    let loopback = IpAddr {
        kind: IpAddrKInd::V6,
        address: String::from("::1"),
    };

}
