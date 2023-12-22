const CALCULATOR_BUFFER_SIZE: usize = 512;
const ALLOWED_CHARACTERS: [char; 14] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '/', '*',
];
const MAX_NUMBER_OF_TOKENS: usize = 256;
static mut CALCULATOR_BUFFER: [char; CALCULATOR_BUFFER_SIZE] = ['\0'; CALCULATOR_BUFFER_SIZE];
static mut CALCULATOR_BUFFER_INDEX: usize = 0;

pub mod keyboard {

    use super::{calculator::*, *};
    use crate::os::interrupts::hardware::{InterruptIndex, PICS};
    use lazy_static::lazy_static;
    use pc_keyboard::{layouts::Us104Key, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            Us104Key,
            HandleControl::Ignore
        ));
    }

    pub extern "x86-interrupt" fn handler(_: InterruptStackFrame) {
        let mut keyboard = KEYBOARD.lock();
        let mut port = Port::new(0x60);

        let scancode: u8 = unsafe { port.read() };
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                eval_key(key);
            }
        }

        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Keyboard.u8());
        }
    }

    fn eval_key(key: DecodedKey) {
        if let DecodedKey::Unicode(character) = key {
            if ALLOWED_CHARACTERS.contains(&character) {
                add_to_calculator(character);
            } else if character == '\n' {
                eval_calculator();
            } else if character == '\x08' {
                remove_from_calculator();
            }
        }
    }
}

mod calculator {
    use crate::{print, println};

    use super::*;

    pub(super) fn add_to_calculator(character: char) {
        unsafe {
            if CALCULATOR_BUFFER_INDEX < CALCULATOR_BUFFER_SIZE {
                CALCULATOR_BUFFER[CALCULATOR_BUFFER_INDEX] = character;
                CALCULATOR_BUFFER_INDEX += 1;
            }
        }

        print!("{}", character);
    }

    pub(super) fn remove_from_calculator() {
        unsafe {
            if CALCULATOR_BUFFER_INDEX > 0 {
                CALCULATOR_BUFFER_INDEX -= 1;
                CALCULATOR_BUFFER[CALCULATOR_BUFFER_INDEX] = '\0';
            }
        }

        println!();
        for character in unsafe { CALCULATOR_BUFFER.iter() } {
            if *character == '\0' {
                break;
            }

            print!("{}", character);
        }
    }

    fn clear_calculator() {
        unsafe {
            CALCULATOR_BUFFER_INDEX = 0;
            for character in CALCULATOR_BUFFER.iter_mut() {
                if *character == '\0' {
                    break;
                }

                *character = '\0';
            }
        }
    }

    pub(super) fn eval_calculator() {
        let mut buffer = arrayvec::ArrayString::<CALCULATOR_BUFFER_SIZE>::new();
        unsafe {
            for character in CALCULATOR_BUFFER.iter() {
                if *character == '\0' {
                    break;
                }

                buffer.push(*character);
            }
        }

        let result = eval(&parse(&buffer));
        println!("{} = {}", buffer, result);

        clear_calculator();
    }

    fn parse(input: &str) -> arrayvec::ArrayVec<Token, { MAX_NUMBER_OF_TOKENS }> {
        let mut tokens = arrayvec::ArrayVec::<Token, { MAX_NUMBER_OF_TOKENS }>::new();

        let mut number = 0;
        for character in input.chars() {
            match character {
                '0'..='9' => {
                    number *= 10;
                    number += character.to_digit(10).unwrap() as i32;
                }
                '+' => {
                    tokens.push(Token::Number(number));
                    tokens.push(Token::Operator(Operator::Add));
                    number = 0;
                }
                '-' => {
                    tokens.push(Token::Number(number));
                    tokens.push(Token::Operator(Operator::Subtract));
                    number = 0;
                }
                '/' => {
                    tokens.push(Token::Number(number));
                    tokens.push(Token::Operator(Operator::Divide));
                    number = 0;
                }
                '*' => {
                    tokens.push(Token::Number(number));
                    tokens.push(Token::Operator(Operator::Multiply));
                    number = 0;
                }
                _ => {}
            }
        }

        tokens.push(Token::Number(number));

        tokens
    }

    #[derive(Copy, Clone)]
    pub enum Operator {
        Add,
        Subtract,
        Divide,
        Multiply,
    }

    pub enum Token {
        Operator(Operator),
        Number(i32),
    }

    pub fn eval(tokens: &[Token]) -> i32 {
        let mut result = 0;
        let mut operator = Operator::Add;

        for token in tokens {
            match token {
                Token::Operator(new_operator) => {
                    operator = *new_operator;
                }
                Token::Number(number) => match operator {
                    Operator::Add => {
                        result += number;
                    }
                    Operator::Subtract => {
                        result -= number;
                    }
                    Operator::Divide => {
                        result /= number;
                    }
                    Operator::Multiply => {
                        result *= number;
                    }
                },
            }
        }

        result
    }
}
