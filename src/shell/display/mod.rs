mod err;
pub mod winsz;
pub mod cursor;
pub mod character;

use std::ops::{self, BitAnd, Add, Sub, Mul, Not};
use std::io::{self, Write};
use std::iter;
use std::fmt;
use std::str;
use std::mem;
use std::cmp;

use ::libc;

pub use self::winsz::Winszed;
pub use self::err::DisplayError;
use self::cursor::Cursor;
use self::character::color;
pub use self::character::Character;
use self::character::attribute::Attribute;

pub const LIMIT_X: libc::size_t = 1000;
pub const LIMIT_Y: libc::size_t = 1000;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coordinate {
    pub x: libc::size_t,
    pub y: libc::size_t,
}

impl Coordinate {
    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

impl PartialEq<(libc::size_t, libc::size_t)> for Coordinate {
    fn eq(&self, &(x, y): &(libc::size_t, libc::size_t)) -> bool {
        x.eq(&self.x).bitand(y.eq(&self.y))
    }
}

impl From<(libc::size_t, libc::size_t)> for Coordinate {
    fn from((x, y): (libc::size_t, libc::size_t)) -> Coordinate {
        Coordinate {
            x: x,
            y: y,
        }
    }
}

impl From<Winszed> for Coordinate {
    fn from(size: Winszed) -> Coordinate {
        Coordinate {
            x: size.get_col(),
            y: size.get_row(),
        }
    }
}

pub struct Newline {
    row: [Coordinate; LIMIT_Y],
    index: libc::size_t,
}

impl Newline {
    pub fn get_row(&self) -> &[Coordinate; LIMIT_Y] {
        &self.row
    }

    pub fn get_index(&self) -> libc::size_t {
        self.index
    }

    pub fn scroll_down(&mut self, mut resize: Coordinate) {
        resize.y -= 1;
        self.row.iter_mut()
                .take(self.index)
                .filter(|pos| pos.y.ge(&resize.x).bitand(pos.y.lt(&resize.y)))
                .all(|pos| {
                    pos.y += 1;
                    true 
                });
    }

    pub fn scroll_up(&mut self, resize: Coordinate, base: &libc::size_t) {
        self.row.iter_mut()
                .take(self.index)
                .filter(|pos| pos.y.gt(base).bitand(pos.y.lt(&resize.y)))
                .all(|pos| {
                    pos.y -= 1;
                    true 
                });
    }

    pub fn push(&mut self, item: Coordinate) {
        if self.index < LIMIT_Y {
            unsafe {
                *self.row.get_unchecked_mut(self.index) = item;
            }
            self.index += 1;
        }
    }

    pub fn reverse(&mut self) {
        let mut row: [Coordinate; LIMIT_Y] = self.row;
        
        row.reverse();
        self.row.iter_mut()
                .zip(row.iter().skip(LIMIT_Y-self.index))
                .all(|(current, after): (&mut Coordinate, &Coordinate)| {
                        *current = *after;
                        true
                });
    }

    pub fn dedup(&mut self) {
        let row: [Coordinate; LIMIT_Y] = self.row;
        
        self.index =
            self.row.iter_mut()
                    .zip(row.windows(2)
                            .filter(|both| both[0].ne(&both[1])))
                    .map(|(current, after): (&mut Coordinate, &[Coordinate])| {
                        *current = after[0];
                        true
                    })
                    .count();
    }

    pub fn sort_by(&mut self) {
        let mut row: Vec<Coordinate> = self.row.iter().take(self.index).map(|coordinate| *coordinate).collect::<Vec<Coordinate>>();
        
        row.sort_by(|a, b| a.y.cmp(&b.y));
        self.row.iter_mut()
                .zip(row.iter())
                .all(|(current, after): (&mut Coordinate, &Coordinate)| {
                        *current = *after;
                        true
                });
    }

    pub fn remove(&mut self, at: usize) {
        if let Some(index) = self.index.checked_sub(1) {
            let row: [Coordinate; LIMIT_Y] = self.row;
            self.index = index;
            self.row.iter_mut()
                    .skip(at)
                    .zip(row.iter().skip(at+1))
                    .take(index-at)
                    .all(|(current, after): (&mut Coordinate, &Coordinate)| {
                        *current = *after;
                        true
                    });
        }
    }

    pub fn clear(&mut self) {
        self.index = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.index.eq(&0)
    }
}

impl ops::Index<usize> for Newline {
    type Output = Coordinate;
    
    fn index(&self, index: usize) -> &Coordinate {
        unsafe {
            self.row.get_unchecked(index)
        }
    }
}

impl ops::IndexMut<usize> for Newline {
    fn index_mut(&mut self, index: usize) -> &mut Coordinate {
        unsafe {
            self.row.get_unchecked_mut(index)
        }
    }
}

impl<'a> IntoIterator for &'a Newline {
    type Item = &'a Coordinate;
    type IntoIter = iter::Take<::std::slice::Iter<'a, Coordinate>>;

    fn into_iter(self) -> Self::IntoIter {
        self.row.into_iter().take(self.index)
    }
}

impl From<Vec<(libc::size_t, libc::size_t)>> for Newline {
    fn from(rhs: Vec<(libc::size_t, libc::size_t)>) -> Newline {
        let mut line: Newline = Newline::default();

        line.index = rhs.len();
        line.row.iter_mut()
                .zip(rhs.iter())
                .all(|(elem, &(x, y)): (&mut Coordinate, &(libc::size_t, libc::size_t))| {
                    *elem = Coordinate::from((x, y));
                    true
                });
        line
    }
}

impl From<Vec<Coordinate>> for Newline {
    fn from(rhs: Vec<Coordinate>) -> Newline {
        let mut line: Newline = Newline::default();

        line.index = rhs.len();
        line.row.iter_mut()
                .zip(rhs.iter())
                .all(|(elem, rhs_elem): (&mut Coordinate, &Coordinate)| {
                    *elem = *rhs_elem;
                    true
                });
        line
    }
}

impl From<Winszed> for Newline {
    fn from(size: Winszed) -> Newline {
        let mut line: Newline = Newline::default();
    
        line.index = size.get_row();
        line.row.iter_mut()
                .zip(iter::repeat(size.get_col().checked_sub(1).unwrap_or_default())
                          .take(line.index)
                          .enumerate())
                .all(|(elem, (index, colum)): (&mut Coordinate, (libc::size_t, libc::size_t))| {
                     *elem = Coordinate::from((colum, index));
                     true
                });
        line
    }
}

impl cmp::PartialEq for Newline {
    fn eq(&self, rhs: &Newline) -> bool {
        self.index.eq(&rhs.index)
    }
}

impl Clone for Newline {
    fn clone(&self) -> Newline {
        Newline {
            row: self.row,
            index: self.index,
        }
    }
}

impl fmt::Debug for Newline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.index < 8 {
            write!(f, "Newline {{ row: {:?}, index: {} }}", &self.row.iter().take(self.index).collect::<Vec<&Coordinate>>(), self.index)
        } else {
            write!(f, "Newline {{ row: {:?}, index: {} }}", &self.row[..8], self.index)
        }
    }
}

impl Default for Newline {
    fn default() -> Newline {
        Newline {
            row: [Coordinate::default(); LIMIT_Y],
            index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub save_position: Coordinate,
    pub show_cursor: bool,
    pub mouse_handle: (bool, bool, bool, bool),
    pub ss_mod: bool,
    pub region: Coordinate,
    pub collection: Character,
    /// Out of Bound
    pub oob: Coordinate,
    pub line_wrap: bool,
    pub size: Winszed,
    pub bell: libc::size_t,
    pub screen: Cursor<Vec<Character>>,
    pub newline: Newline,
}

#[derive(Debug, Clone)]
pub struct Display {
    save_terminal: Option<Table>,
    table: Table,
}

impl Display {
    /// The constructor method `default` returns the `Display`'s interface
    /// from shell.
    pub fn new(fd: libc::c_int) -> Result<Self, DisplayError> {
        match Winszed::new(fd) {
          Err(why) => Err(DisplayError::WinszedFail(why)),
          Ok(wsz) => Ok(Display::from_winszed(wsz)),
        }
    }

    /// The constructor method `default` returns the `Display`'s interface
    /// from shell.
    pub fn from_winszed(size: Winszed) -> Display {
        Display {
            save_terminal: None,
            table: Table {
              save_position: Coordinate::default(),
              show_cursor: true,
              mouse_handle: (false, false, false, false),
              ss_mod: false,
              newline: Newline::from(size),
              region: Coordinate::from((0, size.get_row())),
              collection: Character::default(),
              oob: Coordinate::default(),
              line_wrap: true,
              size: size,
              bell: 0,
              screen: Cursor::new(
                (0..size.row_by_col()).map(|_: usize|
                                              Character::default()
                                          ).collect::<Vec<Character>>()
              ),
          }
        }
    }

    /// The accessor `ss` returns the value of 'ss_mod'.
    pub fn get_ss(&self) -> bool {
        self.table.ss_mod
    }

    /// The accessor method `get_mouse` returns the value of mouse_handle'.
    pub fn get_mouse(&self) -> (bool, bool, bool, bool) {
        self.table.mouse_handle
    }

    /// The accessor `get_window_size` returns the window size interface.
    pub fn get_window_size(&self) -> &Winszed {
        &self.table.size
    }

    /// The mutator `set_window_size` replaces the window size.
    pub fn set_window_size(&mut self, size: &Winszed) {
        self.resize_with(size);
        self.table.size = *size;
    }

    /// The accessor `get_cursor_coords` returns the value of 'oob', that is the coordinates of the cursor.
    pub fn get_cursor_coords(&self) -> &Coordinate {
        &self.table.oob
    }

    /// The accessor `newlines` returns the value of 'newline', that contains all newlines that are now
    /// displayed on the screen.
    pub fn get_newline(&self) -> &Newline {
        &self.table.newline
    }

    /// Converts a Vector of Character into a byte vector.
    pub fn into_bytes(&self) -> Vec<libc::c_uchar> {
        let mut screen: Vec<libc::c_uchar> = Vec::new();
        self.table.screen.get_ref().iter().all(|control: &Character| unsafe {
            let buf: [u8; 4] = mem::transmute::<char, [u8; 4]>(control.get_glyph());
            screen.extend_from_slice(&buf[..1]);
            true
        });
        screen
    }

    /// The method `clear` purges the screen vector.
    pub fn clear(&mut self) -> io::Result<libc::size_t> {
        self.table.screen.get_mut().iter_mut().all(|mut term: &mut Character| {
                                             term.clear();
                                             true});
        self.table.newline.clear();
        {0..self.table.size.ws_row}.all(|i|
        { self.table.newline.push(Coordinate::from((self.table.size.get_col() - 1, i as usize)));
          true });
        Ok(0)
    }

    /// The method `resize` updates the size of the output screen.
    pub fn resize(&mut self) -> Result<(), DisplayError> {
        match Winszed::new(0) {
            Err(why) => Err(DisplayError::WinszedFail(why)),
            Ok(ref size) => Ok(self.resize_with(size)),
        }
    }

    pub fn resize_with(&mut self, size: &Winszed) {
      if size.ws_row.gt(&0).bitand(size.ws_col.gt(&0))
      { if self.table.size.ws_row < size.ws_row
          { {self.table.size.ws_row..size.ws_row}.all(|i|
            { self.table.newline.push(Coordinate::from((self.table.size.get_col() - 1, i as usize)));
              true });
            let srow = size.ws_row as usize;
            if self.table.region.y == self.table.size.ws_row as usize
            { self.table.region.y = srow; }
            match self.table.size.get_col().checked_mul((size.ws_row - self.table.size.ws_row) as usize)
            { Some(get) =>
                { let mut vide = {0..get}.map(|_: usize| Character::default()).collect::<Vec<Character>>();
                  self.table.screen.get_mut().append(&mut vide); },
              None => {}, }}
          else if self.table.size.ws_row > size.ws_row
          { {size.ws_row..self.table.size.ws_row}.all(|i|
            { match self.table.newline.into_iter().position(|pos| pos.y.eq(&(i as usize)))
              { Some(n) =>
                  { self.table.newline.remove(n); },
                None => {}, }
              true });
            let srow = size.ws_row as usize;
            if self.table.region.y > srow
            { self.table.region.y = srow; }
            if self.table.region.x >= srow
            { match srow.checked_sub(1)
              { Some(get) => { self.table.region.x = get; },
                None => { self.table.region.x = 0; }, }}
            match self.table.size.get_col().checked_mul((self.table.size.ws_row - size.ws_row) as usize)
            { Some(get) =>
                { if self.table.oob.y.ge(&(size.ws_row as usize))
                  { let x = self.table.size.get_col() - 1;
                    let _ = self.goto_coord(Coordinate::from((x, size.get_row()-1)));
                  }
                  match self.table.size.row_by_col().checked_sub(get)
                  { Some(start) => { self.table.screen.get_mut().drain(start..); },
                    None => {}, }},
              None => {}, }}

          if self.table.size.ws_col < size.ws_col
          { let col = self.table.size.ws_col;
            let row = size.ws_row;
            {0..row}.all(|i|
            { match self.table.newline.into_iter().position(|pos| pos.y.eq(&(i as usize)))
              { Some(n) => { self.table.newline[n].x = (size.ws_col as usize) - 1; },
                None => {}, }
              true });
            { let coucou = self.table.screen.get_mut();
              {0..row}.all(|i|
              { {0..size.ws_col-col}.all(|_|
                { (*coucou).insert(((row - i) * col) as usize, Character::default());
                  true }) }); }
            self.table.size = *size;
            let x = self.table.oob.x;
            let y = self.table.oob.y;
            let _ = self.goto_coord(Coordinate::from((x, y))); }
          else if self.table.size.ws_col > size.ws_col
          { let col = self.table.size.ws_col;
            let row = size.ws_row;
            {0..row}.all(|i|
            { match self.table.newline.into_iter().position(|pos| pos.y.eq(&(i as usize)))
              { Some(n) => { self.table.newline[n].x = (size.ws_col as usize) - 1; },
                None => {}, }
              true });
            { let coucou = self.table.screen.get_mut();
              {0..row}.all(|i|
              { {0..col-size.ws_col}.all(|k|
                { (*coucou).remove((((row - i) * col) - (k + 1)) as usize);
                  true }) }); }
            self.table.size = *size;
            let x = if self.table.oob.x < size.ws_col as usize
            { self.table.oob.x }
            else
            { size.ws_col as usize - 1 };
            let y = self.table.oob.y;
            let _ = self.goto_coord(Coordinate::from((x, y))); }}
          self.table.size = *size;

    }

    /// The method `tricky_resize` updates the size of the output screen.
    pub fn tricky_resize(&mut self, begin: libc::size_t, end: libc::size_t)
    { if begin > 0 && begin <= end
      { self.table.region = Coordinate::from((begin - 1, end)); }}

    /// The method `goto` moves the cursor position
    pub fn goto(&mut self, index: libc::size_t) -> io::Result<libc::size_t> {
        if self.table.show_cursor
        { self.clear_cursor(); }
        self.table.screen.set_position(index);
        Ok(0)
    }

    /// The method `goto_home` moves the cursor to the top left of the output screen.
    pub fn goto_home(&mut self) -> io::Result<libc::size_t>
    { let _ = self.goto(0);
      self.table.oob.reset();
      Ok(0) }

    /// The method `goto_up` moves the cursor up.
    pub fn goto_up(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let col = self.table.size.get_col();
      let pos = self.table.screen.position();
      if self.table.oob.y >= mv
      { let _ = self.goto(pos.sub(&((col.mul(&mv)))));
        self.table.oob.y = self.table.oob.y.sub(&mv); }
      else
      { self.table.oob.y = 0;
        let x = self.table.oob.x;
        let _ = self.goto_coord(Coordinate::from((x, 0))); }
      Ok(0) }

    /// The method `goto_down` moves the cursor down.
    pub fn goto_down(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let row = self.table.size.get_row();
      let col = self.table.size.get_col();
      let pos = self.table.screen.position();
      if self.table.oob.y + mv <= row - 1
      { let _ = self.goto(pos.add(&(col.mul(&mv))));
        self.table.oob.y = self.table.oob.y.add(&mv); }
      else
      { self.table.oob.y = row - 1;
        let x = self.table.oob.x;
        let _ = self.goto_coord(Coordinate::from((x, row - 1))); }
      Ok(0) }

    /// The method `goto_right` moves the cursor to its right.
    pub fn goto_right(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let col = self.table.size.get_col();
      let pos = self.table.screen.position();
      if self.table.oob.x + mv <= col - 1
      { let _ = self.goto(pos.add(&mv));
        self.table.oob.x = self.table.oob.x.add(&mv); }
      else
      { let _ = self.goto_end_row(); }
      Ok(0) }

    pub fn goto_left(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let pos = self.table.screen.position();
      if self.table.oob.x >= mv
      { let _ = self.goto(pos.sub(&mv));
        self.table.oob.x = self.table.oob.x.sub(&mv); }
      else
      { let _ = self.goto_begin_row(); }
      Ok(0) }

    /// The method `goto_begin_row` moves the cursor to the beginning of the row
    pub fn goto_begin_row(&mut self)
    { let y = self.table.oob.y;
      let _ = self.goto_coord(Coordinate::from((0, y))); }

    /// The method `goto_end_row` moves the cursor to the end of the row
    pub fn goto_end_row(&mut self)
    { let x = self.table.size.get_col() - 1;
      let y = self.table.oob.y;
      let _ = self.goto_coord(Coordinate::from((x, y))); }

    /// The method `goto_coord` moves the cursor to the given coordinates
    pub fn goto_coord(&mut self, mut current: Coordinate) {
        let limit: Coordinate = Coordinate::from(self.table.size);

        if current.x < limit.x {
            self.table.oob.x = current.x;
        } else {
            self.table.oob.x = limit.x - 1;
            current.x = limit.x - 1;
        }
        if current.y < limit.y {
            self.table.oob.y = current.y;
        } else {
            self.table.oob.y = limit.y - 1;
            current.y = limit.y - 1;
        }
        let _ = self.goto(current.x + (current.y * limit.x));
    }

    /// The method `scroll_down` append an empty line on bottom of the screen
    /// (the cursor doesn't move)
    pub fn scroll_down(&mut self, base: libc::size_t)
    { let col = self.table.size.get_col();
      let collection = self.table.collection;
      if self.table.show_cursor
      { self.clear_cursor(); }
      let resize = self.table.region;
      if !self.table.newline.is_empty()
      { match self.table.newline.into_iter().position(|pos| pos.y.eq(&(resize.y - 1)).bitand(pos.y.eq(&(self.table.size.get_row() - 1)).not()))
        { Some(n) => { 
        self.table.newline.remove(n); 
        },
          None => {}, }
            self.table.newline.scroll_down(resize);
      }
      self.table.newline.push(Coordinate::from((self.table.size.get_col() - 1, resize.x)));
      self.table.newline.sort_by();
      self.table.newline.dedup();
      let coucou = self.table.screen.get_mut();
      {0..col}.all(|_|
      { (*coucou).insert(base * col, collection);
        (*coucou).remove(resize.y * col);
        true }); 
    }

    //pos.y.gt(&base).bitand(pos.y.lt(&resize.y
    /// The method `scroll_up` insert an empty line on top of the screen
    /// (the cursor doesn't move)
    pub fn scroll_up(&mut self, base: libc::size_t)
    { let col = self.table.size.get_col();
      let collection = self.table.collection;
      if self.table.show_cursor
      { self.clear_cursor(); }
      let resize = self.table.region;
      if !self.table.newline.is_empty()
      {
      match self.table.newline.into_iter().position(|pos| {
      pos.y.eq(&base)
      })
        { Some(n) => {
        self.table.newline.remove(n); },
          None => {}, }
        self.table.newline.scroll_up(resize, &base);
        }
      self.table.newline.push(Coordinate::from((self.table.size.get_col() - 1, resize.y - 1)));
      self.table.newline.sort_by();
      self.table.newline.dedup();
      let coucou = self.table.screen.get_mut();
      {0..col}.all(|_|
      { (*coucou).insert(resize.y * col, collection);
        (*coucou).remove(base * col);
        true }); }

    /// The method `save_position` save a position in the variable 'save_position' to get
    /// restored with self.table.restore_position() described right after.
    /// If save_position() is called many times, only the newest safe will be kept.
    pub fn save_position(&mut self) {
        self.table.save_position = self.table.oob;
    }

    /// The method `restore_position` move the cursor to coordinates safe
    /// with self.table.save_position() described right before.
    /// If no coordinates were safe, cursor moves to the top left of the output screen
    pub fn restore_position(&mut self) {
        let position: Coordinate = self.table.save_position;

        self.goto_coord(position);
    }

    /// The method `insert_empty_line` insert an empty line on the right of the cursor
    /// (the cursor doesn't move)
    pub fn insert_empty_line(&mut self, mv: libc::size_t)
    { let col = self.table.size.get_col();
      let region = self.table.region;
      let pos = self.table.screen.position();
      let collection = self.table.collection;
      let coucou = self.table.screen.get_mut();
      {0..(col * mv)}.all(|_|
      { (*coucou).insert(pos, collection);
        { (*coucou).remove(region.y * col); }
        true }); }

    /// The method `erase_right_line` erase the current line from the cursor
    /// to the next '\n' encountered
    /// (char under the cursor included)
    pub fn erase_right_line(&mut self, current: libc::size_t)
    { let col = self.table.size.get_col();
      let collection = self.table.collection;
      if !self.table.newline.is_empty()
      { match self.table.newline.into_iter().position(|pos| pos.y.ge(&(current/col)))
        { Some(n) =>
            { match self.table.newline[n].y.checked_mul(col)
              { Some(r) =>
                  { match r.checked_add(self.table.newline[n].x.add(&1))
                    { Some(k) =>
                        { match k.checked_sub(current)
                          { Some(j) =>
                              { self.table.screen.get_mut().into_iter().skip(current).take(j).all(|mut term: &mut Character|  { *term = collection;
                                      true }); },
                            None => { self.erase_down(); }, }},
                      None => { self.erase_down(); }, }},
                None => { self.erase_down(); }, }},
          None =>
            { self.erase_down(); }, }}
      else
      { self.erase_down(); }}

    /// The method `erase_left_line` erase the current line from the previous '\n'
    /// to the cursor
    /// (char under the cursor included)
    pub fn erase_left_line(&mut self, current: libc::size_t)
    { let col = self.table.size.get_col();
      let collection = self.table.collection;
      if !self.table.newline.is_empty()
      { self.table.newline.reverse();
        match self.table.newline.into_iter().position(|pos| pos.y.lt(&(current/col)))
        { Some(n) =>
            { match self.table.newline[n].y.checked_mul(col)
              { Some(r) =>
                  { match r.checked_add(self.table.newline[n].x)
                    { Some(k) =>
                        { match current.add(&1).checked_sub(k)
                          { Some(j) =>
                              { self.table.screen.get_mut().into_iter().skip(k).take(j).all(|mut term: &mut Character|  { *term = collection;
                                    true }); },
                            None => { self.erase_up(); }, }},
                      None => { self.erase_up(); }, }},
                None => { self.erase_up(); }, }},
          None =>
            { self.erase_up(); }, }
        self.table.newline.reverse(); }
      else
      { self.erase_up(); }}

    /// The method `erase_line` erase the entire current line
    pub fn erase_line(&mut self, mv: libc::size_t)
    { let col = self.table.size.get_col();
      let mut x = self.table.oob.x;
      let mut y = self.table.oob.y;
      {0..mv}.all(|_|
      { match self.table.newline.into_iter().position(|pos| pos.y.ge(&y))
        { Some(n) =>
            { self.erase_left_line(x + (y * col));
              self.erase_right_line(x + (y * col));
              x = self.table.newline[n].x;
              y = self.table.newline[n].y + 1;
              true },
          None => { self.erase_down() ; false }, }}); }

    /// The method `erase_up` erase all lines from the current line up to
    /// the top of the screen, and erase the current line from the left border
    /// column to the cursor.
    /// (char under the cursor included)
    pub fn erase_up(&mut self)
    { let pos = self.table.screen.position();
      let collection = self.table.collection;
      self.table.screen.get_mut().into_iter().take(pos + 1).all(|mut term: &mut Character|
      { *term = collection;
        true }); }

    /// The method `erase_down` erase all lines from the current line down to
    /// the bottom of the screen and erase the current line from the cursor to
    /// the right border column
    /// (char under the cursor included)
    pub fn erase_down(&mut self)
    { let pos = self.table.screen.position();
      let len = self.table.size.row_by_col();
      let collection = self.table.collection;
      self.table.screen.get_mut().into_iter().skip(pos).take(len - pos + 1).all(|mut term: &mut Character|
      { *term = collection;
        true }); }

    /// The method `print_enter` reproduce the behavior of a '\n'
    pub fn print_enter(&mut self)
    { if self.table.oob.y.lt(&(self.table.region.y.sub(&1)))
      { let _ = self.goto_down(1); }
      else if self.table.oob.y.eq(&(self.table.region.y.sub(&1)))
      { let x = self.table.region.x;
        self.scroll_up(x); }}

    /// The method `print_char` print an unicode character (1 to 4 chars range)
    pub fn print_char(&mut self, first: char, next: &[u8]) -> io::Result<usize>
    { let col = self.table.size.get_col();
        if self.table.show_cursor
        { self.clear_cursor(); }
        if self.table.oob.x < col - 1
        { self.table.oob.x += 1; }
        else if self.table.oob.y.lt(&self.table.region.y.sub(&1))
        { if self.table.newline.is_empty().not().bitand(self.table.ss_mod.not())
            { match self.table.newline.into_iter().position(|pos| pos.y.eq(&self.table.oob.y))
                { Some(n) => { self.table.newline.remove(n); },
                    None => {}, }; }
            self.table.oob.y += 1;
            self.table.oob.x = 0; }
        else if self.table.oob.y.eq(&(self.table.region.y.sub(&1)))
        { let x = self.table.region.x;
            self.scroll_up(x);
            let _ = self.goto_begin_row();
            { let pos = self.table.screen.position();
                if pos.gt(&0)
                { let _ = self.goto(pos - 1); }}}
        else
        { let pos = self.table.screen.position();
            if pos.gt(&0)
            { let _ = self.goto(pos - 1); }}
        self.table.screen.write_with_color(first, self.table.collection).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) ))
    }

    pub fn catch_numbers<'a>(&self, mut acc: Vec<libc::size_t>, buf: &'a [u8]) -> (Vec<libc::size_t>, &'a [u8])
    { match parse_number!(buf)
      { Some((number, &[b';', ref next..])) =>
          { acc.push(number);
            self.catch_numbers(acc, next) },
        Some((number, &[ref next..])) =>
          { acc.push(number);
            (acc, next) },
        _ =>
          { (acc, buf) }, }}

    /// The method `next_tab` return the size of the current printed tabulation
    pub fn next_tab(&self) -> libc::size_t
    { 8 - (self.table.oob.x % 8) }

    /// The method `save_terminal` saves the terminal Display configuration.
    fn save_terminal(&mut self) {
        self.save_terminal = Some(self.table.clone());
    }

    /// The method `restore_terminal` restore the terminal Display configuration
    /// kept in the 'save_terminal' variable.
    fn restore_terminal(&mut self)
    {
        if let Some(save_terminal) = self.save_terminal.clone() {
            self.table.save_position = save_terminal.save_position;
            self.table.show_cursor = save_terminal.show_cursor;
            self.table.mouse_handle = save_terminal.mouse_handle;
            self.table.ss_mod = save_terminal.ss_mod;
            self.table.newline = save_terminal.newline.clone();
            self.table.region = save_terminal.region;
            self.table.collection = save_terminal.collection;
            self.table.oob = save_terminal.oob;
            self.table.line_wrap = save_terminal.line_wrap;
            self.table.screen = save_terminal.screen.clone();
            self.table.bell = save_terminal.bell;
            if self.table.size != save_terminal.size {
                self.table.size = save_terminal.size;
                let _ = self.resize();
            }
        }
        if self.save_terminal.is_some() {
            self.save_terminal = None;
        }
        let position: Coordinate = self.table.oob;
        self.goto_coord(position);
    }

    /// The method `erase_chars` erases couple of chars in the current line from the cursor.
    pub fn erase_chars(&mut self, mv: libc::size_t)
    { let pos = self.table.screen.position();
      let border = match self.table.newline.into_iter().position(|pos| pos.y.ge(&self.table.oob.y))
      { Some(n) => self.table.newline[n].x + (self.table.newline[n].y * self.table.size.get_col()) + 1,
        None => self.table.size.row_by_col() - 1, };
      let coucou = self.table.screen.get_mut();
      let collection = self.table.collection;
      {0..mv}.all(|_|
      { (*coucou).insert(border, collection);
        (*coucou).remove(pos);
        true }); }

    /// The method `erase_chars` erases couple of chars in the current line from the cursor.
    pub fn insert_chars(&mut self, mv: libc::size_t)
    { let pos = self.table.screen.position();
      let border = match self.table.newline.into_iter().position(|pos| pos.y.ge(&self.table.oob.y))
      { Some(n) => self.table.newline[n].x + (self.table.newline[n].y * self.table.size.get_col()) + 1,
        None => self.table.size.row_by_col() - 1, };
      let coucou = self.table.screen.get_mut();
      let collection = self.table.collection;
      {0..mv}.all(|_|
      { (*coucou).insert(pos, collection);
        (*coucou).remove(border);
        true }); }

    /// Reset the color.
    fn clear_cursor(&mut self) {
        let pos = self.table.screen.position();
        let collection = self.table.collection;
//        let cursor = self.table.cursor;

        if let Some(character) = self.table.screen.get_mut().get_mut(pos) {
          character.set_attribute_from_u8(collection.get_attribute());
          character.set_foreground(collection.get_foreground());
          character.set_background(collection.get_background());

/*          character.set_attribute_from_u8(cursor.get_attribute());
          character.set_foreground(cursor.get_foreground());
          character.set_background(cursor.get_background());*/
        }
    }

    /// Color the cursor.
    fn color_cursor(&mut self) {
        let pos = self.table.screen.position();

        if let Some(character) = self.table.screen.get_mut().get_mut(pos) {
//            self.table.cursor = *character;
            character.set_attribute(Attribute::Dim);
            character.set_foreground([255, 0, 0]);
            character.set_background([0, 255, 255]);
        }
    }
}

impl<'a> IntoIterator for &'a Display {
    type Item = &'a Character;
    type IntoIter = ::std::slice::Iter<'a, Character>;

    fn into_iter(self) -> Self::IntoIter {
        self.table.screen.get_ref().into_iter()
    }
}

impl Default for Display {
    fn default() -> Display {
        unsafe {
            Display {
                save_terminal: mem::zeroed(),
                table: mem::zeroed(),
            }
        }
    }
}

impl fmt::Display for Display
{ fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  { let mut disp: String = String::new();
      let width: usize = self.table.size.get_col() as usize;
    self.into_iter().as_slice()
        .chunks(width)
        .all(|characters| {
        characters.iter().all(|character| {
     disp.push_str(format!("{}", character).as_str());
      true });
        disp.push('\n');
        true
        });
    write!(f, "{}", &disp[..disp.len().checked_sub(1).unwrap_or_default()]) }}

impl Write for Display {
    /// The method `write` from trait `io::Write` inserts a new list of terms
    /// from output.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
      if self.table.size.get_row().gt(&0).bitand(self.table.size.get_col().gt(&0))
      { match buf {
            &[] =>
              { if self.table.show_cursor
                { self.color_cursor(); }
                Ok(0) },

            //---------- TERMINAL SAVE -----------
            &[b'\x1B', b'[', b'?', b'1', b'0', b'4', b'9', b'h', ref next..] =>
              { self.save_terminal();
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'4', b'9', b'l', ref next..] =>
              { self.restore_terminal();
                self.write(next) },

            //----------.mouse_handle -----------
            &[b'\x1B', b'[', b'?', b'9', b'h', ref next..] =>
              { self.table.mouse_handle.0 = true;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'9', b'l', ref next..] =>
              { self.table.mouse_handle.0 = false;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'0', b'0', b'h', ref next..] =>
              { self.table.mouse_handle.1 = true;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'0', b'0', b'l', ref next..] =>
              { self.table.mouse_handle.1 = false;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'0', b'2', b'h', ref next..] =>
              { self.table.mouse_handle.2 = true;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'0', b'2', b'l', ref next..] =>
              { self.table.mouse_handle.2 = false;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'0', b'6', b'h', ref next..] =>
              { self.table.mouse_handle.3 = true;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'0', b'6', b'l', ref next..] =>
              { self.table.mouse_handle.3 = false;
                self.write(next) },

            //------------ SETTINGS -------------
            &[b'\x1B', b'c', ref next..] =>
              { self.write(next) },
            &[b'\x1B', b'[', b'>', b'0', b'c', ref next..] |
            &[b'\x1B', b'[', b'>', b'c', ref next..] =>
              { self.write(next) },
            &[b'\x1B', b'[', b'?', b'7', b'h', ref next..] |
            &[b'\x1B', b'[', b'2', b'0', b'h', ref next..] =>
              { self.table.line_wrap = true;
                self.write(next) },
            &[b'\x1B', b'[', b'7', b'l', ref next..] |
            &[b'\x1B', b'[', b'2', b'0', b'l', ref next..] =>
              { self.table.line_wrap = false;
                self.write(next) },
            &[b'\x1B', b'[', b'r', ref next..] =>
              { self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'h', ref next..] =>
              { self.table.ss_mod = true;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'l', ref next..] =>
              { self.table.ss_mod = false;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'2', b'5', b'h', ref next..] =>
              { self.table.show_cursor = true;
                self.color_cursor();
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'2', b'5', b'l', ref next..] =>
              { self.table.show_cursor = false;
                self.clear_cursor();
                self.write(next) },

            //------------ ERASE -----------------
            &[b'\x1B', b'[', b'K', ref next..] |
            &[b'\x1B', b'[', b'0', b'K', ref next..] =>
              { let pos = self.table.screen.position();
                self.erase_right_line(pos);
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'K', ref next..] =>
              { let pos = self.table.screen.position();
                self.erase_left_line(pos);
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'K', ref next..] =>
              { self.erase_line(1);
                self.write(next) },
            &[b'\x1B', b'[', b'J', ref next..] |
            &[b'\x1B', b'[', b'0', b'J', ref next..] =>
              { self.erase_down();
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'J', ref next..] =>
              { self.erase_up();
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'J', ref next..] => self.clear().and(self.write(next)),
            &[b'\x1B', b'[', b'P', ref next..] =>
              { self.erase_chars(1);
                self.write(next) },

            //------------ INSERT -----------------
            &[b'\x1B', b'[', b'L', ref next..] =>
              { if self.table.oob.y.ge(&self.table.region.x).bitand(self.table.oob.y.lt(&self.table.region.y))
                { self.insert_empty_line(1); }
                self.write(next) },
            &[b'\x1B', b'[', b'@', ref next..] =>
              { self.insert_chars(1);
                self.write(next) },

            //------------- GOTO ------------------
            &[b'\x1B', b'[', b';', b'H', ref next..] |
            &[b'\x1B', b'[', b';', b'f', ref next..] |
            &[b'\x1B', b'[', b'd', ref next..] |
            &[b'\x1B', b'[', b'H', ref next..] |
            &[b'\x1B', b'[', b'f', ref next..] =>
              { let _ = self.goto_home();
                self.write(next) },
            &[b'\x1B', b'[', b'A', ref next..] |
            &[b'\x1B', b'[', b'm', b'A', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'A', ref next..] =>
              { let _ = self.goto_up(1);
                self.write(next) },
            &[b'A', b'\x08'] =>
              { let _ = self.goto_up(1);
                Ok(0) },
            &[b'\x1B', b'[', b'B', ref next..] |
            &[b'\x1B', b'[', b'm', b'B', b'\x08', ref next..] |
            &[b'\x1B', b'D', ref next..] |
            &[b'\x1B', b'O', b'B', ref next..] =>
              { let _ = self.goto_down(1);
                self.write(next) },
            &[b'B', b'\x08'] =>
              { let _ = self.goto_down(1);
                Ok(0) },
            &[b'\x1B', b'[', b'C', ref next..] |
            &[b'\x1B', b'[', b'm', b'C', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'C', ref next..] =>
              { let _ = self.goto_right(1);
                self.write(next) },
            &[b'C', b'\x08'] =>
              { let _ = self.goto_right(1);
                Ok(0) },
            &[b'\x1B', b'[', b'D', ref next..] |
            &[b'\x1B', b'[', b'm', b'D', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'D', ref next..] |
            &[b'\x08', ref next..] =>
              { let _ = self.goto_left(1);
                self.write(next) },
            &[b'D', b'\x08'] =>
              { let _ = self.goto_left(1);
                Ok(0) },

            //--------- POSITION SAVE ----------
            &[b'\x1B', b'[', b's', ref next..] |
            &[b'\x1B', b'7', ref next..] =>
              { self.save_position();
                self.write(next) },
            &[b'\x1B', b'[', b'u', ref next..] |
            &[b'\x1B', b'8', ref next..] =>
              { self.restore_position();
                self.write(next) },

            //------------- SCROLL ---------------
            &[b'\x1B', b'M', ref next..] =>
              { if self.table.oob.y.ge(&self.table.region.x).bitand(self.table.oob.y.lt(&self.table.region.y))
                { let x = self.table.oob.y;
                  if !self.table.ss_mod
                  { self.scroll_up(x); }
                  else
                  { self.scroll_down(x); }}
                self.write(next) },
            &[b'\x1B', b'[', b'M', ref next..] =>
              { if self.table.oob.y.ge(&self.table.region.x).bitand(self.table.oob.y.lt(&self.table.region.y))
                { let x = self.table.oob.y;
                  self.scroll_up(x); }
                self.write(next) },
            &[b'\x1B', b'[', b'S', ref next..] |
            &[b'\x1B', b'S', ref next..] =>
              { if self.table.oob.y.ge(&self.table.region.x).bitand(self.table.oob.y.lt(&self.table.region.y))
                { let x = self.table.region.x;
                  self.scroll_up(x); }
                self.write(next) },
            &[b'\x1B', b'L', ref next..] =>
              { if self.table.oob.y.ge(&self.table.region.x).bitand(self.table.oob.y.lt(&self.table.region.y))
                { let x = self.table.oob.y;
                  self.scroll_down(x); }
                self.write(next) },
            &[b'\x1B', b'[', b'T', ref next..] |
            &[b'\x1B', b'T', ref next..] =>
              { if self.table.oob.y.ge(&self.table.region.x).bitand(self.table.oob.y.lt(&self.table.region.y))
                { let x = self.table.region.x;
                  self.scroll_down(x); }
                self.write(next) },

            //------------ CL ATTR -------------
            &[b'\x1B', b'[', b'?', b'1', b'2', b'l', ref next..] |
            &[b'\x1B', b'[', b'0', b'm', ref next..] |
            &[b'\x1B', b'[', b'm', ref next..] =>
              { self.table.collection.clear();
                self.write(next) },

            &[b'\x1B', b'[', b'?', ref next..] |
            &[b'\x1B', b'[', b'>', ref next..] |
            &[b'\x1B', b'[', ref next..] |
            &[b'\x1B', b']', ref next..] |
            &[b'\x1B', b'(', ref next..] |
            &[b'\x1B', b'?', ref next..] |
            &[b'\x1B', ref next..] =>
            { let (bonjour, coucou) =
              { self.catch_numbers(Vec::new(), next) };
              match coucou
              { //------------- n GOTO ------------------
                &[b'A', ref next..] =>
                  { if bonjour.len() == 1
                    { let _ = self.goto_up(bonjour[0]); }
                    self.write(next) },
                &[b'B', ref next..] =>
                  { if bonjour.len() == 1
                    { let _ = self.goto_down(bonjour[0]); }
                    self.write(next) },
                &[b'C', ref next..] =>
                  { if bonjour.len() == 1
                    { let _ = self.goto_right(bonjour[0]); }
                    self.write(next) },
                &[b'D', ref next..] =>
                  { if bonjour.len() == 1
                    { let _ = self.goto_left(bonjour[0]); }
                    self.write(next) },
                &[b'G', ref next..] =>
                  { if bonjour.len() == 1 && self.table.size.get_col() >= bonjour[0]
                    { let y = self.table.oob.y;
                      let _ = self.goto_coord(Coordinate::from((bonjour[0] - 1, y))); }
                    self.write(next) },
                &[b'd', ref next..] =>
                  { if bonjour.len() == 1 && self.table.size.get_row() >= bonjour[0]
                    { let x = self.table.oob.x;
                      let _ = self.goto_coord(Coordinate::from((x, bonjour[0] - 1))); }
                    self.write(next) },
                &[b'H', ref next..] |
                &[b'f', ref next..] => {
                    if bonjour.len() == 2 && bonjour[0] > 0 && bonjour[1] > 0 {
                        let _ = self.goto_coord(Coordinate::from((bonjour[1] - 1, bonjour[0] - 1)));
                    }
                    self.write(next)
                },
                //-------------- ERASE ----------------
                &[b'P', ref next..] =>
                  { if bonjour.len().eq(&1)
                    { self.erase_chars(bonjour[0]); }
                    self.write(next) },
                &[b'@', ref next..] =>
                  { if bonjour.len().eq(&1)
                    { self.insert_chars(bonjour[0]); }
                    self.write(next) },

                //-------------- SCROLL ----------------
                &[b'M', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.table.oob.y.ge(&self.table.region.x)).bitand(self.table.oob.y.lt(&self.table.region.y))
                    { let x = self.table.oob.y;
                      {0..bonjour[0]}.all(|_|
                      { self.scroll_up(x);
                        true }); }
                    self.write(next) },
                &[b'S', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.table.oob.y.ge(&self.table.region.x)).bitand(self.table.oob.y.lt(&self.table.region.y))
                    { let x = self.table.region.x;
                      {0..bonjour[0]}.all(|_|
                      { self.scroll_up(x);
                        true }); }
                    self.write(next) },
                &[b'L', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.table.oob.y.ge(&self.table.region.x)).bitand(self.table.oob.y.lt(&self.table.region.y))
                    { let x = self.table.oob.y;
                      {0..bonjour[0]}.all(|_|
                      { self.scroll_down(x);
                        true }); }
                    self.write(next) },
                &[b'T', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.table.oob.y.ge(&self.table.region.x)).bitand(self.table.oob.y.lt(&self.table.region.y))
                    { let x = self.table.region.x;
                      {0..bonjour[0]}.all(|_|
                      { self.scroll_down(x);
                        true }); }
                    self.write(next) },

                //------------- ATTRIBUTS ---------------
		            &[b'm', b'%', ref next..] |
                &[b'm', ref next..] =>
                  { //if self.table.show_cursor
                    { bonjour.iter().all(|&attr|
                      { match attr
                        { 0 => { self.table.collection.clear(); },

                          //Set special attributes
                          1 => {
                              self.table.collection.add_attribute(Attribute::Bold);
                          },
                          2 => {
                              self.table.collection.add_attribute(Attribute::Dim);
                          },
                          3 => {
                              self.table.collection.add_attribute(Attribute::Italic);
                          },
                          4 => {
                              self.table.collection.add_attribute(Attribute::Underline);
                          },
                          5 => {
                              self.table.collection.add_attribute(Attribute::Blink);
                          },
                          7 => {
                              self.table.collection.add_attribute(Attribute::Reverse);
                          },
                          8 => {
                              self.table.collection.add_attribute(Attribute::Hidden);
                          },

                          //Unset special attributes
                          22 => {
                              self.table.collection.sub_attribute(Attribute::Bold);
                              self.table.collection.sub_attribute(Attribute::Dim);
                          },
                          23 => { self.table.collection.sub_attribute(Attribute::Italic); },
                          24 => { self.table.collection.sub_attribute(Attribute::Underline); },
                          25 => { self.table.collection.sub_attribute(Attribute::Blink); },
                          27 => { self.table.collection.sub_attribute(Attribute::Reverse); },
                          28 => { self.table.collection.sub_attribute(Attribute::Hidden); },

                          //Foreground colors
                          30 => { self.table.collection.set_foreground(color::BLACK); },
                          31 => { self.table.collection.set_foreground(color::RED); },
                          32 => { self.table.collection.set_foreground(color::GREEN); },
                          33 => { self.table.collection.set_foreground(color::YELLOW); },
                          34 => { self.table.collection.set_foreground(color::BLUE); },
                          35 => { self.table.collection.set_foreground(color::MAGENTA); },
                          36 => { self.table.collection.set_foreground(color::CYAN); },
                          37 => { self.table.collection.set_foreground(color::WHITE); },
                          39 => { self.table.collection.set_foreground(color::BLACK); },

                          //Background colors
                          40 => { self.table.collection.set_background(color::BLACK); },
                          41 => { self.table.collection.set_background(color::RED); },
                          42 => { self.table.collection.set_background(color::GREEN); },
                          43 => { self.table.collection.set_background(color::YELLOW); },
                          44 => { self.table.collection.set_background(color::BLUE); },
                          45 => { self.table.collection.set_background(color::MAGENTA); },
                          46 => { self.table.collection.set_background(color::CYAN); },
                          47 => { self.table.collection.set_background(color::WHITE); },
                          49 => { self.table.collection.set_background(color::WHITE); },

                          _ => {}, }
                        true }); }
                    self.write(next) },

                //----------- TRICKY RESIZE -------------
                &[b'r', ref next..] =>
                  { if bonjour.len() == 2
                    { self.tricky_resize(bonjour[0], bonjour[1]); }
                    self.write(next) },

                //----------- TERM VERSION --------------
                &[b'c', ref next..] =>
                  { self.write(next) },
                &[b';', b'c', ref next..] =>
                  { self.write(next) },

                &[_, ref next..] |
                &[ref next..] =>
                  { self.write(next) }, }},

            &[b'\x07', ref next..] =>
              { self.table.bell += 1;
                self.write(next) },
            &[b'\x0A', b'\x0D', ref next..] |
            &[b'\x0A', ref next..] |
            &[b'\x0D', b'\x0A', ref next..] =>
              { self.print_enter();
                let _ = self.goto_begin_row();
                self.write(next) },
            &[b'\x0D', ref next..] =>
              { let _ = self.goto_begin_row();
                self.write(next) },

            &[b'\x09', ref next..] =>
            { let resize = self.table.region;
              let col = self.table.size.get_col();
              let tab_width = self.next_tab();
              let pos = self.table.screen.position();
              { let coucou = self.table.screen.get_mut();
                {0..tab_width}.all(|_|
                { (*coucou).insert(pos, Character::default());
                  (*coucou).remove(resize.y * col);
                  true }); }
              let _ = self.goto_right(tab_width);
              self.write(next) },

            &[u1, ref next..] =>
            { if u1 & 0b10000000 == 0
              { if u1 > 0
								{ self.print_char(unsafe { mem::transmute::<[u8; 4], char>([u1, 0, 0, 0]) }, next) }
								else
								{ self.print_char(unsafe { mem::transmute::<[u8; 4], char>([b' ', 0, 0, 0]) }, next) }}
              else if (u1 & 0b11111000) == 0b11110000
              { match next
                { &[u2, u3, u4, ref next..] =>
                  { let mut m = 0u32;
                    m |= (u1 as u32 & 0x07) << 18;
                    m |= (u2 as u32 & 0x3F) << 12;
                    m |= (u3 as u32 & 0x3F) << 6;
                    m |= u4 as u32 & 0x3F;
                    self.print_char(unsafe { mem::transmute::<u32, char>(m) }, next) },
                  _ => self.write(next), }}
              else if (u1 & 0b11110000) == 0b11100000
              { match next
                { &[u2, u3, ref next..] =>
                  { let mut m = 0u32;
                    m |= (u1 as u32 & 0x0F) << 12;
                    m |= (u2 as u32 & 0x3F) << 6;
                    m |= u3 as u32 & 0x3F;
                    self.print_char(unsafe { mem::transmute::<u32, char>(m) }, next) },
                  _ => self.write(next), }}
              else if (u1 & 0b11100000) == 0b11000000
              { match next
                { &[u2, ref next..] =>
                  { let mut m = 0u32;
                    m |= (u1 as u32 & 0x3F) << 6;
                    m |= u2 as u32 & 0x3F;
                    self.print_char(unsafe { mem::transmute::<u32, char>(m) }, next) },
                  _ => self.write(next), }}
              else
              { self.write(next) }
            },
        }}
        else
        { Ok(0) }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
