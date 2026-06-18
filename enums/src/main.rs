fn main() {
    enum IpAddrKind {                                                   // Здесь представлено перечисление IpAddrKind
        V4,                                                             // с возможными вариантами видов IP-адреса V4
        V6,                                                             // и V6. Они называются вариантами перечисления
    }


    let four = IpAddrKind::v4;                                          // Так происходит создание экземпляров каждого
    let six = ipAddrKind::v6;                                           // из двух вариантов перечисления


    fn route(ip_kind: ipAddrKind) {

    }

    route(ipAddrKind::v4);
    route(ipAddrKind::v6);
}
