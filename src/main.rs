use std::f64::consts::TAU;
use svg::node::element::path::Data;
use svg::node::element::Animate;
use svg::node::element::Circle;
use svg::node::element::ClipPath;
use svg::node::element::Definitions;
use svg::node::element::Path as SVGPath;
use svg::node::element::Rectangle;
use svg::node::element::Use;
use svg::Document;
const ANIMATION_DIRECTORY_NAME: &str = "animation";
const ROOT_3: f64 = 1.7320508075688772;

fn main() {
    let grid_size: u32 = 4; // Size 0 is a single hexagon, size N+1 is size N plus a ring of hexagons
    let hexagon_radius = 10.0;
    let hexagon_height = hexagon_radius * ROOT_3;
    let stroke_width = 1.0;
    let hexagon_rounded_corner_radius = stroke_width * 1.5;
    let image_width = hexagon_radius * (4.0 * grid_size as f64 - 2.0) + stroke_width * 2.0;
    let image_height = hexagon_height * 2.0 * (grid_size as f64 + 0.5) + stroke_width * 2.0;
    let centre = Coordinates {
        x: image_width / 2.0,
        y: image_height / 2.0,
    };
    let hexagon_angle = TAU / 6.0;
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
        .set("d", hexagon_data.clone())
        .set("id", "hexagon");

    let filled_hexagon_definition = SVGPath::new()
        .set("fill", "black")
        .set("stroke", "none")
        .set("stroke-width", stroke_width)
        .set("d", hexagon_data)
        .set("id", "filled-hexagon");

    let hexagon_clip_definition = ClipPath::new()
        .set("id", "hexagon-clip")
        .add(filled_hexagon_definition);

    let mut definitions = Definitions::new()
        .add(hexagon_definition)
        .add(hexagon_clip_definition);

    let duration = (grid_size as f64 + 1.0) * 2.0 + 2.0;

    for level in 0..=grid_size {
        let circle_definition = Circle::new()
            .set("cx", centre.x)
            .set("cy", centre.y)
            .set("id", format!("level{}-circle", level))
            .set("clip-path", "url(#hexagon-clip)");

        let start_growing = (level as f64 * 2.0) / duration;
        let stop_growing = start_growing + 1.0 / duration;
        let stop_colour_change = stop_growing + 1.0 / duration;
        let start_fading = (duration - 1.0) / duration;

        let animated_radius_definition = Animate::new()
            .set("attributeName", "r")
            .set("dur", format!("{}s", duration))
            .set("repeatCount", "indefinite")
            .set(
                "values",
                format!("0;0;{};{}", hexagon_radius, hexagon_radius),
            )
            .set(
                "keyTimes",
                format!("0;{};{};1", start_growing, stop_growing),
            );

        let animated_colour_definition = Animate::new()
            .set("attributeName", "fill")
            .set("dur", format!("{}s", duration))
            .set("repeatCount", "indefinite")
            .set("values", "red;red;red;purple;purple;white")
            .set(
                "keyTimes",
                format!(
                    "0;{};{};{};{};1",
                    start_growing, stop_growing, stop_colour_change, start_fading
                ),
            );

        let animated_circle_definition = circle_definition
            .add(animated_radius_definition)
            .add(animated_colour_definition);

        definitions = definitions.add(animated_circle_definition);
    }

    let mut document = Document::new()
        .set("viewBox", (0, 0, image_width, image_height))
        .add(definitions)
        .add(
            Rectangle::new()
                .set("width", image_width)
                .set("height", image_height)
                .set("fill", "white"),
        );

    for ring in 0..=grid_size {
        if ring == 0 {
            document = document
                .add(Use::new().set("href", "#level0-circle"))
                .add(Use::new().set("href", "#hexagon"));
        } else {
            for spoke in 0..=5 {
                for offset in 0..ring {
                    let spoke_angle = spoke as f64 * hexagon_angle + grid_cell_location_rotation;
                    let ring_radius = ring as f64 * hexagon_height;
                    let offset_angle = spoke_angle + 2.0 * hexagon_angle;
                    let offset_distance = offset as f64 * hexagon_height;
                    document = document.add(
                        Use::new()
                            .set("href", format!("#level{}-circle", ring))
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

    let svg_save_file_path = format!(
        "{}/centered_hexagonal_numbers.svg",
        ANIMATION_DIRECTORY_NAME
    );
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
