use crate::vertex::Vertex;
use std::f32;

pub fn build_circle(radius: f32, num_vertices: usize) -> (Vec<Vertex>, Vec<u16>) {
    let origin = Vertex::new(0.0, 0.0);

    if num_vertices == 0 || radius <= f32::EPSILON {
        (vec![origin], vec![0])
    } else {
        let verts_size = num_vertices + 1;
        let indices_size = num_vertices * 3;

        let mut verts = Vec::with_capacity(verts_size);
        let mut indices = Vec::with_capacity(indices_size);

        // center point
        verts.push(origin);

        let angle_increment = 2.0 * f32::consts::PI / (num_vertices as f32);

        let mut current_angle = angle_increment;

        verts.push(Vertex::new(radius, 0.0));

        // build the first n-1 wedges.
        for i in 1..num_vertices {
            let x = radius * f32::cos(current_angle);
            let y = radius * f32::sin(current_angle);

            verts.push(Vertex::new(x, y));

            indices.push(0);
            indices.push(i as u16);
            indices.push((i as u16) + 1);

            current_angle += angle_increment;
        }

        // finish the circle off.
        indices.push(0);
        indices.push(num_vertices as u16);
        indices.push(1);

        (verts, indices)
    }
}

#[cfg(test)]
mod tests {

    use super::super::vertex::Vertex;
    use std::f32;

    macro_rules! assert_nearly_eq {
        ($x:expr, $y:expr, $d:expr) => {
            assert!(
                $x - $y > -$d && $x - $y < $d,
                "assertion failed: {} ~= {}",
                $x,
                $y
            );
        };
    }

    fn vertex_distance(v0: Vertex, v1: Vertex) -> f32 {
        let dx = v0.position.x - v1.position.x;
        let dy = v0.position.y - v1.position.y;

        f32::sqrt(dx * dx + dy * dy)
    }

    #[test]
    fn build_circle_standard_circle() {
        // This is the expected use case.
        let num_vertices = 64;
        let radius = 1.0;

        let (verts, indices) = super::build_circle(radius, num_vertices);

        assert_eq!(verts.len(), (num_vertices + 1) as usize);
        assert_eq!(indices.len(), (num_vertices * 3) as usize);

        for i in 1..num_vertices {
            // Each point should be radius away from center point.
            let radial_dist = vertex_distance(verts[0], verts[i]);

            assert_nearly_eq!(radial_dist, radius, f32::EPSILON);

            for j in (i + 1)..num_vertices {
                // Each point should be some distance away from
                // every other point.  This is to catch repeating
                // points.
                let dist = vertex_distance(verts[i], verts[j]);

                assert!(
                    dist > f32::EPSILON,
                    "assertion failed: {} > 0, for indices {} (= {:?}) and {} (= {:?})",
                    dist,
                    i,
                    verts[i].position,
                    j,
                    verts[j].position
                );
            }
        }

        for i in 0..num_vertices {
            // Every third index should be the center point.
            assert_eq!(indices[i * 3], 0);
        }
    }

    #[test]
    fn build_circle_zero_vertices() {
        // Zero vertices should be a single point.
        let num_vertices = 0;
        let radius = 1.0;

        let (verts, indices) = super::build_circle(radius, num_vertices);

        assert_eq!(verts.len(), 1);
        assert_eq!(indices.len(), 1);
    }

    #[test]
    fn build_circle_zero_radius() {
        // Zero radius should be a single point.
        let num_vertices = 25;
        let radius = 0.0;

        let (verts, indices) = super::build_circle(radius, num_vertices);

        assert_eq!(verts.len(), 1);
        assert_eq!(indices.len(), 1);
    }
}
