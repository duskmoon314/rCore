# lab 1.1

**Assembly execution process**

```
0x1000:      auipc   t0,0x0         t0 => 0x1000
0x1004:      addi    a1,t0,32       a1 => 0x1020
0x1008:      csrr    a0,mhartid     a0 0 ?
0x100c:      ld      t0,24(t0)      t0 => 0x80000000
0x1010:      jr      t0

0x1018:      unimp
0x101a:      0x8000

rustsbi code
0x80000000:  csrr    a2,mhartid     a2 0 ?
0x80000004:  lui     t0,0x0         t0 => 0 : %hi(_max_hart_id) = 0
0x80000008:  addi    t0,t0,7        t0 => 7 : %lo(_max_hart_id) = 7
0x8000000c:
 bltu        t0,a2,0x8000003a       t0 7 > a2 0, b F : _stack_abort = 0x8000003a
0x80000010:  auipc   sp,0x200       sp => 0x80200010 <rust_main>
0x80000014:  addi    sp,sp,-16      sp => 0x80200000 <stext>
0x80000018:  lui     t0,0x10        t0 => 0x100000 65536
0x8000001c:  mv      t0,t0
0x80000020:  beqz    a2,0x8000002e  a2 = 0, b T

0x8000002e:  sub     sp,sp,t0       sp => 0x801f0000
0x80000032:  csrw    mscratch,zero  mscrathc => 0 ?
0x80000036:  j       0x80002572     main = 0x80002572 ?

0x80002572:  addi    sp,sp,-1360    sp => 0x801efab0
0x80002576:  sd      ra,1352(sp)    0x801efff8 => ra 0x0
0x8000257a:  sd      a1,328(sp)     0x801efbf8 => a1 0x1020 4128
0x8000257c:  mv      a0,a1          a0 => a1 0x1020 4128
0x8000257e:  sd      a1,1144(sp)    0x801eff28 => a1 0x1020 4128
0x80002582:  sd      a0,320(sp)     0x801efbf0 => a0 0x1020 4128
0x80002584:  auipc   ra,0x0         ra => 0x80002584
0x80002588:  jalr    -188(ra)       pc => 0x800024c8

0x800024c8:  addi    sp,sp,-64      sp => 0x801efa70
0x800024ca:  sd      ra,56(sp)      0x801efaa8 => ra 0x8000258c
0x800024cc:  auipc   ra,0x0         ra => 0x800024cc
0x800024d0:  jalr    -1782(ra)      pc => 0x80001dd6

0x80001dd6:  addi    sp,sp,-16      sp => 0x801efa60
0x80001dd8:  sd      ra,8(sp)       0x801efa68 => ra 0x800024d4
0x80001dda:  auipc   ra,0x0         ra => 0x80001dda
0x80001dde:  jalr    20(ra)         pc => 0x80001dee

0x80001dee:  addi    sp,sp,-16      sp => 0x801efa50
0x80001df0:  csrr    a0,mhartid     a0 0 mhartid 0 ?
0x80001df4:  sd      a0,8(sp)       0x801efa58 => 0x0
0x80001df6:  addi    sp,sp,16       sp => 0x801efa60
0x80001df8:  ret                    pc => 0x80001de2

0x80001de2:  sd      a0,0(sp)       0x801efa60 => 0x0
0x80001de4:  j       0x80001de6
0x80001de6:  ld      a0,0(sp)       a0 => 0x0
0x80001de8:  ld      ra,8(sp)       ra => 0x800024d4
0x80001dea:  addi    sp,sp,16       sp => 0x801efa70
0x80001dec:  ret                    pc => 0x800024d4

0x800024d4:  mv      a1,a0          a1 => a0 0x0
0x800024d6:  sd      a0,48(sp)      0x801efaa0 => 0x0
0x800024d8:  sd      a1,16(sp)      0x801efa80 => 0x0
0x800024da:  j       0x800024dc
0x800024dc:  li      a0,0
0x800024de:  ld      a1,16(sp)
0x800024e0:
 bne a1,a0,0x800024ee               a0 = a1, b F
0x800024e4:  j       0x800024e6
0x800024e6:  li      a0,1           a0 => 0x1
0x800024e8:  sb      a0,31(sp)      0801efa8f => 1 (0x801efa8c => 0x01000000)
0x800024ec:  j       0x80002568

0x80002568:  lbu     a0,31(sp)      a0 => 0x1
0x8000256c:  ld      ra,56(sp)      ra => 0x8000258c
0x8000256e:  addi    sp,sp,64       sp => 0x801efab0
0x80002570:  ret                    pc => 0x8000258c

0x8000258c:  sd      a0,312(sp)     0x801efb38 => 0x1
0x8000258e:  j       0x80002590
0x80002590:  li      a0,0           a0 => 0x0
0x80002592:  ld      a1,312(sp)     a0 => 0x1
0x80002594:
 beq a1,a0,0x8000259c               a0 != a1, b F
0x80002598:  j       0x8000259a
0x8000259a:  j       0x8000259e
0x8000259c:  j       0x8000259e
0x8000259e:  li      a0,0           a0 => 0x0
0x800025a0:  sb      a0,343(sp)     0x801efc07 => 0x0
0x800025a4:  auipc   a1,0x0         a1 => 0x800025a4
0x800025a8:  addi    a1,a1,-556     a1 => 0x80002378
0x800025ac:  sd      a0,304(sp)     0x801efb30 => 0x0
0x800025ae:  mv      a0,a1          a0 => 0x80002378
0x800025b0:  ld      a1,304(sp)     a1 => 0x0
0x800025b2:  auipc   ra,0xfffff     ra => 0x800015b2 (pc - 1000)
0x800025b6:  jalr    236(ra)        pc => 0x8000169e

0x8000169e:  addi    sp,sp,-48      sp => 0x801efa80
0x800016a0:  sd      ra,40(sp)      0x801efaa8 => ra 0x800025ba
0x800016a2:  mv      a2,a1
0x800016a4:  sd      a0,16(sp)      0x801efa90 => 0x80002378
0x800016a6:  sb      a1,31(sp)      0x801efa9f => 0x0
0x800016aa:  add     a1,a1,a0       a1 => 0x80002378
0x800016ac:  sd      a1,8(sp)       0x801efa88 => 0x80002378
0x800016ae:
 bltu        a1,a0,0x800016c8       a1 = a0, b F
0x800016b2:  j       0x800016b4
0x800016b4:  ld      a0,8(sp)       a0 => 0x80002378
0x800016b6:  sd      a0,32(sp)      0x801efaa0 => 0x80002378
0x800016b8:  auipc   ra,0x0         ra => 0x800016b8
0x800016bc:  jalr    44(ra)         pc => 0x800016e4

0x800016e4:  addi    sp,sp,-16      sp => 0x801efa70
0x800016e6:  sd      a0,8(sp)       0x801efa78 => 0x80002378
0x800016e8:  csrw    mtvec,a0       mtvec => 0x80002378
0x800016ec:  addi    sp,sp,16       sp => 0x801efa80
0x800016ee:  ret                    pc => 0x800016c0

0x800016c0:  j       0x800016c2
0x800016c2:  ld      ra,40(sp)      ra => 0x800025ba
0x800016c4:  addi    sp,sp,48       sp => 0x801efab0
0x800016c6:  ret                    pc => 0x800025ba

0x800025ba:  j       0x800025bc
0x800025bc:  auipc   ra,0x0         ra => 0x800025bc
0x800025c0:  jalr    -2022(ra)      pc => 0x80001dd6

0x80001dd6:  addi    sp,sp,-16      sp => 0x801efaa0
0x80001dd8:  sd      ra,8(sp)
0x80001dda:  auipc   ra,0x0         ra => 0x80001dda
0x80001dde:  jalr    20(ra)         pc => 0x80001dee

0x80001dee:  addi    sp,sp,-16      sp => 0x801efa90
0x80001df0:  csrr    a0,mhartid     a0 => 0x0, mhartid 0x0
0x80001df4:  sd      a0,8(sp)
0x80001df6:  addi    sp,sp,16       sp => 0x81efaa0
0x80001df8:  ret

0x80001de2:  sd      a0,0(sp)
0x80001de4:  j       0x80001de6
0x80001de6:  ld      a0,0(sp)       a0 => 0x0
0x80001de8:  ld      ra,8(sp)       ra => 0x800025c4
0x80001dea:  addi    sp,sp,16       sp => 0801efab0
0x80001dec:  ret

0x800025c4:  sd      a0,296(sp)
0x800025c6:  j       0x800025c8
0x800025c8:  li      a0,0
0x800025ca:  ld      a1,296(sp)     a1 => 0x0
0x800025cc:
 bne a1,a0,0x800025fe               b F
0x800025d0:  j       0x800025d2
0x800025d2:  auipc   a0,0x17        a0 => 0x800195d2
0x800025d6:  addi    a0,a0,-274     a0 => 0x800194c0
0x800025da:  sd      a0,1152(sp)
0x800025de:  auipc   a0,0x8001e     a0 => 0x205de
0x800025e2:  addi    a0,a0,-1502    a0 => 0x20000
0x800025e6:  sd      a0,1160(sp)
0x800025ea:  auipc   a0,0x17        a0 => 0x800195ea
0x800025ee:  addi    a0,a0,-626     a0 => 0x80019378
0x800025f2:  auipc   ra,0xb         ra => 0x8000d5f2
0x800025f6:  jalr    -1888(ra)

0x8000ce92:  addi    sp,sp,-16      a0: 0x80019378
0x8000ce94:  sd      a0,8(sp)
0x8000ce96:  addi    sp,sp,16
0x8000ce98:  ret

0x800025fa:  sd      a0,288(sp)
0x800025fc:  j       0x80002600
0x800025fe:  j       0x800026cc
0x80002600:  ld      a0,288(sp)     a0 => 0x80019378
0x80002602:  auipc   ra,0xc         ra => 0x8000e602
0x80002606:  jalr    100(ra)

0x8000e666:  addi    sp,sp,-32
0x8000e668:  sd      ra,24(sp)
0x8000e66a:  sd      a0,16(sp)
0x8000e66c:  sd      a0,8(sp)
0x8000e66e:  auipc   ra,0x0         ra => 0x8000e66e
0x8000e672:  jalr    430(ra)

0x8000e81c:  addi    sp,sp,-64      ra : 0x8000e676
0x8000e81e:  sd      ra,56(sp)
0x8000e820:  sd      a0,48(sp)
0x8000e822:  sd      a0,32(sp)
0x8000e824:  j       0x8000e826
0x8000e826:  ld      a0,32(sp)      a0 => 0x80019378
0x8000e828:  auipc   ra,0x0
0x8000e82c:  jalr    -84(ra)

0x8000e7d4:  addi    sp,sp,-48
0x8000e7d6:  sd      ra,40(sp)
0x8000e7d8:  sd      a0,32(sp)
0x8000e7da:  li      a1,2
0x8000e7dc:  sb      a1,30(sp)
0x8000e7e0:  li      a2,0
0x8000e7e2:  sb      a2,31(sp)
0x8000e7e6:  li      a3,1
0x8000e7e8:  sd      a1,16(sp)
0x8000e7ea:  mv      a1,a2
0x8000e7ec:  sd      a2,8(sp)
0x8000e7ee:  mv      a2,a3
0x8000e7f0:  ld      a3,16(sp)
0x8000e7f2:  ld      a4,8(sp)
0x8000e7f4:  auipc   ra,0x0
0x8000e7f8:  jalr    162(ra)
```
