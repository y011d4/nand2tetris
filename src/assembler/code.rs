pub struct Code {
    code: String,
}

impl Code {
    pub fn new(code: Option<String>) -> Self {
        match code {
            Some(c) => Self { code: c },
            None => Self {
                code: "null".to_string(),
            },
        }
    }

    pub fn dest(&self) -> Result<String, String> {
        match self.code.as_str() {
            "null" => Ok("000".to_string()),
            "M" => Ok("001".to_string()),
            "D" => Ok("010".to_string()),
            "MD" => Ok("011".to_string()),
            "A" => Ok("100".to_string()),
            "AM" => Ok("101".to_string()),
            "AD" => Ok("110".to_string()),
            "AMD" => Ok("111".to_string()),
            _ => Err(format!("mnemonic {} is not allowed in `dest`.", self.code)),
        }
    }

    pub fn comp(&self) -> Result<String, String> {
        match self.code.as_str() {
            "0" => Ok("0101010".to_string()),
            "1" => Ok("0111111".to_string()),
            "-1" => Ok("0111010".to_string()),
            "D" => Ok("0001100".to_string()),
            "A" => Ok("0110000".to_string()),
            "!D" => Ok("0001101".to_string()),
            "!A" => Ok("0110001".to_string()),
            "-D" => Ok("0001111".to_string()),
            "-A" => Ok("0110011".to_string()),
            "D+1" => Ok("0011111".to_string()),
            "A+1" => Ok("0110111".to_string()),
            "D-1" => Ok("0001110".to_string()),
            "A-1" => Ok("0110010".to_string()),
            "D+A" => Ok("0000010".to_string()),
            "D-A" => Ok("0010011".to_string()),
            "A-D" => Ok("0000111".to_string()),
            "D&A" => Ok("0000000".to_string()),
            "D|A" => Ok("0010101".to_string()),
            "M" => Ok("1110000".to_string()),
            "!M" => Ok("1110001".to_string()),
            "-M" => Ok("1110011".to_string()),
            "M+1" => Ok("1110111".to_string()),
            "M-1" => Ok("1110010".to_string()),
            "D+M" => Ok("1000010".to_string()),
            "D-M" => Ok("1010011".to_string()),
            "M-D" => Ok("1000111".to_string()),
            "D&M" => Ok("1000000".to_string()),
            "D|M" => Ok("1010101".to_string()),
            _ => Err(format!("mnemonic {} is not allowed in `comp`.", self.code)),
        }
    }

    pub fn jump(&self) -> Result<String, String> {
        match self.code.as_str() {
            "null" => Ok("000".to_string()),
            "JGT" => Ok("001".to_string()),
            "JEQ" => Ok("010".to_string()),
            "JGE" => Ok("011".to_string()),
            "JLT" => Ok("100".to_string()),
            "JNE" => Ok("101".to_string()),
            "JLE" => Ok("110".to_string()),
            "JMP" => Ok("111".to_string()),
            _ => Err(format!("mnemonic {} is not allowed in `jump`.", self.code)),
        }
    }
}
