pub enum RotationAxis {
    X,
    Y,
    Z,
    XY,
    YZ,
    XZ,
    XYZ,
}

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
            Point3d {
                x: a.x,
                y: a.y,
                z: a.z,
            },
            Point3d {
                x: b.x,
                y: b.y,
                z: b.z,
            },
        ));
    }
    let t = (near - a.z) / (b.z - a.z);
    let intersect = Point3d {
        x: a.x + t * (b.x - a.x),
        y: a.y + t * (b.y - a.y),
        z: near,
    };
    if a_in {
        Some((
            Point3d {
                x: a.x,
                y: a.y,
                z: a.z,
            },
            intersect,
        ))
    } else {
        Some((
            intersect,
            Point3d {
                x: b.x,
                y: b.y,
                z: b.z,
            },
        ))
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

pub fn rotate_y(p: &Point3d, angle: f64) -> Point3d {
    let cs = angle.cos();
    let sn = angle.sin();

    Point3d {
        x: p.x * cs - p.z * sn,
        y: p.y,
        z: p.x * sn + p.z * cs,
    }
}

pub fn rotate_z(p: &Point3d, angle: f64) -> Point3d {
    let cs = angle.cos();
    let sn = angle.sin();

    Point3d {
        x: p.x * cs - p.y * sn,
        y: p.x * sn + p.y * cs,
        z: p.z,
    }
}

pub fn rotate_x(p: &Point3d, angle: f64) -> Point3d {
    let cs = angle.cos();
    let sn = angle.sin();

    Point3d {
        x: p.x,
        y: p.y * cs - p.z * sn,
        z: p.y * sn + p.z * cs,
    }
}

pub fn rotate_xy(p: &Point3d, alpha: f64, beta: f64) -> Point3d {
    let Point3d { x, y, z } = p;

    let a_cs = alpha.cos();
    let b_cs = beta.cos();
    let a_sn = alpha.sin();
    let b_sn = beta.sin();

    Point3d {
        x: x * b_cs - b_sn * (z * a_cs - y * a_sn),
        y: y * a_cs + z * a_sn,
        z: x * b_sn + b_cs * (z * a_cs - y * a_sn),
    }
}

pub fn rotate_yz(p: &Point3d, alpha: f64, beta: f64) -> Point3d {
    let Point3d { x, y, z } = p;

    let a_cs = alpha.cos();
    let b_cs = beta.cos();
    let a_sn = alpha.sin();
    let b_sn = beta.sin();

    Point3d {
        x: y * a_sn + a_cs * (x * b_cs - z * b_sn),
        y: y * a_cs - a_sn * (x * b_cs - z * b_sn),
        z: x * b_sn + z * b_cs,
    }
}

pub fn rotate_zx(p: &Point3d, alpha: f64, beta: f64) -> Point3d {
    let Point3d { x, y, z } = p;

    let a_cs = alpha.cos();
    let b_cs = beta.cos();
    let a_sn = alpha.sin();
    let b_sn = beta.sin();

    Point3d {
        x: x * b_cs + y * b_sn,
        y: z * a_sn + a_cs * (y * b_cs - x * b_sn),
        z: z * a_cs - a_sn * (y * b_cs - x * b_sn),
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
