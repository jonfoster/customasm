; ::: include

#subruledef inner
{
    {x}  => 0x11 @ x`8
    {x}$ => 0x22 @ x`8
}

#ruledef test
{
    ld {x: inner}$ => 0x55 @ x`16
}

; :::
ld 0xaa ; error: no match
; :::
ld 0xaa$ ; = 0x5511aa
; :::
ld 0x99 + 0x11$ ; = 0x5511aa
; :::
ld (0x99 + 0x11)$ ; = 0x5511aa
; :::
ld 0xaa$$ ; = 0x5522aa
