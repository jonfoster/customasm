use crate::*;


impl util::BitVec
{
	pub fn format_binary(&self) -> Vec<u8>
	{
		let mut result = Vec::new();
		
		let mut index = 0;
		while index < self.len()
		{
			let mut byte: u8 = 0;
			for _ in 0..8
			{
				byte <<= 1;
				byte |= if self.read(index) { 1 } else { 0 };
				index += 1;
			}
			
			result.push(byte);
		}
		
		result
    }
    

	pub fn format_binstr(&self) -> String
	{
		self.format_str(1)
	}
	
	
	pub fn format_hexstr(&self) -> String
	{
		self.format_str(4)
	}
	
	
	pub fn format_str(&self, bits_per_digit: usize) -> String
	{
		let mut result = String::new();
		
		let mut index = 0;
		while index < self.len()
		{
			let mut digit: u8 = 0;
			for _ in 0..bits_per_digit
			{
				digit <<= 1;
				digit |= if self.read(index) { 1 } else { 0 };
				index += 1;
			}
			
			let c = if digit < 10
				{ ('0' as u8 + digit) as char }
			else
				{ ('a' as u8 + digit - 10) as char };
				
			result.push(c);
		}
		
		result
    }
    

    pub fn format_bindump(&self) -> String
    {
        self.format_dump(1, 8, 8)
    }


    pub fn format_hexdump(&self) -> String
    {
        self.format_dump(4, 8, 16)
    }


    pub fn format_dump(
        &self,
        digit_bits: usize,
        byte_bits: usize,
        bytes_per_line: usize)
        -> String
    {
        let mut result = String::new();
        
        let line_start = 0 / (byte_bits * bytes_per_line);
        let line_end = (self.len() + (bytes_per_line - 1) * byte_bits) / (byte_bits * bytes_per_line);
        
        let line_end = if self.len() < byte_bits { line_start + 1 } else { line_end };
        
        let addr_max_width = format!("{:x}", (line_end - 1) * bytes_per_line).len();
        
        for line_index in line_start..line_end
        {
            result.push_str(&format!(" {:01$x} | ", line_index * bytes_per_line, addr_max_width));
            
            for byte_index in 0..bytes_per_line
            {
                for digit_index in 0..(byte_bits / digit_bits)
                {
                    let digit_first_bit = (line_index * bytes_per_line + byte_index) * byte_bits + digit_index * digit_bits;
                    
                    if digit_first_bit >= self.len()
                    {
                        result.push('.');
                        continue;
                    }
                    
                    let mut digit = 0;
                    for bit_index in 0..digit_bits
                    {
                        digit <<= 1;
                        digit |= if self.read(digit_first_bit + bit_index) { 1 } else { 0 };
                    }
            
                    let c = if digit < 10
                        { ('0' as u8 + digit) as char }
                    else
                        { ('a' as u8 + digit - 10) as char };
                    
                    result.push(c);
                }
                
                result.push(' ');
                
                if byte_index % 4 == 3 && byte_index < bytes_per_line - 1
                    { result.push(' '); }
            }
            
            result.push_str("| ");
            
            if byte_bits == 8
            {
                for byte_index in 0..bytes_per_line
                {
                    let byte_first_bit = (line_index * bytes_per_line + byte_index) * byte_bits;
                        
                    if byte_first_bit >= self.len()
                    {
                        result.push('.');
                        continue;
                    }
                        
                    let mut byte = 0u8;
                    for bit_index in 0..byte_bits
                    {
                        byte <<= 1;
                        byte |= if self.read(byte_first_bit + bit_index) { 1 } else { 0 };
                    }
                    
                    let c = byte as char;
                    
                    if c == ' ' || c == '\t' || c == '\r' || c == '\n'
                        { result.push(' '); }
                    else if c as u8 >= 0x80 || c < ' ' || c == '|'
                        { result.push('.'); }
                    else
                        { result.push(c); }
                }
                
                result.push_str(" |");
            }
            
            result.push('\n');
        }
        
        result
    }
    
	
	pub fn format_mif(&self) -> String
	{
		let mut result = String::new();
		
		let byte_num = self.len() / 8 + if self.len() % 8 != 0 { 1 } else { 0 };
		
		result.push_str(&format!("DEPTH = {};\n", byte_num));
		result.push_str("WIDTH = 8;\n");
		result.push_str("ADDRESS_RADIX = HEX;\n");
		result.push_str("DATA_RADIX = HEX;\n");
		result.push_str("\n");
		result.push_str("CONTENT\n");
		result.push_str("BEGIN\n");
		
		let addr_max_width = format!("{:x}", byte_num - 1).len();
		
		let mut index = 0;
		while index < self.len()
		{
			result.push_str(&format!(" {:1$X}: ", index / 8, addr_max_width));
			
			let mut byte: u8 = 0;
			for _ in 0..8
			{
				byte <<= 1;
				byte |= if self.read(index) { 1 } else { 0 };
				index += 1;
			}
			
			result.push_str(&format!("{:02X};\n", byte));
		}
		
		result.push_str("END;");
		result
	}
	
	
	pub fn format_intelhex(&self) -> String
	{
		let mut result = String::new();
		
		let mut bytes_left = self.len() / 8 + if self.len() % 8 != 0 { 1 } else { 0 };
		
		let mut index = 0;
		while index < self.len()
		{
			let bytes_in_row = if bytes_left > 32 { 32 } else { bytes_left };
			
			result.push(':');
			result.push_str(&format!("{:02X}", bytes_in_row));
			result.push_str(&format!("{:04X}", index / 8));
			result.push_str("00");
			
			let mut checksum = 0_u8;
			checksum = checksum.wrapping_add(bytes_in_row as u8);
			checksum = checksum.wrapping_add(((index / 8) >> 8) as u8);
			checksum = checksum.wrapping_add((index / 8) as u8);
			
			for _ in 0..bytes_in_row
			{
				let mut byte: u8 = 0;
				for _ in 0..8
				{
					byte <<= 1;
					byte |= if self.read(index) { 1 } else { 0 };
					index += 1;
				}
				
				result.push_str(&format!("{:02X}", byte));
				checksum = checksum.wrapping_add(byte);
			}
			
			bytes_left -= bytes_in_row;
			result.push_str(&format!("{:02X}", (!checksum).wrapping_add(1)));
			result.push('\n');
		}
		
		result.push_str(":00000001FF");
		result
	}
	
	
	pub fn format_comma(&self, radix: usize) -> String
	{
		let mut result = String::new();
		
		let mut index = 0;
		while index < self.len()
		{
			let mut byte: u8 = 0;
			for _ in 0..8
			{
				byte <<= 1;
				byte |= if self.read(index) { 1 } else { 0 };
				index += 1;
			}
			
			match radix
			{
				10 => result.push_str(&format!("{}", byte)),
				16 => result.push_str(&format!("0x{:02x}", byte)),
				_  => panic!("invalid radix")
			}
			
			if index < self.len()
			{ 
				result.push_str(", ");
				
				if (index / 8) % 16 == 0
					{ result.push('\n'); }
			}
		}
		
		result
	}
	
	
	pub fn format_c_array(&self, radix: usize, bits_per_word: usize, output_variable_name: &str) -> String
	{
		let mut result = String::new();
		
		result.push_str("const ");
        
        match bits_per_word
        {
            1..=8 => result.push_str("unsigned char "),
            9..=16 => result.push_str("uint16_t "),
            17..=32 => result.push_str("uint32_t "),
            33..=64 => result.push_str("uint64_t "),
            _  => panic!("invalid bits per word for selected output format")
        }
        result.push_str(&output_variable_name);
        result.push_str("[] = {\n");
		
		let len_in_words = (self.len() + bits_per_word - 1) / bits_per_word;
		let addr_max_width = format!("{:x}", len_in_words - 1).len();
		
		let mut index = 0;
		result.push_str(&format!("\t/* 0x{:01$x} */ ", 0, addr_max_width));
        
        let nibbles_per_word = (bits_per_word + 3) / 4;
        // Words_per_line will be 16 in the normal case of 8-bit words
        // We scale down if using longer words.  We use the same scaling for
        // decimal values, it'll be good enough.
        let words_per_line = 32 / nibbles_per_word;
		
		while index < self.len()
		{
			let mut word: u64 = 0;
			for _ in 0..bits_per_word
			{
				word <<= 1;
				word |= if self.read(index) { 1 } else { 0 };
				index += 1;
			}
			
			match radix
			{
				10 => result.push_str(&format!("{}", word)),
				16 => result.push_str(&format!("0x{:01$x}", word, nibbles_per_word)),
				_  => panic!("invalid radix")
			}
			
			if index < self.len()
			{ 
                let address = index / bits_per_word;
				if address % words_per_line == 0
				{
                    // Avoid trailing space after the comma in this case.
					result.push_str(&format!(",\n\t/* 0x{:01$x} */ ", address, addr_max_width));
				}
                else
                {
                    result.push_str(", ");
                }
			}
		}
		
		result.push_str("\n};");
		result
	}
	
	
	// From: https://github.com/milanvidakovic/customasm/blob/master/src/asm/binary_output.rs#L84
	pub fn format_logisim(&self, bits_per_chunk: usize) -> String
	{
		let mut result = String::new();
		result.push_str("v2.0 raw\n");
		
		let mut index = 0;
		while index < self.len()
		{
			let mut value: u16 = 0;
			for _ in 0..bits_per_chunk
			{
				value <<= 1;
				value |= if self.read(index) { 1 } else { 0 };
				index += 1;
			}
			
			result.push_str(&format!("{:01$x} ", value, bits_per_chunk / 4));
			if (index / 8) % 16 == 0
				{ result.push('\n'); }
		}
		
		result
    }
    

    pub fn format_annotated_bin(&self, fileserver: &dyn util::FileServer) -> String
	{
		self.format_annotated(fileserver, 1, 8)
	}
	
	
	pub fn format_annotated_hex(&self, fileserver: &dyn util::FileServer) -> String
	{
		self.format_annotated(fileserver, 4, 2)
	}
	
	
	pub fn format_annotated(&self, fileserver: &dyn util::FileServer, digit_bits: usize, byte_digits: usize) -> String
	{
		let mut result = String::new();
		
		let byte_bits = byte_digits * digit_bits;
		
		let mut outp_width = 2;
		let outp_bit_width = format!("{:x}", digit_bits - 1).len();
		let mut addr_width = 4;
		let mut content_width = (byte_digits + 1) * 1 - 1;
						
		let mut sorted_spans = self.spans.clone();
        sorted_spans.sort_by(|a, b|
        {
            a.offset.cmp(&b.offset)
        });
		
        for span in &sorted_spans
        {
            if let Some(offset) = span.offset
            {
                outp_width = std::cmp::max(
                    outp_width, 
                    format!("{:x}", offset / byte_bits).len());

                addr_width = std::cmp::max(
                    addr_width,
                    format!("{:x}", span.addr).len());

                let data_digits = span.size / digit_bits + if span.size % digit_bits == 0 { 0 } else { 1 };
				let this_content_width = data_digits + data_digits / byte_digits;

				if this_content_width > 1 && this_content_width <= (byte_digits + 1) * 5
				{
					content_width = std::cmp::max(
						content_width,
						this_content_width - 1);
				}
            }
		}
		
		result.push_str(&format!(" {:>1$} |", "outp", outp_width + outp_bit_width + 1));
		result.push_str(&format!(" {:>1$} | data", "addr", addr_width));
		result.push_str("\n");
		result.push_str("\n");
		
		let mut prev_filename = "";
        let mut prev_file_chars = Vec::new();
        
        for span in &sorted_spans
        {
            if let Some(offset) = span.offset
            {
                result.push_str(&format!(" {:1$x}", offset / byte_bits, outp_width));
                result.push_str(&format!(":{:1$x} | ", offset % byte_bits, outp_bit_width));
            }
            else
            {
                result.push_str(&format!(" {:>1$}", "--", outp_width));
                result.push_str(&format!(":{:>1$} | ", "-", outp_bit_width));
            }
            
            result.push_str(&format!("{:1$x} | ", span.addr, addr_width));
            
            let mut contents_str = String::new();
            
            let digit_num = span.size / digit_bits + if span.size % digit_bits == 0 { 0 } else { 1 };
            for digit_index in 0..digit_num
            {
                if digit_index > 0 && digit_index % byte_digits == 0
                    { contents_str.push_str(" "); }
            
                let mut digit = 0;
                for bit_index in 0..digit_bits
                {
                    let i = span.offset.unwrap() + digit_index * digit_bits + bit_index;
                    let bit = self.read(i);

                    digit <<= 1;
                    digit |= if bit { 1 } else { 0 };
                }
                
                let c = if digit < 10
                    { ('0' as u8 + digit) as char }
                else
                    { ('a' as u8 + digit - 10) as char };
                
                contents_str.push(c);
            }
            
            if &*span.span.file != prev_filename
            {
                prev_filename = &*span.span.file;
                prev_file_chars = fileserver.get_chars(diagn::RcReport::new(), &prev_filename, None).ok().unwrap();
            }
            
            let span_location = span.span.location.unwrap();
            let char_counter = util::CharCounter::new(&prev_file_chars);
            
            result.push_str(&format!("{:1$}", contents_str, content_width));
            result.push_str(&format!(" ; {}", char_counter.get_excerpt(span_location.0, span_location.1).iter().collect::<String>()));
            result.push_str("\n");
		}
		
		result
	}
}