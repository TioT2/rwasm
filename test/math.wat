;; Simple math library

(module
  ;; Fast inverse square root calculation function
  ;; ARGUMENTS:
  ;;   $x f32 - number to calculate fast inverse square root of
  ;; RETURNS:
  ;;   (f32) Fast inverse square root with one newtonian iteration of $x parameter
  (func $f_inv_sqrt (param $x f32) (result f32)
    (local $v f32)
    
    i32.const 0x5F3759DF
    local.get $x
    i32.reinterpret_f32
    i32.const 1
    i32.shr_u
    i32.sub
    f32.reinterpret_i32
    local.set $v

    ;; Multiply x by 0.5
    local.get $x
    f32.const 0.5
    f32.mul
    local.set $x

    ;; Newtonian iteration
    f32.const 1.5
    local.get $v
    local.get $v
    f32.mul
    local.get $x
    f32.mul
    f32.sub
    local.get $v
    f32.mul
    local.set $v

    local.get $v
    return
  ) ;; func $f_inv_sqrt

  (func $f_sign (param $x f32) (result f32)
    (if (f32.gt (local.get $x) (f32.const 0))
      (then
        f32.const 1
        return
      )
    )

    f32.const -1
    return
  )

  (func $f_is_zero (param $x f32) (result i32)
    (if (f32.eq (local.get $x) (f32.const 0))
      (then
        i32.const 1
        return
      )
      (else
        i32.const 0
        return
      )
    )
    i32.const 0xFFFFFFFF
    return
  )

  (func $vec3f_len2 (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.mul (local.get $x) (local.get $x))
    (f32.mul (local.get $y) (local.get $y))
    (f32.mul (local.get $z) (local.get $z))
    f32.add
    f32.add

    return
  )

  (func $vec3f_len (param $x f32) (param $y f32) (param $z f32) (result f32)
    (call $vec3f_len2
      (local.get $x)
      (local.get $y)
      (local.get $z)
    )
    f32.sqrt
    return
  )

  (func $vec3f_norm (param $x f32) (param $y f32) (param $z f32) (result f32 f32 f32)
    (local $inv_len f32)

    (call $vec3f_len2
      (local.get $x)
      (local.get $y)
      (local.get $z)
    )
    call $f_inv_sqrt
    local.set $inv_len

    (f32.mul (local.get $x) (local.get $inv_len))
    (f32.mul (local.get $y) (local.get $inv_len))
    (f32.mul (local.get $z) (local.get $inv_len))

    return
)

  (export "vec3f_len2" (func $vec3f_len2))
  (export "vec3f_len" (func $vec3f_len))
  (export "f_inv_sqrt" (func $f_inv_sqrt))
  (export "f_sign" (func $f_sign))
  (export "f_is_zero" (func $f_is_zero))
  (export "vec3f_norm" (func $vec3f_norm))
)
