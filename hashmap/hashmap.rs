use std::collections::HashMap

//创建hashmap
let mut scores=HashMap::new();
//方法一：插入键值对
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);

//方法二：用两个vector然后zip一对键值对
let teams=vec![String::from("Blue"),String::from("Yellow")];
let initial_scores=vec![10,50];
let scores:HashMap<_,_>=teams.iter().zip(initial_scores.iter()).collect();

//访问键的值
let team_name=String::from("Blue");
let score=scores.get(&team_name);

//使用for循环遍历
//会以任意顺序打印出每个键值对
for (key,value) in &scores {
    println!("{}: {}",key,value);
}

//更新哈希map（覆盖一个值）
//插入相同键将覆盖对应值
//此语句会覆盖上面的（Blue，50）
//根据键更新值的例子
scores.insert(String::from("Blue"), 25);
println!("{:?}",scores);

//根据旧值更新键
let text="hello world wonderful world";
let mut map=HashMap::new();
for word in text.split_whitespace(){
    let count=map.entry(word).or_insert(0);
    *count+=1;
}
println!("{:?}",map);