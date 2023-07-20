#[derive(Debug)]
pub struct Movie {
    name: String,
    url: String,
    year: u32,
    rating: f32,
    director: String,
    adder: String,
}

impl Movie {
    pub fn new(name: String, adder: String) -> Movie {
        Movie {
            name,
            url: String::from(""),
            year: 0,
            rating: 0.0,
            director: String::from(""),
            adder,
        }
    }
}

#[derive(Debug)]
pub struct MovieList {
    name: String,
    creator: String,
    movies: Vec<Movie>,
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl MovieList {
    pub fn new(name: String, creator: String) -> MovieList {
        MovieList {
            name,
            creator,
            movies: Vec::new(),
        }
    }

    pub fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
    }

    pub fn remove_movie(&mut self, movie: Movie) {
        let index = self.movies.iter().position(|x| *x == movie).unwrap();
        self.movies.remove(index);
    }

    pub fn get_movie(&self, name: String) -> Option<&Movie> {
        self.movies.iter().find(|x| x.name == name)
    }
}
