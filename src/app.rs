use std::time::Instant;

use makepad_widgets::*;

const WORK_SECONDS: f32 = 20.*60.;
const BREAK_SECONDS: f32 = 5.*60.;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    TIMER_TEXT_SIZE = 20
    

    App = {{App}} {
        ui:<Window>{
            width: 300,
            height: 200,
            show_bg: true,
            draw_bg: {
                fn sd_circle(p: vec2, r: float, object_pos: vec2) -> float {
                    return length(p-object_pos) - r;
                }
                fn sd_union(dist_1: float, dist_2: float) -> float {
                    return min(dist_1, dist_2);
                }
                fn pixel(self) -> vec4 {
                    let x_pos: float = self.pos.x * 300.;
                    let y_pos: float = self.pos.y * 200.;
                    let cloud_1 = sd_circle(vec2(x_pos, y_pos), 50, vec2(290,150));
                    let cloud_2 = sd_circle(vec2(x_pos, y_pos), 50, vec2(230, 200));
                    let cloud_3 = sd_circle(vec2(x_pos, y_pos), 50, vec2(180, 220));
                    let cloud_4 = sd_circle(vec2(x_pos, y_pos), 50, vec2(330, 100));
                    let dist_cloud = sd_union(cloud_1, cloud_2);
                    dist_cloud = sd_union(dist_cloud, cloud_3);
                    dist_cloud = sd_union(dist_cloud, cloud_4);

                    let dist_sun = sd_circle(vec2(x_pos, y_pos), 50, vec2(0,0));
                    

                    if dist_cloud < 0 {
                        return vec4(1., 1., 1., 1.);
                    } else if dist_sun < 0 {
                        return vec4(1., 0.9, 0., 1.);
                    }
                    else {
                        return vec4(0.53,0.81,0.92,1.)*0.8 + self.pos.y * 0.3*vec4(1.,1.,1.,0.) + 15/length(vec2(x_pos,y_pos)) * vec4(1.,1.,1., 0.);
                    }
                }
            }
            body = <View> {
                flow: Down,
                spacing: 20,

                align: {
                    x: 0.5,
                    y: 0.
                }
                timer_view = <View> {
                    width: Fill,
                    align: {
                        x: 0.5,
                        y: 0.
                    }
                    margin: 20
                    hori_margin = <View> {
                        width: Fill,
                        align: {
                            x: 0.5,
                            y: 0.
                        }
                        flow: Right,
                        spacing: 20,
                        work_timer = <Label> {
                            text:"20:00"
                            draw_text: {
                                text_style: {
                                    font_size: (TIMER_TEXT_SIZE)
                                }
                            }
                        }
                        break_timer = <Label> {
                            text:"05:00"
                            draw_text: {
                                text_style: {
                                    font_size: (TIMER_TEXT_SIZE)
                                }
                            }
                        }
                    }
                    
                    
                }
                button_view = <View> {
                    width: Fill,
                    align: {
                        x: 0.5,
                        y: 0.
                    }
                    start_button = <Button>{text:"Start"}
                    reset_button = <Button>{text:"Reset"}
                }

                
            }
            
        }
    }
}
app_main!(App);

#[derive(Live)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] working: bool,
    #[rust] running: bool,
    #[rust] second_timer: Timer,
    #[rust] work_seconds_left: f32,
    #[rust] break_seconds_left: f32,
    #[rust] last_instant: Option<Instant>,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        if self.second_timer.is_event(event) {
            let passed_seconds = Instant::now().duration_since(self.last_instant.unwrap()).as_secs_f32();
            self.last_instant = Some(Instant::now());
            
            // Update the timers
            if self.working {
                self.work_seconds_left -= passed_seconds;
                self.ui.label(id!(work_timer)).set_text_and_redraw(cx, &seconds_to_label(self.work_seconds_left));
            } else {
                self.break_seconds_left -= passed_seconds;
                self.ui.label(id!(break_timer)).set_text_and_redraw(cx, &seconds_to_label(self.break_seconds_left));
            }

            if self.work_seconds_left <= 0. {
                // End the work period
                self.working = false;
                self.work_seconds_left = WORK_SECONDS;
                self.ui.label(id!(work_timer)).set_text_and_redraw(cx, &seconds_to_label(self.work_seconds_left));
                
            }


            if self.break_seconds_left <= 0. {
                // End the break period
                self.working = true;
                self.break_seconds_left = BREAK_SECONDS;
                self.ui.label(id!(break_timer)).set_text_and_redraw(cx, &seconds_to_label(self.break_seconds_left));
            }
            
        }

        let actions = self.ui.handle_widget_event(cx, event);

        let start_button = self.ui.button(id!(start_button));
        if start_button.clicked(&actions) {
            
            if self.running {
                self.running = false;
                start_button.set_text_and_redraw(cx, "Start");
                cx.stop_timer(self.second_timer);

            } else {
                self.running = true;
                self.working = true;
                self.work_seconds_left = WORK_SECONDS;
                self.break_seconds_left = BREAK_SECONDS;
                start_button.set_text_and_redraw(cx, "Pause");
                self.second_timer = cx.start_interval(1.);
                self.last_instant = Some(Instant::now());

            }
        }

        if self.ui.button(id!(reset_button)).clicked(&actions) {
            self.work_seconds_left = WORK_SECONDS;
            self.break_seconds_left = BREAK_SECONDS;
            self.running = false;
            self.working = true;
            start_button.set_text_and_redraw(cx, "Start");

            self.ui.label(id!(work_timer)).set_text_and_redraw(cx, &seconds_to_label(self.work_seconds_left));
            self.ui.label(id!(break_timer)).set_text_and_redraw(cx, &seconds_to_label(self.work_seconds_left));
            cx.stop_timer(self.second_timer);
        }
    }
}

pub fn seconds_to_label(seconds: f32) -> String {
    let seconds = seconds.ceil() as u32;
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}