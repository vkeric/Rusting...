#[derive(Debug)]
struct Account{
    id:u32,
    balance:i32,
    holder:String,
}
impl Account{
    fn new(id:u32,holder:String) -> Self{
        Account{
            id,
            holder,
            balance:0,
        }
    }
}


// Vec 定义_ 向量可以增大_缩小
#[derive(Debug)]
struct Bank{
    accounts:Vec<Account>, 
}

impl Bank{
    fn new()->Self{
         Bank { accounts:vec![] } // 隐式返回
    }
}

fn print_account(account:Account){
    println!{"{:#?}",account};
}


fn main(){
    let bank = Bank::new(); 
    let account = Account::new(
        1,
        String::from("Vk")
    );
    
  
    print_account(account);

    // println!("{:#?}",bank);
    // println!("{:#?}",account);

}
