; AARCH64 call handling instructions needed

; subroutine to get cache line size
; extern "aarch64" fn () -> usize
get_ctr_el0:
    mrs x0, ctr_el0
    ret

; subroutine to clear a cache line
; extern "aarch64" fn (usize) -> ()
invalidate_cacheline:
    ic ivau, x0
    ret

; subroutine that ensures that all our data and cache modifications are in place and then invalidates any already-fetched instructions
; so we're absolutely sure that the altered code will be executed.
; extern "aarch64" fn () -> ()
invalidate_pipeline:
    dsb ish
    isb sy
    ret

; this should all compile to:
;get_ctr_el0:
; 0x20, 0x00, 0x3b, 0xd5
; 0xc0, 0x03, 0x5f, 0xd6

;invalidate_cacheline:
; 0x20, 0x75, 0x0b, 0xd5
; 0xc0, 0x03, 0x5f, 0xd6

;invalidate_pipeline:
; 0x9f, 0x3b, 0x03, 0xd5
; 0xdf, 0x3f, 0x03, 0xd5
; 0xc0, 0x03, 0x5f, 0xd6
