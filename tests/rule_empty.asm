; :::
#ruledef
{
    {} => 0xff ; error: expected
}

; :::
#subruledef
{
    ld {} => 0xff ; error: expected
}

; :::
#subruledef
{
    {} ld => 0xff ; error: invalid
}

; :::
#subruledef
{
    {}{} => 0xff ; error: invalid
}


; :::
#subruledef reg
{
    {} => 0x00
    a, => 0xaa
    b, => 0xbb
    c, => 0xcc
}

#ruledef
{
    load {r: reg} {val: u8} => 0x55 @ r @ val
}

load 0x11    ; = 0x550011
load a, 0x11 ; = 0x55aa11
load b, 0x11 ; = 0x55bb11
load c, 0x11 ; = 0x55cc11