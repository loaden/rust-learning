mod rect;
use rect::Rectangle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_division() {
        let result = devision(5);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_rectangle_hold() {
        let larger = Rectangle { length: 8, width: 7 };
        let smller = Rectangle { length: 5, width: 6 };
        assert!(larger.can_hold(&smller));
    }

    #[test]
    #[should_panic]
    fn test_rectangle_panic() {
        let r = Rectangle { length: 0, width: 8 };
        r.test_should_panic();
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn devision(value: usize) -> usize {
    value / 2
}
