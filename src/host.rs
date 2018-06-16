use wasmi::{
    Externals, FuncInstance, FuncRef, ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature,
    Trap, TrapKind, ValueType,
};

pub struct Host {
    pub counter: i32,
}

const RAND_INDEX: usize = 0;

impl Externals for Host {
    fn invoke_index(
        &mut self,
        index: usize,
        _args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            RAND_INDEX => {
                self.counter += 1;
                Ok(Some(RuntimeValue::I32(self.counter)))
            }
            _ => Err(Trap::new(TrapKind::UnexpectedSignature)),
        }
    }
}
impl ModuleImportResolver for Host {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, ::wasmi::Error> {
        match field_name {
            "rand" => Ok(FuncInstance::alloc_host(
                Signature::new(&[] as &[_], Some(ValueType::I32)),
                RAND_INDEX,
            )),
            _ => Err(::wasmi::Error::Instantiation(format!(
                "Export {} not found",
                field_name
            ))),
        }
    }
}
