mod stack {
    pub const MAXSIZE: usize = 100;
    #[derive(Debug)]
    pub struct STACK {
        pub data: [char; MAXSIZE],
        pub top: usize,
    }

    impl STACK {
        pub fn create(&mut self) {
            let temp = ['\0'; MAXSIZE];
            self.data = temp;
            self.top = 0;
        }

        pub fn push(&mut self, element: char) {
            self.data[self.top] = element;
            self.top += 1;
        }
        pub fn pop(&mut self) -> char {
            if self.is_empty() {
                return '\0';
            }
            self.top -= 1;
            let temp = self.data[self.top];
            self.data[self.top] = '&';
            temp
        }
        pub fn is_empty(&self) -> bool {
            self.top == 0
        }
        pub fn is_full(&self) -> bool {
            self.top >= MAXSIZE
        }
        pub fn display(&self) {
            println!("----------------------------------");
            for x in self.data {
                if x == '\0' {
                    break;
                }
                println!("{x}");
            }
            println!("----------------------------------");
        }
        pub fn peek(&self) -> char {
            if self.top > 0 {
                return self.data[self.top - 1];
            } else {
                return '\0';
            }
        }
        pub fn reverce(&mut self) {
            let mut temp: [char; MAXSIZE] = ['\0'; MAXSIZE];
            let mut index: usize = 0;

            for element in self.data.into_iter().rev() {
                if element == '\0' {
                    continue;
                }
                temp[index] = element;
                index += 1;
            }
            self.data = temp;
        }
    }
}

pub mod basic {
    use std::collections::HashMap;

    use crate::builtin::operations::stack::{MAXSIZE, STACK};

    pub fn create_stack() -> STACK {
        STACK {
            data: ['\0'; MAXSIZE],
            top: 0,
        }
    }

    pub fn string_to_stuct(s: &mut STACK, string: String) {
        for x in string.chars() {
            s.push(x);
        }
    }

    pub fn priority(c: char) -> i32 {
        let mut op = HashMap::new();
        op.insert(String::from('+'), 1);
        op.insert(String::from('-'), 1);
        op.insert(String::from('*'), 2);
        op.insert(String::from('/'), 2);
        op.insert(String::from('$'), 3);
        op.insert(String::from('^'), 3);

        for (k, v) in &op {
            if k.to_string() == c.to_string() {
                return *v;
            }
        }
        0
    }

    pub fn check_eqvation(s: &STACK) -> bool {
        let mut operator = 0;
        let mut operand = 0;
        let mut temp = create_stack();
        for x in s.data {
            if x == '(' || x == '[' || x == '{' {
                temp.push(x)
            } else if x == ')' || x == ']' || x == '}' {
                if x == '}' && temp.peek() == '{' {
                    temp.pop();
                } else if x == ')' && temp.peek() == '(' {
                    temp.pop();
                } else if x == ']' && temp.peek() == '[' {
                    temp.pop();
                }
            } else if x.is_alphabetic() {
                operand += 1;
            } else if x == '+' || x == '-' || x == '*' || x == '/' || x == '$' || x == '^' {
                operator += 1;
            } else {
                continue;
            }
        }
        if (operand == operator + 1) && temp.is_empty() {
            true
        } else {
            false
        }
    }

    //solve postfix
    pub fn postfix(s: &mut STACK) -> STACK {
        //initial p and operation stack
        let mut p = create_stack();
        let mut operator = create_stack();

        //use the algo to push stack round brakets in stack
        s.push(')');
        operator.push('(');

        for x in s.data {
            if x == '\0' {
                continue;
            } else if x.is_alphanumeric() {
                p.push(x);
            } else if x == '$' || x == '^' {
                operator.push(x);
            } else if x == '+' || x == '-' || x == '*' || x == '/' {
                while priority(x) <= priority(operator.peek()) {
                    p.push(operator.pop());
                }
                operator.push(x);
            } else if x == '(' || x == '{' || x == '[' {
                operator.push(x);
            } else if x == ')' {
                while operator.peek() != '(' {
                    if operator.peek() == '\0' {
                        break;
                    }
                    p.push(operator.pop());
                }
                operator.pop();
            }
        }
        p
    }
}
