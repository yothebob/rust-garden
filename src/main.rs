enum Area {
    Width,
    Height
}

struct Plant {
    id: u32,
    name: String,
    width: u32,
    height: u32,
    coordinance: Vec<u32>,
    est_yield: u32// approx oz yield per plant 
}

struct Garden {
    width: u32,
    height: u32,
    plants: Vec<u32>,
    eta_total_yield: u32,
}

struct PlantStack {
    plants: Vec<Plant>
}

impl Garden {

    fn desc (&self, stack: & PlantStack) {
	println!("your gard is {} feet wide and and {} feet long", self.width, self.height);
	let mut iter = 0;
	for pl in &self.plants {
	    iter += 1;
	    println!("{}: {}", iter, get_plant_by_id(*pl, stack).name)
	};
	println!("total yield: {}", self.eta_total_yield);
    }
    
    fn add_plant(&mut self, new_plant: &Plant, add_area: Area, stack: &mut PlantStack ) {
	self.plants.push(new_plant.id);
	let plant_being_added = get_plant_by_id(new_plant.id, stack);
	match add_area {
	    Area::Width => {
		if self.height < new_plant.height {
		    self.height += new_plant.height;
		}
		self.width += new_plant.width;
	    }
	    Area::Height => {
		if self.width < new_plant.width {
		    self.width += new_plant.width;
		}
		self.height += new_plant.height;
	    }
	}
	self.eta_total_yield += new_plant.est_yield;
    }
}

impl Plant {

    fn update( &mut self, update_value: u32) {
	// this functcion will be for updating plant coords
    }
}


fn create_plant(newid: &mut u32, name: &str, width: u32, height: u32, approx_yield: u32, stack: &mut PlantStack) -> Plant {
    *newid += 1;
    let new_plant = Plant{id: *newid,
			  name: name.to_string(),
			  width: width,
			  coordinance: Vec::new(),
			  height: height,
			  est_yield: approx_yield};
    let to_stack_plant = Plant{name: name.to_string(), coordinance: Vec::new(), ..new_plant};
    stack.plants.push(to_stack_plant);
    new_plant
}

fn get_plant_by_id(search_id: u32, stack: & PlantStack) -> Plant {
    let iterating_plants = stack.plants.as_slice();
    for pl in iterating_plants.iter() {
	if pl.id == search_id {
	    let created_plant = Plant{id: search_id ,
				      name: String::from(&pl.name),
				      coordinance: Vec::new(),
				      ..*pl};
	    return created_plant;
	}
    };
    let created_plant = Plant{id: search_id,
			      height:0,
			      width:0,
			      name: String::from(""),
			      est_yield: 0,
			      coordinance: Vec::new()};
    created_plant
}

fn main () {
    let mut plant_stack = PlantStack{ plants: Vec::new() };
    let mut id_iter: u32 = 0;

    let mut new_garden = Garden{ width: 0, height: 0, plants: Vec::new(),eta_total_yield:0 };
    new_garden.desc(&plant_stack);
    let new_plant = create_plant(&mut id_iter, &String::from("Bean Plants"), 1, 1, 7, &mut plant_stack);
    new_garden.add_plant(&new_plant, Area::Width, &mut plant_stack);
    new_garden.desc(&plant_stack);
    let tomato_plant = create_plant(&mut id_iter, &String::from("tomato Plants"), 2, 2,198, &mut plant_stack);
    new_garden.add_plant(&new_plant, Area::Width, &mut plant_stack);
    new_garden.add_plant(&tomato_plant, Area::Height, &mut plant_stack);
    new_garden.desc(&plant_stack);
    get_plant_by_id(1, &plant_stack);
    get_plant_by_id(2, &plant_stack);
}

