mod cpu;
mod memory;
mod instructions_table;

use sdl2;
use sdl2::event::Event;

use imgui::*;
use imgui_sdl2;
use imgui_opengl_renderer;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    let mut sdl_events = sdl_context.event_pump().unwrap();
    let main_window = sdl_video.window("Rusty PSX - Main Window", 950, 550).position_centered().opengl().resizable().build().unwrap();
    let _gl_context = main_window.gl_create_context().expect("Failed to create OpenGL context");
    gl::load_with(|s| sdl_video.gl_get_proc_address(s) as _);
    sdl_video.gl_set_swap_interval(0).unwrap();

    // Init IMGUI
    let mut imgui_context = imgui::Context::create();
    let mut sdl2_imgui = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &main_window);
    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| sdl_video.gl_get_proc_address(s) as _);

    let mut current_cpu = cpu::Cpu::new();
    let mut show_debugger = false;
    let mut cpu_stepping = false;
    let mut last_cycle = cpu::CycleResult::None;
    let mut cpu_breakpoint = ImString::with_capacity(8);
    let mut range_start_str = ImString::with_capacity(8);
    let mut range_end_str = ImString::with_capacity(8);
    let mut range_start = 0;
    let mut range_end = 0;

    'render_loop: loop {
        for event in sdl_events.poll_iter() {
            sdl2_imgui.handle_event(&mut imgui_context, &event);
            match event {
                Event::Quit {..} => break 'render_loop,
                _ => {}
            }
        }

        sdl2_imgui.prepare_frame(imgui_context.io_mut(), &main_window, &sdl_events.mouse_state());
        let imgui_frame = imgui_context.frame();

        Window::new(im_str!("Rusty PSX - Emulation")).size([300.0, 350.0], Condition::Once).build(&imgui_frame, || {
            imgui_frame.text("Emulation Controls");
            imgui_frame.separator();
            imgui_frame.spacing();

            if std::path::PathBuf::from("bios/SCPH1001.bin").exists() {
                imgui_frame.text_colored([0.0, 1.0, 0.0, 1.0], "BIOS detected and loaded.");
            }
            else {
                imgui_frame.text_colored([1.0, 0.0, 0.0, 1.0], "Couldn't find the BIOS file.");
            }

            imgui_frame.spacing();

            if imgui_frame.button(im_str!("Start emulation"), [120.0, 20.0]) {
                current_cpu.cpu_paused = false;
                cpu_stepping = false;
            }
            if imgui_frame.button(im_str!("Pause"), [120.0, 20.0]) {
                current_cpu.cpu_paused = true;
            }
            if imgui_frame.button(im_str!("Restart"), [120.0, 20.0]) {
                current_cpu = cpu::Cpu::new();
                cpu_stepping = false;
                last_cycle = cpu::CycleResult::None;
            }
            imgui_frame.checkbox(im_str!("Show debugger"), &mut show_debugger);
        });

        if show_debugger {
            Window::new(im_str!("Rusty PSX - Debugger")).size([400.0, 400.0], Condition::Once).build(&imgui_frame, || {
                imgui_frame.text("Debugger Controls");
                imgui_frame.separator();
                if imgui_frame.button(im_str!("Run"), [120.0, 20.0]) {
                    current_cpu.cpu_paused = false;
                    cpu_stepping = false;
                }
                if imgui_frame.button(im_str!("Pause"), [120.0, 20.0]) {
                    current_cpu.cpu_paused = true;
                    last_cycle = cpu::CycleResult::None;
                }
                if imgui_frame.button(im_str!("CPU Step"), [120.0, 20.0]) {
                    current_cpu.cpu_paused = false;
                    cpu_stepping = true;
                }
                if imgui_frame.input_text(im_str!("CPU Breakpoint"), &mut cpu_breakpoint).chars_hexadecimal(true).enter_returns_true(true).build() {
                    let value = u32::from_str_radix(cpu_breakpoint.to_str(), 16).unwrap();
                    current_cpu.debugger_breakpoints.push(value);
                    println!("Breakpoint: {}", cpu_breakpoint);
                }
                match last_cycle {
                    cpu::CycleResult::None => {
                        imgui_frame.text_colored([0.0, 0.5, 1.0, 1.0], "Waiting for user input...");
                    },
                    cpu::CycleResult::Breakpoint => {
                        imgui_frame.text_colored([1.0, 1.0, 0.0, 1.0], "CPU found a breakpoint and stopped.");
                        current_cpu.cpu_paused = true;
                    },
                    cpu::CycleResult::Error => {
                        imgui_frame.text_colored([1.0, 0.0, 0.0, 1.0], "CPU found an error and stopped.");
                        current_cpu.cpu_paused = true;
                    },
                    cpu::CycleResult::Success => {
                        imgui_frame.text_colored([0.0, 1.0, 0.0, 1.0], "Running...");
                    },
                }
                imgui_frame.spacing();
                imgui_frame.separator();
                imgui_frame.text("R3000A Main Registers");
                imgui_frame.separator();
                imgui_frame.spacing();
                imgui_frame.columns(4, im_str!("Register Columns"), false);
                imgui_frame.text(format!("r0 {:08X}", current_cpu.registers[0])); imgui_frame.text(format!("r1 {:08X}", current_cpu.registers[1]));
                imgui_frame.text(format!("r2 {:08X}", current_cpu.registers[2])); imgui_frame.text(format!("r3 {:08X}", current_cpu.registers[3]));
                imgui_frame.text(format!("r4 {:08X}", current_cpu.registers[4])); imgui_frame.text(format!("r5 {:08X}", current_cpu.registers[5]));
                imgui_frame.text(format!("r6 {:08X}", current_cpu.registers[6])); imgui_frame.text(format!("r7 {:08X}", current_cpu.registers[7]));
                imgui_frame.next_column();
                imgui_frame.text(format!("r8 {:08X}", current_cpu.registers[8])); imgui_frame.text(format!("r9 {:08X}", current_cpu.registers[9]));
                imgui_frame.text(format!("r10 {:08X}", current_cpu.registers[10])); imgui_frame.text(format!("r11 {:08X}", current_cpu.registers[11]));
                imgui_frame.text(format!("r12 {:08X}", current_cpu.registers[12])); imgui_frame.text(format!("r13 {:08X}", current_cpu.registers[13]));
                imgui_frame.text(format!("r14 {:08X}", current_cpu.registers[14])); imgui_frame.text(format!("r15 {:08X}", current_cpu.registers[15]));
                imgui_frame.next_column();
                imgui_frame.text(format!("r16 {:08X}", current_cpu.registers[16])); imgui_frame.text(format!("r17 {:08X}", current_cpu.registers[17]));
                imgui_frame.text(format!("r18 {:08X}", current_cpu.registers[18])); imgui_frame.text(format!("r19 {:08X}", current_cpu.registers[19]));
                imgui_frame.text(format!("r20 {:08X}", current_cpu.registers[20])); imgui_frame.text(format!("r21 {:08X}", current_cpu.registers[21]));
                imgui_frame.text(format!("r22 {:08X}", current_cpu.registers[22])); imgui_frame.text(format!("r23 {:08X}", current_cpu.registers[23]));
                imgui_frame.next_column();
                imgui_frame.text(format!("r24 {:08X}", current_cpu.registers[24])); imgui_frame.text(format!("r25 {:08X}", current_cpu.registers[25]));
                imgui_frame.text(format!("r26 {:08X}", current_cpu.registers[26])); imgui_frame.text(format!("r27 {:08X}", current_cpu.registers[27]));
                imgui_frame.text(format!("r28 {:08X}", current_cpu.registers[28])); imgui_frame.text(format!("r29 {:08X}", current_cpu.registers[29]));
                imgui_frame.text(format!("r30 {:08X}", current_cpu.registers[30])); imgui_frame.text(format!("r31 {:08X}", current_cpu.registers[31]));
                imgui_frame.next_column();
                imgui_frame.spacing();
                if current_cpu.pc == 0xBFC00000 {
                    imgui_frame.text(format!("PC {:08X}", current_cpu.pc));
                }
                else {
                    imgui_frame.text(format!("PC {:08X}", current_cpu.pc));
                }
                imgui_frame.next_column();
                imgui_frame.spacing();
                imgui_frame.text(format!("hi {:08X}", current_cpu.hi));
                imgui_frame.next_column();
                imgui_frame.spacing();
                imgui_frame.text(format!("lo {:08X}", current_cpu.lo));
            });

            Window::new(im_str!("Rusty PSX - RAM Viewer")).size([470.0, 300.0], Condition::Always).build(&imgui_frame, || {

                imgui_frame.input_text(im_str!("Memory Range Start"), &mut range_start_str).chars_hexadecimal(true).build();
                if imgui_frame.input_text(im_str!("Memory Range End"), &mut range_end_str).chars_hexadecimal(true).enter_returns_true(true).build() {
                    range_start = u32::from_str_radix(range_start_str.to_str(), 16).unwrap();
                    range_end = u32::from_str_radix(range_end_str.to_str(), 16).unwrap();
                }

                imgui_frame.spacing();
                imgui_frame.text_colored([1.0, 0.0, 0.0, 1.0], "High ranges can cause low performance on the debugger.");
                imgui_frame.spacing();
                imgui_frame.separator();

                if range_end > range_start {
                    imgui_frame.columns(17, im_str!("Mem Viewer Columns"), false);
                    imgui_frame.set_column_width(0, 70.0);
                    imgui_frame.set_column_width(1, 23.0);
                    imgui_frame.set_column_width(2, 23.0);
                    imgui_frame.set_column_width(3, 23.0);
                    imgui_frame.set_column_width(4, 23.0);
                    imgui_frame.set_column_width(5, 23.0);
                    imgui_frame.set_column_width(6, 23.0);
                    imgui_frame.set_column_width(7, 23.0);
                    imgui_frame.set_column_width(8, 23.0);
                    imgui_frame.set_column_width(9, 23.0);
                    imgui_frame.set_column_width(10, 23.0);
                    imgui_frame.set_column_width(11, 23.0);
                    imgui_frame.set_column_width(12, 23.0);
                    imgui_frame.set_column_width(13, 23.0);
                    imgui_frame.set_column_width(14, 23.0);
                    imgui_frame.set_column_width(15, 23.0);
                    imgui_frame.set_column_width(16, 23.0);

                    let mut address = range_start;

                    while address < range_end {
                        imgui_frame.text(format!("{:08X}", address));
                        for offset in 0..16 {
                            imgui_frame.next_column();
                            imgui_frame.text(format!("{:02X}", current_cpu.memory.read_byte(address + offset)));
                        }
                        address += 16;
                        imgui_frame.next_column();
                    }
                }
            });
        }

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      
        imgui_renderer.render(imgui_frame);
        main_window.gl_swap_window();

        if !current_cpu.cpu_paused {
            last_cycle = current_cpu.run_instruction();
            if cpu_stepping {
                current_cpu.cpu_paused = true;
            }
        }
    }
}