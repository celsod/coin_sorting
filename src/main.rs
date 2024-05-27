fn main() {
    println!("Hello, world!");
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), //added the UsState value to the Quarter enum
}

#[derive(Debug)] //for inspection purposes
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { //function matches enum Coin to value and outputs a value
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter: {:?}", state);
            25},
    }
} /*
say quarter = value_in_cents(Coin::Quarter(UsState::Alaska)), then the binding works because it connects the quarter to the quarter and the state to Alaska
match against an enum, bind a variable to the data inside, and then execute code based on it
*/