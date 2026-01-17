use macroquad::prelude::*;

#[macroquad::main("anafida")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut initial_points: Vec<(f32, f32)> = Vec::new();
    let mut chaikin = false;
    let mut timer = 0;
    let mut step_count = 0;

    loop {
        clear_background(BLACK);
        draw_text("#Click Enter to start Chaikin (7 Steps)", 0.0, 15.0, 20.0, RED);
        draw_text(format!("Step: {}/7", step_count).as_str(), 0.0, 30.0, 20.0, WHITE);
        // Create Points
        if !chaikin && is_mouse_button_pressed(MouseButton::Left) {
            let position = mouse_position();
            let pos_tuple = (position.0, position.1);

            if !initial_points.contains(&pos_tuple) {
                initial_points.push(pos_tuple);
                points.push(pos_tuple);
            }
        }
        // Start Chaikin Algorithm
        if is_key_pressed(KeyCode::Enter) && !chaikin {
            if points.len() > 2 {
                chaikin = true;
                step_count = 0;
            }
        }
        // Exit Program
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        // Chaikin Algorithm Logic for 7 Steps
        if chaikin {
            timer += 1;

            // Wait 30 frames between steps
            if timer > 30 {
                timer = 0;

                if step_count < 7 {
                    chaikin_algo(&mut points);
                    step_count += 1;
                } else {
                    // Restart logic: Reset back to the clean initial points
                    points = initial_points.clone();
                    step_count = 0;
                }
            }
        }

        // draw points
        for i in &initial_points {
            draw_circle(i.0, i.1, 4.0, RED);
        }
        // draw lines
        if points.len() > 1 {
            for i in 0..points.len() - 1 {
                draw_line(points[i].0, points[i].1, points[i + 1].0, points[i + 1].1, 2.0, BLUE);
            }
        }

        next_frame().await;
    }
}

fn chaikin_algo(points: &mut Vec<(f32, f32)>) {
    if points.len() < 2 {
        return;
    }

    let mut new_points = Vec::new();

    new_points.push(points[0]);

    for i in 0..points.len() - 1 {
        let p0 = points[i];
        let p1 = points[i + 1];

        let q_x = 0.75 * p0.0 + 0.25 * p1.0;
        let q_y = 0.75 * p0.1 + 0.25 * p1.1;

        let r_x = 0.25 * p0.0 + 0.75 * p1.0;
        let r_y = 0.25 * p0.1 + 0.75 * p1.1;

        new_points.push((q_x, q_y));
        new_points.push((r_x, r_y));
    }

    new_points.push(points[points.len() - 1]);

    *points = new_points;
}
