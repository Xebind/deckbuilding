use crate::market::Market;
use crate::player::Player;

use rand::{Rng, thread_rng};
use std::io;


pub struct Game{
    active_player: Player,
    opponent_player: Player,
    market: Market
}

impl Game{
     pub fn new() -> Self {
         Self{
            active_player: Player::new("You".to_string()),
            opponent_player: Player::new("ROBOT".to_string()),
            market: Market::new()
        }
    }

    fn switch_players(&mut self){
        let previously_active_player = self.active_player.clone();
        self.active_player = self.opponent_player.clone();
        self.opponent_player = previously_active_player;
    }

    fn begin_turn(&mut self){
        self.active_player.start_turn();
        self.opponent_player.receive_dmg(self.active_player.power);
        print!("Player {} gets hit for {} dmg, {} health remaining \n  \n", self.opponent_player.name ,self.active_player.power, self.opponent_player.health);
    }

    fn end_turn(&mut self){
        self.active_player.end_turn();
        self.switch_players();
    }

    fn get_winner(&self) -> Option<Player>{
        if self.active_player.health < 1{
            return Some(self.opponent_player.clone());
        }
        if self.opponent_player.health < 1 {
            return Some(self.active_player.clone());
        }
        None
    }

    fn buy_random_card(&mut self) {
        //todo: fix out of range errors
        let mut possible_cards: Vec<usize> = vec![];
        let mut index: usize = 0;
        for card in &self.market.available {
            if card.price <= self.active_player.value {
                possible_cards.push(index);
            }
            index += 1;
        }
        self.market.render();
        let index = thread_rng().gen_range(0..possible_cards.len());
        let selected_card = self.market.available[possible_cards[index]].clone();
        println!("{} selected option {}, paying the cost {} with power {} and value {}", self.active_player.name, possible_cards[index], selected_card.price, selected_card.power, selected_card.value);
        self.active_player.spend_coins(selected_card.price);
        self.active_player.add_card_to_discard(selected_card);
        self.market.available.remove(possible_cards[index]);
        self.market.new_available();
    }

    fn buy_card(&mut self, index:usize) {
        if index >= self.market.available.len(){
            return println!("Select a valid option");
        }
        let selected_card = self.market.available[index].clone();
        if selected_card.price > self.active_player.value{
            return println!("You don't have enough money");
        }
        println!("{} selected option {}, paying the cost {} with power {} and value {}", self.active_player.name, index, selected_card.price, selected_card.power, selected_card.value);

        self.active_player.spend_coins(selected_card.price);
        self.active_player.add_card_to_discard(selected_card);
        self.market.available.remove(index);
        self.market.new_available();
    }

    fn real_player_turn(&mut self){

            self.market.render();
            let mut market_selection = String::new();
            io::stdin()
                .read_line(&mut market_selection)
                .expect("Failed to read line");
            self.buy_card(market_selection.trim().parse().expect("Please type a number!"));

    }

    pub fn run(&mut self) {
        //todo: check if really the market is being updated or just the copy
        loop {
            match self.get_winner() {
                Some(player) => {
                    return println!("Winner is {:?}!", player.name);
                },
                None => {
                    self.begin_turn();
                    loop {
                        if self.active_player.can_buy_cards(self.market.lowest_price()) && self.market.available.len() > 0 {
                            println!("Player {} has {} coins", self.active_player.name, self.active_player.value);
                            if self.active_player.name == "You".to_string() {
                                self.real_player_turn();
                            } else {
                                self.buy_random_card();
                            }
                        }
                        else{
                            self.end_turn();
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::player::INITIAL_HEALTH;
    use super::*;
    #[test]
    fn game_can_be_instantiated() {
        let game:Game = Game::new();
        assert!(true);
        assert_eq!(game.active_player.name, "You".to_string());
        assert_eq!(game.opponent_player.name, "ROBOT".to_string());

    }

    #[test]
    fn switch_players_work() {
        let mut game:Game = Game::new();
        game.switch_players();
        assert_eq!(game.opponent_player.name, "You".to_string());
        assert_eq!(game.active_player.name, "ROBOT".to_string());
    }

    #[test]
    fn begin_turn_applies_correct_dmg() {
        let mut game:Game = Game::new();
        game.begin_turn();
        assert_eq!(game.opponent_player.health, INITIAL_HEALTH - game.active_player.power);
    }

    #[test]
    fn end_turn_switches_players() {
        let mut game:Game = Game::new();
        game.begin_turn();
        game.end_turn();
        assert_eq!(game.opponent_player.name, "You".to_string());
        assert_eq!(game.active_player.name, "ROBOT".to_string());
    }

    #[test]
    fn buy_card_behaves_well() {
        let mut game:Game = Game::new();
        game.begin_turn();
        game.active_player.value = 10;
        let bought_card:Card = game.market.available[0].clone();
        game.buy_card(0);

        assert_eq!(game.active_player.value, 10 - bought_card.price);
    }


}