extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;

const SIZE: Winszed = Winszed {
    ws_row: 4,
    ws_col: 2,
    ws_xpixel: 0,
    ws_ypixel: 0,
};

#[test]
fn test_screen() {
    let mut screen: Screen = Screen::from(SIZE);

    assert_eq!(screen.into_iter().as_slice(), [Character::default(); 8]);
    screen.insert(0, Character::from('_'));
    assert_eq!(screen.into_iter().next(), Some(&Character::from('_')));
    screen.insert(1, Character::from('_'));
    screen.insert(2, Character::from('_'));
    screen.insert(3, Character::from('_'));
    screen.insert(4, Character::from('_'));
    screen.insert(5, Character::from('_'));
    screen.insert(6, Character::from('_'));
    screen.insert(7, Character::from('_'));
    assert_eq!(screen.into_iter().as_slice(), [Character::from('_'); 8]);
    screen.insert(8, Character::from('f'));
    assert_eq!(screen.into_iter().skip(8).next(), Some(&Character::from('f')));
    screen.remove(8);
    assert_eq!(screen.into_iter().as_slice(), [Character::from('_'); 8]);
    screen.insert(1, Character::from('d'));
    screen.remove(0);
    assert_eq!(screen.into_iter().next(), Some(&Character::from('d')));
    screen.remove(0);
    screen.remove(0);
    screen.remove(0);
    screen.remove(0);
    screen.remove(0);
    assert_eq!(screen.into_iter().as_slice(), [Character::from('_'); 2]);
    screen.push_all(vec![Character::from('d'), Character::from('d')]);
    assert_eq!(screen.into_iter().as_slice(), [Character::from('_'), Character::from('_'), Character::from('d'), Character::from('d')]);
    screen.clear();
    assert_eq!(screen.into_iter().as_slice(), [Character::default(); 4]);

}
