use crate::card::Card;
use rand::{Rng, thread_rng};



pub const INITIAL_HEALTH: i32 = 30;
const INITIAL_POWER: i32 = 0;
const INITIAL_VALUE: i32 = 0;
const HAND_SIZE: usize = 5;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub power: i32,
    pub value: i32,
    deck: Vec<Card>,
    hand: Vec<Card>,
    discard: Vec<Card>
}

impl Player {
    pub fn new(name: String) -> Self{
        Self {
            name,
            health: INITIAL_HEALTH,
            power: INITIAL_POWER,
            value: INITIAL_VALUE,
            deck: Player::deck(),
            hand: vec![],
            discard: vec![]
        }
        }


        fn deck()->Vec<Card>{
        vec![
            Card{ price: 0, power: 1, value: 0},
            Card{ price: 0, power: 1, value: 0},
            Card{ price: 0, power: 0, value: 1},
            Card{ price: 0, power: 0, value: 1},
            Card{ price: 0, power: 0, value: 1},
            Card{ price: 0, power: 0, value: 1},
            Card{ price: 0, power: 0, value: 1},
            Card{ price: 0, power: 0, value: 1},
            Card{ price: 0, power: 0, value: 1},
            Card{ price: 0, power: 0, value: 1}
        ]
    }

    pub fn draw_hand(&mut self){
        while self.hand.len() < HAND_SIZE {
            if self.deck.len() == 0{
                self.shuffle_discard_on_deck()
            }
            let index = thread_rng().gen_range(0..self.deck.len());
            self.hand.push(self.deck[index].clone());
            self.deck.remove(index);
        }
    }
    pub fn add_card_to_discard(&mut self, card:Card){
            self.discard.push(card);
    }

    fn shuffle_discard_on_deck(&mut self){
        //todo: fix magic numbers
        while self.discard.len() > 0 {
            self.deck.push(self.discard[0].clone());
            self.discard.remove(0);
        }
    }

    pub fn start_turn(&mut self){
        self.draw_hand();
        for card in self.hand.clone(){
            self.power += card.power;
            self.value += card.value;

        }
    }

    pub fn end_turn(&mut self){
        self.power = INITIAL_POWER;
        self.value = INITIAL_VALUE;
        self.discard_hand();
    }

    fn discard_hand(&mut self){
        //todo: fix magic numbers
        while self.hand.len() > 0 {
            self.discard.push(self.hand[0].clone());
            self.hand.remove(0);
        }
    }

    pub fn receive_dmg(&mut self, dmg: i32){
        self.health -= dmg;
    }

    pub fn can_buy_cards(&self, lowest_price: i32) -> bool{
        lowest_price <= self.value
    }

    pub fn spend_coins(&mut self, value:i32){
        self.value -= value;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn player_receives_dmg() {
        let dmg:i32 = 4;
        let mut player = Player::new("John".to_string());

        player.receive_dmg(dmg);
        assert_eq!(player.health, 26);
    }

    #[test]
    fn player_initial_values_ok() {
        let player = Player::new("John".to_string());
        assert_eq!(player.health, INITIAL_HEALTH);
        assert_eq!(player.power, INITIAL_POWER);
        assert_eq!(player.value, INITIAL_VALUE);
        assert_eq!(player.hand.len(), 0);
        assert_eq!(player.discard.len(), 0);
        assert_eq!(player.deck.len(), Player::deck().len());
    }

    #[test]
    fn start_turn_is_fine() {
        let mut player = Player::new("John".to_string());
        player.start_turn();
        assert!(player.power + player.value > 0);
        assert_eq!(player.hand.len(), HAND_SIZE);
        assert_eq!(player.deck.len(), HAND_SIZE);
    }

    #[test]
    fn end_turn_is_fine() {
        let mut player = Player::new("John".to_string());
        player.start_turn();
        player.end_turn();
        assert_eq!(player.power + player.value , 0);
        assert_eq!(player.hand.len(), 0);
        assert_eq!(player.discard.len(), HAND_SIZE);
        assert_eq!(player.deck.len(), HAND_SIZE);
    }

    #[test]
    fn shuffle_discard_test() {
        let mut player = Player::new("John".to_string());
        player.start_turn();
        player.end_turn();
        player.start_turn();
        player.end_turn();
        player.start_turn();
        assert!(player.power + player.value > 0);
        assert_eq!(player.hand.len(), HAND_SIZE);
        assert_eq!(player.deck.len(), HAND_SIZE);
        assert_eq!(player.discard.len(), 0);
    }

    #[test]
    fn add_to_discard_works() {
        let mut player = Player::new("John".to_string());
        let card:Card = Card{value:1, power:1, price:1};

        player.add_card_to_discard(card);
        assert!(player.discard.len() > 0);
    }
}