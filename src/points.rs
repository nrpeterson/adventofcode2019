use std::ops::{Add, AddAssign, Sub, SubAssign};
use itertools::Itertools;
use crate::points::Direction2D::{Down, Left, Right, Up};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point2D(pub isize, pub isize);

impl Point2D {
    pub fn l1_norm(&self) -> isize {
        self.0.abs() + self.1.abs()
    }

    pub fn l2_norm_sq(&self) -> isize {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn neighbors(&self) -> Vec<Point2D> {
        [Up, Down, Left, Right].into_iter()
            .map(|d| d.to_step() + self)
            .collect_vec()
    }
}

impl Add for Point2D {
    type Output = Point2D;
    fn add(self, other: Point2D) -> Point2D {
        Point2D(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<&Point2D> for Point2D {
    type Output = Point2D;
    fn add(self, other: &Point2D) -> Point2D {
        Point2D(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Point2D {
    type Output = Point2D;
    fn sub(self, other: Point2D) -> Point2D {
        Point2D(self.0 - other.0, self.1 - other.1)
    }
}

impl Sub<&Point2D> for Point2D {
    type Output = Point2D;
    fn sub(self, other: &Point2D) -> Point2D {
        Point2D(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Point2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction2D { Up, Down, Left, Right }

impl Direction2D {
    pub fn to_step(&self) -> Point2D {
        match self {
            Direction2D::Up => Point2D(0, 1),
            Direction2D::Down => Point2D(0, -1),
            Direction2D::Left => Point2D(-1, 0),
            Direction2D::Right => Point2D(1, 0)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point3D(pub isize, pub isize, pub isize);

impl Point3D {
    pub fn l1_norm(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Add for Point3D {
    type Output = Point3D;
    fn add(self, rhs: Self) -> Self::Output {
        Point3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<&Point3D> for Point3D {
    type Output = Point3D;
    fn add(self, rhs: &Point3D) -> Self::Output {
        Point3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add for &Point3D {
    type Output = Point3D;
    fn add(self, rhs: &Point3D) -> Self::Output {
        Point3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point3D {
    type Output = Point3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Point3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<&Point3D> for Point3D {
    type Output = Point3D;
    fn sub(self, rhs: &Point3D) -> Self::Output {
        Point3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub for &Point3D {
    type Output = Point3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Point3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl AddAssign<&Point3D> for Point3D {
    fn add_assign(&mut self, rhs: &Point3D) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}