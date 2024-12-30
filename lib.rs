#![no_std]

// Import necessary modules from the MultiversX smart contract framework
use rand::Rng;
use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;

// Define the Soldier struct to store attributes of a soldier
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Soldier<M: ManagedTypeApi> {
    defense: u32, // Defense attribute of the soldier
    attack: u32,  // Attack attribute of the soldier
}

// Define the Game struct to store information about an ongoing game
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Game<M: ManagedTypeApi> {
    initiator: ManagedAddress<M>,       // Address of the game initiator
    initiator_soldier: Soldier<M>,      // Soldier sent by the initiator
    competitor: Option<ManagedAddress<M>>, // Address of the competitor
    competitor_soldier: Option<Soldier<M>>, // Soldier sent by the competitor
    entrance_fee: BigUint<M>,           // Entrance fee for the game
}

// Define the contract
#[multiversx_sc::contract]
pub trait EscrowFightContract {
    // Initialize the contract; add any necessary setup here
    #[init]
    fn init(&self) {}

    // Endpoint to create a game
    // The game initiator sends their Soldier to the contract and sets an entrance fee
    #[payable("EGLD")]
    #[endpoint(createGame)]
    fn create_game(
        &self,
        initiator_soldier: Soldier<Self::Api>, // Soldier details sent by the initiator
        entrance_fee: BigUint,                 // Entrance fee for the game
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller(); // Get the caller's address
        let payment = self.call_value().egld_value(); // Get the amount of EGLD sent

        // Ensure the payment matches the specified entrance fee
        require!(payment == entrance_fee, "Entrance fee must be paid");

        // Create a new game with the initiator's details
        let game = Game {
            initiator: caller.clone(),
            initiator_soldier,
            competitor: None,
            competitor_soldier: None,
            entrance_fee,
        };

        // Store the game details using the initiator's address as the key
        self.games(&caller).set(&game);

        Ok(())
    }

    // Endpoint for a competitor to join a game
    // The competitor pays the entrance fee and sends their Soldier to the contract
    #[payable("EGLD")]
    #[endpoint(joinGame)]
    fn join_game(
        &self,
        initiator: ManagedAddress,               // Address of the game initiator
        competitor_soldier: Soldier<Self::Api>, // Soldier details sent by the competitor
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller(); // Get the caller's address
        let payment = self.call_value().egld_value(); // Get the amount of EGLD sent

        // Retrieve the game details using the initiator's address
        let mut game = self.games(&initiator).get();

        // Ensure the game does not already have a competitor
        require!(game.competitor.is_none(), "Game already has a competitor");
        // Ensure the payment matches the entrance fee
        require!(payment == game.entrance_fee, "Incorrect entrance fee");

        // Set the competitor details in the game
        game.competitor = Some(caller.clone());
        game.competitor_soldier = Some(competitor_soldier);

        // Update the game details
        self.games(&initiator).set(&game);

        Ok(())
    }

    // Endpoint to start the fight and determine the winner
    #[endpoint(startFight)]
    fn start_fight(&self, initiator: ManagedAddress) -> SCResult<()> {
        // Retrieve the game details using the initiator's address
        let mut game = self.games(&initiator).get();
        // Ensure a competitor has joined the game
        require!(game.competitor.is_some(), "Competitor not found");

        // Retrieve both soldiers' details
        let initiator_soldier = game.initiator_soldier;
        let competitor_soldier = game.competitor_soldier.unwrap();

        // Calculate winning chances based on defense attribute differences
        let mut rng = rand::thread_rng();
        let initiator_advantage = initiator_soldier.defense as i32 - competitor_soldier.defense as i32;
        let win_chance = (50 + initiator_advantage).clamp(0, 100);

        // Randomly determine the winner based on calculated chances
        let random_value: u32 = rng.gen_range(0..100);
        let winner = if random_value < win_chance {
            game.initiator.clone()
        } else {
            game.competitor.unwrap().clone()
        };

        // Transfer the total prize to the winner
        let total_prize = game.entrance_fee * 2u32;
        self.send().direct(&winner, &total_prize);

        // Clear the game from storage
        self.games(&initiator).clear();

        Ok(())
    }

    // Storage mapper to store games
    #[storage_mapper("games")]
    fn games(&self, initiator: &ManagedAddress) -> SingleValueMapper<Game<Self::Api>>;
}