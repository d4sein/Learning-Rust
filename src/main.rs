enum Object {
	Rectangle {base: f32, height: f32},
	Triangle {base: f32, height: f32},
	Trapeze {base: f32, height: f32, top: f32}
}

fn area(obj: Object) {
	match obj {
		Object::Rectangle {base, height} => {
			let area: f32 = base * height;
			println!("The area of a rectangle with base: {} and height: {} is {}\u{00b2}", base, height, area);
		},
		Object::Triangle {base, height} => {
			let area: f32 = base * height / 2.0;
			println!("The area of a triangle with base: {} and height: {} is {}\u{00b2}", base, height, area);
		},
		Object::Trapeze {base, height, top} => {
			let area: f32 = (base + top) * height / 2.0;
			println!("The area of a trapeze with base: {}, height: {} and top: {} is {}\u{00b2}", base, height, top, area);
		}
	}
}

fn main() {
	let rectangle = Object::Rectangle {base: 5.0, height: 5.0};
	area(rectangle);
	let triangle = Object::Triangle {base: 3.0, height: 2.5};
	area(triangle);
	let trapeze = Object::Trapeze {base: 5.0, height: 5.0, top: 3.0};
	area(trapeze);
}