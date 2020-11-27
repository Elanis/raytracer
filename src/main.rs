fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3");
    println!("{}", nx.to_string() + &" ".to_owned() + &ny.to_string());
    println!("255");

    for j in (0..ny).rev() {
    	for i in 0..nx {
    		let r = i as f64 / nx as f64;
    		let g = j as f64 / ny as f64;
    		let b = 0.2 as f64;

    		let ir = (255.99*r).round();
    		let ig = (255.99*g).round();
    		let ib = (255.99*b).round();

    		println!("{}", ir.to_string() + &" ".to_owned() + &ig.to_string() + &" ".to_owned() + &ib.to_string());
    	}
    }
}
