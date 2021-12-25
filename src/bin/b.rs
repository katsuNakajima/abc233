#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let (l, r) = parse_line!(usize, usize);
    let s = parse_line!(String);
    let str1 = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| i >= l - 1 && i < r)
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
    let str2 = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| i < l - 1)
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
    let str3 = s
        .chars()
        .enumerate()
        .filter(|&(i, _)| i >= r && i < s.len())
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
    let rstr: String = str1.chars().rev().collect();
    let ans = str2 + &rstr + &str3;
    println!("{}", ans);
}
