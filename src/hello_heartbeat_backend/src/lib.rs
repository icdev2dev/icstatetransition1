use std::{ cell::{RefCell}};

thread_local! {
    static HB_COUNTER:RefCell<u32> = RefCell::new(0);
    static HB_PROCESS_INTERVAL:u32 = 20;

    static GAMES:RefCell<Vec<RefCell<Game>>> = RefCell::new(vec![]);
}


#[derive(Debug, Clone, Copy, PartialEq)]

enum  RealGameState {
    WG(WaitingForGuess),
    WRC(WaitingForRandomChoice),
    Done,
}


#[derive(Debug, Clone)]
struct Game   {
    name: String,
    state: RealGameState,
    shared_state: i128,
}


impl Game {
    fn update_shared_state (&mut self, shared_state:i128) {
        self.shared_state = shared_state;
    }

    fn get_shared_state(&self) -> i128 {
        self.shared_state
    }

    fn get_current_state(&self) -> RealGameState {
        self.state
    }

    fn transition_to(&mut self, next_state: RealGameState) {
        self.state = next_state;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

struct WaitingForGuess {
}

impl GameState for WaitingForGuess {
}
#[derive(Debug, Clone, Copy, PartialEq)]

struct WaitingForRandomChoice {
}


impl GameState for WaitingForRandomChoice {
    fn is_non_interactive(&self) -> bool {
        true
    }
}

trait GameState {
    fn is_non_interactive(&self) -> bool {
        false
    }
}


struct GameFactory {}
impl GameFactory {
    fn new_game (name: String) -> Game {
        Game { name: name, state: RealGameState::WG(WaitingForGuess{}), shared_state: 0 }
    }
}



#[ic_cdk_macros::update]
fn new_game(name: String) -> String {

    let cl_name = name.clone();

    GAMES.with(|games|{
         let v_games = &mut *games.borrow_mut();
         v_games.insert(0, RefCell::new(GameFactory::new_game(cl_name)));
     });
    format!("Created New Game {}!", name)

}

#[ic_cdk_macros::update]
fn my_guess (guess: i128) -> String {

    GAMES.with(|games|{
        let v_games = &mut *games.borrow_mut();
        
        let game = v_games.get(0);

        let game = game.unwrap();
        let game = &mut *game.borrow_mut();
        game.update_shared_state(guess);
    
    });

    "ok".to_owned()
}


#[ic_cdk_macros::heartbeat]
fn hb() {

    HB_COUNTER.with(|counter|{
        HB_PROCESS_INTERVAL.with(|hb_process_interval|{
            let counter = &mut *counter.borrow_mut();

            *counter = *counter + 1;
    
            let x = *counter;
    
            if x % hb_process_interval == 0 {
                heartbeat_at_interval();
                *counter = 0;
    
            }   
        })

    });
}

fn heartbeat_at_interval() {
    GAMES.with(|games|{

        let vec_games = &mut *games.borrow_mut();
        let x = vec_games.len();

        ic_cdk::println!("Number of games in play:  {}", x);

        let mut done = false;

        if x > 0 {

            let game = vec_games.get(0);

            let game = game.unwrap();
            let game = &mut *game.borrow_mut();
            ic_cdk::println!("Game :  {:?}", game);


            if game.get_shared_state() != 0 {
                match game.get_current_state() {
                    RealGameState::WG(_) => {

                        ic_cdk::println!("Now transition to Waiting for Random Choice");

                        game.transition_to(RealGameState::WRC(WaitingForRandomChoice{}))
                    },
                    RealGameState::WRC(_) => {
                        ic_cdk::println!("Now transition to Done");
                        game.transition_to(RealGameState::Done);
                        
                    },
                    RealGameState::Done => {
                        ic_cdk::println!("We are done for {:?}", game);
                        done = true;
                    }
                }
                
            }
         }

         if done == true {
            vec_games.remove(0);
         }

     });
}

