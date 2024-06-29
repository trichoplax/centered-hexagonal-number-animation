// Coloured hexagon (masking a filled circle?)
// Animated filling of a hexagon
// Animated colour change of a filled hexagon
// Progressive filling and colour change of 1 hexagon, then its 7 neighbours
// Progressive filling and colour change of each successive layer
// Fade back to white fill at the end
use std::f64::consts::TAU;
use svg::node::element::path::Data;
use svg::node::element::Animate;
use svg::node::element::Circle;
use svg::node::element::Definitions;
use svg::node::element::Path as SVGPath;
use svg::node::element::Rectangle;
use svg::node::element::Use;
use svg::Document;
const ANIMATION_FRAMES_DIRECTORY_NAME: &str = "animation_frames";
const ROOT_3: f64 = 1.7320508075688772;

fn main() {
    let grid_size: u32 = 4; // Size 0 is a single hexagon, size N+1 is size N plus a ring of hexagons
                            //    let number_of_hexagons = (grid_size + 1).pow(3) - grid_size.pow(3);
    let hexagon_radius = 10.0;
    let hexagon_height = hexagon_radius * ROOT_3;
    let stroke_width = 1.0;
    let hexagon_rounded_corner_radius = stroke_width * 1.5;
    let image_width = hexagon_radius * (5.0 + 3.0 * grid_size as f64) + stroke_width * 2.0 / ROOT_3;
    let image_height = hexagon_height * (3.0 + 2.0 * grid_size as f64) + stroke_width;
    let centre = Coordinates {
        x: image_width / 2.0,
        y: image_height / 2.0,
    };
    let hexagon_angle = TAU / 6.0;
    let hexagon_angle_in_degrees = hexagon_angle * 360.0 / TAU;
    let grid_cell_location_rotation = TAU / 12.0;

    let hexagon_straight_line_endpoints: Vec<(f64, f64)> = (0..=5)
        .map(|n| {
            (
                centre.x
                    + hexagon_radius * (n as f64 * hexagon_angle).cos()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 4.0) * hexagon_angle).cos(),
                centre.y
                    + hexagon_radius * (n as f64 * hexagon_angle).sin()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 4.0) * hexagon_angle).sin(),
            )
        })
        .collect();

    let hexagon_rounded_corner_endpoints: Vec<(f64, f64)> = (0..=5)
        .map(|n| {
            (
                centre.x
                    + hexagon_radius * (n as f64 * hexagon_angle).cos()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 2.0) * hexagon_angle).cos(),
                centre.y
                    + hexagon_radius * (n as f64 * hexagon_angle).sin()
                    + hexagon_rounded_corner_radius
                        * hexagon_angle.cos()
                        * ((n as f64 + 2.0) * hexagon_angle).sin(),
            )
        })
        .collect();

    let hexagon_data = Data::new()
        .move_to(hexagon_straight_line_endpoints[0])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[0].0,
            hexagon_rounded_corner_endpoints[0].1,
        ))
        .line_to(hexagon_straight_line_endpoints[1])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[1].0,
            hexagon_rounded_corner_endpoints[1].1,
        ))
        .line_to(hexagon_straight_line_endpoints[2])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[2].0,
            hexagon_rounded_corner_endpoints[2].1,
        ))
        .line_to(hexagon_straight_line_endpoints[3])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[3].0,
            hexagon_rounded_corner_endpoints[3].1,
        ))
        .line_to(hexagon_straight_line_endpoints[4])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[4].0,
            hexagon_rounded_corner_endpoints[4].1,
        ))
        .line_to(hexagon_straight_line_endpoints[5])
        .elliptical_arc_to((
            hexagon_rounded_corner_radius,
            hexagon_rounded_corner_radius,
            0,
            0,
            1,
            hexagon_rounded_corner_endpoints[5].0,
            hexagon_rounded_corner_endpoints[5].1,
        ))
        .close();

    let hexagon_definition = SVGPath::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", hexagon_data)
        .set("id", "hexagon");

    let circle_definition = Circle::new()
        .set("cx", 0)
        .set("cy", 0)
        .set("r", 30)
        .set("fill", "red")
        .set("id", "circle");

    let animated_definition = Animate::new()
        .set("attributeName", "r")
        .set("from", 0)
        .set("to", image_width)
        .set("dur", "5s")
        .set("repeatCount", "indefinite")
        .set("id", "animated");

    let animated_circle_definition = circle_definition.add(animated_definition);

    let definitions = Definitions::new()
        .add(hexagon_definition)
        .add(animated_circle_definition);

    let mut document = Document::new()
        .set("viewBox", (0, 0, image_width, image_height))
        .add(definitions)
        .add(
            Rectangle::new()
                .set("width", image_width)
                .set("height", image_height)
                .set("fill", "cyan"),
        );

    document = document.add(
        Use::new()
            .set("href", "#circle")
            .set("x", image_width / 2.0)
            .set("y", image_height / 2.0),
    );

    for ring in 0..=grid_size {
        if ring == 0 {
            document = document.add(Use::new().set("href", "#hexagon"));
        } else {
            for spoke in 0..=5 {
                for offset in 0..ring {
                    let spoke_angle = spoke as f64 * hexagon_angle + grid_cell_location_rotation;
                    let ring_radius = ring as f64 * hexagon_height;
                    let offset_angle = spoke_angle + 2.0 * hexagon_angle;
                    let offset_distance = offset as f64 * hexagon_height;
                    document = document.add(
                        Use::new()
                            .set("href", "#hexagon")
                            .set(
                                "x",
                                ring_radius * (spoke_angle).cos()
                                    + offset_distance * offset_angle.cos(),
                            )
                            .set(
                                "y",
                                ring_radius * (spoke_angle).sin()
                                    + offset_distance * offset_angle.sin(),
                            ),
                    );
                }
            }
        }
    }

    let svg_save_file_path = format!("{}/test1.svg", ANIMATION_FRAMES_DIRECTORY_NAME);
    match svg::save(&svg_save_file_path, &document) {
        Ok(_) => (),
        Err(error) => panic!(
            "Problem saving SVG file to {}:\n{:?}",
            svg_save_file_path, error
        ),
    };
}

struct Coordinates {
    x: f64,
    y: f64,
}
