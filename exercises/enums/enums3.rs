// enums3.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a hint.

// 

enum Message {
    Quit,
    Move(Point) ,
    ChangeColor((i32,i32,i32)),
    Echo(String)
}

struct Point {
    x: i32,
    y: i32,
}

struct State {
    color: (i32, i32, i32),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (i32, i32, i32)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message{
            Message::Quit =>{self.quit = true;},
            Message::Move(Point{x,y}) => {
                self.position.x = x;
                self.position.y = y;
            },
            Message::Echo(s) => {

            },
            Message::ChangeColor((x,y,z)) =>{
                self.color = (x,y,z);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor((255i32, 0i32, 255i32)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
