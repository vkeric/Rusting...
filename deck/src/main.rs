// 诡异的概念...
// 复杂的struct结构体 和 复杂的向量

// impl 实现 -> fn功能 

// cargo add 包名  use 添加rand的包
use rand::{ thread_rng, seq::SliceRandom};

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

        //  隐式返回不要 这个符号 ;
        Deck{ cards }
    }

    fn shuffle(&mut self){ 
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self,num_cards:usize) -> Vec<String>{
        self.cards.split_off(self.cards.len()- num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let cards = deck.deal(3);

    println!("{:#?}",deck);
    println!("{:#?} cards",cards)
}
