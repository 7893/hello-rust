/*
//定义枚举
enum IpAddrKind{
    V4,
    V6,
}

//创建枚举不同成员实例
let four=IpAddrKind::V4;
let six=IpAddrKind::V6;

//创建IpAddrKind类型函数
fn route(ip_type:IpAddrKind){}

//使用任意成员来调用函数
route(IpAddrKind::V4);
route(IpAddrKind::V6);

//含有枚举类型的结构体
struct IpAddr{
    kind:IpAddrKind,
    address:String,
}

//创建结构体变量
let home=IpAddr{
    kind:IpAddrKind::V4,
    address:String::from("127.0.0.1"),
};
let loopback=IpAddr{
    kind:IpAddrKind::V6,
    address:String::from("::1"),
};

//指定成员类型的声明
enum IpAddr{
    V4(String),
    V6(String),
}

//直接将数据附加到成员上
let home=IpAddr::V4(127.0.0.1);
let loopback=IpAddr::V6(String::from("::1"));

//混合成员类型的枚举
enum Message{
    Quit, //没有关联数据类型
    Move{x:i32,y:i32}, //匿名结构体
    Write(String), 
    ChangColor(i32,i32,i32),
}
//等效于上面的混合成员枚举
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
//定义于Message枚举的方法call()
impl Message{
    fn call(&self){
        //方法体
    }
}
let m=Message::Write(String::from("hello"));
m.call()

//每个枚举成员可以处理不同类型不同数量的数据
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}
let home=IpAddr::V4(127,0,0,1);
let loopback=IpAddr::V6(String::from("::1"));
*/

fn main() {
    println!("Hello, world!");
    
}
