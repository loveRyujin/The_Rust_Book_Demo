#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Factory {
    store: Vec<ShirtColor>
}

impl Factory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_shirts_count = 0;
        let mut blud_shirts_count = 0;

        for color in &self.store {
            match color {
                ShirtColor::Red => red_shirts_count += 1,
                ShirtColor::Blue => blud_shirts_count += 1,
            }
        }

        if red_shirts_count > blud_shirts_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue];
    let factory = Factory{
        store: store
    };

    let user1_preference = ShirtColor::Blue;
    println!("{:?}", factory.giveaway(Some(user1_preference)));

    let user2_preference = ShirtColor::Red;
    println!("{:?}", factory.giveaway(Some(user2_preference)));

    let user3_preference = None;
    println!("{:?}", factory.giveaway(user3_preference));
}
