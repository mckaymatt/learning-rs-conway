
use std::collections::HashMap;

// Eq requires that you derive PartialEq on the type.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pixel{
    x:  i32,
    y:  i32,
}
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct PixelInfo{
    living: bool,
    boundry:  bool,
    next_state: bool,
}

impl PixelInfo {
    fn report_living(&self) -> bool {
        let is_live = self.living == true && self.boundry == false;
        is_live
    }
}

type PixelMap = HashMap<Pixel, PixelInfo>;

fn populate_pixmap(xlen: i32, ylen: i32, pixmap: &mut PixelMap ) {
    for x in 0..xlen+1 {
        for y in 0..ylen+1 {
            let xboard =  x == 0 || x == xlen;
            let yboard =  y == 0 || y == ylen;
            let board = xboard || yboard;
            let pix = Pixel{x: x, y: y};
            let pixinfo = PixelInfo{living: false, boundry: board, next_state: false};
            pixmap.insert(pix, pixinfo);
        }
    }
}

fn update_pixel(x: i32, y: i32, is_living: bool, pixmap: &mut PixelMap) {
    let pix = Pixel{x: x, y: y};
    let new_pixinfo: PixelInfo = {
        let pixinfo = pixmap.get(&pix).unwrap();
        let new_pixinfo = PixelInfo{living: is_living,
                  boundry: pixinfo.boundry,
                  next_state: pixinfo.next_state};
        new_pixinfo
    };
    pixmap.insert(pix, new_pixinfo);
}

fn print_pixelmap(xlen: i32, ylen: i32, pixmap: &PixelMap) {
    let mut sbuf = String::new();
    for x in 0..xlen {
        for y in 0..ylen {
            let key = Pixel{x: x, y: y};
            let pixinfo = pixmap.get(&key).unwrap();
            let repr = match pixinfo.living {
                true => "O",
                false => "-",
            };
            sbuf.push_str(repr);
        }
        sbuf.push_str("\n");
    }
    println!("{}",sbuf);

}

fn cycle_next(pixmap: &mut PixelMap) {
/// make each Pixel's next_state into it's "living" state
    for (_, val) in pixmap.iter_mut() {
        if val.boundry == true {
            continue;
        }
        let new_pixinfo: PixelInfo = {
            let new_pixinfo = PixelInfo{living: val.next_state,
                                        boundry: val.boundry,
                                        next_state: false};
            new_pixinfo
        };
        *val = new_pixinfo;
        }
}

fn get_neighborhood_score(x: i32, y: i32, pixmap: &PixelMap) -> i32 {
    let neightbors=vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1),
                        (1, -1), (1, 0), (1, 1)];
    let mut count: i32 = 0;
    for tup in neightbors {
        let pix = Pixel{x: x + tup.0 , y: y + tup.1};
        let pixinfo = {
            let pixinfo = pixmap.get(&pix).unwrap();
            let picopy = *pixinfo;

            picopy

        };
        let is_living: bool = pixinfo.report_living();
        if is_living == true {
            count += 1;
        };
    }
    count
}

fn get_key_clones(pixmap: &PixelMap) -> Vec<Pixel> {
    let mut keyvec: Vec<Pixel> = Vec::new();
    for key in pixmap.keys() {
        let keyclone: Pixel = *key;
        keyvec.push(keyclone);
    }
    keyvec
}

fn set_next_state(pixmap:  &mut PixelMap, keyvec: &Vec<Pixel>) {
/// set each Pixel's next_state based on the current state of the board
    for key in keyvec.iter() {
        if let Some(pixinfo) = pixmap.get(key) {
            if pixinfo.boundry == true {
                continue;
            }
        } else {
            continue;
        }
        let score: i32 = get_neighborhood_score(key.x, key.y, &pixmap);

        let new_pixinfo: PixelInfo = {
            let val = pixmap.get(key).unwrap();
            if val.living == true {
            }
            let next_state: bool =
                if val.living == true {
                    match score {
                        0|1 => false,
                        4...8 => false,
                        2|3 => true,
                        _ => panic!("living panic"),
                    }
                } else if val.living == false {
                    match score {
                        3 => true,
                        0...2 => false,
                        4...8 => false,
                        _ => panic!("dead panic"),
                    }
                } else {
                    panic!("else panic")
                };

            let new_pixinfo = PixelInfo{living: val.living,
                                        boundry: val.boundry,
                                        next_state: next_state};
            new_pixinfo
            };
        let new_key: Pixel = *key;
        pixmap.insert(new_key, new_pixinfo);
        };
}

fn main() {
    let x: i32 = 20;
    let y: i32 = 20;
    let mut pixmap: PixelMap = HashMap::new();
    populate_pixmap(x, y, &mut pixmap);
    update_pixel(4, 5, true, &mut pixmap);
    update_pixel(4, 6, true, &mut pixmap);
    update_pixel(5, 5, true, &mut pixmap);
    update_pixel(3, 6, true, &mut pixmap);
    update_pixel(3, 4, true, &mut pixmap);
    print_pixelmap(x, y, &pixmap);
    let keyvec: Vec<Pixel> = get_key_clones(&pixmap);

    for _ in 0..60 {
        print_pixelmap(x, y, &pixmap);
        set_next_state(&mut pixmap, &   keyvec);
        cycle_next(&mut pixmap);
    }
}

//   dead = -    -   0
//   alive = O   -   1
  // C   N                 new C
  //  1   0,1             ->  0  # Lonely
  //  1   4,5,6,7,8       ->  0  # Overcrowded
  //  1   2,3             ->  1  # Lives
  //  0   3               ->  1  # It takes three to give birth!
  //  0   0,1,2,4,5,6,7,8 ->  0  # Barren
