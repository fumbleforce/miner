


pub fn pos_to_xy (pos: i32, grid_width: i32) -> (i32, i32) {
    return (pos % grid_width, pos / grid_width);
}

pub fn xy_to_pos (xy: (i32, i32), grid_width: i32) -> i32 {
    let (x, y) = xy;
    return x + (y * grid_width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_correct_pos() {
        assert_eq!(xy_to_pos((1, 1), 20), 21);
        assert_eq!(xy_to_pos((2, 0), 20), 2);
        assert_eq!(xy_to_pos((0, 2), 20), 40);
    }

    #[test]
    fn it_returns_correct_xy() {
        assert_eq!(pos_to_xy(10, 20), (10, 0));
        assert_eq!(pos_to_xy(20, 20), (0, 1));
        assert_eq!(pos_to_xy(21, 20), (1, 1));
        assert_eq!(pos_to_xy(2, 20), (2, 0));
        assert_eq!(pos_to_xy(40, 20), (0, 2));
    }
}