use iced::{widget::float, Element, Rectangle, Vector};

pub fn centered<'a, T, E>(content: E) -> float::Float<'a, T>
where
    E: Into<Element<'a, T>>,
{
    float(content).translate(center_float)
}

/// Make a floating widget perfectly centered in a viewport
fn center_float(container: Rectangle, viewport: Rectangle) -> Vector {
    let x = (-container.x) + (viewport.width / 2.0) - (container.width / 2.0);
    let y = (-container.y) + (viewport.height / 2.0) - (container.height / 2.0);
    Vector::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_centers_the_float() {
        let container = Rectangle {
            x: 221.33594,
            y: 0.0,
            width: 78.66406,
            height: 20.8,
        };
        let viewport = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 300.0,
            height: 400.0,
        };

        assert_eq!(
            center_float(container, viewport),
            Vector::new(-110.66797, 189.6)
        );
    }
}
