#[derive(Clone, Serialize, Deserialize)]
pub struct Doctor {
    pub name: String,
    pub address: String,
    pub zip_code: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub website: String,
    pub jameda_url: String
}

impl PartialEq for Doctor {
fn eq(&self, other: &Doctor) -> bool {
    self.name == other.name &&
    self.address == other.address &&
    self.zip_code == other.zip_code &&
    self.city == other.city
}
}

pub fn split_array(doctors: &Vec<Doctor>) -> Vec<Vec<Doctor>> {
    let len = doctors.len();
    let chunk_len = len/16;

    let mut count = 0;

    let mut chunks = Vec::new();
    let mut chunk = Vec::new();

    for i in 0..len {
        chunk.push(doctors[i].clone());
            if count >= chunk_len {
                chunks.push(chunk);
                chunk = Vec::new();
                count = 0;
            }
            count += 1;
        }
    chunks
}

//TODO: fix this
pub fn remove_duplicates_threaded(doctors: &Vec<Doctor>) -> Vec<Doctor> {
    let mut indexes = Vec::new();
    let mut new_doctors = doctors.clone();
    let doctor_chunks = split_array(doctors);

    let mut threads = Vec::new();

    for chunk in doctor_chunks {
        let doctors_clone = doctors.clone();
        threads.push(::std::thread::spawn(move || {
            get_indexes_to_remove(&chunk, &doctors_clone)
        }));
    }

    for t in threads {
        indexes.extend(t.join().unwrap());
    }

    for i in indexes {
        new_doctors.remove(i); 
    }

    new_doctors
}

pub fn get_indexes_to_remove(doctor_chunk: &Vec<Doctor>, doctors: &Vec<Doctor>) -> Vec<usize> {
    let mut indexes = Vec::new();

    for i in 0..doctor_chunk.len() {
        println!("{}", i);
        if i >= doctor_chunk.len() -2 {
            break;
        }
        let mut temp_doctors = doctors.clone();
        temp_doctors.remove(i);
        for j in 0..temp_doctors.len() {
            if doctor_chunk[i] == temp_doctors[j] {
                indexes.push(j);
            }       
        }
    };
    indexes
}

pub fn remove_duplicates(doctors: &Vec<Doctor>) -> Vec<Doctor> {
    let mut new_doctors = doctors.clone();

    for i in 0..new_doctors.len() {
        println!("{}", i);
        if i >= new_doctors.len() -2 {
            break;
        }
        let mut temp_doctors = new_doctors.clone();
        temp_doctors.remove(i);
        for j in 0..temp_doctors.len() {
            if new_doctors[i] == temp_doctors[j] {
                new_doctors.remove(j);
            }       
        }
    };
    new_doctors
}
