#[derive(Debug)]
//定义枚举
enum IpAddrKind {
    V4,
    V6,
}

//创建IpAddrKind类型函数
fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type);
}

fn main() {
    //创建枚举不同成员实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);

    //使用任意成员来调用函数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    route(four);
    route(six);

    println!("Hello, world!");
}
