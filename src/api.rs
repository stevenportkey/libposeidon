use std::ffi::CStr;
use std::str::FromStr;
use anyhow::Context;
use crate::poseidon_bn254;
use crate::utils::write_to_buffer;
use crate::errors::ParseError;
use crate::returncodes::ReturnCodes;

#[no_mangle]
pub extern "C" fn poseidon_hash(
    input: *const cty::c_char,
    buf: *mut cty::c_char,
    max_len: cty::c_int,
) -> cty::c_int {
    let input = unsafe { CStr::from_ptr(input).to_str() };
    match input {
        Ok(input) => match do_poseidon_hash(input) {
            Ok(result) => {
                write_to_buffer(&result, buf, max_len)
            }
            Err(_) => ReturnCodes::InvalidInput as i32,
        },
        _ => ReturnCodes::InvalidInput as i32,
    }
}

fn parse_inputs(inputs: &Vec<String>) -> anyhow::Result<Vec<ark_bn254::Fr>> {
    let fr_vector: Vec<anyhow::Result<ark_bn254::Fr, ()>> = inputs.iter()
        .map(|s| ark_bn254::Fr::from_str(s).map_err(|_| ()))
        .collect();

    let is_error = fr_vector.iter().any(|r| r.is_err());
    match is_error {
        true => Err(ParseError { message: "Failed to parse inputs".to_string() }.into()),
        false => Ok(fr_vector.into_iter().map(|r| r.unwrap()).collect::<Vec<ark_bn254::Fr>>())
    }
}

fn do_poseidon_hash(input: &str) -> anyhow::Result<String> {
    let inputs = serde_json::from_str::<Vec<String>>(input).context("decode json")?;
    let fr_vector = parse_inputs(&inputs).context("parse fr")?;
    let h = poseidon_bn254::hash_scalars(fr_vector).context("hash poseidon")?;
    Ok(h.to_string())
}

#[cfg(test)]
mod test {
    use crate::api::poseidon_hash;

    #[test]
    fn test_poseidon_hash_of_empty_input() {
        const BUFFER_SIZE: usize = 100;
        let mut buffer: Vec<i8> = vec![0; 100];
        let input = &*b"[]".iter().map(|&b| b as i8).collect::<Vec<i8>>();

        let result_size = poseidon_hash(input.as_ptr(), buffer.as_mut_ptr(), BUFFER_SIZE as i32);
        assert_eq!(result_size, -1);
    }

    #[test]
    fn test_poseidon_hash_of_invalid_input() {
        const BUFFER_SIZE: usize = 100;
        let mut buffer: Vec<i8> = vec![0; 100];
        let input = &*b"[\"\"]".iter().map(|&b| b as i8).collect::<Vec<i8>>();

        let result_size = poseidon_hash(input.as_ptr(), buffer.as_mut_ptr(), BUFFER_SIZE as i32);
        assert_eq!(result_size, -1);
    }

    #[test]
    fn test_poseidon_hash_of_invalid_input_non_digit() {
        const BUFFER_SIZE: usize = 100;
        let mut buffer: Vec<i8> = vec![0; 100];
        let input = &*b"[\"e\"]".iter().map(|&b| b as i8).collect::<Vec<i8>>();

        let result_size = poseidon_hash(input.as_ptr(), buffer.as_mut_ptr(), BUFFER_SIZE as i32);
        assert_eq!(result_size, -1);
    }

    #[test]
    fn test_poseidon_hash_of_input_value_zero() {
        const BUFFER_SIZE: usize = 100;
        let mut buffer: Vec<i8> = vec![0; 100];
        let input = &*b"[\"0\"]".iter().map(|&b| b as i8).collect::<Vec<i8>>();

        let result_size = poseidon_hash(input.as_ptr(), buffer.as_mut_ptr(), BUFFER_SIZE as i32);
        assert_eq!(result_size, 77);

        let binding = buffer.into_iter().take(result_size as usize).map(|b| b as u8).collect::<Vec<u8>>();
        let result = String::from_utf8_lossy(binding.as_slice());
        assert_eq!(result, "19014214495641488759237505126948346942972912379615652741039992445865937985820");
    }

    #[test]
    fn test_poseidon_hash_of_input_value_r() {
        const BUFFER_SIZE: usize = 100;
        let mut buffer: Vec<i8> = vec![0; 100];
        let input = &*b"[\"21888242871839275222246405745257275088548364400416034343698204186575808495617\"]".iter().map(|&b| b as i8).collect::<Vec<i8>>();

        let result_size = poseidon_hash(input.as_ptr(), buffer.as_mut_ptr(), BUFFER_SIZE as i32);
        assert_eq!(result_size, 77);

        let binding = buffer.into_iter().take(result_size as usize).map(|b| b as u8).collect::<Vec<u8>>();
        let result = String::from_utf8_lossy(binding.as_slice());
        assert_eq!(result, "19014214495641488759237505126948346942972912379615652741039992445865937985820");
    }

    #[test]
    fn test_poseidon_hash_of_one_input() {
        const BUFFER_SIZE: usize = 100;
        let mut buffer: Vec<i8> = vec![0; 100];
        let input = &*b"[\"123\"]".iter().map(|&b| b as i8).collect::<Vec<i8>>();

        let result_size = poseidon_hash(input.as_ptr(), buffer.as_mut_ptr(), BUFFER_SIZE as i32);
        assert_eq!(result_size, 76);

        let binding = buffer.into_iter().take(result_size as usize).map(|b| b as u8).collect::<Vec<u8>>();
        let result = String::from_utf8_lossy(binding.as_slice());
        assert_eq!(result, "9904028930859697121695025471312564917337032846528014134060777877259199866166");
    }

    #[test]
    fn test_poseidon_hash_of_two_inputs() {
        const BUFFER_SIZE: usize = 100;
        let mut buffer: Vec<i8> = vec![0; 100];
        let input = &*b"[\"123\", \"456\"]".iter().map(|&b| b as i8).collect::<Vec<i8>>();

        let result_size = poseidon_hash(input.as_ptr(), buffer.as_mut_ptr(), BUFFER_SIZE as i32);
        assert_eq!(result_size, 77);

        let binding = buffer.into_iter().take(result_size as usize).map(|b| b as u8).collect::<Vec<u8>>();
        let result = String::from_utf8_lossy(binding.as_slice());
        assert_eq!(result, "19620391833206800292073497099357851348339828238212863168390691880932172496143");
    }
}