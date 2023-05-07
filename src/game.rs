use crate::market::Market;
use crate::player::Player;

use rand::{Rng, thread_rng};
use std::io;

#[derive(Debug)]
pub struct Game{
    pub active_player: Player,
    pub opponent_player: Player,
    pub market: Market,
    pub logs: String
}

impl Game{
     pub fn new() -> Self {
         Self{
            active_player: Player::new("You".to_string()),
            opponent_player: Player::new("ROBOT".to_string()),
            market: Market::new(),
            logs: String::new()
        }
    }

    pub fn reset(&mut self) {
        self.active_player = Player::new("You".to_string());
        self.opponent_player = Player::new("ROBOT".to_string());
        self.market = Market::new();
        self.logs = String::new();
    }

    fn switch_players(&mut self){
        let previously_active_player = self.active_player.clone();
        self.active_player = self.opponent_player.clone();
        self.opponent_player = previously_active_player;
    }

    pub fn begin_turn(&mut self){
        self.active_player.start_turn();
        self.opponent_player.receive_dmg(self.active_player.power);
        self.add_to_logs(format!("Player {} gets hit for {} dmg, {} health remaining \n  \n", self.opponent_player.name ,self.active_player.power, self.opponent_player.health));
    }

    pub fn end_turn(&mut self){
        self.active_player.end_turn();
        self.switch_players();
    }

    pub fn get_winner(&mut self) -> bool {
        if self.active_player.health < 1{
            self.add_to_logs( format!("\n \n \nWinner is {:?}!", self.opponent_player.name));
            return true;
        }
        if self.opponent_player.health < 1 {
            self.add_to_logs( format!("\n \n \nWinner is {:?}!", self.active_player.name));
            return true;
        }
        return false;
    }
    pub fn add_to_logs(&mut self, log:String){
        self.logs.push_str(&log);
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
        self.add_to_logs( self.market.render());
        let index = thread_rng().gen_range(0..possible_cards.len());
        let selected_card = self.market.available[possible_cards[index]].clone();
        self.add_to_logs( format!("\n {} selected option {}, paying the cost {} with power {} and value {} \n \n ",self.active_player.name, possible_cards[index], selected_card.price, selected_card.power, selected_card.value));
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
        self.add_to_logs( format!("\n {} selected option {}, paying the cost {} with power {} and value {} \n \n", self.active_player.name, index, selected_card.price, selected_card.power, selected_card.value));

        self.active_player.spend_coins(selected_card.price);
        self.active_player.add_card_to_discard(selected_card);
        self.market.available.remove(index);
        self.market.new_available();
    }

    fn real_player_cli_turn(&mut self){

            self.market.render();
            let mut market_selection = String::new();
            io::stdin()
                .read_line(&mut market_selection)
                .expect("Failed to read line");
            self.buy_card(market_selection.trim().parse().expect("Please type a number!"));
    }

    pub fn real_player_api_turn(&mut self, index:usize){
       if self.get_winner(){
            return;
        }
        self.market.render();
        self.buy_card(index);
    }

    pub fn run_random(&mut self){
        loop {
            if self.get_winner(){
                return;
            }
            loop {
                if self.active_player.can_buy_cards(self.market.lowest_price()) && self.market.available.len() > 0 {
                   self.add_to_logs( format!("Player {} has {} coins \n", self.active_player.name, self.active_player.value));
                    self.buy_random_card();
                }
                else{
                    self.end_turn();
                    self.begin_turn();
                    break;
                }
            }
        }
    }


    pub fn run_game(&mut self) {
        if self.get_winner(){
            return;
        }
        loop {
            if self.active_player.can_buy_cards(self.market.lowest_price()) && self.market.available.len() > 0 {
                self.add_to_logs( format!("Player {} has {} coins \n", self.active_player.name, self.active_player.value));
                if self.active_player.name == "You".to_string() {
                    self.add_to_logs( self.market.render());
                    return
                } else {
                    self.buy_random_card();
                }
            } else {
                self.end_turn();
                self.begin_turn();

            }
            if self.get_winner()  {
                return;
            }
        }
    }


    pub fn run_cli(&mut self) {
        //todo: check if really the market is being updated or just the copy
        loop {
            if self.get_winner(){
                return;
            }
            loop {
                if self.active_player.can_buy_cards(self.market.lowest_price()) && self.market.available.len() > 0 {
                    println!("Player {} has {} coins", self.active_player.name, self.active_player.value);
                    if self.active_player.name == "You".to_string() {
                        self.real_player_cli_turn();
                    } else {
                        self.buy_random_card();
                    }
                }
                else{
                    self.end_turn();
                    self.begin_turn();
                    break;
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