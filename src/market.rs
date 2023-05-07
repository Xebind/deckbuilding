use rand::{Rng, thread_rng};
use crate::card::Card;

const MARKET_SIZE : usize = 5;

#[derive(Debug, Clone)]
pub struct Market{
    pub market_deck: Vec<Card>,
    pub available: Vec<Card>
}

impl Market{
    pub fn new() ->Self{
        let mut market:Market = Self{
            market_deck: Market::deck(),
            available: vec![]
        };
        market.initial_available();
        return market;
    }


    fn deck()->Vec<Card>{
        vec![
            Card{ price: 1, power: 1, value: 1},
             Card{ price: 1, power: 1, value: 1},
             Card{ price: 2, power: 0, value: 2},
             Card{ price: 2, power: 2, value: 0},
             Card{ price: 2, power: 0, value: 2},
             Card{ price: 2, power: 2, value: 0},
             Card{ price: 2, power: 2, value: 2},
             Card{ price: 3, power: 0, value: 4},
             Card{ price: 3, power: 4, value: 0},
             Card{ price: 3, power: 2, value: 2},
             Card{ price: 3, power: 1, value: 3},
             Card{ price: 3, power: 2, value: 1},
             Card{ price: 3, power: 3, value: 1},
             Card{ price: 4, power: 1, value: 5},
             Card{ price: 4, power: 2, value: 2},
             Card{ price: 4, power: 4, value: 1},
             Card{ price: 5, power: 2, value: 4},
             Card{ price: 5, power: 4, value: 3},
             Card{ price: 5, power: 6, value: 0},
             Card{ price: 6, power: 8, value: 0},
               Card{ price: 1, power: 1, value: 1},
             Card{ price: 1, power: 1, value: 1},
             Card{ price: 2, power: 0, value: 2},
             Card{ price: 2, power: 2, value: 0},
             Card{ price: 2, power: 0, value: 2},
             Card{ price: 2, power: 2, value: 0},
             Card{ price: 2, power: 2, value: 2},
             Card{ price: 3, power: 0, value: 4},
             Card{ price: 3, power: 4, value: 0},
             Card{ price: 3, power: 2, value: 2},
             Card{ price: 3, power: 1, value: 3},
             Card{ price: 3, power: 2, value: 1},
             Card{ price: 3, power: 3, value: 1},
             Card{ price: 4, power: 1, value: 5},
             Card{ price: 4, power: 2, value: 2},
             Card{ price: 4, power: 4, value: 1},
             Card{ price: 5, power: 2, value: 4},
             Card{ price: 5, power: 4, value: 3},
             Card{ price: 5, power: 6, value: 0},
             Card{ price: 6, power: 8, value: 0}

        ]
    }

     fn initial_available(&mut self){
        while self.available.len() < MARKET_SIZE {
            self.new_available();
        }
    }

    pub fn new_available(&mut self){
        if self.market_deck.len() > 0 {
            let index = thread_rng().gen_range(0..self.market_deck.len());
            self.available.push(self.market_deck[index].clone());
            self.market_deck.remove(index);
        }
    }

   pub fn lowest_price(&self) -> i32{
    let mut lowest : i32 = self.available[0].price;
    for card in &self.available {
        if lowest > card.price {
            lowest = card.price;
        }
    }
    return lowest;
    }

    pub fn render(&self) {
        for i in 0..self.available.len(){
            println!(" Option {} has price {}, power {} and value {}", i, self.available[i].price, self.available[i].power, self.available[i].value);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_market_is_fine() {
        let market:Market = Market::new();
        assert_eq!(market.market_deck.len(), Market::deck().len() - MARKET_SIZE);
        assert_eq!(market.available.len(), MARKET_SIZE);
    }

}