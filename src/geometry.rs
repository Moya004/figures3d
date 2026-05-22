pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn project(p: &Point3d) -> Point2d {
    Point2d {
        x: p.x / p.z,
        y: p.y / p.z,
    }
}

pub fn clip_near(a: &Point3d, b: &Point3d, near: f64) -> Option<(Point3d, Point3d)> {
    let a_in = a.z >= near;
    let b_in = b.z >= near;
    if !a_in && !b_in {
        return None;
    }
    if a_in && b_in {
        return Some((
            Point3d { x: a.x, y: a.y, z: a.z },
            Point3d { x: b.x, y: b.y, z: b.z },
        ));
    }
    let t = (near - a.z) / (b.z - a.z);
    let intersect = Point3d {
        x: a.x + t * (b.x - a.x),
        y: a.y + t * (b.y - a.y),
        z: near,
    };
    if a_in {
        Some((Point3d { x: a.x, y: a.y, z: a.z }, intersect))
    } else {
        Some((intersect, Point3d { x: b.x, y: b.y, z: b.z }))
    }
}

pub fn mutate_z(p: &Point3d, dz: f64) -> Point3d {
    Point3d {
        x: p.x,
        y: p.y,
        z: p.z + dz,
    }
}

pub fn scale(p: &Point3d, s: f64) -> Point3d {
    Point3d {
        x: p.x * s,
        y: p.y * s,
        z: p.z * s,
    }
}

pub fn rotate_xz(p: &Point3d, angle: f64) -> Point3d {
    let cs = angle.cos();
    let sn = angle.sin();

    Point3d {
        x: p.x * cs - p.z * sn,
        y: p.y,
        z: p.x * sn + p.z * cs,
    }
}

pub fn rotate_xy(p: &Point3d, angle: f64) -> Point3d {
    let cs = angle.cos();
    let sn = angle.sin();

    Point3d {
        x: p.x * cs - p.y * sn,
        y: p.x * sn + p.y * cs,
        z: p.z,
    }
}

pub fn rotate_yz(p: &Point3d, angle: f64) -> Point3d {
    let cs = angle.cos();
    let sn = angle.sin();

    Point3d {
        x: p.x,
        y: p.y * cs - p.z * sn,
        z: p.y * sn + p.z * cs,
    }
}

pub fn rotate_xyz(p: &Point3d, angle: f64) -> Point3d {
    let Point3d { x, y, z } = p;
    let cs = angle.cos();
    let sn = angle.sin();

    Point3d {
        x: y * sn * sn * cs - z * cs * sn * cs + y * cs * sn + z * sn * sn + x * cs * cs,

        y: y * cs * cs + z * sn * cs - y * sn * sn * sn + z * cs * sn * sn - x * cs * sn,

        z: z * cs * cs - y * sn * cs + x * sn,
    }
}
