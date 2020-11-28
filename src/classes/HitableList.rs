use super::hitRecord::HitRecord;
use super::hitable::Hitable;
use super::ray::Ray;

pub struct HitableList {
	list : Vec<Box<dyn Hitable>>,
}

impl HitableList {
	pub fn new() -> HitableList {
		HitableList { list: Vec::new() }
	}

	pub fn push(&mut self, hitable: Box<dyn Hitable>) {
		self.list.push(hitable);
	}
}

impl Hitable for HitableList {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for i in 0..self.list.len() {
			let mut temp_record = HitRecord::new();
			if self.list[i].hit(r, t_min, closest_so_far, &mut temp_record) {
				hit_anything = true;
				closest_so_far = temp_record.t;

				*rec = temp_record;
			}
		}

		return hit_anything;
	} 
}