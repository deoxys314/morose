use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Go {
    Left,
    Right,
}

impl TryFrom<char> for Go {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'l' | 'L' => Ok(Self::Left),
            'r' | 'R' => Ok(Self::Right),
            _ => Err("Invalid character"),
        }
    }
}

impl Go {
    pub fn collect_from<T>(collection: T) -> Vec<Go>
    where
        T: IntoIterator,
        T::Item: Into<char>,
    {
        let mut results = vec![];
        for c in collection {
            results.push(Go::try_from(c.into()));
        }
        results.into_iter().filter_map(Result::ok).collect()
    }

    pub fn collect_str(string: &str) -> Vec<Go> {
        Go::collect_from(string.chars())
    }
}

#[derive(Debug, PartialEq)]
pub struct BTrie<T> {
    value: Option<T>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl<T> Default for BTrie<T> {
    fn default() -> Self {
        BTrie {
            value: None,
            left: None,
            right: None,
        }
    }
}

impl<T> BTrie<T> {
    pub fn new(val: Option<T>, left: Option<Self>, right: Option<Self>) -> Self {
        BTrie {
            value: val,
            left: match left {
                Some(node) => Some(Box::new(node)),
                None => None,
            },
            right: match right {
                Some(node) => Some(Box::new(node)),
                None => None,
            },
        }
    }

    pub fn new_leaf(val: T) -> Self {
        BTrie {
            value: Some(val),
            left: None,
            right: None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        if let Self {
            value: _,
            left: None,
            right: None,
        } = self
        {
            return true;
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        if let Self {
            value: None,
            left: None,
            right: None,
        } = self
        {
            return true;
        }
        false
    }

    pub fn height(&self) -> usize {
        if self.is_terminal() {
            return 0;
        }
        std::cmp::max(
            match self.left {
                Some(ref subnode) => subnode.height(),
                None => 0,
            },
            match self.right {
                Some(ref subnode) => subnode.height(),
                None => 0,
            },
        ) + 1
    }

    pub fn get(&self, path: &[Go]) -> Option<&T> {
        if path.is_empty() {
            return self.value.as_ref();
        }

        match path[0] {
            Go::Left => self.left.as_ref()?.get(&path[1..]),
            Go::Right => self.right.as_ref()?.get(&path[1..]),
        }
    }

    pub fn insert(&mut self, path: &[Go], value: T) -> &mut Self {
        if path.is_empty() {
            self.value = Some(value);
            return self;
        }

        let target = match path[0] {
            Go::Left => &mut self.left,
            Go::Right => &mut self.right,
        };

        match target {
            Some(ref mut subnode) => {
                subnode.insert(&path[1..], value);
            }
            None => {
                let mut new_node = Self::default();
                new_node.insert(&path[1..], value);
                *target = Some(Box::new(new_node));
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            BTrie::<usize>::default(),
            BTrie::<usize> {
                value: None,
                left: None,
                right: None
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            BTrie::<usize>::new(Some(15), None, None),
            BTrie::<usize> {
                value: Some(15),
                left: None,
                right: None
            }
        );
    }

    #[test]
    fn terminal() {
        assert!(BTrie::<u8>::default().is_terminal());
        assert!(!(BTrie::<char> {
            value: Some('A'),
            left: Some(Box::new(BTrie::<char>::default())),
            right: None,
        })
        .is_terminal());
    }

    #[test]
    fn empty() {
        assert!((BTrie::<f64> {
            value: None,
            ..BTrie::default()
        })
        .is_empty());
        assert!(!(BTrie::<i32> {
            value: Some(6i32),
            ..BTrie::default()
        })
        .is_empty());
    }

    #[test]
    fn get_one_level() {
        let root = BTrie::<char> {
            value: None,
            left: Some(Box::new(BTrie {
                value: Some('E'),
                left: None,
                right: None,
            })),
            right: None,
        };
        assert_eq!(Some(&'E'), root.get(&[Go::Left]));
    }

    #[test]
    fn get_root() {
        let root = BTrie::<usize> {
            value: Some(15),
            left: None,
            right: None,
        };
        assert_eq!(Some(&15), root.get(&[]));
    }

    #[test]
    fn get_deep_left() {
        let root = BTrie::<usize> {
            value: Some(23),
            left: Some(Box::new(BTrie {
                value: Some(16),
                left: Some(Box::new(BTrie {
                    value: None,
                    left: Some(Box::new(BTrie {
                        value: Some(15),
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            right: None,
        };

        assert_eq!(Some(&15), root.get(&[Go::Left, Go::Left, Go::Left]));
    }

    #[test]
    fn insert() {
        // we're using morse numbers becuase it's easy to check the veracity of it.
        let mut root = BTrie::<usize>::default();
        root.insert(&Go::collect_str("LRRRR"), 1);
        root.insert(&Go::collect_str("LLRRR"), 2);
        root.insert(&Go::collect_str("LLLRR"), 3);
        root.insert(&Go::collect_str("LLLLR"), 4);
        root.insert(&Go::collect_str("LLLLL"), 5);
        root.insert(&Go::collect_str("RLLLL"), 6);
        root.insert(&Go::collect_str("RRLLL"), 7);
        root.insert(&Go::collect_str("RRRLL"), 8);
        root.insert(&Go::collect_str("RRRRL"), 9);
        root.insert(&Go::collect_str("RRRRR"), 0);

        assert_eq!(root.get(&Go::collect_str("LRRRR")), Some(&1));
        assert_eq!(root.get(&Go::collect_str("LLLLL")), Some(&5));
    }

    #[test]
    fn height() {
        assert_eq!(BTrie::<usize>::default().height(), 0);
        let root = BTrie::<usize> {
            value: None,
            left: Some(Box::new(BTrie {
                value: None,
                left: None,
                right: None,
            })),
            right: None,
        };
        assert_eq!(root.height(), 1);
    }
}
