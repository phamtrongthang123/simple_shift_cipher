
#[cfg(test)]
mod tests {
    use super::super::utils;
    #[test]
    fn check_padding(){
        let plain_text: String = String::from("ahhajajaj");
        let pad = utils::padding(&plain_text, 5);
        assert_eq!("ahhajajajA", pad);
    }

    #[test]
    #[allow(non_snake_case)]
    fn check_genIV(){
        let size_block = 10; 
        let IV = utils::generate_IV(size_block);
        assert_eq!(IV.len() as u8, size_block);
    }

    #[test]
    fn check_xor(){
        let a = String::from("thang trong");
        let b = utils::generate_IV(11);
        let c = utils::a_xor_b(&a, &b);
        assert_eq!(utils::a_xor_b(&c, &b), a);
        assert_eq!(utils::a_xor_b(&c, &a), b);
        // println!("{} - {} - {}!!!", a,b, c);
    }


    #[test]
    #[allow(non_snake_case)]
    fn check_OFB(){
        let a = String::from("asd adaaSQWE asd ASD ASD adwq fdgjvnvkwhoq DSG sfkj mankh QL NKLJN H");
        let block_size: usize = 10;
        let b = utils::OFB(&a, block_size as u8);
        let remain = block_size  -  a.len() % block_size;
        assert_eq!(b.2.len() - remain as usize, a.len());
        // println!("{:?}", b);
        let dec_text = utils::decrypt_OFB(&b.2, block_size as u8, b.0, &b.1);
        // println!("{}", dec_text);
        assert_eq!(dec_text, a);
    }

}