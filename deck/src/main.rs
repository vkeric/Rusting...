// 诡异的概念...
// 复杂的struct 和 复杂的向量
struct Deck{
    cards: Vec<String>,
}


fn main() {

    let suits = ["Vk","is","Nb","!!"];
    let values = ["1","2","3","4"];
    // 绑定 _ 结构体实例
    let deck = Deck{ cards : vec![

    ]};

    // println!("Hello, world!");
    println!("Heres yours deck : {}", deck );
}
