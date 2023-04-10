use bevy::prelude::*;
use rand::{Rng, thread_rng, prelude::Distribution, distributions::Standard};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DropCategory {
	Fruit,
	Veg,
	Bakery,
	Produce,
	Tech,
	Drug,
	Special,
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum DropType {
	Apple, //Fruit
	Bagel, //Bakery
	Ball, //Special
	Bell, //Special
	Bun, //Bakery
	Cassette, //Tech
	Cauliflower, //Veg
	Cd, //Tech
	Cheese, //Produce
	Cherry, //Fruit
	Crown, //Special
	Diamond, //Special
	Donut, //Bakery
	Egg, //Produce
	FriedEgg, //Produce
	Garlic, //Veg
	Grapes, //Fruit
	Headphones, //Tech
	Lemon, //Lemon
	Lettuce, //Veg
	Money, //Special
	Mouth, //Special
	Mushroom, //Drug
	Onion, //Veg
	Orange, //Fruit
	Pepper, //Fancy
	Pill, //Drug
	Pumpkin, //Fruit
	Ramen, //Fancy
	Salt, //Fancy
	Seven, //Special
	Shot, //Drug
	Weed, //Drug
}

impl Distribution<DropType> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DropType {
		match rng.gen_range(0..20) {
			0 => DropType::Apple,
			1 => DropType::Bagel,
			2 => DropType::Bun,
			3 => DropType::Cassette,
			4 => DropType::Cauliflower,
			5 => DropType::Cd,
			6 => DropType::Cheese,
			20 => DropType::Cherry,
			7 => DropType::Donut,
			8 => DropType::Egg,
			9 => DropType::FriedEgg,
			10 => DropType::Garlic,
			11 => DropType::Grapes,
			12 => DropType::Headphones,
			21 => DropType::Lemon,
			13 => DropType::Lettuce,
			14 => DropType::Onion,
			15 => DropType::Orange,
			16 => DropType::Pepper,
			17 => DropType::Pumpkin,
			18 => DropType::Ramen,
			19 => DropType::Salt,
			//2 => DropType::Ball,
			//15 => DropType::Mushroom,
			//19 => DropType::Pill,
			//23 => DropType::Shot,
			//24 => DropType::Weed,
			_ => DropType::Ball,
		}
	}
}

impl DropType {
	pub fn get_path(&self) -> &'static str {
		match self {
			DropType::Apple => "droppables/apple.png",
			DropType::Bagel => "droppables/bagel.png",
			DropType::Ball => "droppables/ball.png",
			DropType::Bell => "droppables/bell.png",
			DropType::Bun => "droppables/bun.png",
			DropType::Cassette => "droppables/cassette.png",
			DropType::Cauliflower => "droppables/cauliflower.png",
			DropType::Cd => "droppables/cd.png",
			DropType::Cheese => "droppables/cheese.png",
			DropType::Cherry => "droppables/cherry.png",
			DropType::Crown => "droppables/crown.png",
			DropType::Diamond => "droppables/diamond.png",
			DropType::Donut => "droppables/donut.png",
			DropType::Egg => "droppables/egg.png",
			DropType::FriedEgg => "droppables/fried_egg.png",
			DropType::Garlic => "droppables/garlic.png",
			DropType::Grapes => "droppables/grapes.png",
			DropType::Headphones => "droppables/headphones.png",
			DropType::Lemon => "droppables/lemon.png",
			DropType::Lettuce => "droppables/lettuce.png",
			DropType::Money => "droppables/money.png",
			DropType::Mouth => "droppables/mouth.png",
			DropType::Mushroom => "droppables/mushroom.png",
			DropType::Onion => "droppables/onion.png",
			DropType::Orange => "droppables/orange.png",
			DropType::Pepper => "droppables/onion.png",
			DropType::Pill => "droppables/pill.png",
			DropType::Pumpkin => "droppables/pumpkin.png",
			DropType::Ramen => "droppables/ramen.png",
			DropType::Salt => "droppables/salt.png",
			DropType::Seven => "droppables/seven.png",
			DropType::Shot => "droppables/shot.png",
			DropType::Weed => "droppables/weed.png",
		}
	}

	pub fn get_type(&self) -> DropCategory {
		match self {
			DropType::Apple => DropCategory::Fruit,
			DropType::Bagel => DropCategory::Bakery,
			DropType::Ball => DropCategory::Special,
			DropType::Bell => DropCategory::Fruit,
			DropType::Bun => DropCategory::Bakery,
			DropType::Cassette => DropCategory::Tech,
			DropType::Cauliflower => DropCategory::Veg,
			DropType::Cd => DropCategory::Tech,
			DropType::Cheese => DropCategory::Produce,
			DropType::Cherry => DropCategory::Fruit,
			DropType::Crown => DropCategory::Special,
			DropType::Diamond => DropCategory::Special,
			DropType::Donut => DropCategory::Bakery,
			DropType::Egg => DropCategory::Produce,
			DropType::FriedEgg => DropCategory::Produce,
			DropType::Garlic => DropCategory::Veg,
			DropType::Grapes => DropCategory::Fruit,
			DropType::Headphones => DropCategory::Tech,
			DropType::Lemon => DropCategory::Fruit,
			DropType::Lettuce => DropCategory::Veg,
			DropType::Money => DropCategory::Special,
			DropType::Mouth => DropCategory::Special,
			DropType::Mushroom => DropCategory::Drug,
			DropType::Onion => DropCategory::Veg,
			DropType::Orange => DropCategory::Fruit,
			DropType::Pepper => DropCategory::Special,
			DropType::Pill => DropCategory::Drug,
			DropType::Pumpkin => DropCategory::Veg,
			DropType::Ramen => DropCategory::Special,
			DropType::Salt => DropCategory::Special,
			DropType::Seven => DropCategory::Special,
			DropType::Shot => DropCategory::Drug,
			DropType::Weed => DropCategory::Drug,
		}
	}

	pub fn is_edible(&self) -> bool {
		match self {
			DropType::Apple => true,
			DropType::Bagel => true,
			DropType::Ball => false,
			DropType::Bell => false,
			DropType::Bun => true,
			DropType::Cassette => false,
			DropType::Cauliflower => true,
			DropType::Cd => false,
			DropType::Cheese => true,
			DropType::Cherry => true,
			DropType::Crown => false,
			DropType::Diamond => false,
			DropType::Donut => true,
			DropType::Egg => true,
			DropType::FriedEgg => true,
			DropType::Garlic => true,
			DropType::Grapes => true,
			DropType::Headphones => false,
			DropType::Lemon => true,
			DropType::Lettuce => true,
			DropType::Money => false,
			DropType::Mouth => false,
			DropType::Mushroom => true,
			DropType::Onion => true,
			DropType::Orange => true,
			DropType::Pepper => true,
			DropType::Pill => true,
			DropType::Pumpkin => true,
			DropType::Ramen => true,
			DropType::Salt => true,
			DropType::Seven => false,
			DropType::Shot => true,
			DropType::Weed => true,
		}
	}
}

pub fn random_drug() -> DropType {
	let mut rng = thread_rng();
	let index: u8 = rng.gen_range(0..4);
	match index {
		0 => DropType::Mushroom,
		1 => DropType::Pill,
		2 => DropType::Shot,
		3 => DropType::Weed,
		_ => DropType::Ball,
	}
}

pub fn random_droppable_except_mouth_ball_and_seven() -> DropType {
	let mut rng = thread_rng();
	let index: u8 = rng.gen_range(0..30);
	match index {
		0 => DropType::Apple,
		1 => DropType::Bagel,
		//2 => DropType::Ball,
		3 => DropType::Bell,
		4 => DropType::Bun,
		5 => DropType::Cassette,
		6 => DropType::Cauliflower,
		7 => DropType::Cd,
		8 => DropType::Cheese,
		9 => DropType::Cherry,
		10 => DropType::Crown,
		11 => DropType::Diamond,
		12 => DropType::Donut,
		13 => DropType::Egg,
		14 => DropType::FriedEgg,
		15 => DropType::Garlic,
		16 => DropType::Grapes,
		17 => DropType::Headphones,
		18 => DropType::Lemon,
		19 => DropType::Lettuce,
		//20 => DropType::Mouth,
		21 => DropType::Mushroom,
		22 => DropType::Onion,
		23 => DropType::Orange,
		24 => DropType::Pepper,
		25 => DropType::Pill,
		26 => DropType::Pumpkin,
		27 => DropType::Ramen,
		2 => DropType::Salt,
		//29 => DropType::Seven,
		20 => DropType::Shot,
		29 => DropType::Weed,
		_ => DropType::Apple,
	}
}

pub fn random_non_drug_edible() -> DropType {
	let mut rng = thread_rng();
	let index: u8 = rng.gen_range(0..18);
	match index {
		0 => DropType::Apple,
		1 => DropType::Bagel,
		//2 => DropType::Ball,
		//3 => DropType::Bell,
		2 => DropType::Bun,
		//5 => DropType::Cassette,
		3 => DropType::Cauliflower,
		//7 => DropType::Cd,
		4 => DropType::Cheese,
		5 => DropType::Cherry,
		//10 => DropType::Crown,
		//11 => DropType::Diamond,
		6 => DropType::Donut,
		7 => DropType::Egg,
		8 => DropType::FriedEgg,
		9 => DropType::Garlic,
		10 => DropType::Grapes,
		//17 => DropType::Headphones,
		11 => DropType::Lemon,
		12 => DropType::Lettuce,
		//20 => DropType::Mouth,
		//13 => DropType::Mushroom,
		13 => DropType::Onion,
		14 => DropType::Orange,
		15 => DropType::Pepper,
		//25 => DropType::Pill,
		16 => DropType::Pumpkin,
		17 => DropType::Ramen,
		//28 => DropType::Salt,
		//29 => DropType::Seven,
		//30 => DropType::Shot,
		//31 => DropType::Weed,
		_ => DropType::Apple,
	}
}

pub fn random_inedible_except_special() -> DropType {
	let mut rng = thread_rng();
	let index: u8 = rng.gen_range(0..7);
	match index {
		//0 => DropType::Apple,
		//1 => DropType::Bagel,
		//2 => DropType::Ball,
		0 => DropType::Bell,
		//4 => DropType::Bun,
		1 => DropType::Cassette,
		//6 => DropType::Cauliflower,
		2 => DropType::Cd,
		//8 => DropType::Cheese,
		//9 => DropType::Cherry,
		3 => DropType::Crown,
		4 => DropType::Diamond,
		//12 => DropType::Donut,
		//13 => DropType::Egg,
		//14 => DropType::FriedEgg,
		//15 => DropType::Garlic,
		//16 => DropType::Grapes,
		5 => DropType::Headphones,
		//18 => DropType::Lemon,
		//19 => DropType::Lettuce,
		6 => DropType::Money,
		//20 => DropType::Mouth,
		//21 => DropType::Mushroom,
		//22 => DropType::Onion,
		//23 => DropType::Orange,
		//24 => DropType::Pepper,
		//25 => DropType::Pill,
		//26 => DropType::Pumpkin,
		//27 => DropType::Ramen,
		//28 => DropType::Salt,
		//29 => DropType::Seven,
		//30 => DropType::Shot,
		//31 => DropType::Weed,
		_ => DropType::Cd,
	}
}