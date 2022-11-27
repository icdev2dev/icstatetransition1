# hello_heartbeat

This is a demonstration of state transitions for game playing; both interactive and non-interactive; mixed. 

Essentially the game is about making a guess and then getting a random number to see whether the guess matches
the random number. 

The twist is that the op does not want to wait for the random number to be generated; because it causes UX issues. 

In this demo, we model state transitions as following : 

1. Waiting for a guess
    This is interactive in the sense that till a guess is entered, we don't shift to the next 
    state (which is waiting for random choice)
2. Waiting for a random choice
   In this state (which is non-interactive), the random choice shoud be generated and compared
   and moved to the done state
3. Done
    In this state the game is removed from the board

The did file shows two commands

new_game
     this starts a new game. you can have multiple games at the same time. The game starts at 
     waiting for a guess.
my_guess
    this inputs a guess into the current game. Then automation takes it to completion.
    