fn main() {
    enum IpAddrKind {                                                   // Здесь представлено перечисление IpAddrKind
        V4,                                                             // с возможными вариантами видов IP-адреса V4
        V6,                                                             // и V6. Они называются вариантами перечисления
    }


    let four = IpAddrKind::v4;
    let six = ipAddrKind::v6;


    fn route(ip_kind: ipAddrKind) {

    }

    route(ipAddrKind::v4);

}
