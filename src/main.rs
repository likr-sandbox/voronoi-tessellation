use std::{collections::HashMap, f64::consts::PI};

use rand::prelude::*;
use spade::{ConstrainedDelaunayTriangulation, InsertionError, Point2, Triangulation};
use svg::{
    node::element::{path::Data, Circle, Path},
    Document,
};

fn main() -> Result<(), InsertionError> {
    let n = 10;
    let rx1 = -500.;
    let ry1 = -500.;
    let rx2 = 500.;
    let ry2 = 500.;
    let mut rng = thread_rng();
    let mut cdt = ConstrainedDelaunayTriangulation::<Point2<_>>::new();
    for _ in 0..n {
        let r = rng.gen_range((0.)..(500. / 2f64.sqrt()));
        let t = rng.gen_range((0.)..(2. * PI));
        cdt.insert(Point2::new(r * t.cos(), r * t.sin()))?;
    }
    let outer = vec![
        Point2::new(-500., 0.),
        Point2::new(0.0, 500.),
        Point2::new(500., 0.),
        Point2::new(0., -500.),
    ];
    let p1 = cdt.insert(outer[0].clone())?;
    let p2 = cdt.insert(outer[1].clone())?;
    let p3 = cdt.insert(outer[2].clone())?;
    let p4 = cdt.insert(outer[3].clone())?;
    cdt.add_constraint(p1, p2);
    cdt.add_constraint(p2, p3);
    cdt.add_constraint(p3, p4);
    cdt.add_constraint(p4, p1);
    // cdt.add_constraint(p1, p3);

    let mut face_pos = HashMap::new();
    let mut edge_faces = HashMap::new();
    for face in cdt.inner_faces() {
        let mut bisectors = vec![];
        for edge in face.adjacent_edges() {
            let p = edge.positions()[0];
            let q = edge.positions()[1];
            bisectors.push((
                q.x - p.x,
                q.y - p.y,
                0.5 * (q.x * q.x - p.x * p.x + q.y * q.y - p.y * p.y),
            ));
            edge_faces
                .entry(edge.as_undirected().index())
                .or_insert(vec![])
                .push(face.index());
        }
        let (a1, b1, c1) = bisectors[0];
        let (a2, b2, c2) = bisectors[1];
        let d = a1 * b2 - a2 * b1;
        let x = (c1 * b2 - c2 * b1) / d;
        let y = (c2 * a1 - c1 * a2) / d;
        face_pos.insert(face.index(), (x, y));
    }

    let mut document = Document::new().set("viewBox", (rx1, ry1, rx2 - rx1, ry2 - ry1));
    for faces in edge_faces.values() {
        if faces.len() > 1 {
            let (x1, y1) = face_pos[&faces[0]];
            let (x2, y2) = face_pos[&faces[1]];
            let d = Data::new().move_to((x1, y1)).line_to((x2, y2));
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "red")
                .set("d", d);
            document = document.add(path);
        } else {
            let (x1, y1) = face_pos[&faces[0]];
            let d = Data::new()
                .move_to((x1, y1))
                .line_to((x1 * 100., y1 * 100.));
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "red")
                .set("d", d);
            document = document.add(path);
        }
    }
    for &(x, y) in face_pos.values() {
        let circle = Circle::new()
            .set("fill", "red")
            .set("cx", x)
            .set("cy", y)
            .set("r", 5);
        document = document.add(circle);
    }

    for edge in cdt.undirected_edges() {
        let p = edge.positions()[0];
        let q = edge.positions()[1];
        let d = Data::new().move_to((p.x, p.y)).line_to((q.x, q.y));
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("d", d);
        document = document.add(path);
    }
    for vertex in cdt.vertices() {
        let p = vertex.position();
        let circle = Circle::new()
            .set("fill", "black")
            .set("cx", p.x)
            .set("cy", p.y)
            .set("r", 3);
        document = document.add(circle);
    }
    svg::save("image.svg", &document).unwrap();
    Ok(())
}
