mod llvm_api;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ctx = llvm_api::Ctxt::new();
        llvm_api::LLVMTarget::init();

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
