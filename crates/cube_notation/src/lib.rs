use cube::{map_orientation, rotate_face, Axis, Cube, EdgeSticker, Face, RotatedCube};
use std::{
    fmt::{self, Display},
    mem::swap,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub n: u16,
    pub face: Face,
    pub start: u16,
    pub end: u16,
    pub count: u8,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_move(f, *self)
    }
}

impl Move {
    pub fn inverse(self) -> Move {
        Move {
            n: self.n,
            face: self.face,
            start: self.start,
            end: self.end,
            count: 4 - self.count % 4,
        }
    }

    pub fn is_rotation(&self) -> bool {
        self.start == 0 && self.end == self.n
    }
}

#[derive(Debug)]
struct Canceler {
    moves: Vec<Move>,
    orientation: EdgeSticker,
}

impl Canceler {
    fn cancel_at(&mut self, count: usize) {
        if self.moves.len() < 1 + count {
            return;
        }

        let i = self.moves.len() - 1 - count;
        if self.moves[i].count == 0 {
            self.moves.remove(i);
            return self.cancel_at(count);
        }

        if self.moves.len() < 2 + count {
            return;
        }

        let i = self.moves.len() - 2 - count;
        let [a, b, ..] = &mut self.moves[i..] else {
            unreachable!()
        };

        if b.count == 0 {
            self.moves.remove(i + 1);
            return self.cancel_at(count);
        }

        if a.count == 0 {
            self.moves.remove(i);
            return self.cancel_at(count);
        }

        if a.face == b.face && a.start == b.start && a.end == b.end {
            a.count += b.count;
            a.count %= 4;
            self.moves.remove(i + 1);
            self.cancel_at(count);
            return;
        }

        if a.face == b.face && a.end < b.end || a.face == b.face.opposite() && a.face < b.face {
            swap(a, b);
            self.cancel_at(count + 1);
            self.cancel_at(count);
        }
    }

    fn cancel_mapping_orientation(&mut self, mut mv: Move) {
        mv.face = map_orientation(self.orientation, mv.face);
        self.cancel(mv);
    }

    fn cancel(&mut self, mv: Move) {
        if mv.start == 0 && mv.end == mv.n {
            let (axis, invert) = match mv.face {
                Face::R => (Axis::X, false),
                Face::L => (Axis::X, true),
                Face::U => (Axis::Y, false),
                Face::D => (Axis::Y, true),
                Face::F => (Axis::Z, false),
                Face::B => (Axis::Z, true),
            };
            let count = if invert { 4 - mv.count % 4 } else { mv.count };
            self.orientation = EdgeSticker::from_faces(
                rotate_face(map_orientation(self.orientation, Face::U), axis, count),
                rotate_face(map_orientation(self.orientation, Face::F), axis, count),
            );
            return;
        }

        if mv.start != 0 {
            self.cancel(Move {
                n: mv.n,
                face: mv.face,
                start: 0,
                end: mv.end,
                count: mv.count,
            });
            self.cancel(Move {
                n: mv.n,
                face: mv.face,
                start: 0,
                end: mv.start,
                count: 4 - mv.count % 4,
            });
            return;
        }

        if (mv.end > mv.n / 2
            || (mv.end == mv.n / 2
                && mv.n % 2 == 0
                && mv.face.is_less_ergonomic_than_the_opposite_face()))
            && mv.end != mv.n
        {
            let opposite = Move {
                n: mv.n,
                face: mv.face.opposite(),
                start: 0,
                end: mv.n - mv.end,
                count: mv.count,
            };
            let rotation = Move {
                n: mv.n,
                face: mv.face,
                start: 0,
                end: mv.n,
                count: mv.count,
            };
            self.cancel(opposite);
            self.cancel(rotation);
            return;
        }

        self.moves.push(mv);
        self.cancel_at(0);
    }

    fn new() -> Canceler {
        Canceler {
            moves: Vec::new(),
            orientation: EdgeSticker::Uf,
        }
    }
}

impl Extend<Move> for Canceler {
    fn extend<T: IntoIterator<Item = Move>>(&mut self, iter: T) {
        for mov in iter {
            self.cancel_mapping_orientation(mov);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    None,
    Braces,
    Brackets,
    Parens,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree {
    Move(Move),
    Group(Delimiter, Vec<Tree>),
    Conj(Delimiter, Box<Tree>, Box<Tree>),
    Comm(Delimiter, Box<Tree>, Box<Tree>),
    Slash(Delimiter, Box<Tree>, Box<Tree>),
}

impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_tokens(f, &self.to_tokens())
    }
}

impl Tree {
    // TODO test if this really works for the inverse in all cases
    fn visit_moves_internal(&self, f: &mut impl FnMut(Move), invert: bool) {
        match self {
            &Tree::Move(mv) => {
                if invert {
                    f(mv.inverse());
                } else {
                    f(mv);
                }
            }
            Tree::Group(_, trees) => {
                if invert {
                    for tree in trees.iter().rev() {
                        tree.visit_moves_internal(f, invert);
                    }
                } else {
                    for tree in trees {
                        tree.visit_moves_internal(f, invert);
                    }
                }
            }
            Tree::Conj(_, a, b) => {
                if invert {
                    a.visit_moves_internal(f, !invert);
                    b.visit_moves_internal(f, invert);
                    a.visit_moves_internal(f, invert);
                } else {
                    a.visit_moves_internal(f, invert);
                    b.visit_moves_internal(f, invert);
                    a.visit_moves_internal(f, !invert);
                }
            }
            Tree::Comm(_, a, b) => {
                if invert {
                    b.visit_moves_internal(f, !invert);
                    a.visit_moves_internal(f, !invert);
                    b.visit_moves_internal(f, invert);
                    a.visit_moves_internal(f, invert);
                } else {
                    a.visit_moves_internal(f, invert);
                    b.visit_moves_internal(f, invert);
                    a.visit_moves_internal(f, !invert);
                    b.visit_moves_internal(f, !invert);
                }
            }
            Tree::Slash(_, a, b) => {
                if invert {
                    a.visit_moves_internal(f, !invert);
                    b.visit_moves_internal(f, invert);
                    a.visit_moves_internal(f, !invert);
                    a.visit_moves_internal(f, !invert);
                    b.visit_moves_internal(f, !invert);
                    a.visit_moves_internal(f, !invert);
                } else {
                    a.visit_moves_internal(f, invert);
                    b.visit_moves_internal(f, invert);
                    a.visit_moves_internal(f, invert);
                    a.visit_moves_internal(f, invert);
                    b.visit_moves_internal(f, !invert);
                    a.visit_moves_internal(f, invert);
                }
            }
        }
    }

    pub fn visit_moves(&self, mut f: impl FnMut(Move)) {
        self.visit_moves_internal(&mut f, false)
    }

    pub fn visit_inverse_moves(&self, mut f: impl FnMut(Move)) {
        self.visit_moves_internal(&mut f, true)
    }

    pub fn apply_to(&self, cube: &mut Cube) {
        let mut cube = RotatedCube::new(cube);
        self.visit_moves_internal(
            &mut |mv| cube.rotate(mv.face, mv.start..mv.end, mv.count),
            false,
        );
    }

    pub fn apply_inverse_to(&self, cube: &mut Cube) {
        let mut cube = RotatedCube::new(cube);
        self.visit_moves_internal(
            &mut |mv| cube.rotate(mv.face, mv.start..mv.end, mv.count),
            true,
        );
    }

    pub fn to_moves(&self) -> Vec<Move> {
        let mut result = Vec::new();
        self.visit_moves_internal(&mut |mv| result.push(mv), false);
        result
    }

    pub fn to_canonical_moves(&self) -> Vec<Move> {
        let mut canceler = Canceler::new();
        let mut n = None;
        self.visit_moves_internal(
            &mut |mv| {
                n = Some(mv.n);
                canceler.cancel_mapping_orientation(mv)
            },
            false,
        );
        let mut result = canceler.moves;
        result.extend(rotate_from(n.unwrap(), canceler.orientation));
        result
    }

    pub fn to_inverse_moves(&self) -> Vec<Move> {
        let mut result = Vec::new();
        self.visit_moves_internal(&mut |mv| result.push(mv), true);
        result
    }

    pub fn to_tokens(&self) -> Vec<Token> {
        fn collect(tree: &Tree, out: &mut Vec<Token>) {
            match tree {
                &Tree::Move(mv) => {
                    out.push(Token::Move(mv));
                }
                Tree::Group(_delim, trees) => {
                    for tree in trees {
                        collect(tree, out);
                    }
                }
                Tree::Conj(_delim, a, b) | Tree::Comm(_delim, a, b) | Tree::Slash(_delim, a, b) => {
                    out.push(Token::LBracket);
                    collect(a, out);
                    if let Tree::Slash(..) = tree {
                        out.push(Token::Space);
                    }
                    out.push(match tree {
                        Tree::Conj(..) => Token::Colon,
                        Tree::Comm(..) => Token::Comma,
                        Tree::Slash(..) => Token::Slash,
                        Tree::Move(..) | Tree::Group(..) => unreachable!(),
                    });
                    out.push(Token::Space);
                    collect(b, out);
                    out.push(Token::RBracket);
                }
            }
        }

        let mut result = Vec::new();
        collect(self, &mut result);
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    LAngle,
    RAngle,
    Comma,
    Colon,
    Slash,
    Space,
    Move(Move),
    End,
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl Parser<'_> {
    fn nth(&self, n: usize) -> Token {
        self.tokens.get(self.pos + n).copied().unwrap_or(Token::End)
    }

    fn bump(&mut self, token: Token) {
        assert_eq!(self.nth(0), token);
        self.pos += 1;
    }

    fn mv(&mut self) -> Result<Option<Tree>, &'static str> {
        match self.nth(0) {
            Token::LBracket => {
                self.bump(Token::LBracket);
                Some(self.grouped(Delimiter::Brackets, "expected ']'")).transpose()
            }
            Token::LParen => {
                self.bump(Token::LParen);
                Some(self.grouped(Delimiter::Parens, "expected ']'")).transpose()
            }
            Token::LBrace => {
                self.bump(Token::LBrace);
                Some(self.grouped(Delimiter::Braces, "expected '}'")).transpose()
            }
            Token::LAngle => Err("unexpected '<'"),
            Token::RAngle => Err("unexpected '>'"),
            Token::RBracket => Err("unexpected ']'"),
            Token::RParen => Err("unexpected ')'"),
            Token::RBrace => Err("unexpected ')'"),
            Token::Comma => Err("unexpected ','"),
            Token::Colon => Err("unexpected ':'"),
            Token::Slash => Err("unexpected '/'"),
            Token::Move(mv) => {
                self.bump(Token::Move(mv));
                Ok(Some(Tree::Move(mv)))
            }
            Token::End => Ok(None),
            Token::Space => {
                self.bump(Token::Space);
                Ok(None)
            }
        }
    }

    fn tree(&mut self) -> Result<Tree, &'static str> {
        self.grouped(Delimiter::None, "")
    }

    fn grouped(
        &mut self,
        end: Delimiter,
        expected_delim: &'static str,
    ) -> Result<Tree, &'static str> {
        let mut first = Vec::new();
        let delim = loop {
            match (end, self.nth(0)) {
                (Delimiter::None, Token::End)
                | (Delimiter::Braces, Token::RBrace)
                | (Delimiter::Parens, Token::RParen)
                | (Delimiter::Brackets, Token::RBracket) => {
                    self.bump(self.nth(0));
                    return Ok(Tree::Group(end, first));
                }
                (_, Token::Comma) => {
                    self.bump(Token::Comma);
                    break Token::Comma;
                }
                (_, Token::Colon) => {
                    self.bump(Token::Colon);
                    break Token::Colon;
                }
                (_, Token::Slash) => {
                    self.bump(Token::Slash);
                    break Token::Slash;
                }
                (_, Token::End) => return Err(expected_delim),
                _ => first.extend(self.mv()?),
            }
        };
        let second: Tree = self.grouped(end, expected_delim)?;
        let operator = match delim {
            Token::Comma => Tree::Comm,
            Token::Colon => Tree::Conj,
            Token::Slash => Tree::Slash,
            _ => unreachable!(),
        };
        Ok(operator(
            end,
            Box::new(Tree::Group(Delimiter::None, first)),
            Box::new(second),
        ))
    }
}

pub fn format_move<W: fmt::Write>(mut out: W, mut mv: Move) -> fmt::Result {
    assert!(mv.start <= mv.end);
    let mut inv = false;
    mv.count %= 4;
    if mv.count == 3 {
        mv.count = 1;
        inv = true;
    }
    if mv.start == 0 && mv.end == 1 {
        write!(&mut out, "{:?}", mv.face)?;
    } else if mv.start == 0 && mv.end == 2 {
        write!(&mut out, "{:?}w", mv.face)?;
    } else if mv.start == 0 && mv.end == mv.n {
        let (c, invert) = match mv.face {
            Face::R => ('x', false),
            Face::L => ('x', true),
            Face::U => ('y', false),
            Face::D => ('y', true),
            Face::F => ('z', false),
            Face::B => ('z', true),
        };
        write!(&mut out, "{}", c)?;
        inv ^= invert;
    } else if mv.start == 1 && mv.end == mv.n - 1 {
        let (c, invert) = match mv.face {
            Face::L => ('M', false),
            Face::R => ('M', true),
            Face::D => ('E', false),
            Face::U => ('E', true),
            Face::F => ('S', false),
            Face::B => ('S', true),
        };
        inv ^= invert;
        write!(&mut out, "{}", c)?;
    } else if mv.n % 2 == 1 && mv.start == mv.n / 2 && mv.end == mv.n / 2 + 1 {
        let (c, invert) = match mv.face {
            Face::L => ('m', false),
            Face::R => ('m', true),
            Face::D => ('e', false),
            Face::U => ('e', true),
            Face::F => ('s', false),
            Face::B => ('s', true),
        };
        inv ^= invert;
        write!(&mut out, "{}", c)?;
    } else if mv.start == 0 && mv.end > 1 {
        write!(&mut out, "{}{:?}w", mv.end - mv.start, mv.face)?;
    } else if mv.start == mv.end - 1 {
        let c = match mv.face {
            Face::L => 'l',
            Face::R => 'r',
            Face::D => 'd',
            Face::U => 'u',
            Face::F => 'f',
            Face::B => 'b',
        };
        if mv.start == 1 {
            write!(&mut out, "{}", c)?;
        } else {
            write!(&mut out, "{}{}", mv.start + 1, c)?;
        }
    } else {
        let c = match mv.face {
            Face::L => 'l',
            Face::R => 'r',
            Face::D => 'd',
            Face::U => 'u',
            Face::F => 'f',
            Face::B => 'b',
        };
        write!(&mut out, "{}-{}{}", mv.start - 1, mv.end - 2, c)?;
    }
    if inv {
        mv.count = 4 - mv.count % 4;
    }
    match mv.count % 4 {
        0 => out.write_char('0')?,
        1 => {}
        2 => out.write_char('2')?,
        3 => out.write_char('\'')?,
        _ => panic!(),
    }
    Ok(())
}

pub fn format_tokens<W: fmt::Write>(mut out: W, tokens: &[Token]) -> fmt::Result {
    let mut want_space = false;
    for &token in tokens {
        match token {
            Token::LBracket => {
                if want_space {
                    out.write_char(' ')?;
                }
                out.write_char('[')?;
                want_space = false;
            }
            Token::RBracket => {
                out.write_char(']')?;
                want_space = true;
            }
            Token::LParen => {
                if want_space {
                    out.write_char(' ')?;
                }
                out.write_char('(')?;
                want_space = false;
            }
            Token::RParen => {
                out.write_char(')')?;
                want_space = true;
            }
            Token::LBrace => {
                if want_space {
                    out.write_char(' ')?;
                }
                out.write_char('{')?;
                want_space = false;
            }
            Token::RBrace => {
                out.write_char('}')?;
                want_space = true;
            }
            Token::Comma => {
                out.write_char(',')?;
                want_space = true;
            }
            Token::Colon => {
                out.write_char(':')?;
                want_space = true;
            }
            Token::Slash => {
                out.write_char(' ')?;
                out.write_char('/')?;
                want_space = true;
            }
            Token::Space => {
                want_space = true;
            }
            Token::Move(mv) => {
                if want_space {
                    out.write_char(' ')?;
                }
                format_move(&mut out, mv)?;
                want_space = true;
            }
            Token::End => {}
            Token::LAngle => todo!(),
            Token::RAngle => todo!(),
        };
    }
    Ok(())
}

pub enum ParseMode {
    Wca,
}

pub fn parse_alg(n: u16, _mode: ParseMode, text: &str) -> Result<Tree, &'static str> {
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < text.len() {
        let (tok, len) = tokenize(n, false, &text[i..])?;
        tokens.push(tok);
        i += len;
    }
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
    };
    let tree = parser.tree()?;
    Ok(tree)
}

fn tokenize(n: u16, sign: bool, text: &str) -> Result<(Token, usize), &'static str> {
    let mut chars = text.char_indices().peekable();
    match chars.peek() {
        Some((_, ' ' | '!' | '+')) => Ok((Token::Space, 1)),
        Some((_, '{')) => Ok((Token::LBrace, 1)),
        Some((_, '}')) => Ok((Token::RBrace, 1)),
        Some((_, '[')) => Ok((Token::LBracket, 1)),
        Some((_, ']')) => Ok((Token::RBracket, 1)),
        Some((_, '(')) => Ok((Token::LParen, 1)),
        Some((_, ')')) => Ok((Token::RParen, 1)),
        Some((_, '<')) => Ok((Token::LAngle, 1)),
        Some((_, '>')) => Ok((Token::RAngle, 1)),
        Some((_, ',')) => Ok((Token::Comma, 1)),
        Some((_, ':' | ';')) => Ok((Token::Colon, 1)),
        Some((_, '/')) => Ok((Token::Slash, 1)),
        Some(_) => parse_move(n, sign, text).map(|(mv, len)| (Token::Move(mv), len)),
        None => Err("empty string"),
    }
}

fn parse_move(n: u16, sign: bool, text: &str) -> Result<(Move, usize), &'static str> {
    if text.is_empty() {
        return Err("empty string");
    }

    let mut chars = text.char_indices().peekable();

    let mut start = None;
    while let Some(d) = chars.peek().and_then(|(_, c)| c.to_digit(10)) {
        start = Some(start.unwrap_or(0) * 10 + d as u16);
        chars.next();
    }

    let mut end = None;
    if chars.next_if(|&(_, c)| c == '-').is_some() {
        while let Some(d) = chars.peek().and_then(|(_, c)| c.to_digit(10)) {
            end = Some(end.unwrap_or(0) * 10 + d as u16);
            chars.next();
        }
    }

    let letter = match chars.next() {
        Some((_, c)) => c,
        None => return Err("invalid notation: missing letter"),
    };

    let wide = chars.next_if(|&(_, c)| c == 'w').is_some();

    if end.is_some() && wide {
        return Err("invalid notation: cannot combine layer specifiers and wide marker");
    }

    let mut count = None;
    while let Some(d) = chars.peek().and_then(|(_, c)| c.to_digit(10)) {
        count = Some(u8::checked_mul(count.unwrap_or(0), 10).ok_or("overflow")? + d as u8);
        chars.next();
    }
    let mut count = count.unwrap_or(1);

    if chars.next_if(|&(_, c)| c == '\'' || c == '’').is_some() {
        count = 4 - count % 4;
    }

    let mv = match letter {
        'U' | 'L' | 'F' | 'R' | 'B' | 'D' => {
            let face = match letter {
                'U' => Face::U,
                'L' => Face::L,
                'F' => Face::F,
                'R' => Face::R,
                'B' => Face::B,
                'D' => Face::D,
                _ => todo!(),
            };

            if wide {
                let width = start.unwrap_or(2);
                if width > n {
                    return Err("move too big");
                }
                Move {
                    n,
                    face,
                    start: 0,
                    end: width,
                    count,
                }
            } else {
                if start.is_some() || end.is_some() {
                    return Err("unimplemented notation: extended layer specifiers");
                }
                Move {
                    n,
                    face,
                    start: 0,
                    end: 1,
                    count,
                }
            }
        }
        'u' | 'l' | 'f' | 'r' | 'b' | 'd' => {
            let face = match letter {
                'u' => Face::U,
                'l' => Face::L,
                'f' => Face::F,
                'r' => Face::R,
                'b' => Face::B,
                'd' => Face::D,
                _ => todo!(),
            };

            if wide {
                return Err("invalid notation: lowercase moves cannot be marked wide");
            }

            if sign || n == 3 {
                let width = start.unwrap_or(2);
                if width > n {
                    return Err("move too big");
                }
                Move {
                    n,
                    face,
                    start: 0,
                    end: width,
                    count,
                }
            } else if let Some(start) = start {
                if start > n {
                    return Err("invalid notation: move too big");
                }

                let start = start
                    .checked_sub(1)
                    .ok_or("invalid notation: invalid start index")?;
                let end = end.unwrap_or(start + 1);

                Move {
                    n,
                    face,
                    start,
                    end,
                    count,
                }
            } else {
                Move {
                    n,
                    face,
                    start: 1,
                    end: 2,
                    count,
                }
            }
        }
        'm' | 'e' | 's' => {
            if n < 5 {
                return Err(
                    "invalid notation: inner middle slice moves are not valid on cubes smaller than 5x5x5",
                );
            }

            let face = match letter {
                'm' => Face::L,
                'e' => Face::D,
                's' => Face::F,
                _ => unreachable!(),
            };

            if start.is_some() || end.is_some() {
                return Err("invalid notation: slice moves cannot have layer specifiers");
            }

            if wide {
                return Err("invalid notation: slice moves cannot be wide");
            }

            if n % 2 == 0 {
                return Err("invalid notation: cannot use inner slice moves on even-layered cubes");
            }

            if n / 2 + 1 > n {
                return Err("move too big");
            }

            Move {
                n,
                face,
                start: n / 2,
                end: n / 2 + 1,
                count,
            }
        }
        'M' | 'E' | 'S' => {
            let face = match letter {
                'M' => Face::L,
                'E' => Face::D,
                'S' => Face::F,
                _ => unreachable!(),
            };

            if start.is_some() || end.is_some() {
                return Err("invalid notation: slice moves cannot have layer specifiers");
            }

            if wide {
                return Err("invalid notation: slice moves cannot be wide");
            }

            Move {
                n,
                face,
                start: 1,
                end: n - 1,
                count,
            }
        }
        'x' | 'y' | 'z' => {
            let face = match letter {
                'x' => Face::R,
                'y' => Face::U,
                'z' => Face::F,
                _ => unreachable!(),
            };

            if start.is_some() || end.is_some() {
                return Err("invalid notation: rotations cannot have layer specifiers");
            }

            if wide {
                return Err("invalid notation: rotations cannot be wide");
            }

            Move {
                n,
                face,
                start: 0,
                end: n,
                count,
            }
        }
        _ => {
            return Err("invalid notation");
        }
    };

    match chars.next() {
        Some((i, _)) => Ok((mv, i)),
        None => Ok((mv, text.len())),
    }
}

pub fn format_moves(standard_moves: &[Move]) -> String {
    let mut s = String::new();
    let tokens = standard_moves
        .iter()
        .copied()
        .map(Token::Move)
        .collect::<Vec<_>>();
    format_tokens(&mut s, &tokens).unwrap();
    s
}

pub fn rotate_from(n: u16, orientation: EdgeSticker) -> Vec<Move> {
    let mut moves = Vec::new();
    let (x, y, z) = orientation.xyz();

    if x > 0 {
        moves.push(Move {
            n,
            face: Face::R,
            start: 0,
            end: n,
            count: x,
        });
    }

    if y > 0 {
        moves.push(Move {
            n,
            face: Face::U,
            start: 0,
            end: n,
            count: y,
        });
    }

    if z > 0 {
        moves.push(Move {
            n,
            face: Face::F,
            start: 0,
            end: n,
            count: z,
        });
    }
    moves
}

pub fn canonicalize(moves: &[Move]) -> Vec<Move> {
    if moves.is_empty() {
        return vec![];
    }
    let n = moves[0].n;
    let mut canceler = Canceler::new();
    canceler.extend(moves.iter().copied());
    let mut result = canceler.moves;
    result.extend(rotate_from(n, canceler.orientation));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use cube::{Cube, RotatedCube};
    use expect_test::{expect, Expect};
    use std::collections::HashSet;

    #[test]
    fn parse() {
        fn check(n: u16, text: &str, expected: Expect) {
            let (mv, len) = parse_move(n, false, text).unwrap();
            let dbg = format!(
                "{:?}[{}-{}]{}|{}",
                mv.face,
                mv.start,
                mv.end,
                match mv.count % 4 {
                    0 => "0",
                    1 => "",
                    2 => "2",
                    3 => "'",
                    _ => panic!(),
                },
                &text[len..],
            );
            expected.assert_eq(&dbg);
        }

        check(3, "x", expect!["R[0-3]|"]);
        check(3, "y", expect!["U[0-3]|"]);
        check(3, "z", expect!["F[0-3]|"]);

        check(3, "R", expect!["R[0-1]|"]);
        check(3, "U'", expect!["U[0-1]'|"]);
        check(3, "F", expect!["F[0-1]|"]);

        check(3, "M", expect!["L[1-2]|"]);
        check(3, "E", expect!["D[1-2]|"]);
        check(3, "S", expect!["F[1-2]|"]);

        check(3, "M", expect!["L[1-2]|"]);
        check(3, "E", expect!["D[1-2]|"]);
        check(3, "S", expect!["F[1-2]|"]);

        check(5, "lm", expect!["L[1-2]|m"]);
        check(5, "m", expect!["L[2-3]|"]);

        check(6, "r", expect!["R[1-2]|"]);
        check(6, "2r", expect!["R[1-2]|"]);
        check(6, "3r", expect!["R[2-3]|"]);
    }

    #[test]
    fn parse_and_format() {
        #[track_caller]
        fn check(
            n: u16,
            alg: &str,
            expected_formatted: Expect,
            expected_standard: Expect,
            expected_canonical: Expect,
        ) {
            let tree = parse_alg(n, ParseMode::Wca, alg).unwrap();
            let tokens = tree.to_tokens();
            let standard_moves = tree.to_moves();

            let mut s = String::new();
            format_tokens(&mut s, &tokens).unwrap();
            expected_formatted.assert_eq(&s);

            let standard_formatted = format_moves(&standard_moves);
            expected_standard.assert_eq(&standard_formatted);
            let canonical_formatted = format_moves(&canonicalize(&standard_moves));
            expected_canonical.assert_eq(&canonical_formatted);

            let canonicalized_moves = canonicalize(&standard_moves);
            let mut canonicalized_cube = Cube::new_solved(n);
            let mut canonicalized_cube = RotatedCube::new(&mut canonicalized_cube);
            for mv in &canonicalized_moves {
                canonicalized_cube.rotate(mv.face, mv.start..mv.end, mv.count);
            }
            for mv in rotate_from(n, canonicalized_cube.orientation) {
                canonicalized_cube.rotate(mv.face, mv.start..mv.end, mv.count);
            }

            let mut standard_cube = Cube::new_solved(n);
            let mut standard_cube = RotatedCube::new(&mut standard_cube);
            for mv in &standard_moves {
                standard_cube.rotate(mv.face, mv.start..mv.end, mv.count);
            }
            for mv in rotate_from(n, standard_cube.orientation) {
                standard_cube.rotate(mv.face, mv.start..mv.end, mv.count);
            }

            assert_eq!(
                format!("{:?}", standard_cube.cube),
                format!("{:?}", canonicalized_cube.cube),
                "{standard_formatted} != {canonical_formatted}",
            );

            let mut cube = Cube::new_solved(n);
            let mut cube = RotatedCube::new(&mut cube);
            for mv in canonicalized_moves.iter().rev().map(|x| x.inverse()) {
                cube.rotate(mv.face, mv.start..mv.end, mv.count);
            }
            for mv in &standard_moves {
                cube.rotate(mv.face, mv.start..mv.end, mv.count);
            }

            assert!(
                cube.cube.is_solved_in_any_orientation(),
                "{standard_formatted} != {canonical_formatted}",
            );

            //eprintln!("{tree}");
            //eprintln!("{}", format_moves(&tree.to_moves()));
            //eprintln!("{}", format_moves(&tree.to_inverse_moves()));
            //let mut cube = Cube::new_solved(n);
            //tree.apply_inverse_to(&mut cube);
            //eprintln!("{cube:?}");
            //tree.apply_to(&mut cube);
            //eprintln!("{cube:?}");
            //assert!(cube.is_solved());
        }

        check(
            3,
            "R U F",
            expect!["R U F"],
            expect!["R U F"],
            expect!["R U F"],
        );
        check(
            3,
            "y R U F",
            expect!["y R U F"],
            expect!["y R U F"],
            expect!["B U R y"],
        );
        check(
            5,
            "3Rw R U F",
            expect!["3Rw R U F"],
            expect!["3Rw R U F"],
            expect!["R Lw F D x"],
        );

        check(
            5,
            "[3Uw': [U' m2 U, l']]",
            expect!["[3Uw': [U' m2 U, l']]"],
            expect!["3Uw' U' m2 U l' U' m2 U l 3Uw"],
            expect!["Dw' U' Bw2 Fw2 D Bw' B D' Bw2 Fw2 U Bw B' Dw"],
        );

        check(3, "U U2", expect!["U U2"], expect!["U U2"], expect!["U'"]);

        check(
            3,
            "[U: [U2, M']]",
            expect!["[U: [U2, M']]"],
            expect!["U U2 M' U2 M U'"],
            expect!["U' R' L F2 R L' U'"],
        );

        check(
            3,
            "[x: [U: [U2, M']]]",
            expect!["[x: [U: [U2, M']]]"],
            expect!["x U U2 M' U2 M U' x'"],
            expect!["F' R' L D2 R L' F'"],
        );

        check(
            3,
            "[E': [L' E L, U]]",
            expect!["[E': [L' E L, U]]"],
            expect!["E' L' E L U L' E' L U' E"],
            expect!["D U' F' D' U L U L' D U' F D'"],
        );

        check(
            5,
            "3Uw Uw U",
            expect!["3Uw Uw U"],
            expect!["3Uw Uw U"],
            expect!["Dw Uw U y"],
        );
        check(
            5,
            "U Uw 3Uw",
            expect!["U Uw 3Uw"],
            expect!["U Uw 3Uw"],
            expect!["Dw Uw U y"],
        );

        check(
            4,
            "U2 r2",
            expect!["U2 r2"],
            expect!["U2 r2"],
            expect!["U2 Rw2 R2"],
        );

        check(
            5,
            "F Fw'",
            expect!["F Fw'"],
            expect!["F Fw'"],
            expect!["Fw' F"],
        );

        check(
            4,
            "F Fw'",
            expect!["F Fw'"],
            expect!["F Fw'"],
            expect!["Fw' F"],
        );

        check(4, "Rw", expect!["Rw"], expect!["Rw"], expect!["Rw"]);
        check(4, "3Rw", expect!["3Rw"], expect!["3Rw"], expect!["L x"]);
        check(4, "x", expect!["x"], expect!["x"], expect!["x"]);

        check(
            4,
            "Uw U'",
            expect!["Uw U'"],
            expect!["Uw U'"],
            expect!["Uw U'"],
        );

        check(4, "u", expect!["u"], expect!["u"], expect!["Uw U'"]);

        check(
            4,
            "F Fw'",
            expect!["F Fw'"],
            expect!["F Fw'"],
            expect!["Fw' F"],
        );

        check(
            4,
            "Fw' Lw'",
            expect!["Fw' Lw'"],
            expect!["Fw' Lw'"],
            expect!["Fw' Rw' x"],
        );

        check(
            4,
            "Fw' Lw' r u",
            expect!["Fw' Lw' r u"],
            expect!["Fw' Lw' r u"],
            expect!["Fw' R' Fw F' x"],
        );

        check(
            4,
            "F Fw' Lw' R r u",
            expect!["F Fw' Lw' R r u"],
            expect!["F Fw' Lw' R r u"],
            expect!["x"],
        );

        check(
            4,
            "U' R Lw' B' Fw D' Uw R2 D Uw' B Fw' R' Lw U'",
            expect!["U' R Lw' B' Fw D' Uw R2 D Uw' B Fw' R' Lw U'"],
            expect!["U' R Lw' B' Fw D' Uw R2 D Uw' B Fw' R' Lw U'"],
            expect!["U' Rw' R Uw U' Rw R' U2 Rw' R Uw' U Rw R' U'"],
        );

        check(
            4,
            "r u",
            expect!["r u"],
            expect!["r u"],
            expect!["Rw R' Uw U'"],
        );

        check(
            4,
            "[U: [U2, r' u r]]",
            expect!["[U: [U2, r' u r]]"],
            expect!["U U2 r' u r U2 r' u' r U'"],
            expect!["U' Rw' R Uw U' Rw R' U2 Rw' R Uw' U Rw R' U'"],
        );

        check(
            4,
            "[l u' l', U']",
            expect!["[l u' l', U']"],
            expect!["l u' l' U' l u l' U"],
            expect!["Rw L' B Fw' D Uw' L' D' Uw B' Fw Rw' L U"],
        );

        check(
            4,
            "[U' / r' u r]",
            expect!["[U' / r' u r]"],
            expect!["U' r' u r U' U' r' u' r U'"],
            expect!["U' Rw' R Uw U' Rw R' U2 Rw' R Uw' U Rw R' U'"],
        );

        check(
            4,
            "Uw Dw'",
            expect!["Uw Dw'"],
            expect!["Uw Dw'"],
            expect!["y"],
        );

        check(
            3,
            "[y: [M D M', U2]]",
            expect!["[y: [M D M', U2]]"],
            expect!["y M D M' U2 M D' M' U2 y'"],
            expect!["B F' R B' F U2 B F' R' B' F U2"],
        );

        check(
            3,
            "[M D M', U2]",
            expect!["[M D M', U2]"],
            expect!["M D M' U2 M D' M' U2"],
            expect!["R L' F R' L U2 R L' F' R' L U2"],
        );

        check(
            3,
            "M D M' U2 M D' M' U2",
            expect!["M D M' U2 M D' M' U2"],
            expect!["M D M' U2 M D' M' U2"],
            expect!["R L' F R' L U2 R L' F' R' L U2"],
        );

        check(
            3,
            "y R U F y'",
            expect!["y R U F y'"],
            expect!["y R U F y'"],
            expect!["B U R"],
        );

        check(
            3,
            "[y, R U F]",
            expect!["[y, R U F]"],
            expect!["y R U F y' F' U' R'"],
            expect!["B U R F' U' R'"],
        );

        check(
            3,
            "[3Uw: R U F]",
            expect!["[y: R U F]"],
            expect!["y R U F y'"],
            expect!["B U R"],
        );

        check(3, "M", expect!["M"], expect!["M"], expect!["R L' x'"]);
        check(3, "E", expect!["E"], expect!["E"], expect!["D' U y'"]);
        check(3, "S", expect!["S"], expect!["S"], expect!["B F' z"]);
        check(3, "Fw", expect!["Fw"], expect!["Fw"], expect!["B z"]);
        check(
            3,
            "S2 R2",
            expect!["S2 R2"],
            expect!["S2 R2"],
            expect!["B2 F2 L2 z2"],
        );

        check(4, "3Rw", expect!["3Rw"], expect!["3Rw"], expect!["L x"]);

        check(4, "3Rw", expect!["3Rw"], expect!["3Rw"], expect!["L x"]);

        check(
            5,
            "3Rw U",
            expect!["3Rw U"],
            expect!["3Rw U"],
            expect!["Lw F x"],
        );

        check(
            4,
            "3Rw U",
            expect!["3Rw U"],
            expect!["3Rw U"],
            expect!["L F x"],
        );

        check(
            5,
            "Uw' R U F",
            expect!["Uw' R U F"],
            expect!["Uw' R U F"],
            expect!["Uw' R U F"],
        );
        check(
            5,
            "3Uw' R U F",
            expect!["3Uw' R U F"],
            expect!["3Uw' R U F"],
            expect!["Dw' F U L y'"],
        );
        check(
            5,
            "4Uw' R U F",
            expect!["4Uw' R U F"],
            expect!["4Uw' R U F"],
            expect!["D' F U L y'"],
        );
        check(
            5,
            "5Uw' R U F",
            expect!["y' R U F"],
            expect!["y' R U F"],
            expect!["F U L y'"],
        );

        check(
            3,
            "[R2 U': [S, R2]]",
            expect!["[R2 U': [S, R2]]"],
            expect!["R2 U' S R2 S' R2 U R2"],
            expect!["R2 U' B F' U2 B' F R2 U R2"],
        );

        check(
            3,
            "R2 U' S R2 S' R2 U R2",
            expect!["R2 U' S R2 S' R2 U R2"],
            expect!["R2 U' S R2 S' R2 U R2"],
            expect!["R2 U' B F' U2 B' F R2 U R2"],
        );

        check(
            3,
            "U:[R' S' R,U2]",
            expect!["[U: [R' S' R, U2]]"],
            expect!["U R' S' R U2 R' S R U2 U'"],
            expect!["U R' B' F D R2 D' B F' R U"],
        );
        check(
            3,
            "U:[l' E l,U2]",
            expect!["[U: [Lw' E Lw, U2]]"],
            expect!["U Lw' E Lw U2 Lw' E' Lw U2 U'"],
            expect!["U R' B' F D R2 D' B F' R U"],
        );

        check(
            3,
            "[U' R' U: [U R U', M']]",
            expect!["[U' R' U: [U R U', M']]"],
            expect!["U' R' U U R U' M' U R' U' M U' R U"],
            expect!["U' R' U2 R U' R' L F R' F' R L' U' R U"],
        );

        check(
            3,
            "[U', [S', L F' L']]",
            expect!["[U', [S', L F' L']]"],
            expect!["U' S' L F' L' S L F L' U L F' L' S' L F L' S"],
            expect!["U' B' F U F' U' B F' L F L' U L F' L' B' F U F U' B F'"],
        );

        check(
            3,
            "[U', [S', L F' L']]",
            expect!["[U', [S', L F' L']]"],
            expect!["U' S' L F' L' S L F L' U L F' L' S' L F L' S"],
            expect!["U' B' F U F' U' B F' L F L' U L F' L' B' F U F U' B F'"],
        );

        check(
            3,
            "[U, [R' U' R, D2]]",
            expect!["[U, [R' U' R, D2]]"],
            expect!["U R' U' R D2 R' U R D2 U' D2 R' U' R D2 R' U R"],
            expect!["U R' U' R D2 R' U R U' R' U' R D2 R' U R"],
        );

        check(
            6,
            "[U' 2r U, 3r2]",
            expect!["[U' r U, 3r2]"],
            expect!["U' r U 3r2 U' r' U 3r2"],
            expect!["U' Rw R' U 3Rw2 Rw2 U' Rw' R U 3Rw2 Rw2"],
        );

        check(
            6,
            "[D: [U' 2r2 U, 3r]]",
            expect!["[D: [U' r2 U, 3r]]"],
            expect!["D U' r2 U 3r U' r2 U 3r' D'"],
            expect!["D U' Rw2 R2 U 3Rw Rw' U' Rw2 R2 U 3Rw' Rw D'"],
        );

        check(
            6,
            "[D: [U' 2r2 U, 3r]]",
            expect!["[D: [U' r2 U, 3r]]"],
            expect!["D U' r2 U 3r U' r2 U 3r' D'"],
            expect!["D U' Rw2 R2 U 3Rw Rw' U' Rw2 R2 U 3Rw' Rw D'"],
        );

        check(
            6,
            "[D: [3r, U' 2r2 U]]",
            expect!["[D: [3r, U' r2 U]]"],
            expect!["D 3r U' r2 U 3r' U' r2 U D'"],
            expect!["D 3Rw Rw' U' Rw2 R2 U 3Rw' Rw U' Rw2 R2 D' U"],
        );

        check(
            4,
            "[y': [U', r' d' r]]",
            expect!["[y': [U', r' d' r]]"],
            expect!["y' U' r' d' r U r' d r y"],
            expect!["U' Fw' F D Uw' Rw R' U Rw' R D' Uw Fw F'"],
        );

        check(
            5,
            "[y': [U', r' d' r]]",
            expect!["[y': [U', r' d' r]]"],
            expect!["y' U' r' d' r U r' d r y"],
            expect!["U' Fw' F Dw' D Fw F' U Fw' F Dw D' Fw F'"],
        );

        check(
            5,
            "[U': [r' u r, U2]]",
            expect!["[U': [r' u r, U2]]"],
            expect!["U' r' u r U2 r' u' r U2 U"],
            expect!["U' Rw' R Uw U' Rw R' U2 Rw' R Uw' U Rw R' U'"],
            //       U' Rw' R Uw U' Rw R' U2 Rw' R Uw' U Rw R' U'
        );
        check(
            4,
            "[U': [r' u r, U2]]",
            expect!["[U': [r' u r, U2]]"],
            expect!["U' r' u r U2 r' u' r U2 U"],
            expect!["U' Rw' R Uw U' Rw R' U2 Rw' R Uw' U Rw R' U'"],
            //       U' Rw' R Uw U' Rw R' U2 Rw' R Uw' U Rw R' U'
        );

        check(
            4,
            "[Lw2 3Lw: [U L' U', r]]",
            expect!["[Lw2 3Lw: [U L' U', r]]"],
            expect!["Lw2 3Lw U L' U' r U L U' r' 3Lw' Lw2"],
            expect!["Rw2 R F L' F' Rw R' F L F' Rw"],
        );

        check(
            4,
            "Lw2 3Lw U L' U' r U L U' r' 3Lw' Lw2",
            expect!["Lw2 3Lw U L' U' r U L U' r' 3Lw' Lw2"],
            expect!["Lw2 3Lw U L' U' r U L U' r' 3Lw' Lw2"],
            expect!["Rw2 R F L' F' Rw R' F L F' Rw"],
        );

        check(4, "r R", expect!["r R"], expect!["r R"], expect!["Rw"]);
        check(4, "r", expect!["r"], expect!["r"], expect!["Rw R'"]);

        check(
            4,
            "R Lw2 B L' B' R' Lw U L U' R R' Lw",
            expect!["R Lw2 B L' B' R' Lw U L U' R R' Lw"],
            expect!["R Lw2 B L' B' R' Lw U L U' R R' Lw"],
            expect!["Rw2 R F L' F' Rw R' F L F' Rw"],
        );

        check(4, "R2 U R2 B2 D2 L2 B2 U' R2 U2 F' R' U2 L' D2 B D2 B F2 Uw2 Rw2 D B2 U' B' D R2 Fw2 U F L2 Rw' B R' D2 Rw U2 D2 Uw' F' U2 B2 Rw F2  y'",
            expect!["R2 U R2 B2 D2 L2 B2 U' R2 U2 F' R' U2 L' D2 B D2 B F2 Uw2 Rw2 D B2 U' B' D R2 Fw2 U F L2 Rw' B R' D2 Rw U2 D2 Uw' F' U2 B2 Rw F2 y'"],
            expect!["R2 U R2 B2 D2 L2 B2 U' R2 U2 F' R' U2 L' D2 B D2 B F2 Uw2 Rw2 D B2 U' B' D R2 Fw2 U F L2 Rw' B R' D2 Rw U2 D2 Uw' F' U2 B2 Rw F2 y'"],
            expect!["R2 U R2 B2 D2 L2 B2 U' R2 U2 F' R' U2 L' D2 B D2 B F2 Uw2 Rw2 D B2 U' B' D R2 Fw2 U F Rw' L2 B R' D2 Rw D2 Uw' U2 F' U2 B2 Rw F2 y'"],
        );

        check(
            4,
            "[r' d2 r, U2]",
            expect!["[r' d2 r, U2]"],
            expect!["r' d2 r U2 r' d2 r U2"],
            expect!["Rw' R D2 Uw2 Rw L' B2 Rw' L D2 Uw2 Rw R' U2"],
        );

        check(
            4,
            "[3l d2 3l', U2]",
            expect!["[3l d2 3l', U2]"],
            expect!["3l d2 3l' U2 3l d2 3l' U2"],
            expect!["Rw' R D2 Uw2 Rw L' B2 Rw' L D2 Uw2 Rw R' U2"],
        );

        check(
            3,
            "[R U' R': [U2, R' D R]]",
            expect!["[R U' R': [U2, R' D R]]"],
            expect!["R U' R' U2 R' D R U2 R' D' R R U R'"],
            expect!["R U' R' U2 R' D R U2 R' D' R2 U R'"],
        );

        check(
            3,
            "R' U D R2 U' R' D' R U R' D R' D' U' R",
            expect!["R' U D R2 U' R' D' R U R' D R' D' U' R"],
            expect!["R' U D R2 U' R' D' R U R' D R' D' U' R"],
            expect!["R' D U R2 U' R' D' R U R' D R' D' U' R"],
        );

        check(
            3,
            "R' U D R2 U' R' D' R U R' D R' U' D' R",
            expect!["R' U D R2 U' R' D' R U R' D R' U' D' R"],
            expect!["R' U D R2 U' R' D' R U R' D R' U' D' R"],
            expect!["R' D U R2 U' R' D' R U R' D R' D' U' R"],
        );

        check(
            3,
            "U R D' R': U, R' D R",
            expect!["[U R D' R': [U, R' D R]]"],
            expect!["U R D' R' U R' D R U' R' D' R R D R' U'"],
            expect!["U R D' R' U R' D R U' R' D' R2 D R' U'"],
        );

        check(
            3,
            "R D': R/E",
            expect!["[R D': [R / E]]"],
            expect!["R D' R E R R E' R D R'"],
            expect!["R D' R D' U F2 D U' R D R'"],
        );

        check(
            3,
            "r' U r, E",
            expect!["[Rw' U Rw, E]"],
            expect!["Rw' U Rw E Rw' U' Rw E'"],
            expect!["L' B L D' U B' R' B D U'"],
        );

        check(
            3,
            "l: l E' l', U'",
            expect!["[Lw: [Lw E' Lw', U']]"],
            expect!["Lw Lw E' Lw' U' Lw E Lw' U Lw'"],
            expect!["R2 D' U F' R' F D U' R' B R'"],
        );

        check(
            4,
            "[Rw' U: [R, U r U']]",
            expect!["[Rw' U: [R, U r U']]"],
            expect!["Rw' U R U r U' R' U r' U' U' Rw"],
            expect!["Rw' U R U Rw R' U' R' U Rw' R U2 Rw"],
        );

        check(
            3,
            "U' D: R/E",
            expect!["[U' D: [R / E]]"],
            expect!["U' D R E R R E' R D' U"],
            expect!["D U' R D' U F2 D U' R D' U"],
        );

        check(
            3,
            "U R' F': R S R', F'",
            expect!["[U R' F': [R S R', F']]"],
            expect!["U R' F' R S R' F' R S' R' F F R U'"],
            expect!["U R' F' R B F' U' F' U B' F R' F2 R U'"],
        );

        check(
            3,
            "U R' F2: R S R', F",
            expect!["[U R' F2: [R S R', F]]"],
            expect!["U R' F2 R S R' F R S' R' F' F2 R U'"],
            expect!["U R' F2 R B F' U' F U B' F R' F R U'"],
        );
    }

    #[test]
    fn test_canonicalization() {
        fn check(n: u16, equivalents: &[&str], expected: Expect) {
            let canonicalized = equivalents
                .iter()
                .map(|alg| {
                    parse_alg(n, ParseMode::Wca, alg)
                        .unwrap()
                        .to_canonical_moves()
                })
                .map(|moves| format_moves(&moves))
                .collect::<HashSet<_>>();
            assert_eq!(canonicalized.len(), 1, "{canonicalized:?}");
            expected.assert_eq(&canonicalized.into_iter().next().unwrap());
        }

        check(
            3,
            &[
                "R' U D R2 U' R' D' R U R' D R' D' U' R",
                "R' U D R2 U' R' D' R U R' D R' U' D' R",
            ],
            expect!["R' D U R2 U' R' D' R U R' D R' D' U' R"],
        );
    }
}
