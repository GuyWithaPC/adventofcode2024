#[derive(Clone)]
pub struct Grid<T> 
where T: Clone {
    data: Vec<Option<T>>,
    width: usize,
    height: usize
}

impl <T> Grid<T>
where T: Clone {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.fill(None);
        Grid {
            data,
            width,
            height
        }
    }

    pub fn from_vec(width: usize, height: usize, vec: Vec<T>) -> Result<Self, &'static str> {
        if let Some(0) = (width * height).checked_sub(vec.len()) {
            Ok(Grid {
                data: vec.into_iter().map(|v| Some(v)).collect(),
                width,
                height
            })
        } else {
            Err("Tried to construct a Grid from a vec with the wrong dimensions.")
        }
    }

    pub fn from_option_vec(width: usize, height: usize, vec: Vec<Option<T>>) -> Result<Self, &'static str> {
        if let Some(0) = (width * height).checked_sub(vec.len()) {
            Ok(Grid {
                data: vec,
                width,
                height
            })
        } else {
            Err("Tried to construct a Grid from a vec with the wrong dimensions.")
        }
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn in_bounds<C: TryInto<usize>>(&self, x: C, y: C) -> bool {
        let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) else {
            return false;
        };
        if x >= self.width || y >= self.height {
            return false;
        };
        return true;
    }

    pub fn coord_in_bounds<CC: Into<(C,C)>, C: TryInto<usize>>(&self, coord: CC) -> bool {
        let (x, y) = coord.into();
        self.in_bounds(x, y)
    }

    fn check_bounds<C: TryInto<usize>>(&self, x: C, y: C) -> Option<(usize, usize)> {
        let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) else {
            return None;
        };
        if x >= self.width || y >= self.height {
            return None;
        };
        return Some((x,y));
    }

    fn check_coord_bounds<CC: Into<(C,C)>, C: TryInto<usize>>(&self, coord: CC) -> Option<(usize, usize)> {
        let (x, y) = coord.into();
        self.check_bounds(x, y)
    }

    pub fn get<C: TryInto<usize>>(&self, x: C, y: C) -> Option<&T> {
        let Some((x, y)) = self.check_bounds(x, y) else {
            return None;
        };
        (&self.data[x + y * self.width]).as_ref()
    }

    pub fn get_coord<CC: Into<(C,C)>, C: TryInto<usize>>(&self, coord: CC) -> Option<&T> {
        let Some((x, y)) = self.check_coord_bounds(coord) else {
            return None;
        };
        self.get(x, y)
    }

    pub fn set<C: TryInto<usize>>(&mut self, x: C, y: C, val: T) -> bool {
        let Some((x, y)) = self.check_bounds(x, y) else {
            return false;
        };
        self.data[x + y * self.width] = Some(val);
        return true;
    }

    pub fn set_coord<CC: Into<(C,C)>, C: TryInto<usize>>(&mut self, coord: CC, val: T) -> bool {
        let Some((x, y)) = self.check_coord_bounds(coord) else {
            return false;
        };
        self.set(x, y, val)
    }

    pub fn remove<C: TryInto<usize>>(&mut self, x: C, y: C) -> bool {
        let Some((x, y)) = self.check_bounds(x, y) else {
            return false;
        };
        match self.data[x + y * self.width] {
            Some(_) => {
                self.data[x + y * self.width] = None;
                true
            },
            None => false
        }
    }

    pub fn remove_coord<CC: Into<(C,C)>, C: TryInto<usize>>(&mut self, coord: CC) -> bool {
        let Some((x, y)) = self.check_coord_bounds(coord) else {
            return false;
        };
        self.remove(x, y)
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<&T>> {
        self.data.iter().map(|v| v.as_ref())
    }

    pub fn iter_with_coords<C: From<(usize, usize)>>(&self) -> impl Iterator<Item = (C, Option<&T>)> {
        self.data.iter().enumerate().map(|(i, v)| {
            ((i % self.width, i / self.width).into(), v.as_ref())
        })
    }
}