// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Axis-aligned bounding boxes

use point::{Point, Point2, Point3};
use vector::{Vector, Vec2, Vec3};
use array::build;
use std::num::{zero, one};
use std::iter::Iterator;

pub trait Aabb
<
    S: Primitive,
    V: Vector<S, Slice>,
    P: Point<S, V, Slice>,
    Slice
> {
    fn new(p1: &P, p2: &P) -> Self;
    fn min<'a>(&'a self) -> &'a P;
    fn max<'a>(&'a self) -> &'a P;
    #[inline] fn dim(&self) -> V { self.max().sub_p(self.min()) }
    #[inline] fn volume(&self) -> S { self.dim().comp_mul() }
    #[inline] fn center(&self) -> P {
        let two = one::<S>() + one::<S>();
        self.min().add_v(&self.dim().div_s(two))
    }

    // Tests whether a point is cointained in the box, inclusive for min corner
    // and exclusive for the max corner.
    #[inline] fn contains(&self, p: &P) -> bool {
        p.sub_p(self.min()).iter().all(|x| *x >= zero::<S>()) &&
        self.max().sub_p(p).iter().all(|x| *x > zero::<S>())
    }

    // Returns a new AABB that is grown to include the given point.
    fn grow(&self, p: &P) -> Self {
        let mn : P = build(|i| self.min().i(i).min(p.i(i)));
        let mx : P = build(|i| self.max().i(i).max(p.i(i)));
        Aabb::new(&mn, &mx)
    }
}

#[deriving(Clone, Eq)]
pub struct Aabb2<S> {
    mn: Point2<S>,
    mx: Point2<S>,
}

impl<S: Num + Orderable> Aabb2<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: &Point2<S>, p2: &Point2<S>) -> Aabb2<S> {
        Aabb2 {
            mn: Point2::new(p1.x.min(&p2.x), p1.y.min(&p2.y)),
            mx: Point2::new(p1.x.max(&p2.x), p1.y.max(&p2.y)),
        }
    }
}

impl<S: Primitive> Aabb<S, Vec2<S>, Point2<S>, [S, ..2]> for Aabb2<S> {
    fn new(p1: &Point2<S>, p2: &Point2<S>) -> Aabb2<S> { Aabb2::new(p1, p2) }
    #[inline] fn min<'a>(&'a self) -> &'a Point2<S> { &self.mn }
    #[inline] fn max<'a>(&'a self) -> &'a Point2<S> { &self.mx }
}

#[deriving(Clone, Eq)]
pub struct Aabb3<S> {
    mn: Point3<S>,
    mx: Point3<S>,
}

impl<S: Num + Orderable> Aabb3<S> {
    /// Construct a new axis-aligned bounding box from two points.
    #[inline]
    pub fn new(p1: &Point3<S>, p2: &Point3<S>) -> Aabb3<S> {
        Aabb3 {
            mn: Point3::new(p1.x.min(&p2.x), p1.y.min(&p2.y), p1.z.min(&p2.z)),
            mx: Point3::new(p1.x.max(&p2.x), p1.y.max(&p2.y), p1.z.max(&p2.z)),
        }
    }
}

impl<S: Primitive> Aabb<S, Vec3<S>, Point3<S>, [S, ..3]> for Aabb3<S> {
    fn new(p1: &Point3<S>, p2: &Point3<S>) -> Aabb3<S> { Aabb3::new(p1, p2) }
    #[inline] fn min<'a>(&'a self) -> &'a Point3<S> { &self.mn }
    #[inline] fn max<'a>(&'a self) -> &'a Point3<S> { &self.mx }
}
