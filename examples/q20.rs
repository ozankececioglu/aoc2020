use std::{fs, fs::File};
use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;
use itertools::{sorted};
use std::any::Any;
use std::convert::TryInto;
// use std::slice;


#[derive(Clone, Copy, Debug)]
enum BorderDir {
    Left,
    Right,
    Top,
    Bottom,
}

type Tile = [[bool; 10]; 10];

fn left_border(tile: &Tile) -> [bool; 10] {
    let mut res = [false; 10];
    for r in tile.iter().enumerate() {
        res[r.0] = tile[r.0][0];
    }
    return res;
}

fn right_border(tile: &Tile) -> [bool; 10] {
    let mut res = [false; 10];
    for r in tile.iter().enumerate() {
        res[r.0] = tile[r.0][9];
    }
    return res;
}

fn flip_horizontal<const LEN: usize>(tile: &mut [[bool; LEN]; LEN]) {
    for r in tile {
        r.reverse();
    }
}

fn flip_vertical<const LEN: usize>(tile: &mut [[bool; LEN]; LEN]) {
    for c in 0..LEN {
        for s in 0..LEN / 2 {
            let swap = tile[s][c];
            tile[s][c] = tile[LEN - s - 1][c];
            tile[LEN - s - 1][c] = swap;
        }
    }
}

fn rotate_ccw<const LEN: usize>(tile: &mut [[bool; LEN]; LEN])
{
    for h in 0..LEN / 2 {
        for i in h..LEN - 1 - h {
            let swap = tile[h][i];
            tile[h][i] = tile[i][LEN - h - 1];
            tile[i][LEN - h - 1] = tile[LEN - h - 1][LEN - i - 1];
            tile[LEN - h - 1][LEN - i - 1] = tile[LEN - i - 1][h];
            tile[LEN - i - 1][h] = swap
        }
    }
}

fn print_tile<const LEN: usize>(tile: &[[bool; LEN]; LEN]) {
    for i in 0..LEN {
        for j in 0..LEN {
            print!("{}", if tile[i][j] { '#' } else { '.' });
        }
        print!("|\n");
    }
}

fn format_row(row: &[bool; 10]) -> String {
    let mut res = String::new();
    for j in 0..10 {
        res.push(if row[j] { '#' } else { '.' });
    }
    return res;
}

fn is_palindrome(word: &mut [bool; 10]) -> bool {
    let mut r = word.clone();
    r.reverse();
    return r == *word;
}

fn main() -> io::Result<()> {
    main2();

    return Ok(());
}

fn test() {
    let mut a = [
        [false; 10],
        [true; 10],
        [true; 10],
        [true; 10],
        [false; 10],
        [true; 10],
        [true; 10],
        [true; 10],
        [true; 10],
        [true; 10],
    ];
    print_tile(&a);

    rotate_ccw(&mut a);
    rotate_ccw(&mut a);
    print_tile(&a);

    flip_vertical(&mut a);
    print_tile(&a);
}

fn main2() {
    let inp = Cursor::new(
        "");

    const NTILES: usize = 12;
    let file = File::open("data/q20").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let reg = Regex::new(r##"Tile (?P<tile>\d+):$"##).unwrap();

    let mut tiles: HashMap<u32, Tile> = HashMap::new();
    let mut borders: HashMap<[bool; 10], Vec<(u32, BorderDir, bool)>> = HashMap::new();
    loop {
        let num = reg.captures(lines.next().unwrap().as_str()).unwrap()
            .name("tile").unwrap().as_str()
            .parse::<u32>().unwrap();
        let mut tile = [[false; 10]; 10];
        for i in 0..10 {
            let row = lines.next().unwrap();
            let bytes = row.as_bytes();
            for j in 0..10 {
                tile[i][j] = bytes[j] == ('#' as u8);
            }
        }

        let mut bs: Vec<([bool; 10], BorderDir)> = [
            (tile[0].clone(), BorderDir::Top),
            (tile[tile.len() - 1].clone(), BorderDir::Bottom),
            (left_border(&tile), BorderDir::Left),
            (right_border(&tile), BorderDir::Right)].to_vec();
        for (b, d) in bs.into_iter() {
            borders.entry(b.clone()).
                or_insert(Vec::new()).push((num, d.clone(), false));
            let mut b2 = b.clone();
            b2.reverse();
            if b2 != b {
                borders.entry(b2).
                    or_insert(Vec::new()).push((num, d.clone(), true));
            } else {
                panic!("Palindrome");
            }
        }
        tiles.insert(num, tile);

        if lines.next().map_or(true, |_| false) {
            break;
        }
    }

    let mut left = HashSet::new();
    let mut top = HashSet::new();

    let mut bcounts = HashMap::new();
    for (_, owners) in borders.iter() {
        if owners.len() == 1 && !owners[0].2 {
            *bcounts.entry(owners[0].0).or_insert(0) += 1;
            match owners[0].1 {
                BorderDir::Left => { left.insert(owners[0].0); }
                BorderDir::Top => { top.insert(owners[0].0); }
                _ => ()
            }
        }
    }
    // dbg!(&bcounts, bcounts.len());
    // dbg!(tiles.len());
    // dbg!(&left, left.len(), &top, top.len());
    // dbg!(&right, right.len(), &bottom, bottom.len());

    let mut map = [[0 as u32; NTILES]; NTILES];
    let top_left = left.intersection(&top).next().unwrap();

    map[0][0] = *top_left;
    println!("{}", *top_left);
    print_tile(tiles.get(&map[0][0]).unwrap());

    for i in 1..NTILES {
        let tile_id = map[i - 1][0];
        let tile = &tiles[&tile_id];
        let bottom = tile[9];
        let all = borders[&bottom].iter().filter(|x| x.0 != tile_id).collect::<Vec<_>>();
        if all.len() != 1 {
            panic!("!### {}", all.len());
        }

        let (id, dir, mut flip) = *all[0];
        map[i][0] = id;
        let mut ntile = tiles.get_mut(&id).unwrap();
        let rots = match dir {
            BorderDir::Left => {
                flip = !flip;
                3
            }
            BorderDir::Bottom => {
                flip = !flip;
                2
            }
            BorderDir::Right => 1,
            BorderDir::Top => 0
        };
        for _ in 0..rots {
            rotate_ccw(&mut ntile);
        }
        if flip {
            flip_horizontal(&mut ntile);
        }
        println!("{} {:?} {}", map[i][0], dir, flip);
        print_tile(&ntile);
    }

    println!("=============");

    for i in 0..NTILES {
        let kk = map[i][0];
        print_tile(&tiles[&kk]);

        for j in 1..NTILES {
            let tile_id = map[i][j - 1];
            let tile = &tiles[&tile_id];
            let right = right_border(&tile);
            let all = borders[&right].iter().filter(|x| x.0 != tile_id).collect::<Vec<_>>();
            if all.len() != 1 {
                panic!("!### {}", all.len());
            }

            let (id, dir, mut flip) = all[0];
            map[i][j] = *id;
            let mut ntile = tiles.get_mut(id).unwrap();
            let rots = match *dir {
                BorderDir::Left => {
                    0
                }
                BorderDir::Bottom => {
                    3
                }
                BorderDir::Right => {
                    flip = !flip;
                    2
                }
                BorderDir::Top => {
                    flip = !flip;
                    1
                }
            };
            for _ in 0..rots {
                rotate_ccw(&mut ntile);
            }
            if flip {
                flip_vertical(&mut ntile);
            }
            println!("{} {:?} {}", map[i][j], dir, flip);
            print_tile(&ntile);
        }
        println!("{} =============", i);
    }

    // dbg!(&map);

    const IMAGE_RES: usize = NTILES * 8;
    let mut image = [[false; IMAGE_RES]; IMAGE_RES];
    for i in 0..NTILES {
        for j in 0..NTILES {
            let tile = &tiles[&map[i][j]];
            for x in 1..9 {
                let mut r = image.get_mut((i * 8 + x - 1)).unwrap();
                r[j * 8..(j + 1) * 8].copy_from_slice(&tile[x][1..9]);
            }
        }
    }

    // print_tile(&image);

    let see_monster: [[bool; 20]; 3] =
        "..................#.
#....##....##....###
.#..#..#..#..#..#..."
            .split('\n')
            .map(|s|
                s.chars().map(|c| if c == '#' { true } else { false })
                    .collect::<Vec<_>>().try_into().unwrap()
            ).collect::<Vec<_>>().try_into().unwrap();

    fn check_monsters(image: &[[bool; IMAGE_RES]; IMAGE_RES], see_monster: &[[bool; 20]; 3]) -> usize {
        let mut nummonsters = 0;
        let mut maxbody = 0;
        for i in 0..(IMAGE_RES - 3) {
            for j in 0..(IMAGE_RES - 20) {
                let mut body = 0;
                for x in 0..3 {
                    for y in 0..20 {
                        if see_monster[x][y] && image[i + x][j + y] {
                            body += 1;
                        }
                    }
                }
                if body == 15 {
                    nummonsters += 1;
                    // dbg!(i, j);
                }
                maxbody = std::cmp::max(body, maxbody);
                // println!("{} {} {}", i, j, body);
            }
        }
        // dbg!(maxbody);
        return nummonsters;
    }

    let mut score = image.iter().fold(0, |x, y|
        x + y.iter().filter(|k| **k).count());
    dbg!(score);

    let mut nummonsters = 0;
    for _ in 0..4 {
        dbg!(check_monsters(&image, &see_monster));
        rotate_ccw(&mut image);
    }

    flip_vertical(&mut image);
    for _ in 0..4 {
        dbg!(check_monsters(&image, &see_monster));
        rotate_ccw(&mut image);
    }
}
