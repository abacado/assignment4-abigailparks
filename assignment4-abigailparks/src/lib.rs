use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(_n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    
    if _n == 0 {
        return v;
    }
    
    v.push(0);
    
    if _n == 1 {
        return v;
    }
    
    v.push(1);
    
    for _ in 2.._n {
        let a = v[v.len() - 1];
        let b = v[v.len() - 2];
        v.push(a + b);
    }
    
    v
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(_n: u32) -> bool {
    let s = _n.to_string();
    s.chars().rev().collect::<String>() == s
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(_n: usize, _a: &[i32]) -> Option<i32> {
    if _a.is_empty() {
        return None;
    }
    
    let mut v = _a.to_vec();
    
    v.sort_by(|a, b| b.cmp(a));
    
    v.get(_n).copied()
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(_s: &str) -> String {
    if _s.is_empty() {
        return "".to_string();
    }
    
    let mut map = HashMap::new();
    
    for c in _s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let (best_char, _) = map
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();

    best_char.to_string()
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(
    _arr1: &[String],
    _arr2: &[String],
) -> Option<HashMap<String, String>> {
    if _arr1.len() != _arr2.len() {
        return None;
    }

    let mut map = HashMap::new();

    for (k, v) in _arr1.iter().zip(_arr2.iter()) {
        map.insert(k.clone(), v.clone());
    }

    Some(map)
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(
    _map: &HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut v: Vec<_> = _map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    v.sort_by(|a, b| a.0.cmp(&b.0));

    v
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        // You may also use `Self::default()`
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    fn valid_number(num: &str) -> bool {
        let parts: Vec<&str> = num.split('-').collect();
        if parts.len() != 3 {
            return false;
        }
        parts[0].len() == 3
            && parts[1].len() == 3
            && parts[2].len() == 4
            && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
    }
    
    pub fn add(
        &mut self,
        _name: String,
        _number: String,
        _is_listed: bool,
    ) -> bool {
        if self.entries.iter().any(|e| e.name == _name) {
            return false;
        }

        if !Self::valid_number(&_number) {
            return false;
        }

        if _is_listed {
            if self.entries.iter().any(|e| e.number == _number && e.is_listed) {
                return false;
            }
        }

        self.entries.push(PhoneEntry {
            name: _name,
            number: _number,
            is_listed: _is_listed,
        });
        true
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, _name: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|e| e.name == _name && e.is_listed)
            .map(|e| e.number.clone())
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, _num: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|e| e.number == _num && e.is_listed)
            .map(|e| e.name.clone())
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, _areacode: &str) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| e.number.starts_with(_areacode))
            .map(|e| e.name.clone())
            .collect()
    }
}
