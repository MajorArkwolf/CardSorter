use ascii::{AsciiChar, AsciiStr, AsciiString, IntoAsciiString};
use color_eyre::eyre::{Result, WrapErr, eyre};

pub struct SerialComm {
    comm_port: Box<dyn serialport::SerialPort>
}

impl SerialComm {
    pub fn connect() -> Result<SerialComm> {
        let ports = serialport::available_ports().wrap_err_with(||"no ports found")?;
        for p in ports {
            let comm = serialport::new(p.port_name, 9600).open().wrap_err_with(||"failed to open port")?;
            return Ok(SerialComm{comm_port: comm});
        }
        Err(eyre!("failed to find a valid port"))
    }

    pub fn send(&mut self, s: &AsciiStr) -> Result<()> {
        let x = self.comm_port.write(&[AsciiChar::ENQ.as_byte()])?;
        if x != 1 {
            return Err(eyre!("failed to send SOH byte"));
        }

        loop {
            let mut byte = [0; 1];
            self.comm_port.read_exact(&mut byte)?;
            if byte[0] == AsciiChar::ACK.as_byte() {
                break;
            }
        }
        let string_size = s.len() as u32;
        let output = format!("{}{}{}{}{}{}", AsciiChar::SOH, string_size, AsciiChar::SOX, s, AsciiChar::ETX, AsciiChar::EOT).into_ascii_string()?;
        println!("Payload Size: {}, Payload: {}", string_size, output);
        self.comm_port.write_all(&output.as_bytes()[0..output.len()])?;
        self.comm_port.flush()?;
        Ok(())
    }

    pub fn recieve(&mut self) -> Result<AsciiString> {
        println!("beginning recv");
        loop {
            let mut byte = [0; 1];
            self.comm_port.read_exact(&mut byte)?;
            if byte[0] == AsciiChar::ENQ.as_byte() {
                println!("Found Enquirery");
                let n = self.comm_port.write(&[AsciiChar::ACK.as_byte()])?;
                if n != 1 { return Err(eyre!("failed to write ack back")) }
                break;
            }
        }
    
        let mut output = AsciiString::new();
        println!("Attempting to find start of transmission");
        loop {
            let mut byte = [0; 1];
            self.comm_port.read_exact(&mut byte)?;
            if byte[0] == AsciiChar::SOH.as_byte() {
                println!("Found start of transmission");
                break;
            }
        }
        let mut string_size_buffer = AsciiString::new();
        loop {
            let mut byte = [0; 1];
            self.comm_port.read_exact(&mut byte)?;
            if byte[0] == AsciiChar::SOX.as_byte() {
                break;
            } else {
                string_size_buffer.push(AsciiChar::from_ascii(byte[0]).unwrap());
                println!("Byte: {}, Current Str: {}", AsciiChar::from_ascii(byte[0]).unwrap(), string_size_buffer);
            }
        }
        println!("{}", string_size_buffer);
        let ss: String = string_size_buffer.into();
        println!("Get transmission size");
        let amount_of_bytes = ss.parse::<usize>().unwrap();
        println!("Transmission size: {}", amount_of_bytes);
        let mut byte_array = vec![0; amount_of_bytes];
        println!("Beginning recv");
        self.comm_port.read_exact(&mut byte_array)?;
        let string_in = match AsciiString::from_ascii(byte_array) {
            Ok(v) => v,
            Err(e) => return Err(eyre!("failed to parse ascii stream, {}", e)),
        };
        println!("Message read");
        output.push_str(&string_in);
    
        let mut byte = [0; 2];
        self.comm_port.read_exact(&mut byte)?;
        if byte[0] != AsciiChar::ETX.as_byte() {
            return Err(eyre!("expected etx, found {}", AsciiChar::from_ascii(byte[0])?));
        } 
        if byte[1] != AsciiChar::EOT.as_byte() {
            return Err(eyre!("expected etx, found {}", AsciiChar::from_ascii(byte[1])?));
        }
        Ok(output)
    }
}
