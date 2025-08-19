use macroquad::prelude::*;

pub fn draw_roads(width: f32, height: f32, lane_width: f32) {
    let midle_x = width / 2.0;
    let midle_y = height / 2.0;
    let dashed_line_len: f32 = 4.0;

    draw_line(midle_x - lane_width, 0.0, midle_x - lane_width, (height / 2.0) - lane_width, 2.0, WHITE);
    draw_line(midle_x - lane_width, (height / 2.0) + lane_width, midle_x - lane_width, height, 2.0, WHITE);

    draw_line(midle_x + lane_width, 0.0, midle_x + lane_width, (height / 2.0) - lane_width, 2.0, WHITE);
    draw_line(midle_x + lane_width, (height / 2.0) + lane_width, midle_x + lane_width, height, 2.0, WHITE);

    draw_line(0.0, midle_y - lane_width, (width/2.0) - lane_width, midle_y - lane_width, 2.0, WHITE);
    draw_line((width/2.0) + lane_width, midle_y - lane_width, width, midle_y - lane_width, 2.0, WHITE);

    draw_line(0.0, midle_y + lane_width, (width/2.0) - lane_width, midle_y + lane_width, 2.0, WHITE);
    draw_line((width/2.0) + lane_width, midle_y + lane_width, width, midle_y + lane_width, 2.0, WHITE);

    for i in 0..((midle_y as i32 - lane_width as i32) / dashed_line_len as i32) {
        if i % 2 == 0 {
            draw_line(midle_x, i as f32 * dashed_line_len, midle_x,(i as f32 * dashed_line_len) + dashed_line_len, 2.0, WHITE);
            draw_line(midle_x, midle_y + lane_width + i as f32 * dashed_line_len, midle_x, midle_y + lane_width + (i as f32 * dashed_line_len) + dashed_line_len, 2.0, WHITE);
        }
    }

    for i in 0..((midle_x as i32 - lane_width as i32) / dashed_line_len as i32) {
        if i % 2 == 0 {
            draw_line(i as f32 * dashed_line_len, midle_y, (i as f32 * dashed_line_len) + dashed_line_len, midle_y, 2.0, WHITE);
            draw_line(midle_x + lane_width + i as f32 * dashed_line_len, midle_y, midle_x + lane_width + (i as f32 * dashed_line_len) + dashed_line_len, midle_y, 2.0, WHITE);
        }
    }
}