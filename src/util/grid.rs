#[derive(Clone)]
pub struct Grid<T>
where
    T: Clone,
{
    data: Vec<Option<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            data: vec![None; width * height],
            width,
            height,
        }
    }

    pub fn from_vec(width: usize, height: usize, vec: Vec<T>) -> Result<Self, &'static str> {
        if let Some(0) = (width * height).checked_sub(vec.len()) {
            Ok(Grid {
                data: vec.into_iter().map(|v| Some(v)).collect(),
                width,
                height,
            })
        } else {
            Err("Tried to construct a Grid from a vec with the wrong dimensions.")
        }
    }

    pub fn from_option_vec(
        width: usize,
        height: usize,
        vec: Vec<Option<T>>,
    ) -> Result<Self, &'static str> {
        if let Some(0) = (width * height).checked_sub(vec.len()) {
            Ok(Grid {
                data: vec,
                width,
                height,
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

    pub fn coord_in_bounds<CC: Into<(C, C)>, C: TryInto<usize>>(&self, coord: CC) -> bool {
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
        return Some((x, y));
    }

    fn check_coord_bounds<CC: Into<(C, C)>, C: TryInto<usize>>(
        &self,
        coord: CC,
    ) -> Option<(usize, usize)> {
        let (x, y) = coord.into();
        self.check_bounds(x, y)
    }

    pub fn get<C: TryInto<usize>>(&self, x: C, y: C) -> Option<&T> {
        let Some((x, y)) = self.check_bounds(x, y) else {
            return None;
        };
        (&self.data[x + y * self.width]).as_ref()
    }

    pub fn get_coord<CC: Into<(C, C)>, C: TryInto<usize>>(&self, coord: CC) -> Option<&T> {
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

    pub fn set_coord<CC: Into<(C, C)>, C: TryInto<usize>>(&mut self, coord: CC, val: T) -> bool {
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
            }
            None => false,
        }
    }

    pub fn remove_coord<CC: Into<(C, C)>, C: TryInto<usize>>(&mut self, coord: CC) -> bool {
        let Some((x, y)) = self.check_coord_bounds(coord) else {
            return false;
        };
        self.remove(x, y)
    }

    pub fn iter_coords<C: From<(usize, usize)>>(&self) -> impl Iterator<Item = (C, Option<&T>)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, v)| ((i % self.width, i / self.width).into(), v.as_ref()))
    }
}

impl<T> Grid<Option<T>>
where
    T: Clone,
{
    pub fn unwrap_options(self) -> Grid<T> {
        Grid {
            data: self
                .data
                .into_iter()
                .map(|o| match o {
                    Some(v) => v,
                    None => None,
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T, C> FromIterator<(C, Option<T>)> for Grid<T>
where
    C: Into<(usize, usize)>,
    T: Clone,
{
    fn from_iter<IT: IntoIterator<Item = (C, Option<T>)>>(iter: IT) -> Self {
        let mut new_values: Vec<(usize, usize, T)> = Vec::new();
        let (width, height) =
            iter.into_iter()
                .fold((0usize, 0usize), |(width, height), (coord, val)| {
                    let (x, y) = coord.into();
                    if let Some(v) = val {
                        new_values.push((x, y, v));
                    }
                    let new_width = std::cmp::max(x, width);
                    let new_height = std::cmp::max(y, height);
                    (new_width, new_height)
                });
        let mut grid = Grid::new(width + 1, height + 1);
        for (x, y, v) in new_values {
            grid.set(x, y, v);
        }
        grid
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T>
where
    T: Clone,
{
    fn from_iter<IT: IntoIterator<Item = Vec<T>>>(iter: IT) -> Self {
        let mut grid_storage: Vec<Option<T>> = Vec::new();
        let (width, height) = iter
            .into_iter()
            .map(|i| (i.len(), i.into_iter().map(|v| Some(v)).collect()))
            .fold((0usize, 0usize), |(width, height), (row_width, mut row)| {
                if width != 0 && row_width != width {
                    panic!("Attempted to make a Grid from an iterator with variable row widths");
                }
                grid_storage.append(&mut row);
                (row_width, height + 1)
            });
        Self {
            data: grid_storage,
            width,
            height,
        }
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match &self.data[x + y * self.width] {
                    Some(v) => {
                        write!(f, "{:?}", v)?;
                    }
                    None => {
                        write!(f, " ")?;
                    }
                }

                if x != self.width - 1 {
                    write!(f, " ")?;
                }
            }
            if y != self.height - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
