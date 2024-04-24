/*  
Author:         Ethan Wilson
Class:          CIS 343-01
Professor:      Ira Woodring
Date:           April 24, 2024
*/

//libraries and imports. "use" equivalent of include. uses preludes like C++
use std::io;


//gets the users choice if they wish to be x or o.
fn prompt_user() -> Srting {

    //set choice to be mutable in case of mistakes, and set to "" to indicate string var type
    let mut user_choice = String::new();

    while user_choice != "X" || user_choice != "O" {

        print!("Choose if you wish to be X or O: ");

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Could not read line");
    }



}


//purely for printing out instructions. println! will always have a ! at the end of it to denote that it is a macro
fn instructions() {

    println!("\nWelcome to TicTacToe!!!\n\nThis is your board:\n");
    println!("   |   |   \n-----------\n   |   |   \n-----------\n   |   |   \n");
    println!("These are the postions of your board:\n");
    println!(" 1 | 2 | 3 \n-----------\n 4 | 5 | 6 \n-----------\n 7 | 8 | 9 \n");

    println!("You will always go first. To play your turn, choose a postion (1-9), that is not already occupied.\nThe computer will move after you.\n");
}


//Rust sees 'main()' as the entry point for programs. This is where our gameplay occurs
fn main() {

    instructions();
    let user = prompt_user();

}