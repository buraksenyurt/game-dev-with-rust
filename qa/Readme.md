# Rust ile Programlama

Sunum için kullanılan örnek kodları içerir.

## 01 - Dakika Bir Gol Lifetimes

Aşağıdaki kod ile başlanır.

```rust
fn main() {
    let wilson = find_player(23);
    println!("{}", wilson.nick_name);
}

fn find_player(id: i32) -> Player {
    Player {
        id,
        nick_name: "Don Du Dragon Wilsın",
    }
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}
```

**Missing Lifetime Specifier** hatası vurgulanır ve düzeltilir.

```rust
fn main() {
    let wilson = find_player(23);
    println!("{}", wilson.nick_name);
}

fn find_player<'a>(id: i32) -> Player<'a> {
    Player {
        id,
        nick_name: "Don Du Dragon Wilsın",
    }
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}
```

## 02 - Constructor Görevi Gören new Eklenir

Player nesnesinin kolay örneklenmesi için fonksiyon implementasyonuna geçilir. Önce aşağıdaki gibi program kodu değiştirilir.

```rust
fn main() {
    let wilson = Player::new(23,"Can Kilod Van Dam");
    println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}

impl Player {
    pub fn new(id: i32, nick_name: &str) -> Self {
        Self { id, nick_name }
    }
}
```

**implicit elided lifetime not allowed here** hatası gösterilir ve kod düzeltilerek ilerlenir.

```rust
fn main() {
    let wilson = Player::new(23,"Can Kilod Van Dam");
    println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str) -> Self {
        Self { id, nick_name }
    }
}
```

## 03 - Serüvene Enum Veri Yapısı Katılır

Oyuncunun seviyesini tutan bir enum sabiti eklenir. Bu enum sabitinde Score şeklinde bir veri tutulması da gösterilir.

```rust
fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

## 04 - Enum Var da Pattern Matching Olmaz mı?

Enum ile tutulan Level'ın durumuna göre karar verilmek istendiğinde pattern matching ile ilerlenebilir. Kod aşağıdaki hale getirilir.

```rust
fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            10..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}/{}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level.win, wilson.level.lose, revenue
    );

    //println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

Burada özellikle println! fonksiyonunda level üstünden score bilgilerine erişilemediği görülür ve no field `win` on type `Level` hatası belirtilir. Çözüm için Display trait'lerinin uygulanması sağlanır.

Kod bu durumda aşağıdaki hale getirilir.

```rust
use std::fmt::{Display, Formatter};

fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            10..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level, revenue
    );

    //println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner(s) => {
                write!(f, "({:?})", s)
            }
            Level::Pro(s) => {
                write!(f, "({:?})", s)
            }
            Level::Veteran(s) => {
                write!(f, "({:?})", s)
            }
            Level::Elit => {
                write!(f, "(Elit)")
            }
        }
    }
}

#[derive(Debug)]
pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```

Bu durumda çok daha güzel çalışma zamanı hataları ile karşılaşılır. Borrow Checker devrededir.

```text
error[E0382]: borrow of partially moved value: `wilson.level`
  --> src/main.rs:24:27
   |
10 |         Level::Beginner(s) => match s.win {
   |                         - value partially moved here
...
24 |         wilson.nick_name, wilson.level, revenue
   |                           ^^^^^^^^^^^^ value borrowed here after partial move
```

Score nesnesinin kısmi olarak ödünç alınması nedeniyle referansının başkası tarafından aynı anda ödünç alınamadığını düşünebiliriz. Çözüm için derleyicide önerildiği gibi Score veri yapısına Clone ve Copy trait'lerini uygulayabiliriz.

_Bu arada gözden kaçan bazı durumlar için uyarılara da bakmakta yarar var. Cargo Clippy' de denenebilir_

```rust
use std::fmt::{Display, Formatter};

fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level, revenue
    );

    //println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner(s) => {
                write!(f, "({:?})", s)
            }
            Level::Pro(s) => {
                write!(f, "({:?})", s)
            }
            Level::Veteran(s) => {
                write!(f, "({:?})", s)
            }
            Level::Elit => {
                write!(f, "(Elit)")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Score {
    pub win: u16,
    pub lose: u16,
}
```