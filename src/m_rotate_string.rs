//src/m_rotate_string.rs
pub fn rotate_string(s: String, goal: String) -> bool {

            if s.len() != goal.len(){
                return false;
            }
            let mut s_rotated = s.to_string();
            for _ in 0..s.len() {
                let first_char = s_rotated.remove(0);
                s_rotated.push(first_char);
                if s_rotated == goal {
                    return true;
                }
            }
             false

    }
