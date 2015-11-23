%ifdef ARCH_i386
    %include "header-i386.asm"
    %include "boot-i386.asm"
%endif

%ifdef ARCH_x86_64
    %include "header-x86_64.asm"
    %include "boot-x86_64.asm"
%endif