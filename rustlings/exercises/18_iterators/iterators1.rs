// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.

use std::vec;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    // You can optionally experiment here.
    let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut iterator = my_fav_fruits.iter();
    println!("{:?}", iterator.next());

    let v1: Vec<i32> = vec![1, 2, 3];
    let mappings: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", mappings);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shoes_in_size;

    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach")); // TODO: Replace `todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        // assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Replace `todo!()`
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("Sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
