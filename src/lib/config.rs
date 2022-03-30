pub const STOP_WORDS_IN_LINK: [&str; 18] = [
    "babes", "forsale", "girl", "jewelry", "nudit", "poker", "porn", "pron", "sex", "teen",
    "money", "free", "jwh", "cash", "xxx", "fuck", "devki", "devushki"
];

pub const STOP_FULL_NAME_WORDS: [&str; 11] = [
    "18+", "sex", "секс", "pron", "porn", "порн", "прон", "ставки", "betting", "знакомств",
    "dating"
];

pub const POLITIC_WORDS: [&str; 13] = ["хохол", "нацист", "москал", "хохлят", "фашист", "салоед", "болбаш", "укроп", "нацик", "спецоперация", "путин", "Путин", "кацап"];
pub const INSULT_WORDS: [&str; 10] = ["дебил", "долбоёб", "долбойоб", "дибил", "дебіл", "дібіл", "гондон", "гандон", "тупой", "биомусор"];

pub const BASE_RATING: i32 = 100;
pub const BASE_RATING_ADMIN_MULTIPLIER: i32 = 5;
pub const RATING_COOLDOWN: i32 = 30;
