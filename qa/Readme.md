# Rust ile Programlama

Sunum için kullanılan örnek kodları içerir.

## 01 - İlk Örnek

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