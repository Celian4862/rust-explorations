fn main() {
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    for (day_index, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas, my true love sent to me: ");
        for gift in gifts.iter().take(day_index + 1).rev() {
            if day_index == 0 && gift == &gifts[0] {
                println!("{}.\n", gift);
            } else if gift == &gifts[0] {
                println!("and {}.\n", gift);
            } else {
                println!("{},", gift);
            }
        }
    }
}