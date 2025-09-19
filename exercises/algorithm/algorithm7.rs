/*
    stack
    This question requires you to use a stack to achieve a bracket match
*/

// 括号匹配，左括号进栈，右括号出栈

use std::clone::Clone;

#[derive(Debug)]
struct Stack<T: Copy> {
    size: usize,
    data: Vec<T>,
}
impl<T: Copy> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    fn len(&self) -> usize {
        self.size
    }
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        // TODO
        if !self.is_empty() {
            let len = self.data.len();
            let tem = self.data[len - 1];
            self.data.remove(len - 1);
            self.size -= 1;
            return Some(tem);
        }
        None
    }
    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}
struct IntoIter<T: Copy>(Stack<T>);
impl<T: Clone + Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}
struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn bracket_match(bracket: &str) -> bool {
    //TODO
    let mut tem = Stack::new();
    for i in bracket.chars() {
        if i == '(' || i == '[' || i == '{' {
            tem.push(i);
        }
        if i == ')' {
            if !tem.is_empty() {
                println!("{}{}", *tem.peek().unwrap(), i);
            }
            if tem.is_empty() || *tem.peek().unwrap() != '(' {
                return false;
            }
            tem.pop();
        }
        if i == ']' {
            if !tem.is_empty() {
                println!("{}{}", *tem.peek().unwrap(), i);
            }
            if tem.is_empty() || *tem.peek().unwrap() != '[' {
                return false;
            }
            tem.pop();
        }
        if i == '}' {
            if !tem.is_empty() {
                println!("{}{}", *tem.peek().unwrap(), i);
            }
            if tem.is_empty() || *tem.peek().unwrap() != '{' {
                return false;
            }
            tem.pop();
        }
    }
    if !tem.is_empty() {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
