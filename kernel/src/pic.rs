use x86_64::instructions::port::Port;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = 40;

fn io_wait() {
    unsafe {
        Port::<u8>::new(0x80).write(0);
    }
}

pub fn init() {
    unsafe {
        let mut pic1_cmd = Port::<u8>::new(PIC1_CMD);
        let mut pic1_data = Port::<u8>::new(PIC1_DATA);
        let mut pic2_cmd = Port::<u8>::new(PIC2_CMD);
        let mut pic2_data = Port::<u8>::new(PIC2_DATA);

        // ICW1: begin initialization, expect ICW4
        pic1_cmd.write(0x11);
        io_wait();
        pic2_cmd.write(0x11);
        io_wait();

        // ICW2: vector offsets
        pic1_data.write(PIC1_OFFSET);
        io_wait();
        pic2_data.write(PIC2_OFFSET);
        io_wait();

        // ICW3: cascading
        pic1_data.write(4); // slave on IRQ2
        io_wait();
        pic2_data.write(2); // cascade identity
        io_wait();

        // ICW4: 8086 mode
        pic1_data.write(0x01);
        io_wait();
        pic2_data.write(0x01);
        io_wait();

        // Mask all IRQs
        pic1_data.write(0xFF);
        io_wait();
        pic2_data.write(0xFF);
        io_wait();
    }
}

pub fn unmask_irq(irq: u8) {
    unsafe {
        if irq < 8 {
            let mut port = Port::<u8>::new(PIC1_DATA);
            let mask = port.read();
            port.write(mask & !(1 << irq));
        } else {
            let irq = irq - 8;
            let mut port = Port::<u8>::new(PIC2_DATA);
            let mask = port.read();
            port.write(mask & !(1 << irq));
            // Also unmask cascade (IRQ2) on master
            let mut master = Port::<u8>::new(PIC1_DATA);
            let master_mask = master.read();
            master.write(master_mask & !(1 << 2));
        }
    }
}

pub fn send_eoi(vector: u8) {
    unsafe {
        if vector >= PIC2_OFFSET {
            Port::<u8>::new(PIC2_CMD).write(0x20);
        }
        Port::<u8>::new(PIC1_CMD).write(0x20);
    }
}
