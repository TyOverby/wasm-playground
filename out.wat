(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32) (result i32)))
  (import "host" "rand" (func (;0;) (type 0)))
  (func (;1;) (type 1) (param i32) (result i32)
    (local i32)
    i32.const 0
    get_local 0
    call 0
    i32.add
    tee_local 1
    i32.store align=1
    get_local 1)
  (memory (;0;) 1)
  (export "_call" (func 1))
  (export "mem" (memory 0)))
