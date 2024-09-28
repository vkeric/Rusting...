// 诡异的概念...
// 复杂的struct 和 复杂的向量
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck{
    fn new()-> Self{
         let suits = ["A","B","C"];
         let values = ["1","2","3"];

         let mut cards =vec![];

         for suit in suits{
            for value in values {
                let card = format!("{}of{}",value,suit);
                cards.push(card);
            }
         }

         let deck = Deck{ cards };
         
         return deck;
    }
}

fn main() {
    let deck = Deck::new();

    println!("{:#?}",deck);
}
