use std::path::{PathBuf};

use cpu::CycleResult;

use iui::prelude::*;
use iui::controls::{Button, Entry, Group, Label, Spacer, TabGroup, HorizontalBox, VerticalBox};

mod cpu;
mod memory;
mod instructions_table;


fn main() {

    let mut ui = UI::init().expect("Failed to initialize libui");
    let mut window = Window::new(&ui, "Rusty PSX", 600, 400, WindowType::NoMenubar);
    let mut ui_events = ui.event_loop();

    let mut tab_control = TabGroup::new(&ui);

    let mut bios_box = VerticalBox::new(&ui);
    let mut bios_group = Group::new(&ui, "BIOS\n");
    let mut bios_label = Label::new(&ui, &String::from("BIOS not loaded\n"));
    let mut bios_path = PathBuf::new();
    let mut bios_label_text = String::new();
    let mut bios_button = Button::new(&ui, "Load BIOS");

    bios_box.set_padded(&ui, true);
    bios_box.append(&ui, bios_label.clone(), LayoutStrategy::Compact);
    bios_box.append(&ui, bios_button.clone(), LayoutStrategy::Compact);
    bios_group.set_child(&ui, bios_box);
    bios_group.set_margined(&ui, true);
    tab_control.append(&ui, "BIOS", bios_group);


    let mut emu_box = VerticalBox::new(&ui);
    let mut emu_group = Group::new(&ui, "Emulation\n");
    let mut emu_start_button = Button::new(&ui, "Start Emulation");
    let mut emu_pause_button = Button::new(&ui, "Pause");

    emu_box.set_padded(&ui, true);
    emu_box.append(&ui, emu_start_button.clone(), LayoutStrategy::Compact);
    emu_box.append(&ui, emu_pause_button.clone(), LayoutStrategy::Compact);
    emu_group.set_child(&ui, emu_box);
    emu_group.set_margined(&ui, true);
    tab_control.append(&ui, "Emulation", emu_group);

    
    let mut debug_main_box = VerticalBox::new(&ui);
    let mut debug_controls_box = VerticalBox::new(&ui);
    let mut debug_registers_box = HorizontalBox::new(&ui);
    let mut debug_labels_boxes = vec![VerticalBox::new(&ui), VerticalBox::new(&ui)];
    let mut debug_group = Group::new(&ui, "Debugging");
    let mut debug_controls_group = Group::new(&ui, "Debugging Controls");
    let mut debug_registers_group = Group::new(&ui, "R3000A Registers");
    let debug_spacer = Spacer::new(&ui);


    // Debugger controls
    let mut debug_cpu_run = Button:: new(&ui, "Run");
    let mut debug_cpu_pause = Button:: new(&ui, "Pause");
    let mut debug_cpu_step = Button:: new(&ui, "CPU Step");
    let mut debug_start_debug = Button::new(&ui, "Run and Break");
    let mut debug_breakpoint_entry = Entry::new(&ui);
    let mut debug_set_breakpoint = Button::new(&ui, "Set Breakpoint");
    let mut debug_status_label = Label::new(&ui, "Waiting...");

    // Registers labels
    let mut debug_labels = vec![
    Label::new(&ui, &String::from(format!("r0: {:08X}", 0))), Label::new(&ui, &String::from(format!("r1: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r2: {:08X}", 0))), Label::new(&ui, &String::from(format!("r3: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r4: {:08X}", 0))), Label::new(&ui, &String::from(format!("r5: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r6: {:08X}", 0))), Label::new(&ui, &String::from(format!("r7: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r8: {:08X}", 0))), Label::new(&ui, &String::from(format!("r9: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r10: {:08X}", 0))), Label::new(&ui, &String::from(format!("r11: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r12: {:08X}", 0))), Label::new(&ui, &String::from(format!("r13: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r14: {:08X}", 0))), Label::new(&ui, &String::from(format!("r15: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r16: {:08X}", 0))), Label::new(&ui, &String::from(format!("r17: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r18: {:08X}", 0))), Label::new(&ui, &String::from(format!("r19: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r20: {:08X}", 0))), Label::new(&ui, &String::from(format!("r21: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r22: {:08X}", 0))), Label::new(&ui, &String::from(format!("r23: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r24: {:08X}", 0))), Label::new(&ui, &String::from(format!("r25: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r26: {:08X}", 0))), Label::new(&ui, &String::from(format!("r27: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r28: {:08X}", 0))), Label::new(&ui, &String::from(format!("r29: {:08X}", 0))),
    Label::new(&ui, &String::from(format!("r30: {:08X}", 0))), Label::new(&ui, &String::from(format!("r31: {:08X}", 0)))];

    let mut debug_hi = Label::new(&ui, &String::from(format!("hi: {:08X}", 0)));
    let mut debug_lo = Label::new(&ui, &String::from(format!("lo: {:08X}", 0)));
    let mut debug_pc = Label::new(&ui, &String::from(format!("PC: {:08X}", 0)));
    let mut debug_next_inst = Label::new(&ui, &String::from(format!("Next Instruction: {:08X}", 0)));
    let mut debug_current_inst = Label::new(&ui, &String::from(format!("Instruction: {:08X}", 0)));

    debug_controls_box.append(&ui, debug_start_debug.clone(), LayoutStrategy::Compact);
    debug_controls_box.append(&ui, debug_cpu_run.clone(), LayoutStrategy::Compact);
    debug_controls_box.append(&ui, debug_cpu_pause.clone(), LayoutStrategy::Compact);
    debug_controls_box.append(&ui, debug_cpu_step.clone(), LayoutStrategy::Compact);
    debug_controls_box.append(&ui, debug_breakpoint_entry.clone(), LayoutStrategy::Compact);
    debug_controls_box.append(&ui, debug_set_breakpoint.clone(), LayoutStrategy::Compact);
    debug_controls_box.append(&ui, debug_status_label.clone(), LayoutStrategy::Compact);

    for index in 0..16 {
        let label = debug_labels[index].clone();
        debug_labels_boxes[0].append(&ui, label, LayoutStrategy::Compact);
    }

    for index in 16..32 {
        let label = debug_labels[index].clone();
        debug_labels_boxes[1].append(&ui, label, LayoutStrategy::Compact);
    }

    debug_registers_box.append(&ui, debug_labels_boxes[0].clone(), LayoutStrategy::Compact);
    debug_registers_box.append(&ui, debug_spacer, LayoutStrategy::Stretchy);
    debug_registers_box.append(&ui, debug_labels_boxes[1].clone(), LayoutStrategy::Compact);
    
    debug_controls_group.set_child(&ui, debug_controls_box);
    debug_registers_group.set_child(&ui, debug_registers_box);
    debug_main_box.append(&ui, debug_controls_group, LayoutStrategy::Compact);
    debug_main_box.append(&ui, debug_registers_group, LayoutStrategy::Compact);
    debug_main_box.append(&ui, debug_pc.clone(), LayoutStrategy::Compact);
    debug_main_box.append(&ui, debug_hi.clone(), LayoutStrategy::Compact);
    debug_main_box.append(&ui, debug_lo.clone(), LayoutStrategy::Compact);
    debug_main_box.append(&ui, debug_current_inst.clone(), LayoutStrategy::Stretchy);
    debug_main_box.append(&ui, debug_next_inst.clone(), LayoutStrategy::Stretchy);

    debug_group.set_child(&ui, debug_main_box);
    debug_group.set_margined(&ui, true);
    tab_control.append(&ui, "CPU Debugger", debug_group);
    
    
    window.set_child(&ui, tab_control);
    window.show(&ui);

    
    let mut should_run = true;
    let mut debug_mode = true;
    let mut cpu = cpu::Cpu::new();
    let mut last_cycle: CycleResult;
    let mut can_set_breakpoint = false;
    let mut breakpoint_addr = String::new();

    let primary_table = instructions_table::make_primary_opcodes_hashmap();
    let secondary_table = instructions_table::make_secondary_opcodes_hashmap();

    bios_button.on_clicked(&ui, {
        |_| {
            if let Some(path) = window.open_file(&ui) {
                bios_path = path;
                bios_label_text.push_str("BIOS path: ");
                bios_label_text.push_str(bios_path.to_str().unwrap());
                bios_label_text.push_str("\n");
                bios_label.set_text(&ui, bios_label_text.as_str());
            }
        }
    });

    emu_start_button.on_clicked(&ui, {
        |_| {
            debug_mode = false;
            cpu.cpu_paused = false;
        }
    });

    emu_pause_button.on_clicked(&ui, {
        |_| {
            cpu.cpu_paused = true;
        }
    });

    debug_cpu_run.on_clicked(&ui, {
        |_| {
            cpu.cpu_paused = false;
            debug_mode = false;
        }
    });

    debug_cpu_pause.on_clicked(&ui, {
        |_| {
            let mut ui = ui.clone();
            cpu.cpu_paused = true;
            debug_mode = true;
            ui.set_enabled(debug_cpu_step.clone(), true);
            for index in 0..32 {
                let value = cpu.registers[index];
                debug_labels[index].set_text(&ui, format!("r{}: {:08X}", index, value).as_str());
            }

            debug_hi.set_text(&ui, format!("hi: {:08X}", cpu.hi).as_str());
            debug_lo.set_text(&ui, format!("lo: {:08X}", cpu.lo).as_str());
            debug_pc.set_text(&ui, format!("PC: {:08X}", cpu.pc).as_str());

            if cpu.current_instruction.op() != 0 {
                let label = format!("Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                    primary_table.get(&cpu.current_instruction.op()).unwrap(),
                    cpu.current_instruction.rs(),
                    cpu.current_instruction.rd(),
                    cpu.current_instruction.rt(),
                    cpu.current_instruction.shift(),
                    cpu.current_instruction.immediate(),
                    cpu.current_instruction.target(),
                );

                debug_current_inst.set_text(&ui, label.as_str());
            }
            else {
                let label = format!("Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                    secondary_table.get(&cpu.current_instruction.op()).unwrap(),
                    cpu.current_instruction.rs(),
                    cpu.current_instruction.rd(),
                    cpu.current_instruction.rt(),
                    cpu.current_instruction.shift(),
                    cpu.current_instruction.immediate(),
                    cpu.current_instruction.target(),
                    );

                    debug_current_inst.set_text(&ui, label.as_str());
                }

                if cpu.next_instruction.op() != 0 {
                    let label = format!("Next Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                        primary_table.get(&cpu.next_instruction.op()).unwrap(),
                        cpu.next_instruction.rs(),
                        cpu.next_instruction.rd(),
                        cpu.next_instruction.rt(),
                        cpu.next_instruction.shift(),
                        cpu.next_instruction.immediate(),
                        cpu.next_instruction.target(),
                    );
                    debug_next_inst.set_text(&ui, label.as_str());
                }
                else {
                    let label = format!("Next Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                        secondary_table.get(&cpu.next_instruction.op()).unwrap(),
                        cpu.next_instruction.rs(),
                        cpu.next_instruction.rd(),
                        cpu.next_instruction.rt(),
                        cpu.next_instruction.shift(),
                        cpu.next_instruction.immediate(),
                        cpu.next_instruction.target(),
                    );
                    debug_next_inst.set_text(&ui, label.as_str());
                }
        }
    });

    debug_cpu_step.on_clicked(&ui, {
        |_| {
            cpu.cpu_paused = false;
            debug_mode = true;
        }
    });

    debug_start_debug.on_clicked(&ui, {
        |_| {
            cpu.cpu_paused = true;
            debug_mode = true;
        }
    });

    debug_set_breakpoint.on_clicked(&ui, {
        |_| {
            let value = breakpoint_addr.as_str();
            let new_breakpoint: u32 = u32::from_str_radix(value, 16).unwrap();
            cpu.debugger_breakpoints.push(new_breakpoint);
        }
    });

    debug_breakpoint_entry.on_changed(&ui, {
        |entry| {
            can_set_breakpoint = entry.len() == 8;
            breakpoint_addr = entry;
        }
    });

    

    while should_run {

        if can_set_breakpoint {
            ui.set_enabled(debug_set_breakpoint.clone(), true);
        }
        else {
            ui.set_enabled(debug_set_breakpoint.clone(), false);
        }

        if !cpu.cpu_paused {
            ui.set_enabled(emu_start_button.clone(), false);
            ui.set_enabled(debug_start_debug.clone(), false);
            ui.set_enabled(emu_pause_button.clone(), true);
            last_cycle = cpu.run_instruction();

            match last_cycle {
                CycleResult::Breakpoint => {
                    debug_mode = true;
                    cpu.cpu_paused = true;
                    ui.set_enabled(debug_cpu_step.clone(), true);
                    debug_status_label.set_text(&ui, format!("CPU emulation stopped on a breakpoint at PC 0x{:08X}", cpu.pc).as_str());
                },
                CycleResult::Error => {
                    debug_mode = true;
                    cpu.cpu_paused = true;
                    ui.set_enabled(debug_cpu_step.clone(), false);
                    debug_status_label.set_text(&ui, format!("CPU emulation errored at PC 0x{:08X}", cpu.pc).as_str());
                },
                CycleResult::Success => {
                    debug_status_label.set_text(&ui, "Running");
                },
            }

            if debug_mode {
                cpu.cpu_paused = true;
                ui.set_enabled(debug_cpu_step.clone(), true);

                for index in 0..32 {
                    let value = cpu.registers[index];
                    debug_labels[index].set_text(&ui, format!("r{}: {:08X}", index, value).as_str());
                }
    
                debug_hi.set_text(&ui, format!("hi: {:08X}", cpu.hi).as_str());
                debug_lo.set_text(&ui, format!("lo: {:08X}", cpu.lo).as_str());
                debug_pc.set_text(&ui, format!("PC: {:08X}", cpu.pc).as_str());

                if cpu.current_instruction.op() != 0 {
                    let label = format!("Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                        primary_table.get(&cpu.current_instruction.op()).unwrap(),
                        cpu.current_instruction.rs(),
                        cpu.current_instruction.rd(),
                        cpu.current_instruction.rt(),
                        cpu.current_instruction.shift(),
                        cpu.current_instruction.immediate(),
                        cpu.current_instruction.target(),
                    );

                    debug_current_inst.set_text(&ui, label.as_str());
                }
                else {
                    let label = format!("Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                        secondary_table.get(&cpu.current_instruction.op()).unwrap(),
                        cpu.current_instruction.rs(),
                        cpu.current_instruction.rd(),
                        cpu.current_instruction.rt(),
                        cpu.current_instruction.shift(),
                        cpu.current_instruction.immediate(),
                        cpu.current_instruction.target(),
                    );

                    debug_current_inst.set_text(&ui, label.as_str());
                }

                if cpu.next_instruction.op() != 0 {
                    let label = format!("Next Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                        primary_table.get(&cpu.next_instruction.op()).unwrap(),
                        cpu.next_instruction.rs(),
                        cpu.next_instruction.rd(),
                        cpu.next_instruction.rt(),
                        cpu.next_instruction.shift(),
                        cpu.next_instruction.immediate(),
                        cpu.next_instruction.target(),
                    );
                    debug_next_inst.set_text(&ui, label.as_str());
                }
                else {
                    let label = format!("Next Instruction: {}, rs: {}, rd: {}, rt: {}, shift: {}, immediate: {:08X}, target address: {:08X}",
                        secondary_table.get(&cpu.next_instruction.op()).unwrap(),
                        cpu.next_instruction.rs(),
                        cpu.next_instruction.rd(),
                        cpu.next_instruction.rt(),
                        cpu.next_instruction.shift(),
                        cpu.next_instruction.immediate(),
                        cpu.next_instruction.target(),
                    );
                    debug_next_inst.set_text(&ui, label.as_str());
                }
            }
            else {
                ui.set_enabled(debug_cpu_step.clone(), false);
            }
        }
        else {
            ui.set_enabled(emu_pause_button.clone(), false);
        }
        
        should_run = ui_events.next_tick(&ui);
    }
}
