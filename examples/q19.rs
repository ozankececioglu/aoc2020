use std::{fs, fs::File};

use std::vec;
use std::io::{self, prelude::*, BufReader, Cursor, Error, ErrorKind};
use std::slice::Iter;
use std::{iter, iter::Map};
use std::collections::{HashSet, HashMap};
use regex::{Regex, Captures};
use std::ops::Index;
use regex::internal::Input;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::borrow::Borrow;
use std::hash::Hash;


fn main() -> io::Result<()> {
    let inp = Cursor::new(
        "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb");

    let file = File::open("data/q19")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    enum RuleType<'a> {
        Leaf(&'a str),
        Ref(usize),
        Sequence((usize, usize)),
        Disjunction((Vec<usize>, Vec<usize>)),
    }

    struct Rule<'a> {
        set: Weak<RuleSet<'a>>,
        id: usize,
        tip: RuleType<'a>,
        recursive: bool,
    }

    struct RuleSet<'a> {
        rules: HashMap<usize, RuleType<'a>>,
        expanded: HashMap<usize, Rc<Vec<String>>>,
    }

    let reg = Regex::new(
        r##"^(?P<id>\d+): ("(?P<str>.*)"|(?P<ref>\d+)|((?P<seq1>\d+) (?P<seq2>\d+))|((?P<disj1>.+) \| (?P<disj2>.+)))$"##).unwrap();

    let mut rules: HashMap<usize, RuleType> = HashMap::new();
    let mut expanded: HashMap<usize, Rc<Vec<String>>> = HashMap::new();
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }
        let ca = reg.captures(line).unwrap();
        let id = ca.name("id").unwrap().as_str().parse::<usize>().unwrap();
        let mut rule = None;

        if let Some(x) = ca.name("str") {
            rule = Some(RuleType::Leaf(x.as_str()));
        } else if let Some(x) = ca.name("ref") {
            rule = Some(RuleType::Ref(x.as_str().parse().unwrap()));
        } else if let Some(seq1) = ca.name("seq1") {
            let seq1 = ca.name("seq1").unwrap().as_str().parse().unwrap();
            let seq2 = ca.name("seq2").unwrap().as_str().parse().unwrap();
            rule = Some(RuleType::Sequence((seq1, seq2)));
        } else if let Some(disj1) = ca.name("disj1") {
            let disj1 = disj1.as_str()
                .split_whitespace().map(|f| f.parse().unwrap()).collect();
            let disj2 = ca.name("disj2").unwrap().as_str()
                .split_whitespace().map(|f| f.parse().unwrap()).collect();
            rule = Some(RuleType::Disjunction((disj1, disj2)));
        } else {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        rules.insert(id, rule.unwrap());
    }

    // let recursive_rules: HashMap<_, _> = [0, 8, 11].iter()
    //     .map(|r| (*r, rules.remove(r).unwrap())).collect();

    fn expand_rule<'a>(id: usize, rules: &mut HashMap<usize, RuleType>, expanded: &mut HashMap<usize, Rc<Vec<String>>>) -> Rc<Vec<String>> {
        if let Some(r) = expanded.get(&id) {
            return r.clone();
        }

        fn combinate(a: &Rc<Vec<String>>, b: &Rc<Vec<String>>) -> Vec<String> {
            let mut res: Vec<String> = Vec::new();
            for i in a.iter() {
                for j in b.iter() {
                    let mut nstr = i.clone();
                    nstr.push_str(j);
                    res.push(nstr);
                }
            }
            return res;
        }

        let rule = rules.remove(&id).unwrap();
        let mut expansion = match rule {
            RuleType::Leaf(t) => Rc::new(vec![t.to_owned()]),
            RuleType::Ref(id) => expand_rule(id, rules, expanded).clone(),
            RuleType::Sequence((id1, id2)) => {
                let res = combinate(&expand_rule(id1, rules, expanded), &expand_rule(id2, rules, expanded));
                Rc::new(res)
            }
            RuleType::Disjunction((l1, l2)) => {
                let mut res = Vec::new();
                if l1.len() == 1 {
                    res = (*expand_rule(l1[0], rules, expanded)).clone();
                } else {
                    res = combinate(&expand_rule(l1[0], rules, expanded), &expand_rule(l1[1], rules, expanded));
                }
                if l2.len() == 1 {
                    res.append(&mut (*expand_rule(l2[0], rules, expanded)).clone());
                } else {
                    res.append(&mut combinate(&expand_rule(l2[0], rules, expanded), &expand_rule(l2[1], rules, expanded)));
                }
                Rc::new(res)
            }
        };
        expanded.insert(id, expansion.clone());

        return expansion;
    }

    while !rules.is_empty() {
        expand_rule(*rules.keys().next().unwrap(), &mut rules, &mut expanded);
    }

    let mut valid_messages = HashSet::new();
    for v in expanded.values() {
        for s in v.iter() {
            valid_messages.insert(s.clone());
        }
    }

    fn check_recursive_rule<'a>(input: &'a str,
                            id: usize,
                            rules: &HashMap<usize, RuleType>,
                            expanded: &'a HashMap<usize, Rc<Vec<String>>>) -> Vec<&'a str> {
        let mut res: Vec<&'a str> = Vec::new();
        if let Some(vals) = expanded.get(&id) {
            for v in vals.iter() {
                if input.starts_with(v) {
                    res.push(input.get(0..v.len()).unwrap());
                }
            }
            return res;
        }

        let rule = &rules[&id];
        match rule {
            RuleType::Ref(_id) => {
                return check_recursive_rule(input, *_id, rules, expanded);
            }
            RuleType::Sequence((id1, id2)) => {
                let r1 = check_recursive_rule(input, *id1, rules, expanded);
                for r in r1.iter() {
                    let r2 = check_recursive_rule(r, *id2, rules, expanded);

                }
            }
            RuleType::Disjunction((l1, l2)) => {

            }
            _ => { }
        };

        return res;
    }

    dbg!(&valid_messages.len());

    let mut count = 0;
    for line in lines.iter() {
        if valid_messages.contains(line) {
            count += 1;
        }
    }

    dbg!(count);


    Ok(())
}

