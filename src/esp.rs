use libc;
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/i386/_types.h:20"]
pub mod _types_h {
    #[src_loc = "41:1"]
    pub type __uint8_t = libc::c_uchar;
    #[src_loc = "44:1"]
    pub type __int32_t = libc::c_int;
    #[src_loc = "45:1"]
    pub type __uint32_t = libc::c_uint;
    #[src_loc = "92:1"]
    pub type __darwin_size_t = libc::c_ulong;
    #[src_loc = "118:1"]
    pub type __darwin_socklen_t = __uint32_t;
    #[src_loc = "119:1"]
    pub type __darwin_ssize_t = libc::c_long;
    /*
 * The rune type below is declared to be an ``int'' instead of the more natural
 * ``unsigned long'' or ``long''.  Two things are happening here.  It is not
 * unsigned so that EOF (-1) can be naturally assigned to it and used.  Also,
 * it looks like 10646 will be a 31 bit standard.  This means that if your
 * ints cannot hold 32 bits, you will be in trouble.  The reason an int was
 * chosen over a long is that the is*() and to*() routines take ints (says
 * ANSI C), but they use __darwin_ct_rune_t instead of int.  By changing it
 * here, you lose a bit of ANSI conformance, but your programs will still
 * work.
 *
 * NOTE: rune_t is not covered by ANSI nor other standards, and should not
 * be instantiated outside of lib/libc/locale.  Use wchar_t.  wchar_t and
 * rune_t must be the same type.  Also wint_t must be no narrower than
 * wchar_t, and should also be able to hold all members of the largest
 * character set plus one extra value (WEOF). wint_t must be at least 16 bits.
 */
    /* ct_rune_t */
    /*
 * mbstate_t is an opaque object to keep conversion state, during multibyte
 * stream conversions.  The content must not be referenced by user programs.
 */
    /* for alignment */
    /* mbstate_t */
    /* ptr1 - ptr2 */
    /* __GNUC__ */
    /* sizeof() */
    /* va_list */
    /* wchar_t */
    /* rune_t */
    /* wint_t */
    /* clock() */
    /* socklen_t (duh) */
    /* byte count or error */
    #[src_loc = "120:1"]
    pub type __darwin_time_t = libc::c_long;
    /* _BSD_I386__TYPES_H_ */
    /* time() */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:20"]
pub mod sys__types_h {
    /* Used by statvfs and fstatvfs */
    #[src_loc = "60:1"]
    pub type __darwin_gid_t = __uint32_t;
    /* [???] signal set */
    #[src_loc = "74:1"]
    pub type __darwin_suseconds_t = __int32_t;
    /* [???] microseconds */
    #[src_loc = "75:1"]
    pub type __darwin_uid_t = __uint32_t;
    use super::_types_h::{__uint32_t, __int32_t};
    /* _SYS__TYPES_H_ */
    /* (gcc >= 3.5) */
    /* !(gcc >= 3.5) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:20"]
pub mod _size_t_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __darwin_size_t */
    #[src_loc = "31:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
    /* _SIZE_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:20"]
pub mod _ssize_t_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __darwin_ssize_t */
    #[src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
    /* _SSIZE_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:21"]
pub mod _uint8_t_h {
    /*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 * 
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    #[src_loc = "31:1"]
    pub type uint8_t = libc::c_uchar;
    /* _UINT8_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:21"]
pub mod _uint32_t_h {
    /*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 * 
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    #[src_loc = "31:1"]
    pub type uint32_t = libc::c_uint;
    /* _UINT32_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:21"]
pub mod _uint64_t_h {
    /*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 * 
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    #[src_loc = "31:1"]
    pub type uint64_t = libc::c_ulonglong;
    /* _UINT64_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:22"]
pub mod _uid_t_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __darwin_uid_t */
    #[src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
    /* _UID_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:22"]
pub mod _gid_t_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __darwin_gid_t */
    #[src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_fd_def.h:22"]
pub mod _fd_def_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "49:9"]
    pub struct fd_set {
        pub fds_bits: [__int32_t; 32],
    }
    use super::_types_h::__int32_t;
    /* _FD_SET */
    /*
 * Use the built-in bzero function instead of the library version so that
 * we do not pollute the namespace or introduce prototype warnings.
 */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:22"]
pub mod _timeval_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __darwin_time_t */
    /* __darwin_suseconds_t */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "34:1"]
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
        /* and microseconds */
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
    /* _STRUCT_TIMEVAL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:22"]
pub mod _time_t_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __darwin_time_t */
    #[src_loc = "31:1"]
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
    /* _TIME_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:27"]
pub mod _sa_family_t_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __uint8_t */
    #[src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
    /* _SA_FAMILY_T */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:27"]
pub mod _socklen_t_h {
    /*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
    /* __darwin_socklen_t */
    #[src_loc = "31:1"]
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:27"]
pub mod socket_h {
    /*
 * Copyright (c) 2000-2018 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/*
 * Copyright (c) 1982, 1986, 1990, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)in.h	8.3 (Berkeley) 1/3/94
 * $FreeBSD: src/sys/netinet/in.h,v 1.48.2.2 2001/04/21 14:53:06 ume Exp $
 */
    /* uint(8|16|32)_t */
    /*
 * POSIX 1003.1-2003
 * "Inclusion of the <netinet/in.h> header may also make visible all
 *  symbols from <inttypes.h> and <sys/socket.h>".
 */
    /*
 * The following two #includes insure htonl and family are defined
 */
    /*
 * Constants and structures defined by the internet system,
 * Per RFC 790, September 1981, and numerous additions.
 */
    /*
 * Protocols (RFC 1700)
 */
    /* dummy for IP */
    /* IP6 hop-by-hop options */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* control message protocol */
    /* group mgmt protocol */
    /* gateway^2 (deprecated) */
    /* IPv4 encapsulation */
    /* for compatibility */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* tcp */
    /* Stream protocol II */
    /* exterior gateway protocol */
    /* private interior gateway */
    /* BBN RCC Monitoring */
    /* network voice protocol*/
    /* pup */
    /* Argus */
    /* EMCON */
    /* Cross Net Debugger */
    /* Chaos*/
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* user datagram protocol */
    /* Multiplexing */
    /* DCN Measurement Subsystems */
    /* Host Monitoring */
    /* Packet Radio Measurement */
    /* xns idp */
    /* Trunk-1 */
    /* Trunk-2 */
    /* Leaf-1 */
    /* Leaf-2 */
    /* Reliable Data */
    /* Reliable Transaction */
    /* tp-4 w/ class negotiation */
    /* Bulk Data Transfer */
    /* Network Services */
    /* Merit Internodal */
    /* Sequential Exchange */
    /* Third Party Connect */
    /* InterDomain Policy Routing */
    /* XTP */
    /* Datagram Delivery */
    /* Control Message Transport */
    /* TP++ Transport */
    /* IL transport protocol */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* IP6 header */
    /* Source Demand Routing */
    /* IP6 routing header */
    /* IP6 fragmentation header */
    /* InterDomain Routing*/
    /* resource reservation */
    /* General Routing Encap. */
    /* Mobile Host Routing */
    /* BHA */
    /* IP6 Encap Sec. Payload */
    /* IP6 Auth Header */
    /* Integ. Net Layer Security */
    /* IP with encryption */
    /* Next Hop Resolution */
    /* 55-57: Unassigned */
    /* ICMP6 */
    /* IP6 no next header */
    /* IP6 destination option */
    /* any host internal protocol */
    /* CFTP */
    /* "hello" routing protocol */
    /* SATNET/Backroom EXPAK */
    /* Kryptolan */
    /* Remote Virtual Disk */
    /* Pluribus Packet Core */
    /* Any distributed FS */
    /* Satnet Monitoring */
    /* VISA Protocol */
    /* Packet Core Utility */
    /* Comp. Prot. Net. Executive */
    /* Comp. Prot. HeartBeat */
    /* Wang Span Network */
    /* Packet Video Protocol */
    /* BackRoom SATNET Monitoring */
    /* Sun net disk proto (temp.) */
    /* WIDEBAND Monitoring */
    /* WIDEBAND EXPAK */
    /* ISO cnlp */
    /* VMTP */
    /* Secure VMTP */
    /* Banyon VINES */
    /* TTP */
    /* NSFNET-IGP */
    /* dissimilar gateway prot. */
    /* TCF */
    /* Cisco/GXS IGRP */
    /* OSPFIGP */
    /* Strite RPC protocol */
    /* Locus Address Resoloution */
    /* Multicast Transport */
    /* AX.25 Frames */
    /* IP encapsulated in IP */
    /* Mobile Int.ing control */
    /* Semaphore Comm. security */
    /* Ethernet IP encapsulation */
    /* encapsulation header */
    /* any private encr. scheme */
    /* GMTP*/
    /* 101-252: Partly Unassigned */
    /* Protocol Independent Mcast */
    /* payload compression (IPComp) */
    /* PGM */
    /* SCTP */
    /* 253-254: Experimentation and testing; 255: Reserved (RFC3692) */
/* BSD Private, local use, namespace incursion */
    /* divert pseudo-protocol */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* raw IP packet */
    /* last return value of *_input(), meaning "all job for this pkt is done".  */
    /* (_POSIX_C_SOURCE && !_DARWIN_C_SOURCE) */
    /*
 * Local port number conventions:
 *
 * When a user does a bind(2) or connect(2) with a port number of zero,
 * a non-conflicting local port address is chosen.
 * The default range is IPPORT_RESERVED through
 * IPPORT_USERRESERVED, although that is settable by sysctl.
 *
 * A user may set the IPPROTO_IP option IP_PORTRANGE to change this
 * default assignment range.
 *
 * The value IP_PORTRANGE_DEFAULT causes the default behavior.
 *
 * The value IP_PORTRANGE_HIGH changes the range of candidate port numbers
 * into the "high" range.  These are reserved for client outbound connections
 * which do not want to be filtered by any firewalls.
 *
 * The value IP_PORTRANGE_LOW changes the range to the "low" are
 * that is (by convention) restricted to privileged processes.  This
 * convention is based on "vouchsafe" principles only.  It is only secure
 * if you trust the remote host to restrict these ports.
 *
 * The default range of ports and the high range can be changed by
 * sysctl(3).  (net.inet.ip.port{hi,low}{first,last}_auto)
 *
 * Changing those values has bad security implications if you are
 * using a a stateless firewall that is allowing packets outside of that
 * range in order to allow transparent outgoing connections.
 *
 * Such a firewall configuration will generally depend on the use of these
 * default values.  If you change them, you may find your Security
 * Administrator looking for you with a heavy object.
 *
 * For a slightly more orthodox text view on this:
 *
 *            ftp://ftp.isi.edu/in-notes/iana/assignments/port-numbers
 *
 *    port numbers are divided into three ranges:
 *
 *                0 -  1023 Well Known Ports
 *             1024 - 49151 Registered Ports
 *            49152 - 65535 Dynamic and/or Private Ports
 *
 */
    /*
 * Ports < IPPORT_RESERVED are reserved for
 * privileged processes (e.g. root).         (IP_PORTRANGE_LOW)
 * Ports > IPPORT_USERRESERVED are reserved
 * for servers, not necessarily privileged.  (IP_PORTRANGE_DEFAULT)
 */
    /*
 * Default local port range to use by setting IP_PORTRANGE_HIGH
 */
    /*
 * Scanning for a free reserved port return a value below IPPORT_RESERVED,
 * but higher than IPPORT_RESERVEDSTART.  Traditionally the start value was
 * 512, but that conflicts with some well-known-services that firewalls may
 * have a fit if we use.
 */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /*
 * Internet address (a structure for historical reasons)
 */
    /*
 * Definitions of bits in internet address integers.
 * On subnets, the decomposition of addresses to host and net parts
 * is done according to subnet mask, not the masks here.
 */
    /* must be masked */
    /* These ones aren't really */
    /* net and host fields, but */
    /* routing needn't know.    */
    /* -1 return */
    /* 224.0.0.0 */
    /* 224.0.0.1 */
    /* 224.0.0.2 */
    /* 224.0.0.22, IGMPv3 */
    /* 224.0.0.18 */
    /* 224.0.0.240 */
    /* 224.0.0.251 */
    /* 224.0.0.255 */
    /* 169.254.0.0 */
    /* __APPLE__ */
    /* official! */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /*
 * Socket address, internet style.
 */
    /*
 * Structure used to describe IP options.
 * Used to store options internally, to pass them to a process,
 * or to restore options retrieved earlier.
 * The ip_dst is used for the first-hop gateway when using a source route
 * (this gets put into the header proper).
 */
    /* first hop, 0 w/o src rt */
    /* actually variable in size */
    /*
 * Options for use with [gs]etsockopt at the IP level.
 * First word of comment is data type; bool is stored in int.
 */
    /* buf/ip_opts; set/get IP options */
    /* int; header is included with data */
    /* int; IP type of service and preced. */
    /* int; IP time to live */
    /* bool; receive all IP opts w/dgram */
    /* bool; receive IP opts for response */
    /* bool; receive IP dst addr w/dgram */
    /* ip_opts; set/get IP options */
    /* u_char; set/get IP multicast i/f  */
    /* u_char; set/get IP multicast ttl */
    /* u_char; set/get IP multicast loopback */
    /* ip_mreq; add an IP group membership */
    /* ip_mreq; drop an IP group membership */
    /* set/get IP mcast virt. iface */
    /* enable RSVP in kernel */
    /* disable RSVP in kernel */
    /* set RSVP per-vif socket */
    /* unset RSVP per-vif socket */
    /* int; range to choose for unspec port */
    /* bool; receive reception if w/dgram */
    /* for IPSEC */
    /* int; set/get security policy */
    /* deprecated */
    /* bool: drop receive of raw IP header */
    /* bool; receive reception TTL w/dgram */
    /* int; set/get bound interface */
    /* get pktinfo on recv socket, set src on sent dgram  */
    /* receive pktinfo w/dgram */
    /* bool; receive IP TOS w/dgram */
    /* add a firewall rule to chain */
    /* delete a firewall rule from chain */
    /* flush firewall rule chain */
    /* clear single/all firewall counter(s) */
    /* get entire firewall rule chain */
    /* reset logging counters */
    /* These older firewall socket option codes are maintained for backward compatibility. */
    /* add a firewall rule to chain */
    /* delete a firewall rule from chain */
    /* flush firewall rule chain */
    /* clear single/all firewall counter(s) */
    /* get entire firewall rule chain */
    /* set/get NAT opts XXX Deprecated, do not use */
    /* reset logging counters */
    /* add/configure a dummynet pipe */
    /* delete a dummynet pipe from chain */
    /* flush dummynet */
    /* get entire dummynet pipes */
    /* int*; get background IO flags; set background IO */
    /* int*; set/get IP multicast i/f index */
    /* IPv4 Source Filter Multicast API [RFC3678] */
    /* join a source-specific group */
    /* drop a single source */
    /* block a source */
    /* unblock a source */
    /* The following option is private; do not use it from user applications. */
    /* set/get filter list */
    /* Protocol Independent Multicast API [RFC3678] */
    /* join an any-source group */
    /* leave all sources for group */
    /* join a source-specific group */
    /* leave a single source */
    /* block a source */
    /* unblock a source */
    /*
 * Defaults and limits for options
 */
    /* normally limit m'casts to 1 hop  */
    /* normally hear sends if a member  */
    /*
 * The imo_membership vector for each socket is now dynamically allocated at
 * run-time, bounded by USHRT_MAX, and is reallocated when needed, sized
 * according to a power-of-two increment.
 */
    /*
 * Default resource limits for IPv4 multicast source filtering.
 * These may be modified by sysctl.
 */
    /* sources per group */
    /* sources per socket/group */
    /* XXX no longer used */
    /*
 * Argument structure for IP_ADD_MEMBERSHIP and IP_DROP_MEMBERSHIP.
 */
    /* IP multicast address of group */
    /* local IP address of interface */
    /*
 * Modified argument structure for IP_MULTICAST_IF, obtained from Linux.
 * This is used to specify an interface index for multicast sends, as
 * the IPv4 legacy APIs do not support this (unless IP_SENDIF is available).
 */
    /* IP multicast address of group */
    /* local IP address of interface */
    /* Interface index; cast to uint32_t */
    /*
 * Argument structure for IPv4 Multicast Source Filter APIs. [RFC3678]
 */
    /* IP multicast address of group */
    /* IP address of source */
    /* local IP address of interface */
    /*
 * Argument structures for Protocol-Independent Multicast Source
 * Filter APIs. [RFC3678]
 */
    /* interface index */
    /* group address */
    /* interface index */
    /* group address */
    /* source address */
    /*
 * The following structure is private; do not use it from user applications.
 * It is used to communicate IP_MSFILTER/IPV6_MSFILTER information between
 * the RFC 3678 libc functions and the kernel.
 */
    /* interface index */
    /* filter mode for group */
    /* # of sources in msfr_srcs */
    /* group address */
    /* __MSFILTERREQ_DEFINED */
    /*
 * Advanced (Full-state) APIs [RFC3678]
 * The RFC specifies uint_t for the 6th argument to [sg]etsourcefilter().
 * We use uint32_t here to be consistent.
 */
    /*
 * Filter modes; also used to represent per-socket filter mode internally.
 */
    /* fmode: not yet defined */
    /* fmode: include these source(s) */
    /* fmode: exclude these source(s) */
    /*
 * Argument for IP_PORTRANGE:
 * - which range to search when port is unspecified at bind() or connect()
 */
    /* default range */
    /* "high" - request firewall bypass */
    /* "low" - vouchsafe security */
    /*
 * IP_PKTINFO: Packet information (equivalent to  RFC2292 sec 5 for IPv4)
 * This structure is used for
 *
 * 1) Receiving ancilliary data about the datagram if IP_PKTINFO sockopt is
 *    set on the socket. In this case ipi_ifindex will contain the interface
 *    index the datagram was received on, ipi_addr is the IP address the
 *    datagram was received to.
 *
 * 2) Sending a datagram using a specific interface or IP source address.
 *    if ipi_ifindex is set to non-zero when in_pktinfo is passed as
 *    ancilliary data of type IP_PKTINFO, this will be used as the source
 *    interface to send the datagram from. If ipi_ifindex is null, ip_spec_dst
 *    will be used for the source address.
 *
 *    Note: if IP_BOUND_IF is set on the socket, ipi_ifindex in the ancillary
 *    IP_PKTINFO option silently overrides the bound interface when it is
 *    specified during send time.
 */
    /* send/recv interface index */
    /* Local address */
    /* IP Header dst address */
    /*
 * Definitions for inet sysctl operations.
 *
 * Third level is protocol number.
 * Fourth level is desired variable within that protocol.
 */
    /* don't list to IPPROTO_MAX */
    /*
 * Names for IP sysctl objects
 */
    /* act as router */
    /* may send redirects when forwarding */
    /* default TTL */
    /* cloned route expiration time */
    /* min value for expiration time */
    /* trigger level for dynamic expire */
    /* may perform source routes */
    /* may re-broadcast received packets */
    /* max length of netisr queue */
    /* number of netisr q drops */
    /* ipstat structure */
    /* may accept source routed packets */
    /* use fast IP forwarding code */
    /* deprecated */
    /* default TTL for gif encap packet */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* INET6 stuff */
    /*
 * Copyright (c) 2000-2019 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* Copyright (c) 1998, 1999 Apple Computer, Inc. All Rights Reserved */
/* Copyright (c) 1995 NeXT Computer, Inc. All Rights Reserved */
/*
 * Copyright (c) 1982, 1985, 1986, 1988, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)socket.h	8.4 (Berkeley) 2/21/94
 * $FreeBSD: src/sys/sys/socket.h,v 1.39.2.7 2001/07/03 11:02:01 ume Exp $
 */
/*
 * NOTICE: This file was modified by SPARTA, Inc. in 2005 to introduce
 * support for mandatory and extensible security protections.  This notice
 * is included in support of clause 2.2 (b) of the Apple Public License,
 * Version 2.0.
 */
    /*
 * Definitions related to sockets: types, address families, options.
 */
    /*
 * Data types.
 */
    /* XXX Not explicitly defined by POSIX, but function return types are */
    /* XXX Not explicitly defined by POSIX, but function return types are */
    /*
 * [XSI] The iovec structure shall be defined as described in <sys/uio.h>.
 */
    /*
 * Types
 */
    /* stream socket */
    /* datagram socket */
    /* raw-protocol interface */
    /* reliably-delivered message */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* sequenced packet stream */
    /*
 * Option flags per-socket.
 */
    /* turn on debugging info recording */
    /* socket has had listen() */
    /* allow local address reuse */
    /* keep connections alive */
    /* just use interface addresses */
    /* permit sending of broadcast msgs */
    /* bypass hardware when possible */
    /* linger on close if data present (in ticks) */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /* leave received OOB data in line */
    /* allow local address & port reuse */
    /* timestamp received dgram traffic */
    /* Monotonically increasing timestamp on rcvd dgram */
    /* APPLE: Retain unread data */
                                        /*  (ATOMIC proto) */
    /* APPLE: Give hint when more data ready */
    /* APPLE: Want OOB in MSG_FLAG on receive */
    /* (!__APPLE__) */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
    /*
 * Additional options, not kept in so_options.
 */
    /* send buffer size */
    /* receive buffer size */
    /* send low-water mark */
    /* receive low-water mark */
    /* send timeout */
    /* receive timeout */
    /* get error status and clear */
    /* get socket type */
    /* socket's MAC label */
    /* socket's peer MAC label */
    /* APPLE: get 1st-packet byte count */
    /* APPLE: Install socket-level NKE */
    /* APPLE: No SIGPIPE on EPIPE */
    /* APPLE: Returns EADDRNOTAVAIL when src is not available anymore */
    /* APPLE: Get number of bytes currently in send socket buffer */
    /* APPLE: Allow reuse of port/socket by different userids */
    /* APPLE: send notification if there is a bind on a port which is already in use */
    /* APPLE: block on close until an upcall returns */
    /* linger on close if data present (in seconds) */
    /* APPLE: request local port randomization */
    /* To turn off some POSIX behavior */
    /* number of datagrams in receive socket buffer */
    /* Network service type */
    /* Get QoS marking in effect for socket */
    /*
 * Network Service Type for option SO_NET_SERVICE_TYPE
 *
 * The vast majority of sockets should use Best Effort that is the default
 * Network Service Type. Other Network Service Types have to be used only if
 * the traffic actually matches the description of the Network Service Type.
 *
 * Network Service Types do not represent priorities but rather describe
 * different categories of delay, jitter and loss parameters.
 * Those parameters may influence protocols from layer 4 protocols like TCP
 * to layer 2 protocols like Wi-Fi. The Network Service Type can determine
 * how the traffic is queued and scheduled by the host networking stack and
 * by other entities on the network like switches and routers. For example
 * for Wi-Fi, the Network Service Type can select the marking of the
 * layer 2 packet with the appropriate WMM Access Category.
 *
 * There is no point in attempting to game the system and use
 * a Network Service Type that does not correspond to the actual
 * traffic characteristic but one that seems to have a higher precedence.
 * The reason is that for service classes that have lower tolerance
 * for delay and jitter, the queues size is lower than for service
 * classes that are more tolerant to delay and jitter.
 *
 * For example using a voice service type for bulk data transfer will lead
 * to disastrous results as soon as congestion happens because the voice
 * queue overflows and packets get dropped. This is not only bad for the bulk
 * data transfer but it is also bad for VoIP apps that legitimately are using
 * the voice  service type.
 *
 * The characteristics of the Network Service Types are based on the service
 * classes defined in RFC 4594 "Configuration Guidelines for DiffServ Service
 * Classes"
 *
 * When system detects the outgoing interface belongs to a DiffServ domain
 * that follows the recommendation of the IETF draft "Guidelines for DiffServ to
 * IEEE 802.11 Mapping", the packet will marked at layer 3 with a DSCP value
 * that corresponds to Network Service Type.
 *
 * NET_SERVICE_TYPE_BE
 *	"Best Effort", unclassified/standard.  This is the default service
 *	class and cover the majority of the traffic.
 *
 * NET_SERVICE_TYPE_BK
 *	"Background", high delay tolerant, loss tolerant. elastic flow,
 *	variable size & long-lived. E.g: non-interactive network bulk transfer
 *	like synching or backup.
 *
 * NET_SERVICE_TYPE_RD
 *	"Responsive Data", a notch higher than "Best Effort", medium delay
 *	tolerant, elastic & inelastic flow, bursty, long-lived. E.g. email,
 *	instant messaging, for which there is a sense of interactivity and
 *	urgency (user waiting for output).
 *
 * NET_SERVICE_TYPE_OAM
 *	"Operations, Administration, and Management", medium delay tolerant,
 *	low-medium loss tolerant, elastic & inelastic flows, variable size.
 *	E.g. VPN tunnels.
 *
 * NET_SERVICE_TYPE_AV
 *	"Multimedia Audio/Video Streaming", medium delay tolerant, low-medium
 *	loss tolerant, elastic flow, constant packet interval, variable rate
 *	and size. E.g. video and audio playback with buffering.
 *
 * NET_SERVICE_TYPE_RV
 *	"Responsive Multimedia Audio/Video", low delay tolerant, low-medium
 *	loss tolerant, elastic flow, variable packet interval, rate and size.
 *	E.g. screen sharing.
 *
 * NET_SERVICE_TYPE_VI
 *	"Interactive Video", low delay tolerant, low-medium loss tolerant,
 *	elastic flow, constant packet interval, variable rate & size. E.g.
 *	video telephony.
 *
 * NET_SERVICE_TYPE_SIG
 *	"Signaling", low delay tolerant, low loss tolerant, inelastic flow,
 *	jitter tolerant, rate is bursty but short, variable size. E.g. SIP.
 *
 * NET_SERVICE_TYPE_VO
 *	"Interactive Voice", very low delay tolerant, very low loss tolerant,
 *	inelastic flow, constant packet rate, somewhat fixed size.
 *	E.g. VoIP.
 */
    /* Best effort */
    /* Background system initiated */
    /* Signaling */
    /* Interactive Video */
    /* Interactive Voice */
    /* Responsive Multimedia Audio/Video */
    /* Multimedia Audio/Video Streaming */
    /* Operations, Administration, and Management */
    /* Responsive Data */
    /* These are supported values for SO_NETSVC_MARKING_LEVEL */
    /* The outgoing network interface is not known */
    /* Default marking at layer 2 (for example Wi-Fi WMM) */
    /* Layer 3 DSCP marking and layer 2 marking for all Network Service Types */
    /* The system policy limits layer 3 DSCP marking and layer 2 marking
	                                         * to background Network Service Types */
    /* connectx() flag parameters */
    /* resume connect() on read/write */
    /* data is idempotent */
    /* data includes security that replaces the TFO-cookie */
    /* sockaddr endpoints */
    /* optional source interface */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "305:8"]
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::_types_h::__uint8_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "705:1"]
        pub fn send(_: libc::c_int, _: *const libc::c_void, _: size_t,
                    _: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[src_loc = "701:1"]
        pub fn recv(_: libc::c_int, _: *mut libc::c_void, _: size_t,
                    _: libc::c_int) -> ssize_t;
    }
    /* !_SYS_SOCKET_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netdb.h:27"]
pub mod netdb_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "147:1"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut libc::c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    /* !_NETDB_H_ */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect.h:27"]
pub mod openconnect_h {
    /* ***************************************************************************/
    /* Authentication form processing */
    /* char * fields are static (owned by XML parser) and don't need to be
   freed by the form handling code — except for value, which for TEXT
   and PASSWORD options is allocated by openconnect_set_option_value()
   when process_form() interacts with the user and must be freed. */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "221:1"]
    pub struct oc_form_opt {
        pub next: *mut oc_form_opt,
        pub type_0: libc::c_int,
        pub name: *mut libc::c_char,
        pub label: *mut libc::c_char,
        pub _value: *mut libc::c_char,
        pub flags: libc::c_uint,
        pub reserved: *mut libc::c_void,
    }
    /* All fields are static, owned by the XML parser */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "235:1"]
    pub struct oc_choice {
        pub name: *mut libc::c_char,
        pub label: *mut libc::c_char,
        pub auth_type: *mut libc::c_char,
        pub override_name: *mut libc::c_char,
        pub override_label: *mut libc::c_char,
        pub second_auth: libc::c_int,
        pub secondary_username: *mut libc::c_char,
        pub secondary_username_editable: libc::c_int,
        pub noaaa: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "249:1"]
    pub struct oc_form_opt_select {
        pub form: oc_form_opt,
        pub nr_choices: libc::c_int,
        pub choices: *mut *mut oc_choice,
    }
    /* All char * fields are static, owned by the XML parser */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "256:1"]
    pub struct oc_auth_form {
        pub banner: *mut libc::c_char,
        pub message: *mut libc::c_char,
        pub error: *mut libc::c_char,
        pub auth_id: *mut libc::c_char,
        pub method: *mut libc::c_char,
        pub action: *mut libc::c_char,
        pub opts: *mut oc_form_opt,
        pub authgroup_opt: *mut oc_form_opt_select,
        pub authgroup_selection: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "268:1"]
    pub struct oc_split_include {
        pub route: *const libc::c_char,
        pub next: *mut oc_split_include,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "273:1"]
    pub struct oc_ip_info {
        pub addr: *const libc::c_char,
        pub netmask: *const libc::c_char,
        pub addr6: *const libc::c_char,
        pub netmask6: *const libc::c_char,
        pub dns: [*const libc::c_char; 3],
        pub nbns: [*const libc::c_char; 3],
        pub domain: *const libc::c_char,
        pub proxy_pac: *const libc::c_char,
        pub mtu: libc::c_int,
        pub split_dns: *mut oc_split_include,
        pub split_includes: *mut oc_split_include,
        pub split_excludes: *mut oc_split_include,
        pub gateway_addr: *mut libc::c_char,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "294:1"]
    pub struct oc_vpn_option {
        pub option: *mut libc::c_char,
        pub value: *mut libc::c_char,
        pub next: *mut oc_vpn_option,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "300:1"]
    pub struct oc_stats {
        pub tx_pkts: uint64_t,
        pub tx_bytes: uint64_t,
        pub rx_pkts: uint64_t,
        pub rx_bytes: uint64_t,
    }
    /* ***************************************************************************/
    /* Byte commands to write into the cmd_fd:
 *
 *  CANCEL closes network connections, logs off the session (cookie)
 *    and shuts down the tun device.
 *  PAUSE closes network connections and returns. The caller is expected
 *    to call openconnect_mainloop() again soon.
 *  DETACH closes network connections and shuts down the tun device.
 *    It is not legal to call openconnect_mainloop() again after this,
 *    but a new instance of openconnect can be started using the same
 *    cookie.
 *  STATS calls the stats_handler.
 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "340:1"]
    pub struct openconnect_info {
        pub proto: *const vpn_proto,
        pub ic_legacy_to_utf8: iconv_t,
        pub ic_utf8_to_legacy: iconv_t,
        pub redirect_url: *mut libc::c_char,
        pub redirect_type: libc::c_int,
        pub esp_hmac: libc::c_uchar,
        pub esp_enc: libc::c_uchar,
        pub esp_compr: libc::c_uchar,
        pub esp_replay_protect: uint32_t,
        pub esp_lifetime_bytes: uint32_t,
        pub esp_lifetime_seconds: uint32_t,
        pub esp_ssl_fallback: uint32_t,
        pub current_esp_in: libc::c_int,
        pub old_esp_maxseq: libc::c_int,
        pub esp_in: [esp; 2],
        pub esp_out: esp,
        pub enc_key_len: libc::c_int,
        pub hmac_key_len: libc::c_int,
        pub hmac_out_len: libc::c_int,
        pub esp_magic: uint32_t,
        pub tncc_fd: libc::c_int,
        pub csd_xmltag: *const libc::c_char,
        pub csd_nostub: libc::c_int,
        pub platname: *mut libc::c_char,
        pub mobile_platform_version: *mut libc::c_char,
        pub mobile_device_type: *mut libc::c_char,
        pub mobile_device_uniqueid: *mut libc::c_char,
        pub csd_token: *mut libc::c_char,
        pub csd_ticket: *mut libc::c_char,
        pub csd_stuburl: *mut libc::c_char,
        pub csd_starturl: *mut libc::c_char,
        pub csd_waiturl: *mut libc::c_char,
        pub csd_preurl: *mut libc::c_char,
        pub csd_scriptname: *mut libc::c_char,
        pub opaque_srvdata: *mut xmlNode,
        pub profile_url: *mut libc::c_char,
        pub profile_sha1: *mut libc::c_char,
        pub proxy_factory: *mut pxProxyFactory,
        pub proxy_type: *mut libc::c_char,
        pub proxy: *mut libc::c_char,
        pub proxy_port: libc::c_int,
        pub proxy_fd: libc::c_int,
        pub proxy_user: *mut libc::c_char,
        pub proxy_pass: *mut libc::c_char,
        pub proxy_close_during_auth: libc::c_int,
        pub retry_on_auth_fail: libc::c_int,
        pub try_http_auth: libc::c_int,
        pub http_auth: [http_auth_state; 4],
        pub proxy_auth: [http_auth_state; 4],
        pub localname: *mut libc::c_char,
        pub hostname: *mut libc::c_char,
        pub unique_hostname: *mut libc::c_char,
        pub port: libc::c_int,
        pub urlpath: *mut libc::c_char,
        pub cert_expire_warning: libc::c_int,
        pub cert: *mut libc::c_char,
        pub sslkey: *mut libc::c_char,
        pub cert_password: *mut libc::c_char,
        pub cafile: *mut libc::c_char,
        pub no_system_trust: libc::c_uint,
        pub xmlconfig: *const libc::c_char,
        pub xmlsha1: [libc::c_char; 41],
        pub authgroup: *mut libc::c_char,
        pub nopasswd: libc::c_int,
        pub xmlpost: libc::c_int,
        pub dtls_ciphers: *mut libc::c_char,
        pub dtls12_ciphers: *mut libc::c_char,
        pub csd_wrapper: *mut libc::c_char,
        pub no_http_keepalive: libc::c_int,
        pub dump_http_traffic: libc::c_int,
        pub token_mode: libc::c_int,
        pub token_bypassed: libc::c_int,
        pub token_tries: libc::c_int,
        pub token_time: time_t,
        pub stoken_ctx: *mut stoken_ctx,
        pub stoken_pin: *mut libc::c_char,
        pub stoken_concat_pin: libc::c_int,
        pub stoken_interval: libc::c_int,
        pub oath_secret: *mut libc::c_char,
        pub oath_secret_len: size_t,
        pub oath_hmac_alg: C2RustUnnamed_12,
        pub hotp_secret_format: C2RustUnnamed_11,
        pub pcsc: *mut oc_pcsc_ctx,
        pub yubikey_pwhash: [libc::c_uchar; 16],
        pub lock_token: openconnect_lock_token_vfn,
        pub unlock_token: openconnect_unlock_token_vfn,
        pub tok_cbdata: *mut libc::c_void,
        pub peer_cert: *mut libc::c_void,
        pub peer_cert_sha1_raw: [uint8_t; 20],
        pub peer_cert_sha256_raw: [uint8_t; 32],
        pub peer_cert_hash: *mut libc::c_char,
        pub cert_list_handle: *mut libc::c_void,
        pub cert_list_size: libc::c_int,
        pub cookie: *mut libc::c_char,
        pub cookies: *mut oc_vpn_option,
        pub cstp_options: *mut oc_vpn_option,
        pub dtls_options: *mut oc_vpn_option,
        pub script_env: *mut oc_vpn_option,
        pub csd_env: *mut oc_vpn_option,
        pub pfs: libc::c_uint,
        pub no_tls13: libc::c_uint,
        pub cert_x509: *mut X509,
        pub https_ctx: *mut SSL_CTX,
        pub https_ssl: *mut SSL,
        pub ttls_bio_meth: *mut BIO_METHOD,
        pub ttls_pushbuf: *mut oc_text_buf,
        pub ttls_eap_ident: uint8_t,
        pub ttls_recvbuf: *mut libc::c_uchar,
        pub ttls_recvpos: libc::c_int,
        pub ttls_recvlen: libc::c_int,
        pub pin_cache: *mut pin_cache,
        pub ssl_times: keepalive_info,
        pub owe_ssl_dpd_response: libc::c_int,
        pub deflate_pkt_size: libc::c_int,
        pub deflate_pkt: *mut pkt,
        pub pending_deflated_pkt: *mut pkt,
        pub current_ssl_pkt: *mut pkt,
        pub oncp_control_queue: pkt_q,
        pub oncp_rec_size: libc::c_int,
        pub cstp_pkt: *mut pkt,
        pub dtls_pkt: *mut pkt,
        pub tun_pkt: *mut pkt,
        pub pkt_trailer: libc::c_int,
        pub inflate_strm: z_stream,
        pub inflate_adler32: uint32_t,
        pub deflate_strm: z_stream,
        pub deflate_adler32: uint32_t,
        pub disable_ipv6: libc::c_int,
        pub reconnect_timeout: libc::c_int,
        pub reconnect_interval: libc::c_int,
        pub dtls_attempt_period: libc::c_int,
        pub new_dtls_started: time_t,
        pub dtls_ctx: *mut SSL_CTX,
        pub dtls_ssl: *mut SSL,
        pub cstp_cipher: *mut libc::c_char,
        pub dtls_state: libc::c_int,
        pub dtls_need_reconnect: libc::c_int,
        pub dtls_times: keepalive_info,
        pub dtls_session_id: [libc::c_uchar; 32],
        pub dtls_secret: [libc::c_uchar; 48],
        pub dtls_app_id: [libc::c_uchar; 32],
        pub dtls_app_id_size: libc::c_uint,
        pub ift_seq: uint32_t,
        pub cisco_dtls12: libc::c_int,
        pub dtls_cipher: *mut libc::c_char,
        pub vpnc_script: *mut libc::c_char,
        pub uid_csd_given: libc::c_int,
        pub uid_csd: uid_t,
        pub gid_csd: gid_t,
        pub uid: uid_t,
        pub gid: gid_t,
        pub use_tun_script: libc::c_int,
        pub script_tun: libc::c_int,
        pub ifname: *mut libc::c_char,
        pub cmd_ifname: *mut libc::c_char,
        pub reqmtu: libc::c_int,
        pub basemtu: libc::c_int,
        pub banner: *const libc::c_char,
        pub ip_info: oc_ip_info,
        pub cstp_basemtu: libc::c_int,
        pub idle_timeout: libc::c_int,
        pub _select_nfds: libc::c_int,
        pub _select_rfds: fd_set,
        pub _select_wfds: fd_set,
        pub _select_efds: fd_set,
        pub tun_fd: libc::c_int,
        pub ssl_fd: libc::c_int,
        pub dtls_fd: libc::c_int,
        pub dtls_tos_current: libc::c_int,
        pub dtls_pass_tos: libc::c_int,
        pub dtls_tos_proto: libc::c_int,
        pub dtls_tos_optname: libc::c_int,
        pub cmd_fd: libc::c_int,
        pub cmd_fd_write: libc::c_int,
        pub got_cancel_cmd: libc::c_int,
        pub got_pause_cmd: libc::c_int,
        pub cancel_type: libc::c_char,
        pub incoming_queue: pkt_q,
        pub outgoing_queue: pkt_q,
        pub max_qlen: libc::c_int,
        pub stats: oc_stats,
        pub stats_handler: openconnect_stats_vfn,
        pub peer_addrlen: socklen_t,
        pub peer_addr: *mut sockaddr,
        pub dtls_addr: *mut sockaddr,
        pub dtls_local_port: libc::c_int,
        pub req_compr: libc::c_int,
        pub cstp_compr: libc::c_int,
        pub dtls_compr: libc::c_int,
        pub is_dyndns: libc::c_int,
        pub useragent: *mut libc::c_char,
        pub version_string: *mut libc::c_char,
        pub quit_reason: *const libc::c_char,
        pub verbose: libc::c_int,
        pub cbdata: *mut libc::c_void,
        pub validate_peer_cert: openconnect_validate_peer_cert_vfn,
        pub write_new_config: openconnect_write_new_config_vfn,
        pub process_auth_form: openconnect_process_auth_form_vfn,
        pub progress: openconnect_progress_vfn,
        pub protect_socket: openconnect_protect_socket_vfn,
        pub getaddrinfo_override: openconnect_getaddrinfo_vfn,
        pub setup_tun: openconnect_setup_tun_vfn,
        pub reconnected: openconnect_reconnected_vfn,
        pub ssl_read: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                  _: *mut libc::c_char,
                                                  _: size_t) -> libc::c_int>,
        pub ssl_gets: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                  _: *mut libc::c_char,
                                                  _: size_t) -> libc::c_int>,
        pub ssl_write: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                   _: *mut libc::c_char,
                                                   _: size_t) -> libc::c_int>,
        /* All strings are UTF-8. If operating in a legacy environment where
   nl_langinfo(CODESET) returns anything other than UTF-8, or on Windows,
   the library will take appropriate steps to convert back to the legacy
   character set (or UTF-16) for file handling and wherever else it is
   appropriate to do so. Library functions may (but probably don't yet)
   return -EILSEQ if passed invalid UTF-8 strings. */
        /* Unlike previous versions of openconnect, no functions will take ownership
   of the provided strings. */
        /* Provide environment variables to be set in the CSD trojan environment
   before spawning it. Some callers may need to set $TMPDIR, $PATH and
   other such things if not running from a standard UNIX-like environment.
   To ensure that a variable is unset, pass its name with value==NULL.
   To clear all settings and allow the CSD trojan to inherit an unmodified
   environment, call with name==NULL. */
    }
    /* This string is static, valid only while the connection lasts. If you
 * are going to cache this to remember which certs the user has accepted,
 * make sure you also store the host/port for which it was accepted and
 * don't just accept this cert from *anywhere*. Also use use the check
 * function below instead of manually comparing. When this function
 * returns a string which *doesn't* match the previously-stored hash
 * matched with openconnect_check_peer_cert_hash(), you should store
 * the new result from this function in place of the old. It means
 * we have upgraded to a better hash function. */
    /* Check if the current peer certificate matches a hash previously
 * obtained from openconect_get_peer_cert_hash(). Clients should not
 * attempt to do this using strcmp() and the *current* result of
 * openconnect_get_peer_cert_hash() because it might use
 * a different hash function today. This function will get it right.
 * Returns 0 on match; 1 on mismatch, -errno on failure. */
    /* The buffers returned by these two functions must be freed with
   openconnect_free_cert_info(), especially on Windows. */
    /* Returns the length of the created DER output, in a newly-allocated buffer
   that will need to be freed by openconnect_free_cert_info(). */
    /* Creates a list of all certs in the peer's chain, returning the
   number of certs in the chain (or <0 on error). Only valid inside the
   validate_peer_cert callback. The caller should free the chain,
   but should not modify the contents. */
    /* Contains a comma-separated list of authentication methods to enabled.
   Currently supported: Negotiate,NTLM,Digest,Basic */
    /* These are strictly cosmetic. The strings differ depending on
 * whether OpenSSL or GnuTLS is being used. And even depending on the
 * version of GnuTLS. Do *not* attempt to do anything meaningful based
 * on matching these strings; if you want to do something like that then
 * ask for an API that *does* offer you what you need. */
    /* These return a descriptive string of the compression algorithm 
 * in use (LZS, LZ4, ...). If no compression then NULL is returned. */
    /* Returns the IP address of the exact host to which the connection
 * was made. In --cookieonly mode or in any other scenario involving
 * a "two stage" connection, it is important to reconnect by IP because
 * the server side may be using DNS trickery for load balancing.
 *
 * If the IP address is unavailable due to the use of a proxy, this will
 * fall back to returning the DNS name. */
    /* Returns the hostname parsed out of the server name URL. This is
 * intended to be used by the validate_peer_cert callback to check that
 * the certificate matches the server name. */
    /* Some software tokens, such as HOTP tokens, include a counter which
 * needs to be stored in persistent storage.
 *
 * For such tokens, the lock function is first invoked to obtain a lock
 * on the storage because we're about to generate a new code. It is
 * permitted to call openconnect_set_token_mode() from the lock function,
 * if the token storage has been updated since it was first loaded. The
 * token mode must not change; only the token secret.
 *
 * The unlock function is called when a token code has been generated,
 * with a new token secret to be written to the persistent storage. The
 * secret will be in the same format as it was originally received by
 * openconnect_set_token_mode(). The new token may be NULL if an error
 * was encountered generating the code, in which case it is only
 * necessary for the callback function to unlock the storage.
 */
    /* Legacy stoken-only function; do not use */
    /* The size must be 41 bytes, since that's the size of a 20-byte SHA1
   represented as hex with a trailing NUL. */
    /* call this function to disable the system trust from being used to
 * verify the server certificate. @val is a boolean value.
 *
 * For backwards compatibility reasons this is enabled by default.
 */
    /* Valid choices are: "linux", "linux-64", "win", "mac-intel",
   "android", and "apple-ios". This also selects the corresponding CSD
   trojan binary. */
    /* The returned structures are owned by the library and may be freed/replaced
   due to rekey or reconnect. Assume that once the mainloop starts, the
   pointers are no longer valid. For similar reasons, it is unsafe to call
   this function from another thread. */
    /* If this is set, then openconnect_obtain_cookie() will abort and return
   failure if the file descriptor is readable. Typically a user may create
   a pair of pipes with the pipe(2) system call, hand the readable one to
   this function, and then write a byte to the other end if it ever wants
   to cancel the connection. This way, a multi-threaded UI (which will be
   running openconnect_obtain_cookie() in a separate thread since it blocks)
   has the ability to cancel that call, reap its thread and free the
   vpninfo structure (or retry). An 'fd' argument of -1 will render the
   cancellation mechanism inactive. */
    /* Create a nonblocking pipe used to send cancellations and other commands
   to the library. This returns a file descriptor to the write side of
   the pipe. Both sides will be closed by openconnect_vpninfo_free().
   This replaces openconnect_set_cancel_fd(). */
    /* Open CSTP connection; on success, IP information will be available. */
    /* Create a tun device through the OS kernel (typical use case). Both
   strings are optional and can be NULL if desired. */
    /* Pass traffic to a script program (no tun device). */
    /* Caller will provide a file descriptor for the tunnel traffic. */
    /* Optional call to enable DTLS on the connection. */
    /* Start the main loop; exits if OC_CMD_CANCEL is received on cmd_fd or
   the remote site aborts. */
    /* The first (privdata) argument to each of these functions is either
   the privdata argument provided to openconnect_vpninfo_new_with_cbdata(),
   or if that argument was NULL then it'll be the vpninfo itself. */
    /* When the server's certificate fails validation via the normal means,
   this function is called with the offending certificate along with
   a textual reason for the failure (which may not be translated, if
   it comes directly from OpenSSL, but will be if it is rejected for
   "certificate does not match hostname", because that check is done
   in OpenConnect and *is* translated). The function shall return zero
   if the certificate is (or has in the past been) explicitly accepted
   by the user, and non-zero to abort the connection. */
    /* On a successful connection, the server may provide us with a new XML
   configuration file. This contains the list of servers that can be
   chosen by the user to connect to, amongst other stuff that we mostly
   ignore. By "new", we mean that the SHA1 indicated by the server does
   not match the SHA1 set with the openconnect_set_xmlsha1() above. If
   they don't match, or openconnect_set_xmlsha1() has not been called,
   then the new XML is downloaded and this function is invoked. */
    /* Handle an authentication form, requesting input from the user.
 * Return value:
 *  < 0, on error
 *  = 0, when form was parsed and POST required
 *  = 1, when response was cancelled by user
 */
    /* Logging output which the user *may* want to see. */
    /* Callback to allow binding a newly created socket's file descriptor to
   a specific interface, e.g. with SO_BINDTODEVICE. This tells the kernel
   not to route the traffic in question over the VPN tunnel. */
    /* Callback for obtaining traffic stats via OC_CMD_STATS.
 */
    /* SSL certificate capabilities. openconnect_has_pkcs11_support() means that we
   can accept PKCS#11 URLs in place of filenames, for the certificate and key. */
    /* The OpenSSL TPM ENGINE stores keys in a PEM file labelled with the string
   -----BEGIN TSS KEY BLOB-----. */
    /* Software token capabilities. */
    /* Query and select from among supported protocols */
    /* Callback for configuring the interface after MTU detection finishes. */
    /* Callback for indicating that a TCP reconnection succeeded. */
    #[src_loc = "690:1"]
    pub type openconnect_reconnected_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[src_loc = "685:1"]
    pub type openconnect_setup_tun_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[src_loc = "680:1"]
    pub type openconnect_getaddrinfo_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char,
                                    _: *const addrinfo, _: *mut *mut addrinfo)
                   -> libc::c_int>;
    #[src_loc = "644:1"]
    pub type openconnect_protect_socket_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int)
                   -> ()>;
    #[src_loc = "630:1"]
    pub type openconnect_progress_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int,
                                    _: *const libc::c_char, _: ...) -> ()>;
    #[src_loc = "627:1"]
    pub type openconnect_process_auth_form_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut oc_auth_form) -> libc::c_int>;
    #[src_loc = "619:1"]
    pub type openconnect_write_new_config_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char, _: libc::c_int)
                   -> libc::c_int>;
    #[src_loc = "610:1"]
    pub type openconnect_validate_peer_cert_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char) -> libc::c_int>;
    #[src_loc = "654:1"]
    pub type openconnect_stats_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const oc_stats)
                   -> ()>;
    #[src_loc = "478:1"]
    pub type openconnect_unlock_token_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char) -> libc::c_int>;
    #[src_loc = "477:1"]
    pub type openconnect_lock_token_vfn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
    use super::_uint64_t_h::uint64_t;
    use super::openconnect_internal_h::{vpn_proto, esp, http_auth_state,
                                        C2RustUnnamed_12, C2RustUnnamed_11,
                                        oc_pcsc_ctx, oc_text_buf, pin_cache,
                                        keepalive_info, pkt, pkt_q};
    use super::iconv_h::iconv_t;
    use super::_uint32_t_h::uint32_t;
    use super::tree_h::xmlNode;
    use super::proxy_h::pxProxyFactory;
    use super::_time_t_h::time_t;
    use super::stoken_h::stoken_ctx;
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    use super::ossl_typ_h::{X509, SSL_CTX, SSL};
    use super::bio_h::BIO_METHOD;
    use super::zlib_h::z_stream;
    use super::_uid_t_h::uid_t;
    use super::_gid_t_h::gid_t;
    use super::_fd_def_h::fd_set;
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    use super::netdb_h::addrinfo;
    /* __OPENCONNECT_H__ */
}
#[header_src = "/Users/cameron/github/openconnect/openconnect-internal.h:27"]
pub mod openconnect_internal_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "304:1"]
    pub struct pkt_q {
        pub head: *mut pkt,
        pub tail: *mut *mut pkt,
        pub count: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "123:1"]
    pub struct pkt {
        pub len: libc::c_int,
        pub next: *mut pkt,
        pub c2rust_unnamed: C2RustUnnamed,
        pub data: [libc::c_uchar; 0],
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "126:2"]
    pub union C2RustUnnamed {
        pub esp: C2RustUnnamed_4,
        pub oncp: C2RustUnnamed_3,
        pub cstp: C2RustUnnamed_2,
        pub gpst: C2RustUnnamed_1,
        pub pulse: C2RustUnnamed_0,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "146:3"]
    pub struct C2RustUnnamed_0 {
        pub pad: [libc::c_uchar; 8],
        pub vendor: uint32_t,
        pub type_0: uint32_t,
        pub len: uint32_t,
        pub ident: uint32_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "142:3"]
    pub struct C2RustUnnamed_1 {
        pub pad: [libc::c_uchar; 8],
        pub hdr: [libc::c_uchar; 16],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "138:3"]
    pub struct C2RustUnnamed_2 {
        pub pad: [libc::c_uchar; 16],
        pub hdr: [libc::c_uchar; 8],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "133:3"]
    pub struct C2RustUnnamed_3 {
        pub pad: [libc::c_uchar; 2],
        pub rec: [libc::c_uchar; 2],
        pub kmp: [libc::c_uchar; 20],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "127:3"]
    pub struct C2RustUnnamed_4 {
        pub spi: uint32_t,
        pub seq: uint32_t,
        pub iv: [libc::c_uchar; 16],
        pub payload: [libc::c_uchar; 0],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "189:1"]
    pub struct keepalive_info {
        pub dpd: libc::c_int,
        pub keepalive: libc::c_int,
        pub rekey: libc::c_int,
        pub rekey_method: libc::c_int,
        pub last_rekey: time_t,
        pub last_tx: time_t,
        pub last_rx: time_t,
        pub last_dpd: time_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "200:1"]
    pub struct pin_cache {
        pub next: *mut pin_cache,
        pub token: *mut libc::c_char,
        pub pin: *mut libc::c_char,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "206:1"]
    pub struct oc_text_buf {
        pub data: *mut libc::c_char,
        pub pos: libc::c_int,
        pub buf_len: libc::c_int,
        pub error: libc::c_int,
    }
    #[src_loc = "471:2"]
    pub type C2RustUnnamed_11 = libc::c_uint;
    #[src_loc = "475:3"]
    pub const HOTP_SECRET_PSKC: C2RustUnnamed_11 = 4;
    #[src_loc = "474:3"]
    pub const HOTP_SECRET_HEX: C2RustUnnamed_11 = 3;
    #[src_loc = "473:3"]
    pub const HOTP_SECRET_RAW: C2RustUnnamed_11 = 2;
    #[src_loc = "472:3"]
    pub const HOTP_SECRET_BASE32: C2RustUnnamed_11 = 1;
    #[src_loc = "466:2"]
    pub type C2RustUnnamed_12 = libc::c_uint;
    #[src_loc = "469:3"]
    pub const OATH_ALG_HMAC_SHA512: C2RustUnnamed_12 = 2;
    #[src_loc = "468:3"]
    pub const OATH_ALG_HMAC_SHA256: C2RustUnnamed_12 = 1;
    #[src_loc = "467:3"]
    pub const OATH_ALG_HMAC_SHA1: C2RustUnnamed_12 = 0;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "237:1"]
    pub struct http_auth_state {
        pub state: libc::c_int,
        pub challenge: *mut libc::c_char,
        pub c2rust_unnamed: C2RustUnnamed_13,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "240:2"]
    pub union C2RustUnnamed_13 {
        pub c2rust_unnamed: C2RustUnnamed_15,
        pub c2rust_unnamed_0: C2RustUnnamed_14,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "258:3"]
    pub struct C2RustUnnamed_14 {
        pub ntlm_helper_fd: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "242:3"]
    pub struct C2RustUnnamed_15 {
        pub gss_target_name: gss_name_t,
        pub gss_context: gss_ctx_id_t,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "347:1"]
    pub struct esp {
        pub hmac: *mut HMAC_CTX,
        pub cipher: *mut EVP_CIPHER_CTX,
        pub seq_backlog: uint64_t,
        pub seq: uint64_t,
        pub spi: uint32_t,
        pub enc_key: [libc::c_uchar; 64],
        pub hmac_key: [libc::c_uchar; 64],
        pub iv: [libc::c_uchar; 16],
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "265:1"]
    pub struct vpn_proto {
        pub name: *const libc::c_char,
        pub pretty_name: *const libc::c_char,
        pub description: *const libc::c_char,
        pub udp_protocol: *const libc::c_char,
        pub flags: libc::c_uint,
        pub vpn_close_session: Option<unsafe extern "C" fn(_:
                                                               *mut openconnect_info,
                                                           _:
                                                               *const libc::c_char)
                                          -> libc::c_int>,
        pub obtain_cookie: Option<unsafe extern "C" fn(_:
                                                           *mut openconnect_info)
                                      -> libc::c_int>,
        pub tcp_connect: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                    -> libc::c_int>,
        pub tcp_mainloop: Option<unsafe extern "C" fn(_:
                                                          *mut openconnect_info,
                                                      _: *mut libc::c_int,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub add_http_headers: Option<unsafe extern "C" fn(_:
                                                              *mut openconnect_info,
                                                          _: *mut oc_text_buf)
                                         -> ()>,
        pub udp_setup: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
        pub udp_mainloop: Option<unsafe extern "C" fn(_:
                                                          *mut openconnect_info,
                                                      _: *mut libc::c_int,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub udp_close: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                  -> ()>,
        pub udp_shutdown: Option<unsafe extern "C" fn(_:
                                                          *mut openconnect_info)
                                     -> ()>,
        pub udp_send_probes: Option<unsafe extern "C" fn(_:
                                                             *mut openconnect_info)
                                        -> libc::c_int>,
        pub udp_catch_probe: Option<unsafe extern "C" fn(_:
                                                             *mut openconnect_info,
                                                         _: *mut pkt)
                                        -> libc::c_int>,
    }
    #[derive ( Copy, Clone )]
    #[repr(C, packed)]
    #[src_loc = "1186:1"]
    pub struct oc_packed_uint32_t {
        pub d: uint32_t,
    }
    #[inline]
    #[src_loc = "310:1"]
    pub unsafe extern "C" fn dequeue_packet(mut q: *mut pkt_q) -> *mut pkt {
        let mut ret: *mut pkt = (*q).head;
        if !ret.is_null() {
            (*q).head = (*ret).next;
            (*q).count -= 1;
            if (*q).count == 0 { (*q).tail = &mut (*q).head }
        }
        return ret;
    }
    #[inline]
    #[src_loc = "330:1"]
    pub unsafe extern "C" fn queue_packet(mut q: *mut pkt_q, mut p: *mut pkt)
     -> libc::c_int {
        *(*q).tail = p;
        (*p).next = 0 as *mut pkt;
        (*q).tail = &mut (*p).next;
        (*q).count += 1;
        return (*q).count;
    }
    #[inline]
    #[src_loc = "1205:1"]
    pub unsafe extern "C" fn store_be32(mut _p: *mut libc::c_void,
                                        mut d: uint32_t) {
        let mut p: *mut oc_packed_uint32_t = _p as *mut oc_packed_uint32_t;
        (*p).d =
            if 0 != 0 {
                ((d & 0xff000000u32) >> 24i32 |
                     (d & 0xff0000i32 as libc::c_uint) >> 8i32 |
                     (d & 0xff00i32 as libc::c_uint) << 8i32) |
                    (d & 0xffi32 as libc::c_uint) << 24i32
            } else { _OSSwapInt32(d) };
    }
    use super::_uint32_t_h::uint32_t;
    use super::_time_t_h::time_t;
    use super::gssapi_h::{gss_name_t, gss_ctx_id_t};
    use super::hmac_h::HMAC_CTX;
    use super::ossl_typ_h::EVP_CIPHER_CTX;
    use super::_uint64_t_h::uint64_t;
    use super::openconnect_h::openconnect_info;
    use super::_OSByteOrder_h::_OSSwapInt32;
    extern "C" {
        #[src_loc = "363:1"]
        pub type oc_pcsc_ctx;
        #[no_mangle]
        #[src_loc = "896:1"]
        pub fn oncp_esp_send_probes(vpninfo: *mut openconnect_info)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "983:1"]
        pub fn encrypt_esp_packet(vpninfo: *mut openconnect_info,
                                  pkt: *mut pkt, crypt_len: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1014:1"]
        pub fn keepalive_action(ka: *mut keepalive_info,
                                timeout: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "982:1"]
        pub fn decrypt_esp_packet(vpninfo: *mut openconnect_info,
                                  esp: *mut esp, pkt: *mut pkt)
         -> libc::c_int;
        #[no_mangle]
        #[src_loc = "1016:1"]
        pub fn ka_check_deadline(timeout: *mut libc::c_int, now: time_t,
                                 due: time_t) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "980:1"]
        pub fn destroy_esp_ciphers(esp: *mut esp);
        #[no_mangle]
        #[src_loc = "981:1"]
        pub fn init_esp_ciphers(vpninfo: *mut openconnect_info, out: *mut esp,
                                in_0: *mut esp) -> libc::c_int;
        #[no_mangle]
        #[src_loc = "996:1"]
        pub fn openconnect_random(bytes: *mut libc::c_void, len: libc::c_int)
         -> libc::c_int;
    }
    /* __OPENCONNECT_INTERNAL_H__ */
    /* !Not known to be little-endian */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ossl_typ.h:27"]
pub mod ossl_typ_h {
    /* ====================================================================
 * Copyright (c) 1998-2001 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
    #[src_loc = "178:1"]
    pub type SSL = ssl_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "178:9"]
    pub struct ssl_st {
        pub version: libc::c_int,
        pub type_0: libc::c_int,
        pub method: *const SSL_METHOD,
        pub rbio: *mut BIO,
        pub wbio: *mut BIO,
        pub bbio: *mut BIO,
        pub rwstate: libc::c_int,
        pub in_handshake: libc::c_int,
        pub handshake_func: Option<unsafe extern "C" fn(_: *mut SSL)
                                       -> libc::c_int>,
        pub server: libc::c_int,
        pub new_session: libc::c_int,
        pub quiet_shutdown: libc::c_int,
        pub shutdown: libc::c_int,
        pub state: libc::c_int,
        pub rstate: libc::c_int,
        pub init_buf: *mut BUF_MEM,
        pub init_msg: *mut libc::c_void,
        pub init_num: libc::c_int,
        pub init_off: libc::c_int,
        pub packet: *mut libc::c_uchar,
        pub packet_length: libc::c_uint,
        pub s2: *mut ssl2_state_st,
        pub s3: *mut ssl3_state_st,
        pub d1: *mut dtls1_state_st,
        pub read_ahead: libc::c_int,
        pub msg_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const libc::c_void,
                                                      _: size_t, _: *mut SSL,
                                                      _: *mut libc::c_void)
                                     -> ()>,
        pub msg_callback_arg: *mut libc::c_void,
        pub hit: libc::c_int,
        pub param: *mut X509_VERIFY_PARAM,
        pub cipher_list: *mut stack_st_SSL_CIPHER,
        pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
        pub mac_flags: libc::c_int,
        pub enc_read_ctx: *mut EVP_CIPHER_CTX,
        pub read_hash: *mut EVP_MD_CTX,
        pub expand: *mut COMP_CTX,
        pub enc_write_ctx: *mut EVP_CIPHER_CTX,
        pub write_hash: *mut EVP_MD_CTX,
        pub compress: *mut COMP_CTX,
        pub cert: *mut cert_st,
        pub sid_ctx_length: libc::c_uint,
        pub sid_ctx: [libc::c_uchar; 32],
        pub session: *mut SSL_SESSION,
        pub generate_session_id: GEN_SESSION_CB,
        pub verify_mode: libc::c_int,
        pub verify_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _:
                                                             *mut X509_STORE_CTX)
                                        -> libc::c_int>,
        pub info_callback: Option<unsafe extern "C" fn(_: *const SSL,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
        pub error: libc::c_int,
        pub error_code: libc::c_int,
        pub psk_client_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                             _:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_char,
                                                             _: libc::c_uint,
                                                             _:
                                                                 *mut libc::c_uchar,
                                                             _: libc::c_uint)
                                            -> libc::c_uint>,
        pub psk_server_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                             _:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_uchar,
                                                             _: libc::c_uint)
                                            -> libc::c_uint>,
        pub ctx: *mut SSL_CTX,
        pub debug: libc::c_int,
        pub verify_result: libc::c_long,
        pub ex_data: CRYPTO_EX_DATA,
        pub client_CA: *mut stack_st_X509_NAME,
        pub references: libc::c_int,
        pub options: libc::c_ulong,
        pub mode: libc::c_ulong,
        pub max_cert_list: libc::c_long,
        pub first_packet: libc::c_int,
        pub client_version: libc::c_int,
        pub max_send_fragment: libc::c_uint,
        pub tlsext_debug_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_int,
                                                         _: *mut libc::c_void)
                                        -> ()>,
        pub tlsext_debug_arg: *mut libc::c_void,
        pub tlsext_hostname: *mut libc::c_char,
        pub servername_done: libc::c_int,
        pub tlsext_status_type: libc::c_int,
        pub tlsext_status_expected: libc::c_int,
        pub tlsext_ocsp_ids: *mut stack_st_OCSP_RESPID,
        pub tlsext_ocsp_exts: *mut X509_EXTENSIONS,
        pub tlsext_ocsp_resp: *mut libc::c_uchar,
        pub tlsext_ocsp_resplen: libc::c_int,
        pub tlsext_ticket_expected: libc::c_int,
        pub tlsext_ecpointformatlist_length: size_t,
        pub tlsext_ecpointformatlist: *mut libc::c_uchar,
        pub tlsext_ellipticcurvelist_length: size_t,
        pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
        pub tlsext_opaque_prf_input: *mut libc::c_void,
        pub tlsext_opaque_prf_input_len: size_t,
        pub tlsext_session_ticket: *mut TLS_SESSION_TICKET_EXT,
        pub tls_session_ticket_ext_cb: tls_session_ticket_ext_cb_fn,
        pub tls_session_ticket_ext_cb_arg: *mut libc::c_void,
        pub tls_session_secret_cb: tls_session_secret_cb_fn,
        pub tls_session_secret_cb_arg: *mut libc::c_void,
        pub initial_ctx: *mut SSL_CTX,
        pub next_proto_negotiated: *mut libc::c_uchar,
        pub next_proto_negotiated_len: libc::c_uchar,
        pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
        pub srtp_profile: *mut SRTP_PROTECTION_PROFILE,
        pub tlsext_heartbeat: libc::c_uint,
        pub tlsext_hb_pending: libc::c_uint,
        pub tlsext_hb_seq: libc::c_uint,
        pub renegotiate: libc::c_int,
        pub srp_ctx: SRP_CTX,
        pub alpn_client_proto_list: *mut libc::c_uchar,
        pub alpn_client_proto_list_len: libc::c_uint,
        /* If placed in pkcs12.h, we end up with a circular depency with pkcs7.h */
        /* Nothing */
        /* Nothing */
        /* Callback types for crypto.h */
        /* def HEADER_OPENSSL_TYPES_H */
    }
    #[src_loc = "120:1"]
    pub type BIGNUM = bignum_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "120:9"]
    pub struct bignum_st {
        pub d: *mut libc::c_ulong,
        pub top: libc::c_int,
        pub dmax: libc::c_int,
        pub neg: libc::c_int,
        pub flags: libc::c_int,
    }
    #[src_loc = "179:1"]
    pub type SSL_CTX = ssl_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "179:9"]
    pub struct ssl_ctx_st {
        pub method: *const SSL_METHOD,
        pub cipher_list: *mut stack_st_SSL_CIPHER,
        pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
        pub cert_store: *mut x509_store_st,
        pub sessions: *mut lhash_st_SSL_SESSION,
        pub session_cache_size: libc::c_ulong,
        pub session_cache_head: *mut ssl_session_st,
        pub session_cache_tail: *mut ssl_session_st,
        pub session_cache_mode: libc::c_int,
        pub session_timeout: libc::c_long,
        pub new_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_st,
                                                        _: *mut SSL_SESSION)
                                       -> libc::c_int>,
        pub remove_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_ctx_st,
                                                           _:
                                                               *mut SSL_SESSION)
                                          -> ()>,
        pub get_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_st,
                                                        _: *mut libc::c_uchar,
                                                        _: libc::c_int,
                                                        _: *mut libc::c_int)
                                       -> *mut SSL_SESSION>,
        pub stats: C2RustUnnamed_8,
        pub references: libc::c_int,
        pub app_verify_callback: Option<unsafe extern "C" fn(_:
                                                                 *mut X509_STORE_CTX,
                                                             _:
                                                                 *mut libc::c_void)
                                            -> libc::c_int>,
        pub app_verify_arg: *mut libc::c_void,
        pub default_passwd_callback: Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut libc::c_void)
                                                -> libc::c_int>,
        pub default_passwd_callback_userdata: *mut libc::c_void,
        pub client_cert_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                        _: *mut *mut X509,
                                                        _: *mut *mut EVP_PKEY)
                                       -> libc::c_int>,
        pub app_gen_cookie_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                           _:
                                                               *mut libc::c_uchar,
                                                           _:
                                                               *mut libc::c_uint)
                                          -> libc::c_int>,
        pub app_verify_cookie_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                              _:
                                                                  *mut libc::c_uchar,
                                                              _: libc::c_uint)
                                             -> libc::c_int>,
        pub ex_data: CRYPTO_EX_DATA,
        pub rsa_md5: *const EVP_MD,
        pub md5: *const EVP_MD,
        pub sha1: *const EVP_MD,
        pub extra_certs: *mut stack_st_X509,
        pub comp_methods: *mut stack_st_SSL_COMP,
        pub info_callback: Option<unsafe extern "C" fn(_: *const SSL,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
        pub client_CA: *mut stack_st_X509_NAME,
        pub options: libc::c_ulong,
        pub mode: libc::c_ulong,
        pub max_cert_list: libc::c_long,
        pub cert: *mut cert_st,
        pub read_ahead: libc::c_int,
        pub msg_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const libc::c_void,
                                                      _: size_t, _: *mut SSL,
                                                      _: *mut libc::c_void)
                                     -> ()>,
        pub msg_callback_arg: *mut libc::c_void,
        pub verify_mode: libc::c_int,
        pub sid_ctx_length: libc::c_uint,
        pub sid_ctx: [libc::c_uchar; 32],
        pub default_verify_callback: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut X509_STORE_CTX)
                                                -> libc::c_int>,
        pub generate_session_id: GEN_SESSION_CB,
        pub param: *mut X509_VERIFY_PARAM,
        pub quiet_shutdown: libc::c_int,
        pub max_send_fragment: libc::c_uint,
        pub client_cert_engine: *mut ENGINE,
        pub tlsext_servername_callback: Option<unsafe extern "C" fn(_:
                                                                        *mut SSL,
                                                                    _:
                                                                        *mut libc::c_int,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> libc::c_int>,
        pub tlsext_servername_arg: *mut libc::c_void,
        pub tlsext_tick_key_name: [libc::c_uchar; 16],
        pub tlsext_tick_hmac_key: [libc::c_uchar; 16],
        pub tlsext_tick_aes_key: [libc::c_uchar; 16],
        pub tlsext_ticket_key_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                              _:
                                                                  *mut libc::c_uchar,
                                                              _:
                                                                  *mut libc::c_uchar,
                                                              _:
                                                                  *mut EVP_CIPHER_CTX,
                                                              _:
                                                                  *mut HMAC_CTX,
                                                              _: libc::c_int)
                                             -> libc::c_int>,
        pub tlsext_status_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut libc::c_void)
                                         -> libc::c_int>,
        pub tlsext_status_arg: *mut libc::c_void,
        pub tlsext_opaque_prf_input_callback: Option<unsafe extern "C" fn(_:
                                                                              *mut SSL,
                                                                          _:
                                                                              *mut libc::c_void,
                                                                          _:
                                                                              size_t,
                                                                          _:
                                                                              *mut libc::c_void)
                                                         -> libc::c_int>,
        pub tlsext_opaque_prf_input_callback_arg: *mut libc::c_void,
        pub psk_identity_hint: *mut libc::c_char,
        pub psk_client_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                             _:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_char,
                                                             _: libc::c_uint,
                                                             _:
                                                                 *mut libc::c_uchar,
                                                             _: libc::c_uint)
                                            -> libc::c_uint>,
        pub psk_server_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                             _:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_uchar,
                                                             _: libc::c_uint)
                                            -> libc::c_uint>,
        pub freelist_max_len: libc::c_uint,
        pub wbuf_freelist: *mut ssl3_buf_freelist_st,
        pub rbuf_freelist: *mut ssl3_buf_freelist_st,
        pub srp_ctx: SRP_CTX,
        pub next_protos_advertised_cb: Option<unsafe extern "C" fn(_:
                                                                       *mut SSL,
                                                                   _:
                                                                       *mut *const libc::c_uchar,
                                                                   _:
                                                                       *mut libc::c_uint,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
        pub next_protos_advertised_cb_arg: *mut libc::c_void,
        pub next_proto_select_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                              _:
                                                                  *mut *mut libc::c_uchar,
                                                              _:
                                                                  *mut libc::c_uchar,
                                                              _:
                                                                  *const libc::c_uchar,
                                                              _: libc::c_uint,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> libc::c_int>,
        pub next_proto_select_cb_arg: *mut libc::c_void,
        pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
        pub alpn_select_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                        _:
                                                            *mut *const libc::c_uchar,
                                                        _: *mut libc::c_uchar,
                                                        _:
                                                            *const libc::c_uchar,
                                                        _: libc::c_uint,
                                                        _: *mut libc::c_void)
                                       -> libc::c_int>,
        pub alpn_select_cb_arg: *mut libc::c_void,
        pub alpn_client_proto_list: *mut libc::c_uchar,
        pub alpn_client_proto_list_len: libc::c_uint,
        pub tlsext_ecpointformatlist_length: size_t,
        pub tlsext_ecpointformatlist: *mut libc::c_uchar,
        pub tlsext_ellipticcurvelist_length: size_t,
        pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    }
    #[src_loc = "132:1"]
    pub type EVP_MD_CTX = env_md_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "132:9"]
    pub struct env_md_ctx_st {
        pub digest: *const EVP_MD,
        pub engine: *mut ENGINE,
        pub flags: libc::c_ulong,
        pub md_data: *mut libc::c_void,
        pub pctx: *mut EVP_PKEY_CTX,
        pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                                _: *const libc::c_void,
                                                _: size_t) -> libc::c_int>,
    }
    #[src_loc = "138:1"]
    pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
    #[src_loc = "177:1"]
    pub type ENGINE = engine_st;
    #[src_loc = "131:1"]
    pub type EVP_MD = env_md_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "131:9"]
    pub struct env_md_st {
        pub type_0: libc::c_int,
        pub pkey_type: libc::c_int,
        pub md_size: libc::c_int,
        pub flags: libc::c_ulong,
        pub init: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX)
                             -> libc::c_int>,
        pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                                _: *const libc::c_void,
                                                _: size_t) -> libc::c_int>,
        pub final_0: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                                 _: *mut libc::c_uchar)
                                -> libc::c_int>,
        pub copy: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                              _: *const EVP_MD_CTX)
                             -> libc::c_int>,
        pub cleanup: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX)
                                -> libc::c_int>,
        pub sign: Option<unsafe extern "C" fn(_: libc::c_int,
                                              _: *const libc::c_uchar,
                                              _: libc::c_uint,
                                              _: *mut libc::c_uchar,
                                              _: *mut libc::c_uint,
                                              _: *mut libc::c_void)
                             -> libc::c_int>,
        pub verify: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: *const libc::c_uchar,
                                                _: libc::c_uint,
                                                _: *const libc::c_uchar,
                                                _: libc::c_uint,
                                                _: *mut libc::c_void)
                               -> libc::c_int>,
        pub required_pkey_type: [libc::c_int; 5],
        pub block_size: libc::c_int,
        pub ctx_size: libc::c_int,
        pub md_ctrl: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: *mut libc::c_void)
                                -> libc::c_int>,
    }
    #[src_loc = "130:1"]
    pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "130:9"]
    pub struct evp_cipher_ctx_st {
        pub cipher: *const EVP_CIPHER,
        pub engine: *mut ENGINE,
        pub encrypt: libc::c_int,
        pub buf_len: libc::c_int,
        pub oiv: [libc::c_uchar; 16],
        pub iv: [libc::c_uchar; 16],
        pub buf: [libc::c_uchar; 32],
        pub num: libc::c_int,
        pub app_data: *mut libc::c_void,
        pub key_len: libc::c_int,
        pub flags: libc::c_ulong,
        pub cipher_data: *mut libc::c_void,
        pub final_used: libc::c_int,
        pub block_mask: libc::c_int,
        pub final_0: [libc::c_uchar; 32],
    }
    #[src_loc = "129:1"]
    pub type EVP_CIPHER = evp_cipher_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "129:9"]
    pub struct evp_cipher_st {
        pub nid: libc::c_int,
        pub block_size: libc::c_int,
        pub key_len: libc::c_int,
        pub iv_len: libc::c_int,
        pub flags: libc::c_ulong,
        pub init: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                              _: *const libc::c_uchar,
                                              _: *const libc::c_uchar,
                                              _: libc::c_int) -> libc::c_int>,
        pub do_cipher: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                                   _: *mut libc::c_uchar,
                                                   _: *const libc::c_uchar,
                                                   _: size_t) -> libc::c_int>,
        pub cleanup: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX)
                                -> libc::c_int>,
        pub ctx_size: libc::c_int,
        pub set_asn1_parameters: Option<unsafe extern "C" fn(_:
                                                                 *mut EVP_CIPHER_CTX,
                                                             _:
                                                                 *mut ASN1_TYPE)
                                            -> libc::c_int>,
        pub get_asn1_parameters: Option<unsafe extern "C" fn(_:
                                                                 *mut EVP_CIPHER_CTX,
                                                             _:
                                                                 *mut ASN1_TYPE)
                                            -> libc::c_int>,
        pub ctrl: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                              _: libc::c_int, _: libc::c_int,
                                              _: *mut libc::c_void)
                             -> libc::c_int>,
        pub app_data: *mut libc::c_void,
    }
    #[src_loc = "98:1"]
    pub type ASN1_STRING = asn1_string_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "83:9"]
    pub struct asn1_string_st {
        pub length: libc::c_int,
        pub type_0: libc::c_int,
        pub data: *mut libc::c_uchar,
        pub flags: libc::c_long,
    }
    #[src_loc = "97:1"]
    pub type ASN1_UTF8STRING = asn1_string_st;
    #[src_loc = "96:1"]
    pub type ASN1_VISIBLESTRING = asn1_string_st;
    #[src_loc = "95:1"]
    pub type ASN1_GENERALIZEDTIME = asn1_string_st;
    #[src_loc = "93:1"]
    pub type ASN1_UTCTIME = asn1_string_st;
    #[src_loc = "91:1"]
    pub type ASN1_UNIVERSALSTRING = asn1_string_st;
    #[src_loc = "92:1"]
    pub type ASN1_BMPSTRING = asn1_string_st;
    #[src_loc = "90:1"]
    pub type ASN1_GENERALSTRING = asn1_string_st;
    #[src_loc = "89:1"]
    pub type ASN1_IA5STRING = asn1_string_st;
    #[src_loc = "88:1"]
    pub type ASN1_T61STRING = asn1_string_st;
    #[src_loc = "87:1"]
    pub type ASN1_PRINTABLESTRING = asn1_string_st;
    #[src_loc = "86:1"]
    pub type ASN1_OCTET_STRING = asn1_string_st;
    #[src_loc = "85:1"]
    pub type ASN1_BIT_STRING = asn1_string_st;
    #[src_loc = "84:1"]
    pub type ASN1_ENUMERATED = asn1_string_st;
    #[src_loc = "83:1"]
    pub type ASN1_INTEGER = asn1_string_st;
    #[src_loc = "103:1"]
    pub type ASN1_OBJECT = asn1_object_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "103:9"]
    pub struct asn1_object_st {
        pub sn: *const libc::c_char,
        pub ln: *const libc::c_char,
        pub nid: libc::c_int,
        pub length: libc::c_int,
        pub data: *const libc::c_uchar,
        pub flags: libc::c_int,
    }
    #[src_loc = "99:1"]
    pub type ASN1_BOOLEAN = libc::c_int;
    #[src_loc = "162:1"]
    pub type X509_STORE_CTX = x509_store_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "162:9"]
    pub struct x509_store_ctx_st {
        pub ctx: *mut X509_STORE,
        pub current_method: libc::c_int,
        pub cert: *mut X509,
        pub untrusted: *mut stack_st_X509,
        pub crls: *mut stack_st_X509_CRL,
        pub param: *mut X509_VERIFY_PARAM,
        pub other_ctx: *mut libc::c_void,
        pub verify: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                               -> libc::c_int>,
        pub verify_cb: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *mut X509_STORE_CTX)
                                  -> libc::c_int>,
        pub get_issuer: Option<unsafe extern "C" fn(_: *mut *mut X509,
                                                    _: *mut X509_STORE_CTX,
                                                    _: *mut X509)
                                   -> libc::c_int>,
        pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                      _: *mut X509,
                                                      _: *mut X509)
                                     -> libc::c_int>,
        pub check_revocation: Option<unsafe extern "C" fn(_:
                                                              *mut X509_STORE_CTX)
                                         -> libc::c_int>,
        pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut *mut X509_CRL,
                                                 _: *mut X509)
                                -> libc::c_int>,
        pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                   _: *mut X509_CRL)
                                  -> libc::c_int>,
        pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_CRL,
                                                  _: *mut X509)
                                 -> libc::c_int>,
        pub check_policy: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                     -> libc::c_int>,
        pub lookup_certs: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                      _: *mut X509_NAME)
                                     -> *mut stack_st_X509>,
        pub lookup_crls: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                     _: *mut X509_NAME)
                                    -> *mut stack_st_X509_CRL>,
        pub cleanup: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                -> libc::c_int>,
        pub valid: libc::c_int,
        pub last_untrusted: libc::c_int,
        pub chain: *mut stack_st_X509,
        pub tree: *mut X509_POLICY_TREE,
        pub explicit_policy: libc::c_int,
        pub error_depth: libc::c_int,
        pub error: libc::c_int,
        pub current_cert: *mut X509,
        pub current_issuer: *mut X509,
        pub current_crl: *mut X509_CRL,
        pub current_crl_score: libc::c_int,
        pub current_reasons: libc::c_uint,
        pub parent: *mut X509_STORE_CTX,
        pub ex_data: CRYPTO_EX_DATA,
    }
    #[src_loc = "197:1"]
    pub type CRYPTO_EX_DATA = crypto_ex_data_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "197:9"]
    pub struct crypto_ex_data_st {
        pub sk: *mut stack_st_void,
        pub dummy: libc::c_int,
    }
    #[src_loc = "156:1"]
    pub type X509_CRL = X509_crl_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "156:9"]
    pub struct X509_crl_st {
        pub crl: *mut X509_CRL_INFO,
        pub sig_alg: *mut X509_ALGOR,
        pub signature: *mut ASN1_BIT_STRING,
        pub references: libc::c_int,
        pub flags: libc::c_int,
        pub akid: *mut AUTHORITY_KEYID,
        pub idp: *mut ISSUING_DIST_POINT,
        pub idp_flags: libc::c_int,
        pub idp_reasons: libc::c_int,
        pub crl_number: *mut ASN1_INTEGER,
        pub base_crl_number: *mut ASN1_INTEGER,
        pub sha1_hash: [libc::c_uchar; 20],
        pub issuers: *mut stack_st_GENERAL_NAMES,
        pub meth: *const X509_CRL_METHOD,
        pub meth_data: *mut libc::c_void,
    }
    #[src_loc = "157:1"]
    pub type X509_CRL_METHOD = x509_crl_method_st;
    #[src_loc = "190:1"]
    pub type ISSUING_DIST_POINT = ISSUING_DIST_POINT_st;
    #[src_loc = "188:1"]
    pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
    #[src_loc = "155:1"]
    pub type X509_ALGOR = X509_algor_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "155:9"]
    pub struct X509_algor_st {
        pub algorithm: *mut ASN1_OBJECT,
        pub parameter: *mut ASN1_TYPE,
    }
    #[src_loc = "94:1"]
    pub type ASN1_TIME = asn1_string_st;
    #[src_loc = "159:1"]
    pub type X509_NAME = X509_name_st;
    /* we always keep X509_NAMEs in 2 forms. */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "159:9"]
    pub struct X509_name_st {
        pub entries: *mut stack_st_X509_NAME_ENTRY,
        pub modified: libc::c_int,
        pub bytes: *mut BUF_MEM,
        pub canon_enc: *mut libc::c_uchar,
        pub canon_enclen: libc::c_int,
    }
    #[src_loc = "127:1"]
    pub type BUF_MEM = buf_mem_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "127:9"]
    pub struct buf_mem_st {
        pub length: size_t,
        pub data: *mut libc::c_char,
        pub max: size_t,
    }
    #[src_loc = "154:1"]
    pub type X509 = x509_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "154:9"]
    pub struct x509_st {
        pub cert_info: *mut X509_CINF,
        pub sig_alg: *mut X509_ALGOR,
        pub signature: *mut ASN1_BIT_STRING,
        pub valid: libc::c_int,
        pub references: libc::c_int,
        pub name: *mut libc::c_char,
        pub ex_data: CRYPTO_EX_DATA,
        pub ex_pathlen: libc::c_long,
        pub ex_pcpathlen: libc::c_long,
        pub ex_flags: libc::c_ulong,
        pub ex_kusage: libc::c_ulong,
        pub ex_xkusage: libc::c_ulong,
        pub ex_nscert: libc::c_ulong,
        pub skid: *mut ASN1_OCTET_STRING,
        pub akid: *mut AUTHORITY_KEYID,
        pub policy_cache: *mut X509_POLICY_CACHE,
        pub crldp: *mut stack_st_DIST_POINT,
        pub altname: *mut stack_st_GENERAL_NAME,
        pub nc: *mut NAME_CONSTRAINTS,
        pub sha1_hash: [libc::c_uchar; 20],
        pub aux: *mut X509_CERT_AUX,
    }
    #[src_loc = "191:1"]
    pub type NAME_CONSTRAINTS = NAME_CONSTRAINTS_st;
    #[src_loc = "186:1"]
    pub type X509_POLICY_CACHE = X509_POLICY_CACHE_st;
    #[src_loc = "160:1"]
    pub type X509_PUBKEY = X509_pubkey_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "160:9"]
    pub struct X509_pubkey_st {
        pub algor: *mut X509_ALGOR,
        pub public_key: *mut ASN1_BIT_STRING,
        pub pkey: *mut EVP_PKEY,
    }
    #[src_loc = "133:1"]
    pub type EVP_PKEY = evp_pkey_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "133:9"]
    pub struct evp_pkey_st {
        pub type_0: libc::c_int,
        pub save_type: libc::c_int,
        pub references: libc::c_int,
        pub ameth: *const EVP_PKEY_ASN1_METHOD,
        pub engine: *mut ENGINE,
        pub pkey: C2RustUnnamed_6,
        pub save_parameters: libc::c_int,
        pub attributes: *mut stack_st_X509_ATTRIBUTE,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "140:9"]
    pub struct dh_st {
        pub pad: libc::c_int,
        pub version: libc::c_int,
        pub p: *mut BIGNUM,
        pub g: *mut BIGNUM,
        pub length: libc::c_long,
        pub pub_key: *mut BIGNUM,
        pub priv_key: *mut BIGNUM,
        pub flags: libc::c_int,
        pub method_mont_p: *mut BN_MONT_CTX,
        pub q: *mut BIGNUM,
        pub j: *mut BIGNUM,
        pub seed: *mut libc::c_uchar,
        pub seedlen: libc::c_int,
        pub counter: *mut BIGNUM,
        pub references: libc::c_int,
        pub ex_data: CRYPTO_EX_DATA,
        pub meth: *const DH_METHOD,
        pub engine: *mut ENGINE,
    }
    #[src_loc = "141:1"]
    pub type DH_METHOD = dh_method;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "141:9"]
    pub struct dh_method {
        pub name: *const libc::c_char,
        pub generate_key: Option<unsafe extern "C" fn(_: *mut DH)
                                     -> libc::c_int>,
        pub compute_key: Option<unsafe extern "C" fn(_: *mut libc::c_uchar,
                                                     _: *const BIGNUM,
                                                     _: *mut DH)
                                    -> libc::c_int>,
        pub bn_mod_exp: Option<unsafe extern "C" fn(_: *const DH,
                                                    _: *mut BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *mut BN_CTX,
                                                    _: *mut BN_MONT_CTX)
                                   -> libc::c_int>,
        pub init: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
        pub finish: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
        pub flags: libc::c_int,
        pub app_data: *mut libc::c_char,
        pub generate_params: Option<unsafe extern "C" fn(_: *mut DH,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *mut BN_GENCB)
                                        -> libc::c_int>,
    }
    #[src_loc = "125:1"]
    pub type BN_GENCB = bn_gencb_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "125:9"]
    pub struct bn_gencb_st {
        pub ver: libc::c_uint,
        pub arg: *mut libc::c_void,
        pub cb: C2RustUnnamed_7,
    }
    #[src_loc = "140:1"]
    pub type DH = dh_st;
    #[src_loc = "123:1"]
    pub type BN_MONT_CTX = bn_mont_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "123:9"]
    pub struct bn_mont_ctx_st {
        pub ri: libc::c_int,
        pub RR: BIGNUM,
        pub N: BIGNUM,
        pub Ni: BIGNUM,
        pub n0: [libc::c_ulong; 2],
        pub flags: libc::c_int,
    }
    #[src_loc = "121:1"]
    pub type BN_CTX = bignum_ctx;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "143:9"]
    pub struct dsa_st {
        pub pad: libc::c_int,
        pub version: libc::c_long,
        pub write_params: libc::c_int,
        pub p: *mut BIGNUM,
        pub q: *mut BIGNUM,
        pub g: *mut BIGNUM,
        pub pub_key: *mut BIGNUM,
        pub priv_key: *mut BIGNUM,
        pub kinv: *mut BIGNUM,
        pub r: *mut BIGNUM,
        pub flags: libc::c_int,
        pub method_mont_p: *mut BN_MONT_CTX,
        pub references: libc::c_int,
        pub ex_data: CRYPTO_EX_DATA,
        pub meth: *const DSA_METHOD,
        pub engine: *mut ENGINE,
    }
    #[src_loc = "144:1"]
    pub type DSA_METHOD = dsa_method;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "144:9"]
    pub struct dsa_method {
        pub name: *const libc::c_char,
        pub dsa_do_sign: Option<unsafe extern "C" fn(_: *const libc::c_uchar,
                                                     _: libc::c_int,
                                                     _: *mut DSA)
                                    -> *mut DSA_SIG>,
        pub dsa_sign_setup: Option<unsafe extern "C" fn(_: *mut DSA,
                                                        _: *mut BN_CTX,
                                                        _: *mut *mut BIGNUM,
                                                        _: *mut *mut BIGNUM)
                                       -> libc::c_int>,
        pub dsa_do_verify: Option<unsafe extern "C" fn(_:
                                                           *const libc::c_uchar,
                                                       _: libc::c_int,
                                                       _: *mut DSA_SIG,
                                                       _: *mut DSA)
                                      -> libc::c_int>,
        pub dsa_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BIGNUM,
                                                     _: *mut BN_CTX,
                                                     _: *mut BN_MONT_CTX)
                                    -> libc::c_int>,
        pub bn_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA,
                                                    _: *mut BIGNUM,
                                                    _: *mut BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *mut BN_CTX,
                                                    _: *mut BN_MONT_CTX)
                                   -> libc::c_int>,
        pub init: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
        pub finish: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
        pub flags: libc::c_int,
        pub app_data: *mut libc::c_char,
        pub dsa_paramgen: Option<unsafe extern "C" fn(_: *mut DSA,
                                                      _: libc::c_int,
                                                      _: *const libc::c_uchar,
                                                      _: libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_ulong,
                                                      _: *mut BN_GENCB)
                                     -> libc::c_int>,
        pub dsa_keygen: Option<unsafe extern "C" fn(_: *mut DSA)
                                   -> libc::c_int>,
    }
    #[src_loc = "143:1"]
    pub type DSA = dsa_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "146:9"]
    pub struct rsa_st {
        pub pad: libc::c_int,
        pub version: libc::c_long,
        pub meth: *const RSA_METHOD,
        pub engine: *mut ENGINE,
        pub n: *mut BIGNUM,
        pub e: *mut BIGNUM,
        pub d: *mut BIGNUM,
        pub p: *mut BIGNUM,
        pub q: *mut BIGNUM,
        pub dmp1: *mut BIGNUM,
        pub dmq1: *mut BIGNUM,
        pub iqmp: *mut BIGNUM,
        pub ex_data: CRYPTO_EX_DATA,
        pub references: libc::c_int,
        pub flags: libc::c_int,
        pub _method_mod_n: *mut BN_MONT_CTX,
        pub _method_mod_p: *mut BN_MONT_CTX,
        pub _method_mod_q: *mut BN_MONT_CTX,
        pub bignum_data: *mut libc::c_char,
        pub blinding: *mut BN_BLINDING,
        pub mt_blinding: *mut BN_BLINDING,
    }
    #[src_loc = "122:1"]
    pub type BN_BLINDING = bn_blinding_st;
    #[src_loc = "147:1"]
    pub type RSA_METHOD = rsa_meth_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "147:9"]
    pub struct rsa_meth_st {
        pub name: *const libc::c_char,
        pub rsa_pub_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *const libc::c_uchar,
                                                     _: *mut libc::c_uchar,
                                                     _: *mut RSA,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
        pub rsa_pub_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *const libc::c_uchar,
                                                     _: *mut libc::c_uchar,
                                                     _: *mut RSA,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
        pub rsa_priv_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: *mut RSA,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub rsa_priv_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_uchar,
                                                      _: *mut libc::c_uchar,
                                                      _: *mut RSA,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub rsa_mod_exp: Option<unsafe extern "C" fn(_: *mut BIGNUM,
                                                     _: *const BIGNUM,
                                                     _: *mut RSA,
                                                     _: *mut BN_CTX)
                                    -> libc::c_int>,
        pub bn_mod_exp: Option<unsafe extern "C" fn(_: *mut BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *const BIGNUM,
                                                    _: *mut BN_CTX,
                                                    _: *mut BN_MONT_CTX)
                                   -> libc::c_int>,
        pub init: Option<unsafe extern "C" fn(_: *mut RSA) -> libc::c_int>,
        pub finish: Option<unsafe extern "C" fn(_: *mut RSA) -> libc::c_int>,
        pub flags: libc::c_int,
        pub app_data: *mut libc::c_char,
        pub rsa_sign: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_uchar,
                                                  _: libc::c_uint,
                                                  _: *mut libc::c_uchar,
                                                  _: *mut libc::c_uint,
                                                  _: *const RSA)
                                 -> libc::c_int>,
        pub rsa_verify: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *const libc::c_uchar,
                                                    _: libc::c_uint,
                                                    _: *const libc::c_uchar,
                                                    _: libc::c_uint,
                                                    _: *const RSA)
                                   -> libc::c_int>,
        pub rsa_keygen: Option<unsafe extern "C" fn(_: *mut RSA,
                                                    _: libc::c_int,
                                                    _: *mut BIGNUM,
                                                    _: *mut BN_GENCB)
                                   -> libc::c_int>,
    }
    #[src_loc = "146:1"]
    pub type RSA = rsa_st;
    #[src_loc = "135:1"]
    pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
    #[src_loc = "185:1"]
    pub type X509_POLICY_TREE = X509_POLICY_TREE_st;
    #[src_loc = "161:1"]
    pub type X509_STORE = x509_store_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "161:9"]
    pub struct x509_store_st {
        pub cache: libc::c_int,
        pub objs: *mut stack_st_X509_OBJECT,
        pub get_cert_methods: *mut stack_st_X509_LOOKUP,
        pub param: *mut X509_VERIFY_PARAM,
        pub verify: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                               -> libc::c_int>,
        pub verify_cb: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *mut X509_STORE_CTX)
                                  -> libc::c_int>,
        pub get_issuer: Option<unsafe extern "C" fn(_: *mut *mut X509,
                                                    _: *mut X509_STORE_CTX,
                                                    _: *mut X509)
                                   -> libc::c_int>,
        pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                      _: *mut X509,
                                                      _: *mut X509)
                                     -> libc::c_int>,
        pub check_revocation: Option<unsafe extern "C" fn(_:
                                                              *mut X509_STORE_CTX)
                                         -> libc::c_int>,
        pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut *mut X509_CRL,
                                                 _: *mut X509)
                                -> libc::c_int>,
        pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                   _: *mut X509_CRL)
                                  -> libc::c_int>,
        pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_CRL,
                                                  _: *mut X509)
                                 -> libc::c_int>,
        pub lookup_certs: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                      _: *mut X509_NAME)
                                     -> *mut stack_st_X509>,
        pub lookup_crls: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                     _: *mut X509_NAME)
                                    -> *mut stack_st_X509_CRL>,
        pub cleanup: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                -> libc::c_int>,
        pub ex_data: CRYPTO_EX_DATA,
        pub references: libc::c_int,
    }
    #[src_loc = "181:1"]
    pub type COMP_METHOD = comp_method_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "181:9"]
    pub struct comp_method_st {
        pub type_0: libc::c_int,
        pub name: *const libc::c_char,
        pub init: Option<unsafe extern "C" fn(_: *mut COMP_CTX)
                             -> libc::c_int>,
        pub finish: Option<unsafe extern "C" fn(_: *mut COMP_CTX) -> ()>,
        pub compress: Option<unsafe extern "C" fn(_: *mut COMP_CTX,
                                                  _: *mut libc::c_uchar,
                                                  _: libc::c_uint,
                                                  _: *mut libc::c_uchar,
                                                  _: libc::c_uint)
                                 -> libc::c_int>,
        pub expand: Option<unsafe extern "C" fn(_: *mut COMP_CTX,
                                                _: *mut libc::c_uchar,
                                                _: libc::c_uint,
                                                _: *mut libc::c_uchar,
                                                _: libc::c_uint)
                               -> libc::c_int>,
        pub ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
        pub callback_ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
    }
    use super::ssl_h::{SSL_METHOD, ssl2_state_st, ssl3_state_st,
                       dtls1_state_st, stack_st_SSL_CIPHER, cert_st,
                       SSL_SESSION, GEN_SESSION_CB, stack_st_OCSP_RESPID,
                       TLS_SESSION_TICKET_EXT, tls_session_ticket_ext_cb_fn,
                       tls_session_secret_cb_fn,
                       stack_st_SRTP_PROTECTION_PROFILE,
                       SRTP_PROTECTION_PROFILE, SRP_CTX, lhash_st_SSL_SESSION,
                       ssl_session_st, C2RustUnnamed_8, stack_st_SSL_COMP,
                       ssl3_buf_freelist_st};
    use super::bio_h::BIO;
    use super::_size_t_h::size_t;
    use super::x509_vfy_h::{X509_VERIFY_PARAM, stack_st_X509_OBJECT,
                            stack_st_X509_LOOKUP};
    use super::comp_h::COMP_CTX;
    use super::x509_h::{stack_st_X509_NAME, X509_EXTENSIONS, stack_st_X509,
                        stack_st_X509_CRL, X509_CRL_INFO,
                        stack_st_GENERAL_NAMES, stack_st_X509_NAME_ENTRY,
                        X509_CINF, stack_st_DIST_POINT, stack_st_GENERAL_NAME,
                        X509_CERT_AUX};
    use super::pem_h::pem_password_cb;
    use super::hmac_h::HMAC_CTX;
    use super::asn1_h::ASN1_TYPE;
    use super::crypto_h::stack_st_void;
    use super::evp_h::{C2RustUnnamed_6, stack_st_X509_ATTRIBUTE};
    use super::bn_h::C2RustUnnamed_7;
    use super::dsa_h::DSA_SIG;
    extern "C" {
        #[src_loc = "138:9"]
        pub type evp_pkey_ctx_st;
        #[src_loc = "177:9"]
        pub type engine_st;
        #[src_loc = "157:9"]
        pub type x509_crl_method_st;
        #[src_loc = "190:9"]
        pub type ISSUING_DIST_POINT_st;
        #[src_loc = "188:9"]
        pub type AUTHORITY_KEYID_st;
        #[src_loc = "191:9"]
        pub type NAME_CONSTRAINTS_st;
        #[src_loc = "186:9"]
        pub type X509_POLICY_CACHE_st;
        #[src_loc = "121:9"]
        pub type bignum_ctx;
        #[src_loc = "122:9"]
        pub type bn_blinding_st;
        #[src_loc = "135:9"]
        pub type evp_pkey_asn1_method_st;
        #[src_loc = "185:9"]
        pub type X509_POLICY_TREE_st;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl.h:27"]
pub mod ssl_h {
    #[src_loc = "849:1"]
    pub type SRP_CTX = srp_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "849:9"]
    pub struct srp_ctx_st {
        pub SRP_cb_arg: *mut libc::c_void,
        pub TLS_ext_srp_username_callback: Option<unsafe extern "C" fn(_:
                                                                           *mut SSL,
                                                                       _:
                                                                           *mut libc::c_int,
                                                                       _:
                                                                           *mut libc::c_void)
                                                      -> libc::c_int>,
        pub SRP_verify_param_callback: Option<unsafe extern "C" fn(_:
                                                                       *mut SSL,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
        pub SRP_give_srp_client_pwd_callback: Option<unsafe extern "C" fn(_:
                                                                              *mut SSL,
                                                                          _:
                                                                              *mut libc::c_void)
                                                         ->
                                                             *mut libc::c_char>,
        pub login: *mut libc::c_char,
        pub N: *mut BIGNUM,
        pub g: *mut BIGNUM,
        pub s: *mut BIGNUM,
        pub B: *mut BIGNUM,
        pub A: *mut BIGNUM,
        pub a: *mut BIGNUM,
        pub b: *mut BIGNUM,
        pub v: *mut BIGNUM,
        pub info: *mut libc::c_char,
        pub strength: libc::c_int,
        pub srp_Mask: libc::c_ulong,
    }
    #[src_loc = "383:1"]
    pub type SRTP_PROTECTION_PROFILE = srtp_protection_profile_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "383:9"]
    pub struct srtp_protection_profile_st {
        pub name: *const libc::c_char,
        pub id: libc::c_ulong,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "388:1"]
    pub struct stack_st_SRTP_PROTECTION_PROFILE {
        pub stack: _STACK,
    }
    #[src_loc = "905:1"]
    pub type GEN_SESSION_CB
        =
        Option<unsafe extern "C" fn(_: *const SSL, _: *mut libc::c_uchar,
                                    _: *mut libc::c_uint) -> libc::c_int>;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "922:1"]
    pub struct stack_st_SSL_COMP {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "964:5"]
    pub struct C2RustUnnamed_8 {
        pub sess_connect: libc::c_int,
        pub sess_connect_renegotiate: libc::c_int,
        pub sess_connect_good: libc::c_int,
        pub sess_accept: libc::c_int,
        pub sess_accept_renegotiate: libc::c_int,
        pub sess_accept_good: libc::c_int,
        pub sess_miss: libc::c_int,
        pub sess_timeout: libc::c_int,
        pub sess_cache_full: libc::c_int,
        pub sess_hit: libc::c_int,
        pub sess_cb_hit: libc::c_int,
    }
    #[src_loc = "376:1"]
    pub type SSL_SESSION = ssl_session_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "376:9"]
    pub struct ssl_session_st {
        pub ssl_version: libc::c_int,
        pub key_arg_length: libc::c_uint,
        pub key_arg: [libc::c_uchar; 8],
        pub master_key_length: libc::c_int,
        pub master_key: [libc::c_uchar; 48],
        pub session_id_length: libc::c_uint,
        pub session_id: [libc::c_uchar; 32],
        pub sid_ctx_length: libc::c_uint,
        pub sid_ctx: [libc::c_uchar; 32],
        pub psk_identity_hint: *mut libc::c_char,
        pub psk_identity: *mut libc::c_char,
        pub not_resumable: libc::c_int,
        pub sess_cert: *mut sess_cert_st,
        pub peer: *mut X509,
        pub verify_result: libc::c_long,
        pub references: libc::c_int,
        pub timeout: libc::c_long,
        pub time: libc::c_long,
        pub compress_meth: libc::c_uint,
        pub cipher: *const SSL_CIPHER,
        pub cipher_id: libc::c_ulong,
        pub ciphers: *mut stack_st_SSL_CIPHER,
        pub ex_data: CRYPTO_EX_DATA,
        pub prev: *mut ssl_session_st,
        pub next: *mut ssl_session_st,
        pub tlsext_hostname: *mut libc::c_char,
        pub tlsext_ecpointformatlist_length: size_t,
        pub tlsext_ecpointformatlist: *mut libc::c_uchar,
        pub tlsext_ellipticcurvelist_length: size_t,
        pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
        pub tlsext_tick: *mut libc::c_uchar,
        pub tlsext_ticklen: size_t,
        pub tlsext_tick_lifetime_hint: libc::c_long,
        pub srp_username: *mut libc::c_char,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "380:1"]
    pub struct stack_st_SSL_CIPHER {
        pub stack: _STACK,
    }
    #[src_loc = "375:1"]
    pub type SSL_CIPHER = ssl_cipher_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "375:9"]
    pub struct ssl_cipher_st {
        pub valid: libc::c_int,
        pub name: *const libc::c_char,
        pub id: libc::c_ulong,
        pub algorithm_mkey: libc::c_ulong,
        pub algorithm_auth: libc::c_ulong,
        pub algorithm_enc: libc::c_ulong,
        pub algorithm_mac: libc::c_ulong,
        pub algorithm_ssl: libc::c_ulong,
        pub algo_strength: libc::c_ulong,
        pub algorithm2: libc::c_ulong,
        pub strength_bits: libc::c_int,
        pub alg_bits: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "923:1"]
    pub struct lhash_st_SSL_SESSION {
        pub dummy: libc::c_int,
    }
    #[src_loc = "374:1"]
    pub type SSL_METHOD = ssl_method_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "374:9"]
    pub struct ssl_method_st {
        pub version: libc::c_int,
        pub ssl_new: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
        pub ssl_clear: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
        pub ssl_free: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
        pub ssl_accept: Option<unsafe extern "C" fn(_: *mut SSL)
                                   -> libc::c_int>,
        pub ssl_connect: Option<unsafe extern "C" fn(_: *mut SSL)
                                    -> libc::c_int>,
        pub ssl_read: Option<unsafe extern "C" fn(_: *mut SSL,
                                                  _: *mut libc::c_void,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
        pub ssl_peek: Option<unsafe extern "C" fn(_: *mut SSL,
                                                  _: *mut libc::c_void,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
        pub ssl_write: Option<unsafe extern "C" fn(_: *mut SSL,
                                                   _: *const libc::c_void,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
        pub ssl_shutdown: Option<unsafe extern "C" fn(_: *mut SSL)
                                     -> libc::c_int>,
        pub ssl_renegotiate: Option<unsafe extern "C" fn(_: *mut SSL)
                                        -> libc::c_int>,
        pub ssl_renegotiate_check: Option<unsafe extern "C" fn(_: *mut SSL)
                                              -> libc::c_int>,
        pub ssl_get_message: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_long,
                                                         _: *mut libc::c_int)
                                        -> libc::c_long>,
        pub ssl_read_bytes: Option<unsafe extern "C" fn(_: *mut SSL,
                                                        _: libc::c_int,
                                                        _: *mut libc::c_uchar,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
        pub ssl_write_bytes: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _: libc::c_int,
                                                         _:
                                                             *const libc::c_void,
                                                         _: libc::c_int)
                                        -> libc::c_int>,
        pub ssl_dispatch_alert: Option<unsafe extern "C" fn(_: *mut SSL)
                                           -> libc::c_int>,
        pub ssl_ctrl: Option<unsafe extern "C" fn(_: *mut SSL, _: libc::c_int,
                                                  _: libc::c_long,
                                                  _: *mut libc::c_void)
                                 -> libc::c_long>,
        pub ssl_ctx_ctrl: Option<unsafe extern "C" fn(_: *mut SSL_CTX,
                                                      _: libc::c_int,
                                                      _: libc::c_long,
                                                      _: *mut libc::c_void)
                                     -> libc::c_long>,
        pub get_cipher_by_char: Option<unsafe extern "C" fn(_:
                                                                *const libc::c_uchar)
                                           -> *const SSL_CIPHER>,
        pub put_cipher_by_char: Option<unsafe extern "C" fn(_:
                                                                *const SSL_CIPHER,
                                                            _:
                                                                *mut libc::c_uchar)
                                           -> libc::c_int>,
        pub ssl_pending: Option<unsafe extern "C" fn(_: *const SSL)
                                    -> libc::c_int>,
        pub num_ciphers: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub get_cipher: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const SSL_CIPHER>,
        pub get_ssl_method: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *const ssl_method_st>,
        pub get_timeout: Option<unsafe extern "C" fn() -> libc::c_long>,
        pub ssl3_enc: *mut ssl3_enc_method,
        pub ssl_version: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub ssl_callback_ctrl: Option<unsafe extern "C" fn(_: *mut SSL,
                                                           _: libc::c_int,
                                                           _:
                                                               Option<unsafe extern "C" fn()
                                                                          ->
                                                                              ()>)
                                          -> libc::c_long>,
        pub ssl_ctx_callback_ctrl: Option<unsafe extern "C" fn(_:
                                                                   *mut SSL_CTX,
                                                               _: libc::c_int,
                                                               _:
                                                                   Option<unsafe extern "C" fn()
                                                                              ->
                                                                                  ()>)
                                              -> libc::c_long>,
    }
    #[src_loc = "393:1"]
    pub type tls_session_secret_cb_fn
        =
        Option<unsafe extern "C" fn(_: *mut SSL, _: *mut libc::c_void,
                                    _: *mut libc::c_int,
                                    _: *mut stack_st_SSL_CIPHER,
                                    _: *mut *mut SSL_CIPHER,
                                    _: *mut libc::c_void) -> libc::c_int>;
    #[src_loc = "390:1"]
    pub type tls_session_ticket_ext_cb_fn
        =
        Option<unsafe extern "C" fn(_: *mut SSL, _: *const libc::c_uchar,
                                    _: libc::c_int, _: *mut libc::c_void)
                   -> libc::c_int>;
    #[src_loc = "373:1"]
    pub type TLS_SESSION_TICKET_EXT = tls_session_ticket_ext_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "373:9"]
    pub struct tls_session_ticket_ext_st {
        pub length: libc::c_ushort,
        pub data: *mut libc::c_void,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "1493:5"]
    pub struct dtls1_state_st {
        pub send_cookie: libc::c_uint,
        pub cookie: [libc::c_uchar; 256],
        pub rcvd_cookie: [libc::c_uchar; 256],
        pub cookie_len: libc::c_uint,
        pub r_epoch: libc::c_ushort,
        pub w_epoch: libc::c_ushort,
        pub bitmap: DTLS1_BITMAP,
        pub next_bitmap: DTLS1_BITMAP,
        pub handshake_write_seq: libc::c_ushort,
        pub next_handshake_write_seq: libc::c_ushort,
        pub handshake_read_seq: libc::c_ushort,
        pub last_write_sequence: [libc::c_uchar; 8],
        pub unprocessed_rcds: record_pqueue,
        pub processed_rcds: record_pqueue,
        pub buffered_messages: pqueue,
        pub sent_messages: pqueue,
        pub buffered_app_data: record_pqueue,
        pub listen: libc::c_uint,
        pub link_mtu: libc::c_uint,
        pub mtu: libc::c_uint,
        pub w_msg_hdr: hm_header_st,
        pub r_msg_hdr: hm_header_st,
        pub timeout: dtls1_timeout_st,
        pub next_timeout: timeval,
        pub timeout_duration: libc::c_ushort,
        pub alert_fragment: [libc::c_uchar; 2],
        pub alert_fragment_len: libc::c_uint,
        pub handshake_fragment: [libc::c_uchar; 12],
        pub handshake_fragment_len: libc::c_uint,
        pub retransmitting: libc::c_uint,
        pub change_cipher_spec_ok: libc::c_uint,
    }
    /*
 * SSL3_CT_NUMBER is used to size arrays and it must be large enough to
 * contain all of the cert types defined either for SSLv3 and TLSv1.
 */
    /*
 * Set when the handshake is ready to process peer's ChangeCipherSpec message.
 * Cleared after the message has been processed.
 */
    /* SSL3_FLAGS_SGC_RESTART_DONE is no longer used */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "1492:5"]
    pub struct ssl3_state_st {
        pub flags: libc::c_long,
        pub delay_buf_pop_ret: libc::c_int,
        pub read_sequence: [libc::c_uchar; 8],
        pub read_mac_secret_size: libc::c_int,
        pub read_mac_secret: [libc::c_uchar; 64],
        pub write_sequence: [libc::c_uchar; 8],
        pub write_mac_secret_size: libc::c_int,
        pub write_mac_secret: [libc::c_uchar; 64],
        pub server_random: [libc::c_uchar; 32],
        pub client_random: [libc::c_uchar; 32],
        pub need_empty_fragments: libc::c_int,
        pub empty_fragment_done: libc::c_int,
        pub init_extra: libc::c_int,
        pub rbuf: SSL3_BUFFER,
        pub wbuf: SSL3_BUFFER,
        pub rrec: SSL3_RECORD,
        pub wrec: SSL3_RECORD,
        pub alert_fragment: [libc::c_uchar; 2],
        pub alert_fragment_len: libc::c_uint,
        pub handshake_fragment: [libc::c_uchar; 4],
        pub handshake_fragment_len: libc::c_uint,
        pub wnum: libc::c_uint,
        pub wpend_tot: libc::c_int,
        pub wpend_type: libc::c_int,
        pub wpend_ret: libc::c_int,
        pub wpend_buf: *const libc::c_uchar,
        pub handshake_buffer: *mut BIO,
        pub handshake_dgst: *mut *mut EVP_MD_CTX,
        pub change_cipher_spec: libc::c_int,
        pub warn_alert: libc::c_int,
        pub fatal_alert: libc::c_int,
        pub alert_dispatch: libc::c_int,
        pub send_alert: [libc::c_uchar; 2],
        pub renegotiate: libc::c_int,
        pub total_renegotiations: libc::c_int,
        pub num_renegotiations: libc::c_int,
        pub in_read_app_data: libc::c_int,
        pub client_opaque_prf_input: *mut libc::c_void,
        pub client_opaque_prf_input_len: size_t,
        pub server_opaque_prf_input: *mut libc::c_void,
        pub server_opaque_prf_input_len: size_t,
        pub tmp: C2RustUnnamed_9,
        pub previous_client_finished: [libc::c_uchar; 64],
        pub previous_client_finished_len: libc::c_uchar,
        pub previous_server_finished: [libc::c_uchar; 64],
        pub previous_server_finished_len: libc::c_uchar,
        pub send_connection_binding: libc::c_int,
        pub next_proto_neg_seen: libc::c_int,
        pub is_probably_safari: libc::c_char,
        pub alpn_selected: *mut libc::c_uchar,
        pub alpn_selected_len: libc::c_uint,
    }
    #[src_loc = "908:1"]
    pub type SSL_COMP = ssl_comp_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "908:9"]
    pub struct ssl_comp_st {
        pub id: libc::c_int,
        pub name: *const libc::c_char,
        pub method: *mut COMP_METHOD,
    }
    /* ssl/ssl2.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    /* Protocol Version Codes */
    /* #define SSL2_CLIENT_VERSION  0x0002 */
/* #define SSL2_SERVER_VERSION  0x0002 */
    /* Protocol Message Codes */
    /* Error Message Codes */
    /* Cipher Kind Values */
    /* v3 */
    /* v3 */
    /* v3 */
    /* MS hack */
    /* SSLeay */
    /* SSLeay */
    /* Flags for the SSL_CIPHER.algorithm2 field */
    /* Certificate Type Codes */
    /* Authentication Type Code */
    /* Upper/Lower Bounds */
    /* 2^15-1 */
    /* 2^14-1 */
    /*
 * #define SSL2_CHALLENGE_LENGTH 32
 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "1491:5"]
    pub struct ssl2_state_st {
        pub three_byte_header: libc::c_int,
        pub clear_text: libc::c_int,
        pub escape: libc::c_int,
        pub ssl2_rollback: libc::c_int,
        pub wnum: libc::c_uint,
        pub wpend_tot: libc::c_int,
        pub wpend_buf: *const libc::c_uchar,
        pub wpend_off: libc::c_int,
        pub wpend_len: libc::c_int,
        pub wpend_ret: libc::c_int,
        pub rbuf_left: libc::c_int,
        pub rbuf_offs: libc::c_int,
        pub rbuf: *mut libc::c_uchar,
        pub wbuf: *mut libc::c_uchar,
        pub write_ptr: *mut libc::c_uchar,
        pub padding: libc::c_uint,
        pub rlength: libc::c_uint,
        pub ract_data_length: libc::c_int,
        pub wlength: libc::c_uint,
        pub wact_data_length: libc::c_int,
        pub ract_data: *mut libc::c_uchar,
        pub wact_data: *mut libc::c_uchar,
        pub mac_data: *mut libc::c_uchar,
        pub read_key: *mut libc::c_uchar,
        pub write_key: *mut libc::c_uchar,
        pub challenge_length: libc::c_uint,
        pub challenge: [libc::c_uchar; 32],
        pub conn_id_length: libc::c_uint,
        pub conn_id: [libc::c_uchar; 16],
        pub key_material_length: libc::c_uint,
        pub key_material: [libc::c_uchar; 48],
        pub read_sequence: libc::c_ulong,
        pub write_sequence: libc::c_ulong,
        pub tmp: C2RustUnnamed_10,
    }
    use super::ossl_typ_h::{SSL, BIGNUM, X509, CRYPTO_EX_DATA, SSL_CTX,
                            EVP_MD_CTX, COMP_METHOD};
    use super::stack_h::_STACK;
    use super::_size_t_h::size_t;
    use super::dtls1_h::{DTLS1_BITMAP, record_pqueue, hm_header_st,
                         dtls1_timeout_st};
    use super::pqueue_h::pqueue;
    use super::_timeval_h::timeval;
    use super::ssl3_h::{SSL3_BUFFER, SSL3_RECORD, C2RustUnnamed_9};
    use super::bio_h::BIO;
    use super::ssl2_h::C2RustUnnamed_10;
    extern "C" {
        #[src_loc = "1114:5"]
        pub type ssl3_buf_freelist_st;
        #[src_loc = "1035:5"]
        pub type cert_st;
        #[src_loc = "531:5"]
        pub type sess_cert_st;
        #[src_loc = "466:5"]
        pub type ssl3_enc_method;
        #[src_loc = "1610:5"]
        pub type stack_st_OCSP_RESPID;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the SSL functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/stack.h:27"]
pub mod stack_h {
    /* crypto/stack/stack.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    #[src_loc = "66:1"]
    pub type _STACK = stack_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "66:9"]
    pub struct stack_st {
        pub num: libc::c_int,
        pub data: *mut *mut libc::c_char,
        pub sorted: libc::c_int,
        pub num_alloc: libc::c_int,
        pub comp: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                              _: *const libc::c_void)
                             -> libc::c_int>,
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/hmac.h:27"]
pub mod hmac_h {
    /* crypto/hmac/hmac.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    /* largest known is SHA512 */
    #[src_loc = "75:1"]
    pub type HMAC_CTX = hmac_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "75:9"]
    pub struct hmac_ctx_st {
        pub md: *const EVP_MD,
        pub md_ctx: EVP_MD_CTX,
        pub i_ctx: EVP_MD_CTX,
        pub o_ctx: EVP_MD_CTX,
        pub key_length: libc::c_uint,
        pub key: [libc::c_uchar; 128],
    }
    use super::ossl_typ_h::{EVP_MD, EVP_MD_CTX};
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/asn1.h:27"]
pub mod asn1_h {
    /* crypto/asn1/asn1.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    /* let the recipient choose */
    /* used in ASN1_TYPE */
    /* used in ASN1 template code */
    /* negative flag */
    /* */
    /* */
    /* alias */
    /* */
    /* */
    /* */
    /* */
    /* alias */
    /* */
    /* */
    /* For use with d2i_ASN1_type_bytes() */
    /* For use with ASN1_mbstring_copy() */
    /* filled in by mkstack.pl */
    /* nothing, no longer needed */
    /*
 * We MUST make sure that, except for constness, asn1_ctx_st and
 * asn1_const_ctx are exactly the same.  Fortunately, as soon as the old ASN1
 * parsing macros are gone, we can throw this away as well...
 */
    /* work char pointer */
    /* end of sequence read for indefinite
                                 * encoding */
    /* error code to use when returning an error */
    /* constructed if 0x20, indefinite is 0x21 */
    /* tag from last 'get object' */
    /* class from last 'get object' */
    /* length of last 'get object' */
    /* largest value of p allowed */
    /* temporary variable */
    /* variable */
    /* used in error processing */
    /* work char pointer */
    /* end of sequence read for indefinite
                                 * encoding */
    /* error code to use when returning an error */
    /* constructed if 0x20, indefinite is 0x21 */
    /* tag from last 'get object' */
    /* class from last 'get object' */
    /* length of last 'get object' */
    /* largest value of p allowed */
    /* temporary variable */
    /* variable */
    /* used in error processing */
    /*
 * These are used internally in the ASN1_OBJECT to keep track of whether the
 * names and data need to be free()ed
 */
    /* internal use */
    /* critical x509v3 object id */
    /* internal use */
    /* internal use */
    /* data remains const after init */
    /* Should we free this one */
    /* Set if 0x07 has bits left value */
    /*
 * This indicates that the ASN1_STRING is not a real value but just a place
 * holder for the location where indefinite length constructed data should be
 * inserted in the memory buffer
 */
    /*
 * This flag is used by the CMS code to indicate that a string is not
 * complete and is a place holder for content when it had all been accessed.
 * The flag will be reset when content has been written to it.
 */
    /*
 * This flag is used by ASN1 code to indicate an ASN1_STRING is an MSTRING
 * type.
 */
    /* This is the base type that holds just about everything :-) */
    /*
     * The value of the following field depends on the type being held.  It
     * is mostly being used for BIT_STRING so if the input data has a
     * non-zero 'unused bits' value, it will be handled correctly
     */
    /*
 * ASN1_ENCODING structure: this is used to save the received encoding of an
 * ASN1 type. This is useful to get round problems with invalid encodings
 * which can break signatures.
 */
    /* DER encoding */
    /* Length of encoding */
    /* set to 1 if 'enc' is invalid */
    /* Used with ASN1 LONG type: if a long is set to this it is omitted */
    /* size limits: this stuff is taken straight from RFC2459 */
    /*
 * Declarations for template structures: for full definitions see asn1t.h
 */
    /* This is just an opaque pointer */
    /* Declare ASN1 functions: the implement macro in in asn1t.h */
    /*-
 * The following macros and typedefs allow an ASN1_ITEM
 * to be embedded in a structure and referenced. Since
 * the ASN1_ITEM pointers need to be globally accessible
 * (possibly from shared libraries) they may exist in
 * different forms. On platforms that support it the
 * ASN1_ITEM structure itself will be globally exported.
 * Other platforms will export a function that returns
 * an ASN1_ITEM pointer.
 *
 * To handle both cases transparently the macros below
 * should be used instead of hard coding an ASN1_ITEM
 * pointer in a structure.
 *
 * The structure will look like this:
 *
 * typedef struct SOMETHING_st {
 *      ...
 *      ASN1_ITEM_EXP *iptr;
 *      ...
 * } SOMETHING;
 *
 * It would be initialised as e.g.:
 *
 * SOMETHING somevar = {...,ASN1_ITEM_ref(X509),...};
 *
 * and the actual pointer extracted with:
 *
 * const ASN1_ITEM *it = ASN1_ITEM_ptr(somevar.iptr);
 *
 * Finally an ASN1_ITEM pointer can be extracted from an
 * appropriate reference with: ASN1_ITEM_rptr(X509). This
 * would be used when a function takes an ASN1_ITEM * argument.
 *
 */
    /* ASN1_ITEM pointer exported type */
    /* Macro to obtain ASN1_ITEM pointer from exported type */
    /* Macro to include ASN1_ITEM pointer from base type */
    /* Parameters used by ASN1_STRING_print_ex() */
    /*
 * These determine which characters to escape: RFC2253 special characters,
 * control characters and MSB set characters
 */
    /*
 * This flag determines how we do escaping: normally RC2253 backslash only,
 * set this to use backslash and quote.
 */
    /* These three flags are internal use only. */
    /* Character is a valid PrintableString character */
    /* Character needs escaping if it is the first character */
    /* Character needs escaping if it is the last character */
    /*
 * NB the internal flags are safely reused below by flags handled at the top
 * level.
 */
    /*
 * If this is set we convert all character strings to UTF8 first
 */
    /*
 * If this is set we don't attempt to interpret content: just assume all
 * strings are 1 byte per character. This will produce some pretty odd
 * looking output!
 */
    /* If this is set we include the string type in the output */
    /*
 * This determines which strings to display and which to 'dump' (hex dump of
 * content octets or DER encoding). We can only dump non character strings or
 * everything. If we don't dump 'unknown' they are interpreted as character
 * strings with 1 octet per character and are subject to the usual escaping
 * options.
 */
    /*
 * These determine what 'dumping' does, we can dump the content octets or the
 * DER encoding: both use the RFC2253 #XXXXX notation.
 */
    /*
 * All the string flags consistent with RFC2253, escaping control characters
 * isn't essential in RFC2253 but it is advisable anyway.
 */
    #[src_loc = "524:1"]
    pub type ASN1_TYPE = asn1_type_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "524:9"]
    pub struct asn1_type_st {
        pub type_0: libc::c_int,
        pub value: C2RustUnnamed_5,
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "526:5"]
    pub union C2RustUnnamed_5 {
        pub ptr: *mut libc::c_char,
        pub boolean: ASN1_BOOLEAN,
        pub asn1_string: *mut ASN1_STRING,
        pub object: *mut ASN1_OBJECT,
        pub integer: *mut ASN1_INTEGER,
        pub enumerated: *mut ASN1_ENUMERATED,
        pub bit_string: *mut ASN1_BIT_STRING,
        pub octet_string: *mut ASN1_OCTET_STRING,
        pub printablestring: *mut ASN1_PRINTABLESTRING,
        pub t61string: *mut ASN1_T61STRING,
        pub ia5string: *mut ASN1_IA5STRING,
        pub generalstring: *mut ASN1_GENERALSTRING,
        pub bmpstring: *mut ASN1_BMPSTRING,
        pub universalstring: *mut ASN1_UNIVERSALSTRING,
        pub utctime: *mut ASN1_UTCTIME,
        pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
        pub visiblestring: *mut ASN1_VISIBLESTRING,
        pub utf8string: *mut ASN1_UTF8STRING,
        pub set: *mut ASN1_STRING,
        pub sequence: *mut ASN1_STRING,
        pub asn1_value: *mut ASN1_VALUE,
    }
    #[src_loc = "299:1"]
    pub type ASN1_VALUE = ASN1_VALUE_st;
    /*
         * set and sequence are left complete and still contain the set or
         * sequence bytes
         */
    /* This is used to contain a list of bit names */
    /* Macros for string operations */
    /* for the is_set parameter to i2d_ASN1_SET */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "793:1"]
    pub struct stack_st_ASN1_OBJECT {
        pub stack: _STACK,
    }
    #[src_loc = "257:1"]
    pub type ASN1_ENCODING = ASN1_ENCODING_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "257:9"]
    pub struct ASN1_ENCODING_st {
        pub enc: *mut libc::c_uchar,
        pub len: libc::c_long,
        pub modified: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "162:1"]
    pub struct stack_st_X509_ALGOR {
        pub stack: _STACK,
    }
    use super::ossl_typ_h::{ASN1_BOOLEAN, ASN1_STRING, ASN1_OBJECT,
                            ASN1_INTEGER, ASN1_ENUMERATED, ASN1_BIT_STRING,
                            ASN1_OCTET_STRING, ASN1_PRINTABLESTRING,
                            ASN1_T61STRING, ASN1_IA5STRING,
                            ASN1_GENERALSTRING, ASN1_BMPSTRING,
                            ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
                            ASN1_GENERALIZEDTIME, ASN1_VISIBLESTRING,
                            ASN1_UTF8STRING};
    use super::stack_h::_STACK;
    extern "C" {
        #[src_loc = "299:9"]
        pub type ASN1_VALUE_st;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the ASN1 functions. */
}
#[header_src =
  "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509_vfy.h:27"]
pub mod x509_vfy_h {
    /* crypto/x509/x509_vfy.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    /* number of paths to files or directories */
    /* the list of paths or directories */
    /* ******************************/
/*-
SSL_CTX -> X509_STORE
                -> X509_LOOKUP
                        ->X509_LOOKUP_METHOD
                -> X509_LOOKUP
                        ->X509_LOOKUP_METHOD

SSL     -> X509_STORE_CTX
                ->X509_STORE

The X509_STORE holds the tables etc for verification stuff.
A X509_STORE_CTX is used while validating a single certificate.
The X509_STORE has X509_LOOKUPs for looking up certs.
The X509_STORE then calls a function to actually verify the
certificate chain.
*/
    /* one of the above types */
    /* This is a static that defines the function interface */
    /*
 * This structure hold all parameters associated with a verify operation by
 * including an X509_VERIFY_PARAM structure in related structures the
 * parameters used can be customized
 */
    #[src_loc = "167:1"]
    pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "167:9"]
    pub struct X509_VERIFY_PARAM_st {
        pub name: *mut libc::c_char,
        pub check_time: time_t,
        pub inh_flags: libc::c_ulong,
        pub flags: libc::c_ulong,
        pub purpose: libc::c_int,
        pub trust: libc::c_int,
        pub depth: libc::c_int,
        pub policies: *mut stack_st_ASN1_OBJECT,
        pub id: *mut X509_VERIFY_PARAM_ID,
    }
    #[src_loc = "159:1"]
    pub type X509_VERIFY_PARAM_ID = X509_VERIFY_PARAM_ID_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "136:1"]
    pub struct stack_st_X509_LOOKUP {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "137:1"]
    pub struct stack_st_X509_OBJECT {
        pub stack: _STACK,
    }
    use super::_time_t_h::time_t;
    use super::asn1_h::stack_st_ASN1_OBJECT;
    use super::stack_h::_STACK;
    extern "C" {
        #[src_loc = "159:9"]
        pub type X509_VERIFY_PARAM_ID_st;
    }
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/crypto.h:27"]
pub mod crypto_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "293:5"]
    pub struct stack_st_void {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "290:9"]
    pub struct bio_st {
        pub method: *mut BIO_METHOD,
        pub callback: Option<unsafe extern "C" fn(_: *mut bio_st,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: libc::c_long,
                                                  _: libc::c_long)
                                 -> libc::c_long>,
        pub cb_arg: *mut libc::c_char,
        pub init: libc::c_int,
        pub shutdown: libc::c_int,
        pub flags: libc::c_int,
        pub retry_reason: libc::c_int,
        pub num: libc::c_int,
        pub ptr: *mut libc::c_void,
        pub next_bio: *mut bio_st,
        pub prev_bio: *mut bio_st,
        pub references: libc::c_int,
        pub num_read: libc::c_ulong,
        pub num_write: libc::c_ulong,
        pub ex_data: CRYPTO_EX_DATA,
        /* Time to use */
        /* Inheritance flags */
        /* Various verify flags */
        /* purpose to check untrusted certificates */
        /* trust setting to check */
        /* Verify depth */
        /* Permissible policies */
        /* opaque ID data */
        /*
 * This stuff is basically class callback functions The current classes are
 * SSL_CTX, SSL, SSL_SESSION, and a few more
 */
        /* Arbitary long */
        /* Arbitary void * */
        /*
 * Per class, we have a STACK of CRYPTO_EX_DATA_FUNCS for each CRYPTO_EX_DATA
 * entry.
 */
    }
    use super::stack_h::_STACK;
    use super::bio_h::BIO_METHOD;
    use super::ossl_typ_h::CRYPTO_EX_DATA;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the CRYPTO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/x509.h:27"]
pub mod x509_h {
    #[src_loc = "441:1"]
    pub type X509_CRL_INFO = X509_crl_info_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "441:9"]
    pub struct X509_crl_info_st {
        pub version: *mut ASN1_INTEGER,
        pub sig_alg: *mut X509_ALGOR,
        pub issuer: *mut X509_NAME,
        pub lastUpdate: *mut ASN1_TIME,
        pub nextUpdate: *mut ASN1_TIME,
        pub revoked: *mut stack_st_X509_REVOKED,
        pub extensions: *mut stack_st_X509_EXTENSION,
        pub enc: ASN1_ENCODING,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "202:9"]
    pub struct stack_st_X509_EXTENSION {
        pub stack: _STACK,
    }
    /* [0] */
    /* optional */
    /* Set up if indirect CRL */
    /* Revocation reason */
    /* load sequence */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "438:1"]
    pub struct stack_st_X509_REVOKED {
        pub stack: _STACK,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "175:1"]
    pub struct stack_st_X509_NAME_ENTRY {
        pub stack: _STACK,
    }
    /*
 * This stuff is certificate "auxiliary info" it contains details which are
 * useful in certificate stores and databases. When used this is tagged onto
 * the end of the certificate itself
 */
    #[src_loc = "262:1"]
    pub type X509_CERT_AUX = x509_cert_aux_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "262:9"]
    pub struct x509_cert_aux_st {
        pub trust: *mut stack_st_ASN1_OBJECT,
        pub reject: *mut stack_st_ASN1_OBJECT,
        pub alias: *mut ASN1_UTF8STRING,
        pub keyid: *mut ASN1_OCTET_STRING,
        pub other: *mut stack_st_X509_ALGOR,
    }
    #[src_loc = "242:1"]
    pub type X509_CINF = x509_cinf_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "242:9"]
    pub struct x509_cinf_st {
        pub version: *mut ASN1_INTEGER,
        pub serialNumber: *mut ASN1_INTEGER,
        pub signature: *mut X509_ALGOR,
        pub issuer: *mut X509_NAME,
        pub validity: *mut X509_VAL,
        pub subject: *mut X509_NAME,
        pub key: *mut X509_PUBKEY,
        pub issuerUID: *mut ASN1_BIT_STRING,
        pub subjectUID: *mut ASN1_BIT_STRING,
        pub extensions: *mut stack_st_X509_EXTENSION,
        pub enc: ASN1_ENCODING,
    }
    #[src_loc = "152:1"]
    pub type X509_VAL = X509_val_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "152:9"]
    pub struct X509_val_st {
        pub notBefore: *mut ASN1_TIME,
        pub notAfter: *mut ASN1_TIME,
    }
    /* trusted uses */
    /* rejected uses */
    /* "friendly name" */
    /* key id of private key */
    /* other unspecified info */
    /* [ 0 ] default of v1 */
    /* [ 1 ] optional in v2 */
    /* [ 2 ] optional in v2 */
    /* [ 3 ] optional in v3 */
    /* These contain copies of various extension values */
    /* X509 */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "301:1"]
    pub struct stack_st_X509 {
        pub stack: _STACK,
    }
    /* actual signature */
    /* Copies of various extensions */
    /* Convenient breakdown of IDP */
    /* CRL and base CRL numbers for delta processing */
    /* X509_CRL */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "476:1"]
    pub struct stack_st_X509_CRL {
        pub stack: _STACK,
    }
    /* true if 'bytes' needs to be built */
    /*      unsigned long hash; Keep the hash around for lookups */
    /* X509_NAME */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "192:1"]
    pub struct stack_st_X509_NAME {
        pub stack: _STACK,
    }
    #[src_loc = "202:1"]
    pub type X509_EXTENSIONS = stack_st_X509_EXTENSION;
    use super::ossl_typ_h::{ASN1_INTEGER, X509_ALGOR, X509_NAME, ASN1_TIME,
                            ASN1_UTF8STRING, ASN1_OCTET_STRING, X509_PUBKEY,
                            ASN1_BIT_STRING};
    use super::asn1_h::{ASN1_ENCODING, stack_st_ASN1_OBJECT,
                        stack_st_X509_ALGOR};
    use super::stack_h::_STACK;
    extern "C" {
        #[src_loc = "471:5"]
        pub type stack_st_GENERAL_NAMES;
        #[src_loc = "289:5"]
        pub type stack_st_GENERAL_NAME;
        #[src_loc = "288:5"]
        pub type stack_st_DIST_POINT;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the X509 functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/evp.h:27"]
pub mod evp_h {
    /* crypto/evp/evp.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    /*-
#define EVP_RC2_KEY_SIZE                16
#define EVP_RC4_KEY_SIZE                16
#define EVP_BLOWFISH_KEY_SIZE           16
#define EVP_CAST5_KEY_SIZE              16
#define EVP_RC5_32_12_16_KEY_SIZE       16
*/
    /* longest known is SHA512 */
    /* Default PKCS#5 iteration count */
    /*
 * Type needs to be a bit field Sub-type needs to be for variations on the
 * method, as in, can it do arbitrary encryption....
 */
    /* RSA */
    /* DSA */
    /* DH */
    /* ECC */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "151:5"]
    pub struct stack_st_X509_ATTRIBUTE {
        pub stack: _STACK,
        /* [ 0 ] */
        /* EVP_PKEY */
        /* FIXME: prototype these some day */
        /* EVP_PKEY_xxx */
        /* how big does the ctx->md_data need to be */
        /* control function */
        /* EVP_MD */
        /* digest can only handle a single block */
        /*
 * digest is a "clone" digest used
 * which is a copy of an existing
 * one for a specific public key type.
 * EVP_dss1() etc
 */
        /* Digest uses EVP_PKEY_METHOD for signing instead of MD specific signing */
        /* DigestAlgorithmIdentifier flags... */
        /* NULL or absent parameter accepted. Use NULL */
        /* NULL or absent parameter accepted. Use NULL for PKCS#1 otherwise absent */
        /* Custom handling via ctrl */
        /* Note if suitable for use in FIPS mode */
    }
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "135:5"]
    pub union C2RustUnnamed_6 {
        pub ptr: *mut libc::c_char,
        pub rsa: *mut rsa_st,
        pub dsa: *mut dsa_st,
        pub dh: *mut dh_st,
        pub ec: *mut ec_key_st,
    }
    use super::stack_h::_STACK;
    use super::ossl_typ_h::{rsa_st, dsa_st, dh_st};
    extern "C" {
        #[src_loc = "147:9"]
        pub type ec_key_st;
    }
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the EVP functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bn.h:27"]
pub mod bn_h {
    #[derive ( Copy, Clone )]
    #[repr ( C )]
    #[src_loc = "352:5"]
    pub union C2RustUnnamed_7 {
        pub cb_1: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                              _: *mut libc::c_void) -> ()>,
        pub cb_2: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                              _: *mut BN_GENCB)
                             -> libc::c_int>,
    }
    use super::ossl_typ_h::BN_GENCB;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BN functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dsa.h:27"]
pub mod dsa_h {
    #[src_loc = "124:1"]
    pub type DSA_SIG = DSA_SIG_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "124:9"]
    pub struct DSA_SIG_st {
        pub r: *mut BIGNUM,
        pub s: *mut BIGNUM,
    }
    use super::ossl_typ_h::BIGNUM;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the DSA functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pem.h:27"]
pub mod pem_h {
    /* crypto/pem/pem.h */
/* Copyright (C) 1995-1997 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    /*
   * Note that this structure is initialised by PEM_SealInit and cleaned up
   * by PEM_SealFinal (at least for now)
   */
    /* enc_type is one off */
    /*      char iv[8]; unused and wrong size */
    /* what type of object */
    /*-
        unused, and wrong size
        unsigned char iv[8]; */
    /*-
    XXX(ben): don#t think this is used!
        STACK *x509_chain;      / * certificate chain */
    /* signature type */
    /* is the md encrypted or not? */
    /* length of md_data */
    /* message digest, could be pkey encrypted */
    /* date encryption cipher */
    /* key length */
    /* key */
    /*-
    unused, and wrong size
    unsigned char iv[8]; */
    /* is the data encrypted */
    /*
 * These macros make the PEM_read/PEM_write functions easier to maintain and
 * write. Now they are all implemented with either: IMPLEMENT_PEM_rw(...) or
 * IMPLEMENT_PEM_rw_cb(...)
 */
    /* These are the same except they are for the declarations */
    /* "userdata": new with OpenSSL 0.9.4 */
    #[src_loc = "389:1"]
    pub type pem_password_cb
        =
        unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int,
                             _: libc::c_int, _: *mut libc::c_void)
            -> libc::c_int;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the PEM functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/comp.h:27"]
pub mod comp_h {
    #[src_loc = "15:1"]
    pub type COMP_CTX = comp_ctx_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "15:9"]
    pub struct comp_ctx_st {
        pub meth: *mut COMP_METHOD,
        pub compress_in: libc::c_ulong,
        pub compress_out: libc::c_ulong,
        pub expand_in: libc::c_ulong,
        pub expand_out: libc::c_ulong,
        pub ex_data: CRYPTO_EX_DATA,
    }
    use super::ossl_typ_h::{COMP_METHOD, CRYPTO_EX_DATA};
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the COMP functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/dtls1.h:27"]
pub mod dtls1_h {
    /* ssl/dtls1.h */
/*
 * DTLS implementation written by Nagendra Modadugu
 * (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
 */
/* ====================================================================
 * Copyright (c) 1999-2005 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.OpenSSL.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@OpenSSL.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.OpenSSL.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
    /* Special value for method supporting multiple versions */
    /* lengths of messages */
    /* Max MTU overhead we know about so far is 40 for IPv6 + 8 for UDP */
    /* track 32 packets on 32-bit systems and 64
                                 * - on 64-bit systems */
    /* max record number seen so far, 64-bit
                                   * value in big-endian encoding */
    /* cryptographic state */
    /* used for mac generation */
    /* compression */
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "162:1"]
    pub struct dtls1_timeout_st {
        pub read_timeouts: libc::c_uint,
        pub write_timeouts: libc::c_uint,
        pub num_alerts: libc::c_uint,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "147:1"]
    pub struct hm_header_st {
        pub type_0: libc::c_uchar,
        pub msg_len: libc::c_ulong,
        pub seq: libc::c_ushort,
        pub frag_off: libc::c_ulong,
        pub frag_len: libc::c_ulong,
        pub is_ccs: libc::c_uint,
        pub saved_retransmit_state: dtls1_retransmit_state,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "135:1"]
    pub struct dtls1_retransmit_state {
        pub enc_write_ctx: *mut EVP_CIPHER_CTX,
        pub write_hash: *mut EVP_MD_CTX,
        pub compress: *mut COMP_CTX,
        pub session: *mut SSL_SESSION,
        pub epoch: libc::c_ushort,
    }
    #[src_loc = "171:1"]
    pub type record_pqueue = record_pqueue_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "171:9"]
    pub struct record_pqueue_st {
        pub epoch: libc::c_ushort,
        pub q: pqueue,
    }
    #[src_loc = "128:1"]
    pub type DTLS1_BITMAP = dtls1_bitmap_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "128:9"]
    pub struct dtls1_bitmap_st {
        pub map: libc::c_ulong,
        pub max_seq_num: [libc::c_uchar; 8],
    }
    use super::ossl_typ_h::{EVP_CIPHER_CTX, EVP_MD_CTX};
    use super::comp_h::COMP_CTX;
    use super::ssl_h::SSL_SESSION;
    use super::pqueue_h::pqueue;
    /* Timeout multipliers (timeout slice is defined in apps/timeouts.h */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/pqueue.h:27"]
pub mod pqueue_h {
    /* crypto/pqueue/pqueue.h */
/*
 * DTLS implementation written by Nagendra Modadugu
 * (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
 */
/* ====================================================================
 * Copyright (c) 1999-2005 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.OpenSSL.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@OpenSSL.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.OpenSSL.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
    #[src_loc = "70:1"]
    pub type pqueue = *mut _pqueue;
    extern "C" {
        #[src_loc = "70:9"]
        pub type _pqueue;
    }
    /* ! HEADER_PQUEUE_H */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl3.h:27"]
pub mod ssl3_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "553:5"]
    pub struct C2RustUnnamed_9 {
        pub cert_verify_md: [libc::c_uchar; 128],
        pub finish_md: [libc::c_uchar; 128],
        pub finish_md_len: libc::c_int,
        pub peer_finish_md: [libc::c_uchar; 128],
        pub peer_finish_md_len: libc::c_int,
        pub message_size: libc::c_ulong,
        pub message_type: libc::c_int,
        pub new_cipher: *const SSL_CIPHER,
        pub dh: *mut DH,
        pub ecdh: *mut EC_KEY,
        pub next_state: libc::c_int,
        pub reuse_message: libc::c_int,
        pub cert_req: libc::c_int,
        pub ctype_num: libc::c_int,
        pub ctype: [libc::c_char; 9],
        pub ca_names: *mut stack_st_X509_NAME,
        pub use_rsa_tmp: libc::c_int,
        pub key_block_length: libc::c_int,
        pub key_block: *mut libc::c_uchar,
        pub new_sym_enc: *const EVP_CIPHER,
        pub new_hash: *const EVP_MD,
        pub new_mac_pkey_type: libc::c_int,
        pub new_mac_secret_size: libc::c_int,
        pub new_compression: *const SSL_COMP,
        pub cert_request: libc::c_int,
    }
    /* ssl/ssl3.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* ====================================================================
 * Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
/* ====================================================================
 * Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
 * ECC cipher suite support in OpenSSL originally developed by
 * SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
 */
    /*
 * Signalling cipher suite value from RFC 5746
 * (TLS_EMPTY_RENEGOTIATION_INFO_SCSV)
 */
    /*
 * Signalling cipher suite value from draft-ietf-tls-downgrade-scsv-00
 * (TLS_FALLBACK_SCSV)
 */
    /*
 * VRS Additional Kerberos5 entries
 */
    /*
 * This next block of six "EDH" labels is for backward compatibility with
 * older versions of OpenSSL.  New code should use the six "DHE" labels above
 * instead:
 */
    /*
  * Some will argue that this increases memory footprint, but it's not
  * actually true. Point is that malloc has to return at least 64-bit aligned
  * pointers, meaning that allocating 5 bytes wastes 3 bytes in either case.
  * Suggested pre-gaping simply moves these wasted bytes from the end of
  * allocated region to its front, but makes data payload aligned, which
  * improves performance:-)
  */
    /*
 * This is the maximum MAC (digest) size used by the SSL library. Currently
 * maximum of 20 is used by SHA1, but we reserve for future extension for
 * 512-bit hashes.
 */
    /*
 * Maximum block size used in all ciphersuites. Currently 16 for AES.
 */
    /* Maximum plaintext length: defined by SSL/TLS standards */
    /* Maximum compression overhead: defined by SSL/TLS standards */
    /*
 * The standards give a maximum encryption overhead of 1024 bytes. In
 * practice the value is lower than this. The overhead is the maximum number
 * of padding bytes (256) plus the mac size.
 */
    /*
 * OpenSSL currently only uses a padding length of at most one block so the
 * send overhead is smaller.
 */
    /* If compression isn't used don't include the compression overhead */
    /* Pseudo content types to indicate additional parameters */
    /* Pseudo content type for SSL/TLS header info */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    #[src_loc = "403:1"]
    pub type SSL3_RECORD = ssl3_record_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "403:9"]
    pub struct ssl3_record_st {
        pub type_0: libc::c_int,
        pub length: libc::c_uint,
        pub off: libc::c_uint,
        pub data: *mut libc::c_uchar,
        pub input: *mut libc::c_uchar,
        pub comp: *mut libc::c_uchar,
        pub epoch: libc::c_ulong,
        pub seq_num: [libc::c_uchar; 8],
    }
    #[src_loc = "438:1"]
    pub type SSL3_BUFFER = ssl3_buffer_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "438:9"]
    pub struct ssl3_buffer_st {
        pub buf: *mut libc::c_uchar,
        pub len: size_t,
        pub offset: libc::c_int,
        pub left: libc::c_int,
    }
    use super::ssl_h::{SSL_CIPHER, SSL_COMP};
    use super::ossl_typ_h::{DH, EVP_CIPHER, EVP_MD};
    use super::ec_h::EC_KEY;
    use super::x509_h::stack_st_X509_NAME;
    use super::_size_t_h::size_t;
    /* type of record */
    /*
     * r
     */
    /* How many bytes available */
    /*
     * rw
     */
    /* read/write offset into 'buf' */
    /*
     * r
     */
    /* pointer to the record data */
    /*
     * rw
     */
    /* where the decode bytes are */
    /*
     * rw
     */
    /* only used with decompression - malloc()ed */
    /*
     * r
     */
    /* epoch number, needed by DTLS1 */
    /*
     * r
     */
    /* sequence number, needed by DTLS1 */
    /*
     * r
     */
    /* at least SSL3_RT_MAX_PACKET_SIZE bytes, see ssl3_setup_buffers() */
    /* buffer size */
    /* where to 'copy from' */
    /* how many bytes left */
    /* These are used when changing over to a new cipher */
    /* write to client */
    /* read from client */
    /* write to client */
    /* read from client */
/* Do not change the number values, they do matter */
    /* server */
/* extra state */
    /* read from server */
    /* write to server */
    /* read from server */
    /* write to server */
    /* SSLv3 */
/*
 * client
 */
/* extra state */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ec.h:27"]
pub mod ec_h {
    /* crypto/ec/ec.h */
/*
 * Originally written by Bodo Moeller for the OpenSSL project.
 */
/* *
 * \file crypto/ec/ec.h Include file for the OpenSSL EC functions
 * \author Originally written by Bodo Moeller for the OpenSSL project
 */
/* ====================================================================
 * Copyright (c) 1998-2019 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
/* ====================================================================
 * Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
 *
 * Portions of the attached software ("Contribution") are developed by
 * SUN MICROSYSTEMS, INC., and are contributed to the OpenSSL project.
 *
 * The Contribution is licensed pursuant to the OpenSSL open source
 * license provided above.
 *
 * The elliptic curve binary polynomial software is originally written by
 * Sheueling Chang Shantz and Douglas Stebila of Sun Microsystems Laboratories.
 *
 */
    /* * Enum for the point conversion form as defined in X9.62 (ECDSA)
 *  for the encoding of a elliptic curve point (x,y) */
    /* * the point is encoded as z||x, where the octet z specifies
         *  which solution of the quadratic equation y is  */
    /* * the point is encoded as z||x||y, where z is the octet 0x04  */
    /* * the point is encoded as z||x||y, where the octet z specifies
         *  which solution of the quadratic equation y is  */
    /*-
     EC_METHOD *meth;
     -- field definition
     -- curve coefficients
     -- optional generator with associated information (order, cofactor)
     -- optional extra data (precomputed table for fast computation of multiples of generator)
     -- ASN1 stuff
    */
    /* *******************************************************************/
/*               EC_METHODs for curves over GF(p)                   */
/* *******************************************************************/
    /* * Returns the basic GFp ec methods which provides the basis for the
 *  optimized methods.
 *  \return  EC_METHOD object
 */
    /* * Returns GFp methods using montgomery multiplication.
 *  \return  EC_METHOD object
 */
    /* * Returns GFp methods using optimized methods for NIST recommended curves
 *  \return  EC_METHOD object
 */
    /* * Returns 64-bit optimized methods for nistp224
 *  \return  EC_METHOD object
 */
    /* * Returns 64-bit optimized methods for nistp256
 *  \return  EC_METHOD object
 */
    /* * Returns 64-bit optimized methods for nistp521
 *  \return  EC_METHOD object
 */
    /* *******************************************************************/
/*           EC_METHOD for curves over GF(2^m)                      */
/* *******************************************************************/
    /* * Returns the basic GF2m ec method
 *  \return  EC_METHOD object
 */
    /* *******************************************************************/
/*                   EC_GROUP functions                             */
/* *******************************************************************/
    /* * Creates a new EC_GROUP object
 *  \param   meth  EC_METHOD to use
 *  \return  newly created EC_GROUP object or NULL in case of an error.
 */
    /* * Frees a EC_GROUP object
 *  \param  group  EC_GROUP object to be freed.
 */
    /* * Clears and frees a EC_GROUP object
 *  \param  group  EC_GROUP object to be cleared and freed.
 */
    /* * Copies EC_GROUP objects. Note: both EC_GROUPs must use the same EC_METHOD.
 *  \param  dst  destination EC_GROUP object
 *  \param  src  source EC_GROUP object
 *  \return 1 on success and 0 if an error occurred.
 */
    /* * Creates a new EC_GROUP object and copies the copies the content
 *  form src to the newly created EC_KEY object
 *  \param  src  source EC_GROUP object
 *  \return newly created EC_GROUP object or NULL in case of an error.
 */
    /* * Returns the EC_METHOD of the EC_GROUP object.
 *  \param  group  EC_GROUP object
 *  \return EC_METHOD used in this EC_GROUP object.
 */
    /* * Returns the field type of the EC_METHOD.
 *  \param  meth  EC_METHOD object
 *  \return NID of the underlying field type OID.
 */
    /* * Sets the generator and it's order/cofactor of a EC_GROUP object.
 *  \param  group      EC_GROUP object
 *  \param  generator  EC_POINT object with the generator.
 *  \param  order      the order of the group generated by the generator.
 *  \param  cofactor   the index of the sub-group generated by the generator
 *                     in the group of all points on the elliptic curve.
 *  \return 1 on success and 0 if an error occured
 */
    /* * Returns the generator of a EC_GROUP object.
 *  \param  group  EC_GROUP object
 *  \return the currently used generator (possibly NULL).
 */
    /* * Returns the montgomery data for order(Generator)
 *  \param  group  EC_GROUP object
 *  \return the currently used generator (possibly NULL).
*/
    /* * Gets the order of a EC_GROUP
 *  \param  group  EC_GROUP object
 *  \param  order  BIGNUM to which the order is copied
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the cofactor of a EC_GROUP
 *  \param  group     EC_GROUP object
 *  \param  cofactor  BIGNUM to which the cofactor is copied
 *  \param  ctx       BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the name of a EC_GROUP object
 *  \param  group  EC_GROUP object
 *  \param  nid    NID of the curve name OID
 */
    /* * Returns the curve name of a EC_GROUP object
 *  \param  group  EC_GROUP object
 *  \return NID of the curve name OID or 0 if not set.
 */
    /* * Sets the parameter of a ec over GFp defined by y^2 = x^3 + a*x + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM with the prime number
 *  \param  a      BIGNUM with parameter a of the equation
 *  \param  b      BIGNUM with parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the parameter of the ec over GFp defined by y^2 = x^3 + a*x + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM for the prime number
 *  \param  a      BIGNUM for parameter a of the equation
 *  \param  b      BIGNUM for parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the parameter of a ec over GF2m defined by y^2 + x*y = x^3 + a*x^2 + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM with the polynomial defining the underlying field
 *  \param  a      BIGNUM with parameter a of the equation
 *  \param  b      BIGNUM with parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the parameter of the ec over GF2m defined by y^2 + x*y = x^3 + a*x^2 + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM for the polynomial defining the underlying field
 *  \param  a      BIGNUM for parameter a of the equation
 *  \param  b      BIGNUM for parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Returns the number of bits needed to represent a field element
 *  \param  group  EC_GROUP object
 *  \return number of bits needed to represent a field element
 */
    /* * Checks whether the parameter in the EC_GROUP define a valid ec group
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if group is a valid ec group and 0 otherwise
 */
    /* * Checks whether the discriminant of the elliptic curve is zero or not
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if the discriminant is not zero and 0 otherwise
 */
    /* * Compares two EC_GROUP objects
 *  \param  a    first EC_GROUP object
 *  \param  b    second EC_GROUP object
 *  \param  ctx  BN_CTX object (optional)
 *  \return 0 if both groups are equal and 1 otherwise
 */
    /*
 * EC_GROUP_new_GF*() calls EC_GROUP_new() and EC_GROUP_set_GF*() after
 * choosing an appropriate EC_METHOD
 */
    /* * Creates a new EC_GROUP object with the specified parameters defined
 *  over GFp (defined by the equation y^2 = x^3 + a*x + b)
 *  \param  p    BIGNUM with the prime number
 *  \param  a    BIGNUM with the parameter a of the equation
 *  \param  b    BIGNUM with the parameter b of the equation
 *  \param  ctx  BN_CTX object (optional)
 *  \return newly created EC_GROUP object with the specified parameters
 */
    /* * Creates a new EC_GROUP object with the specified parameters defined
 *  over GF2m (defined by the equation y^2 + x*y = x^3 + a*x^2 + b)
 *  \param  p    BIGNUM with the polynomial defining the underlying field
 *  \param  a    BIGNUM with the parameter a of the equation
 *  \param  b    BIGNUM with the parameter b of the equation
 *  \param  ctx  BN_CTX object (optional)
 *  \return newly created EC_GROUP object with the specified parameters
 */
    /* * Creates a EC_GROUP object with a curve specified by a NID
 *  \param  nid  NID of the OID of the curve name
 *  \return newly created EC_GROUP object with specified curve or NULL
 *          if an error occurred
 */
    /* *******************************************************************/
/*               handling of internal curves                        */
/* *******************************************************************/
    /*
 * EC_builtin_curves(EC_builtin_curve *r, size_t size) returns number of all
 * available curves or zero if a error occurred. In case r ist not zero
 * nitems EC_builtin_curve structures are filled with the data of the first
 * nitems internal groups
 */
    /* *******************************************************************/
/*                    EC_POINT functions                            */
/* *******************************************************************/
    /* * Creates a new EC_POINT object for the specified EC_GROUP
 *  \param  group  EC_GROUP the underlying EC_GROUP object
 *  \return newly created EC_POINT object or NULL if an error occurred
 */
    /* * Frees a EC_POINT object
 *  \param  point  EC_POINT object to be freed
 */
    /* * Clears and frees a EC_POINT object
 *  \param  point  EC_POINT object to be cleared and freed
 */
    /* * Copies EC_POINT object
 *  \param  dst  destination EC_POINT object
 *  \param  src  source EC_POINT object
 *  \return 1 on success and 0 if an error occured
 */
    /* * Creates a new EC_POINT object and copies the content of the supplied
 *  EC_POINT
 *  \param  src    source EC_POINT object
 *  \param  group  underlying the EC_GROUP object
 *  \return newly created EC_POINT object or NULL if an error occurred
 */
    /* * Returns the EC_METHOD used in EC_POINT object
 *  \param  point  EC_POINT object
 *  \return the EC_METHOD used
 */
    /* * Sets a point to infinity (neutral element)
 *  \param  group  underlying EC_GROUP object
 *  \param  point  EC_POINT to set to infinity
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the jacobian projective coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  z      BIGNUM with the z-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the jacobian projective coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  z      BIGNUM for the z-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the affine coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the affine coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the x9.62 compressed coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with x-coordinate
 *  \param  y_bit  integer with the y-Bit (either 0 or 1)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the affine coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Gets the affine coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Sets the x9.62 compressed coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with x-coordinate
 *  \param  y_bit  integer with the y-Bit (either 0 or 1)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Encodes a EC_POINT object to a octet string
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  form   point conversion form
 *  \param  buf    memory buffer for the result. If NULL the function returns
 *                 required buffer size.
 *  \param  len    length of the memory buffer
 *  \param  ctx    BN_CTX object (optional)
 *  \return the length of the encoded octet string or 0 if an error occurred
 */
    /* * Decodes a EC_POINT from a octet string
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  buf    memory buffer with the encoded ec point
 *  \param  len    length of the encoded ec point
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* other interfaces to point2oct/oct2point: */
    /* *******************************************************************/
/*         functions for doing EC_POINT arithmetic                  */
/* *******************************************************************/
    /* * Computes the sum of two EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result (r = a + b)
 *  \param  a      EC_POINT object with the first summand
 *  \param  b      EC_POINT object with the second summand
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Computes the double of a EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result (r = 2 * a)
 *  \param  a      EC_POINT object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Computes the inverse of a EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  a      EC_POINT object to be inverted (it's used for the result as well)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Checks whether the point is the neutral element of the group
 *  \param  group  the underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \return 1 if the point is the neutral element and 0 otherwise
 */
    /* * Checks whether the point is on the curve
 *  \param  group  underlying EC_GROUP object
 *  \param  point  EC_POINT object to check
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if point if on the curve and 0 otherwise
 */
    /* * Compares two EC_POINTs
 *  \param  group  underlying EC_GROUP object
 *  \param  a      first EC_POINT object
 *  \param  b      second EC_POINT object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 0 if both points are equal and a value != 0 otherwise
 */
    /* * Computes r = generator * n sum_{i=0}^{num-1} p[i] * m[i]
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result
 *  \param  n      BIGNUM with the multiplier for the group generator (optional)
 *  \param  num    number futher summands
 *  \param  p      array of size num of EC_POINT objects
 *  \param  m      array of size num of BIGNUM objects
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Computes r = generator * n + q * m
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result
 *  \param  n      BIGNUM with the multiplier for the group generator (optional)
 *  \param  q      EC_POINT object with the first factor of the second summand
 *  \param  m      BIGNUM with the second factor of the second summand
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Stores multiples of generator for faster point multiplication
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
    /* * Reports whether a precomputation has been done
 *  \param  group  EC_GROUP object
 *  \return 1 if a pre-computation has been done and 0 otherwise
 */
    /* *******************************************************************/
/*                       ASN1 stuff                                 */
/* *******************************************************************/
    /*
 * EC_GROUP_get_basis_type() returns the NID of the basis type used to
 * represent the field elements
 */
    /* *******************************************************************/
/*                      EC_KEY functions                            */
/* *******************************************************************/
    #[src_loc = "741:1"]
    pub type EC_KEY = ec_key_st;
    use super::evp_h::ec_key_st;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the EC functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/bio.h:27"]
pub mod bio_h {
    /* crypto/bio/bio.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
    /* These are the 'types' of BIOs */
    /* passive filter */
    /* filter */
    /* filter */
    /* filter */
    /* socket - connect */
    /* socket for accept */
    /* client proxy BIO */
    /* server proxy BIO */
    /* server proxy BIO */
    /* BER -> bin filter */
    /* (half a) BIO pair */
    /* filter */
    /* filter */
    /* filter */
    /* socket, fd, connect or accept */
    /*
 * BIO_FILENAME_READ|BIO_CLOSE to open or close on free.
 * BIO_set_fp(in,stdin,BIO_NOCLOSE);
 */
    /*
 * These are used in the following macros and are passed to BIO_ctrl()
 */
    /* opt - rewind/zero etc */
    /* opt - are we at the eof */
    /* opt - extra tit-bits */
    /* man - set the 'IO' type */
    /* man - get the 'IO' type */
    /* opt - internal, used to signify change */
    /* opt - internal, used to signify change */
    /* man - set the 'close' on free */
    /* man - set the 'close' on free */
    /* opt - is their more data buffered */
    /* opt - 'flush' buffered output */
    /* man - extra stuff for 'duped' BIO */
    /* opt - number of bytes still to write */
    /* callback is int cb(BIO *bio,state,ret); */
    /* opt - set callback function */
    /* opt - set callback function */
    /* BIO_s_file special */
    /* dgram BIO stuff */
    /* BIO dgram special */
    /* allow for an externally connected
                                         * socket to be passed in */
    /* setsockopt, essentially */
    /* getsockopt, essentially */
    /* setsockopt, essentially */
    /* getsockopt, essentially */
    /* flag whether the last */
    /* I/O operation tiemd out */
    /* #ifdef IP_MTU_DISCOVER */
    /* set DF bit on egress packets */
    /* #endif */
    /* as kernel for current MTU */
    /* get cached value for MTU */
    /* set cached value for MTU.
                                              * want to use this if asking
                                              * the kernel fails */
    /* check whether the MTU was
                                              * exceed in the previous write
                                              * operation */
    /* Destination for the data */
    /* Next DTLS handshake timeout
                                              * to adjust socket timeouts */
    /* modifiers */
    /*
 * "UPLINK" flag denotes file descriptors provided by application. It
 * defaults to 0, as most platforms don't require UPLINK interface.
 */
    /* Used in BIO_gethostbyname() */
    /* Mostly used in the SSL BIO */
/*-
 * Not used anymore
 * #define BIO_FLAGS_PROTOCOL_DELAYED_READ 0x10
 * #define BIO_FLAGS_PROTOCOL_DELAYED_WRITE 0x20
 * #define BIO_FLAGS_PROTOCOL_STARTUP   0x40
 */
    /*
 * This is used with memory BIOs: it means we shouldn't free up or change the
 * data in any way.
 */
    #[src_loc = "238:1"]
    pub type BIO = bio_st;
    #[src_loc = "312:1"]
    pub type BIO_METHOD = bio_method_st;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "312:9"]
    pub struct bio_method_st {
        pub type_0: libc::c_int,
        pub name: *const libc::c_char,
        pub bwrite: Option<unsafe extern "C" fn(_: *mut BIO,
                                                _: *const libc::c_char,
                                                _: libc::c_int)
                               -> libc::c_int>,
        pub bread: Option<unsafe extern "C" fn(_: *mut BIO,
                                               _: *mut libc::c_char,
                                               _: libc::c_int)
                              -> libc::c_int>,
        pub bputs: Option<unsafe extern "C" fn(_: *mut BIO,
                                               _: *const libc::c_char)
                              -> libc::c_int>,
        pub bgets: Option<unsafe extern "C" fn(_: *mut BIO,
                                               _: *mut libc::c_char,
                                               _: libc::c_int)
                              -> libc::c_int>,
        pub ctrl: Option<unsafe extern "C" fn(_: *mut BIO, _: libc::c_int,
                                              _: libc::c_long,
                                              _: *mut libc::c_void)
                             -> libc::c_long>,
        pub create: Option<unsafe extern "C" fn(_: *mut BIO) -> libc::c_int>,
        pub destroy: Option<unsafe extern "C" fn(_: *mut BIO) -> libc::c_int>,
        pub callback_ctrl: Option<unsafe extern "C" fn(_: *mut BIO,
                                                       _: libc::c_int,
                                                       _:
                                                           Option<unsafe extern "C" fn(_:
                                                                                           *mut bio_st,
                                                                                       _:
                                                                                           libc::c_int,
                                                                                       _:
                                                                                           *const libc::c_char,
                                                                                       _:
                                                                                           libc::c_int,
                                                                                       _:
                                                                                           libc::c_long,
                                                                                       _:
                                                                                           libc::c_long)
                                                                      -> ()>)
                                      -> libc::c_long>,
    }
    #[src_loc = "309:1"]
    pub type bio_info_cb
        =
        unsafe extern "C" fn(_: *mut bio_st, _: libc::c_int,
                             _: *const libc::c_char, _: libc::c_int,
                             _: libc::c_long, _: libc::c_long) -> ();
    use super::crypto_h::bio_st;
    /* Reason codes. */
    /* Function codes. */
    /* Error codes for the BIO functions. */
}
#[header_src = "/usr/local/Cellar/openssl/1.0.2t/include/openssl/ssl2.h:27"]
pub mod ssl2_h {
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "203:5"]
    pub struct C2RustUnnamed_10 {
        pub conn_id_length: libc::c_uint,
        pub cert_type: libc::c_uint,
        pub cert_length: libc::c_uint,
        pub csl: libc::c_uint,
        pub clear: libc::c_uint,
        pub enc: libc::c_uint,
        pub ccl: [libc::c_uchar; 32],
        pub cipher_spec_length: libc::c_uint,
        pub session_id_length: libc::c_uint,
        pub clen: libc::c_uint,
        pub rlen: libc::c_uint,
    }
    /* server */
    /* SSLv2 */
/* client */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zlib.h:27"]
pub mod zlib_h {
    /* zlib.h -- interface of the 'zlib' general purpose compression library
  version 1.2.11, January 15th, 2017

  Copyright (C) 1995-2017 Jean-loup Gailly and Mark Adler

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

  Jean-loup Gailly        Mark Adler
  jloup@gzip.org          madler@alumni.caltech.edu


  The data format used by the zlib library is described by RFCs (Request for
  Comments) 1950 to 1952 in the files http://tools.ietf.org/html/rfc1950
  (zlib format), rfc1951 (deflate format) and rfc1952 (gzip format).
*/
    /* !__APPLE__ */
    /* !__APPLE__ */
    /*
    The 'zlib' compression library provides in-memory compression and
  decompression functions, including integrity checks of the uncompressed data.
  This version of the library supports only one compression method (deflation)
  but other algorithms will be added later and will have the same stream
  interface.

    Compression can be done in a single step if the buffers are large enough,
  or can be done by repeated calls of the compression function.  In the latter
  case, the application must provide more input and/or consume the output
  (providing more output space) before each call.

    The compressed data format used by default by the in-memory functions is
  the zlib format, which is a zlib wrapper documented in RFC 1950, wrapped
  around a deflate stream, which is itself documented in RFC 1951.

    The library also supports reading and writing files in gzip (.gz) format
  with an interface similar to that of stdio using the functions that start
  with "gz".  The gzip format is different from the zlib format.  gzip is a
  gzip wrapper, documented in RFC 1952, wrapped around a deflate stream.

    This library can optionally read and write gzip and raw deflate streams in
  memory as well.

    The zlib format was designed to be compact and fast for use in memory
  and on communications channels.  The gzip format was designed for single-
  file compression on file systems, has a larger header than zlib to maintain
  directory information, and uses a different, slower check method than zlib.

    The library does not install any signal handler.  The decoder checks
  the consistency of the compressed data, so the library should never crash
  even in the case of corrupted input.
*/
    #[src_loc = "91:1"]
    pub type z_stream = z_stream_s;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "91:9"]
    pub struct z_stream_s {
        pub next_in: *mut Bytef,
        pub avail_in: uInt,
        pub total_in: uLong,
        pub next_out: *mut Bytef,
        pub avail_out: uInt,
        pub total_out: uLong,
        pub msg: *mut libc::c_char,
        pub state: *mut internal_state,
        pub zalloc: alloc_func,
        pub zfree: free_func,
        pub opaque: voidpf,
        pub data_type: libc::c_int,
        pub adler: uLong,
        pub reserved: uLong,
    }
    #[src_loc = "87:1"]
    pub type free_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
    #[src_loc = "86:1"]
    pub type alloc_func
        =
        Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
    use super::zconf_h::{Bytef, uInt, uLong, voidpf};
    extern "C" {
        #[src_loc = "89:1"]
        pub type internal_state;
    }
    /* next input byte */
    /* number of bytes available at next_in */
    /* total number of input bytes read so far */
    /* next output byte will go here */
    /* remaining free space at next_out */
    /* total number of bytes output so far */
    /* last error message, NULL if no error */
    /* not visible by applications */
    /* used to allocate the internal state */
    /* used to free the internal state */
    /* private data object passed to zalloc and zfree */
    /* best guess about the data type: binary or text
                           for deflate, or the decoding state for inflate */
    /* Adler-32 or CRC-32 value of the uncompressed data */
    /* reserved for future use */
    /* ZLIB_H */
    /* !__APPLE__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/zconf.h:27"]
pub mod zconf_h {
    /* zconf.h -- configuration of the zlib compression library
 * Copyright (C) 1995-2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
    /* @(#) $Id$ */
    /*
 * If you *really* need a unique prefix for all types and library functions,
 * compile with -DZ_PREFIX. The "standard" zlib should be compiled without it.
 * Even better than compiling with -DZ_PREFIX would be to use configure to set
 * this permanently in zconf.h using "./configure --zprefix".
 */
    /* may be set to #if 1 by ./configure */
    /*
 * Compile with -DMAXSEG_64K if the alloc function cannot allocate more
 * than 64k bytes at a time (needed on systems with 16-bit int).
 */
    /* iSeries (formerly AS/400). */
    /* Maximum value for memLevel in deflateInit2 */
    /* Maximum value for windowBits in deflateInit2 and inflateInit2.
 * WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
 * created by gzip. (Files created by minigzip can still be extracted by
 * gzip.)
 */
    /* 32K LZ77 window */
    /* The memory requirements for deflate are (in bytes):
            (1 << (windowBits+2)) +  (1 << (memLevel+9))
 that is: 128K for windowBits=15  +  128K for memLevel = 8  (default values)
 plus a few kilobytes for small objects. For example, if you want to reduce
 the default memory requirements from 256K to 128K, compile with
     make CFLAGS="-O -DMAX_WBITS=14 -DMAX_MEM_LEVEL=7"
 Of course this will generally degrade compression (there's no free lunch).

   The memory requirements for inflate are (in bytes) 1 << windowBits
 that is, 32K for windowBits=15 (default value) plus about 7 kilobytes
 for small objects.
*/
    /* Type declarations */
    /* function prototypes */
    /* function prototypes for stdarg */
    /* The following definitions for FAR are needed only for MSDOS mixed
 * model programming (small or medium model with some far allocations).
 * This was tested only with MSC; for other MSDOS compilers you may have
 * to define NO_MEMCPY in zutil.h.  If you don't need the mixed model,
 * just define FAR to be empty.
 */
    /* 8 bits */
    /* 16 bits or more */
    #[src_loc = "394:1"]
    pub type uLong = libc::c_ulong;
    /* 32 bits or more */
    #[src_loc = "409:4"]
    pub type voidpf = *mut libc::c_void;
    #[src_loc = "393:1"]
    pub type uInt = libc::c_uint;
    #[src_loc = "400:4"]
    pub type Bytef = Byte;
    #[src_loc = "391:1"]
    pub type Byte = libc::c_uchar;
    /* ZCONF_H */
    /* MVS linker does not support external names larger than 8 bytes */
    /* for SEEK_*, off_t, and _LFS64_LARGEFILE */
    /* a little trick to accommodate both "#define _LARGEFILE64_SOURCE" and
 * "#define _LARGEFILE64_SOURCE 1" as requesting 64-bit operations, (even
 * though the former does not conform to the LFS document), but considering
 * both "#undef _LARGEFILE64_SOURCE" and "#define _LARGEFILE64_SOURCE 0" as
 * equivalently requesting no 64-bit operations
 */
    /* for va_list */
    /* for off_t */
    /* was set to #if 1 by ./configure */
    /* defined(__APPLE__) */
    /* was set to #if 1 by ./configure */
    /* avoid unistd.h on Win32 */
}
#[header_src = "/usr/local/Cellar/stoken/0.92/include/stoken.h:27"]
pub mod stoken_h {
    extern "C" {
        /*
 * stoken.h - public libstoken library interface
 *
 * Copyright 2012 Kevin Cernekee <cernekee@gmail.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
        /* Before API version 1.3 (stoken 0.8) this macro didn't exist.
 * Somewhat ironic, that the API version check itself needs to be
 * conditionally used depending on the API version. A very simple way
 * for users to handle this with an approximately correct answer is
 *   #include <stoken.h>
 *   #ifndef STOKEN_CHECK_VER
 *   #define STOKEN_CHECK_VER(x,y) 0
 *   #endif
 */
        #[src_loc = "59:1"]
        pub type stoken_ctx;
    }
    /* !__STOKEN_H__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/gssapi/gssapi.h:27"]
pub mod gssapi_h {
    #[src_loc = "108:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[src_loc = "102:1"]
    pub type gss_name_t = *mut gss_name_struct;
    extern "C" {
        #[src_loc = "107:1"]
        pub type gss_ctx_id_struct;
        #[src_loc = "101:1"]
        pub type gss_name_struct;
    }
    /* __GSSAPI__ */
    /* _GSSAPI_H_ */
    /* XXXX This is a necessary evil until the spec is fixed */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* output_name */
}
#[header_src = "/usr/local/Cellar/libproxy/0.4.15_1/include/proxy.h:27"]
pub mod proxy_h {
    /* ******************************************************************************
 * libproxy - A library for proxy configuration
 * Copyright (C) 2006 Nathaniel McCallum <nathaniel@natemccallum.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301 USA
 ******************************************************************************/
    #[src_loc = "28:1"]
    pub type pxProxyFactory = pxProxyFactory_;
    extern "C" {
        #[src_loc = "28:9"]
        pub type pxProxyFactory_;
    }
    /*PROXY_H_*/
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/tree.h:27"]
pub mod tree_h {
    /*
 * Summary: interfaces for tree manipulation
 * Description: this module describes the structures found in an tree resulting
 *              from an XML or HTML parsing, as well as the API provided for
 *              various processing on that tree
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /*
 * Some of the basic types pointer to structures:
 */
/* xmlIO.h */
    /* parser.h */
    /* entities.h */
    /* *
 * BASE_BUFFER_SIZE:
 *
 * default buffer size 4000.
 */
    /* *
 * LIBXML_NAMESPACE_DICT:
 *
 * Defines experimental behaviour:
 * 1) xmlNs gets an additional field @context (a xmlDoc)
 * 2) when creating a tree, xmlNs->href is stored in the dict of xmlDoc.
 */
/* #define LIBXML_NAMESPACE_DICT */
    /* *
 * xmlBufferAllocationScheme:
 *
 * A buffer allocation scheme can be defined to either match exactly the
 * need or double it's allocated size each time it is found too small.
 */
    /* double each time one need to grow */
    /* grow only to the minimal size */
    /* immutable buffer */
    /* special allocation scheme used for I/O */
    /* exact up to a threshold, and doubleit thereafter */
    /* limit the upper size of the buffer */
    /* *
 * xmlBuffer:
 *
 * A buffer structure, this old construct is limited to 2GB and
 * is being deprecated, use API with xmlBuf instead
 */
    /* The buffer content UTF8 */
    /* The buffer size used */
    /* The buffer size */
    /* The realloc method */
    /* in IO mode we may have a different base */
    /* *
 * xmlBuf:
 *
 * A buffer structure, new one, the actual structure internals are not public
 */
    /* *
 * xmlBufPtr:
 *
 * A pointer to a buffer structure, the actual structure internals are not
 * public
 */
    /*
 * A few public routines for xmlBuf. As those are expected to be used
 * mostly internally the bulk of the routines are internal in buf.h
 */
    /*
 * LIBXML2_NEW_BUFFER:
 *
 * Macro used to express that the API use the new buffers for
 * xmlParserInputBuffer and xmlOutputBuffer. The change was
 * introduced in 2.9.0.
 */
    /* *
 * XML_XML_NAMESPACE:
 *
 * This is the namespace for the special xml: prefix predefined in the
 * XML Namespace specification.
 */
    /* *
 * XML_XML_ID:
 *
 * This is the name for the special xml:id attribute
 */
    /*
 * The different element types carried by an XML tree.
 *
 * NOTE: This is synchronized with DOM Level1 values
 *       See http://www.w3.org/TR/REC-DOM-Level-1/
 *
 * Actually this had diverged a bit, and now XML_DOCUMENT_TYPE_NODE should
 * be deprecated to use an XML_DTD_NODE.
 */
    /* *
 * xmlNotation:
 *
 * A DTD Notation definition.
 */
    /* Notation name */
    /* Public identifier, if any */
    /* System identifier, if any */
    /* *
 * xmlAttributeType:
 *
 * A DTD Attribute type definition.
 */
    /* *
 * xmlAttributeDefault:
 *
 * A DTD Attribute default definition.
 */
    /* *
 * xmlEnumeration:
 *
 * List structure used when there is an enumeration in DTDs.
 */
    /* next one */
    /* Enumeration name */
    /* *
 * xmlAttribute:
 *
 * An Attribute declaration in a DTD.
 */
    /* application data */
    /* XML_ATTRIBUTE_DECL, must be second ! */
    /* Attribute name */
    /* NULL */
    /* NULL */
    /* -> DTD */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* next in hash table */
    /* The attribute type */
    /* the default */
    /* or the default value */
    /* or the enumeration tree if any */
    /* the namespace prefix if any */
    /* Element holding the attribute */
    /* *
 * xmlElementContentType:
 *
 * Possible definitions of element content types.
 */
    /* *
 * xmlElementContentOccur:
 *
 * Possible definitions of element content occurrences.
 */
    /* *
 * xmlElementContent:
 *
 * An XML Element content as stored after parsing an element definition
 * in a DTD.
 */
    /* PCDATA, ELEMENT, SEQ or OR */
    /* ONCE, OPT, MULT or PLUS */
    /* Element name */
    /* first child */
    /* second child */
    /* parent */
    /* Namespace prefix */
    /* *
 * xmlElementTypeVal:
 *
 * The different possibilities for an element content type.
 */
    /* *
 * xmlElement:
 *
 * An XML Element declaration from a DTD.
 */
    /* application data */
    /* XML_ELEMENT_DECL, must be second ! */
    /* Element name */
    /* NULL */
    /* NULL */
    /* -> DTD */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* The type */
    /* the allowed element content */
    /* List of the declared attributes */
    /* the namespace prefix if any */
    /* the validating regexp */
    /* *
 * XML_LOCAL_NAMESPACE:
 *
 * A namespace declaration node.
 */
    /* *
 * xmlNs:
 *
 * An XML namespace.
 * Note that prefix == NULL is valid, it defines the default namespace
 * within the subtree (until overridden).
 *
 * xmlNsType is unified with xmlElementType.
 */
    /* next Ns link for this node  */
    /* global or local */
    /* URL for the namespace */
    /* prefix for the namespace */
    /* application data */
    /* normally an xmlDoc */
    /* *
 * xmlDtd:
 *
 * An XML DTD, as defined by <!DOCTYPE ... There is actually one for
 * the internal subset and for the external subset.
 */
    /* application data */
    /* XML_DTD_NODE, must be second ! */
    /* Name of the DTD */
    /* the value of the property link */
    /* last child link */
    /* child->parent link */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* End of common part */
    /* Hash table for notations if any */
    /* Hash table for elements if any */
    /* Hash table for attributes if any */
    /* Hash table for entities if any */
    /* External identifier for PUBLIC DTD */
    /* URI for a SYSTEM or PUBLIC DTD */
    /* Hash table for param entities if any */
    /* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
    /* application data */
    /* XML_ATTRIBUTE_NODE, must be second ! */
    /* the name of the property */
    /* the value of the property */
    /* NULL */
    /* child->parent link */
    /* next sibling link  */
    /* previous sibling link  */
    /* the containing document */
    /* pointer to the associated namespace */
    /* the attribute type if validating */
    /* for type/PSVI informations */
    /* *
 * xmlID:
 *
 * An XML ID instance.
 */
    /* next ID */
    /* The ID name */
    /* The attribute holding it */
    /* The attribute if attr is not available */
    /* The line number if attr is not available */
    /* The document holding the ID */
    /* *
 * xmlRef:
 *
 * An XML IDREF instance.
 */
    /* next Ref */
    /* The Ref name */
    /* The attribute holding it */
    /* The attribute if attr is not available */
    /* The line number if attr is not available */
    /* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
    #[src_loc = "487:1"]
    pub type xmlNode = _xmlNode;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "257:5"]
    pub struct _xmlNode {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub ns: *mut xmlNs,
        pub content: *mut xmlChar,
        pub properties: *mut _xmlAttr,
        pub nsDef: *mut xmlNs,
        pub psvi: *mut libc::c_void,
        pub line: libc::c_ushort,
        pub extra: libc::c_ushort,
    }
    #[src_loc = "387:1"]
    pub type xmlNs = _xmlNs;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "387:9"]
    pub struct _xmlNs {
        pub next: *mut _xmlNs,
        pub type_0: xmlNsType,
        pub href: *const xmlChar,
        pub prefix: *const xmlChar,
        pub _private: *mut libc::c_void,
        pub context: *mut _xmlDoc,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "262:5"]
    pub struct _xmlDoc {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *mut libc::c_char,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub compression: libc::c_int,
        pub standalone: libc::c_int,
        pub intSubset: *mut _xmlDtd,
        pub extSubset: *mut _xmlDtd,
        pub oldNs: *mut _xmlNs,
        pub version: *const xmlChar,
        pub encoding: *const xmlChar,
        pub ids: *mut libc::c_void,
        pub refs: *mut libc::c_void,
        pub URL: *const xmlChar,
        pub charset: libc::c_int,
        pub dict: *mut _xmlDict,
        pub psvi: *mut libc::c_void,
        pub parseFlags: libc::c_int,
        pub properties: libc::c_int,
    }
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "259:5"]
    pub struct _xmlDtd {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlDoc,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub notations: *mut libc::c_void,
        pub elements: *mut libc::c_void,
        pub attributes: *mut libc::c_void,
        pub entities: *mut libc::c_void,
        pub ExternalID: *const xmlChar,
        pub SystemID: *const xmlChar,
        pub pentities: *mut libc::c_void,
    }
    #[src_loc = "159:9"]
    pub type xmlElementType = libc::c_uint;
    #[src_loc = "181:5"]
    pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
    #[src_loc = "179:5"]
    pub const XML_XINCLUDE_END: xmlElementType = 20;
    #[src_loc = "178:5"]
    pub const XML_XINCLUDE_START: xmlElementType = 19;
    #[src_loc = "177:5"]
    pub const XML_NAMESPACE_DECL: xmlElementType = 18;
    #[src_loc = "176:5"]
    pub const XML_ENTITY_DECL: xmlElementType = 17;
    #[src_loc = "175:5"]
    pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
    #[src_loc = "174:5"]
    pub const XML_ELEMENT_DECL: xmlElementType = 15;
    #[src_loc = "173:5"]
    pub const XML_DTD_NODE: xmlElementType = 14;
    #[src_loc = "172:5"]
    pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
    #[src_loc = "171:5"]
    pub const XML_NOTATION_NODE: xmlElementType = 12;
    #[src_loc = "170:5"]
    pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
    #[src_loc = "169:5"]
    pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
    #[src_loc = "168:5"]
    pub const XML_DOCUMENT_NODE: xmlElementType = 9;
    #[src_loc = "167:5"]
    pub const XML_COMMENT_NODE: xmlElementType = 8;
    #[src_loc = "166:5"]
    pub const XML_PI_NODE: xmlElementType = 7;
    #[src_loc = "165:5"]
    pub const XML_ENTITY_NODE: xmlElementType = 6;
    #[src_loc = "164:5"]
    pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
    #[src_loc = "163:5"]
    pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
    #[src_loc = "162:5"]
    pub const XML_TEXT_NODE: xmlElementType = 3;
    #[src_loc = "161:5"]
    pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
    #[src_loc = "160:5"]
    pub const XML_ELEMENT_NODE: xmlElementType = 1;
    #[src_loc = "375:1"]
    pub type xmlNsType = xmlElementType;
    #[derive ( Copy, Clone )]
    #[repr(C)]
    #[src_loc = "432:9"]
    pub struct _xmlAttr {
        pub _private: *mut libc::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlAttr,
        pub prev: *mut _xmlAttr,
        pub doc: *mut _xmlDoc,
        pub ns: *mut xmlNs,
        pub atype: xmlAttributeType,
        pub psvi: *mut libc::c_void,
    }
    #[src_loc = "206:9"]
    pub type xmlAttributeType = libc::c_uint;
    #[src_loc = "216:5"]
    pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
    #[src_loc = "215:5"]
    pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
    #[src_loc = "214:5"]
    pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
    #[src_loc = "213:5"]
    pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
    #[src_loc = "212:5"]
    pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
    #[src_loc = "211:5"]
    pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
    #[src_loc = "210:5"]
    pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
    #[src_loc = "209:5"]
    pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
    #[src_loc = "208:5"]
    pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
    #[src_loc = "207:5"]
    pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
    use super::xmlstring_h::xmlChar;
    use super::dict_h::_xmlDict;
    /* __XML_TREE_H__ */
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/dict.h:27"]
pub mod dict_h {
    extern "C" {
        /*
 * Summary: string dictionary
 * Description: dictionary of reusable strings, just used to avoid allocation
 *         and freeing operations.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
        /*
 * The dictionary.
 */
        #[src_loc = "23:23"]
        pub type _xmlDict;
    }
    /* ! __XML_DICT_H__ */
}
#[header_src =
  "/usr/local/Cellar/libxml2/2.9.9_2/include/libxml2/libxml/xmlstring.h:27"]
pub mod xmlstring_h {
    /*
 * Summary: set of routines to process strings
 * Description: type and interfaces needed for the internal string handling
 *              of the library, especially UTF8 processing.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * xmlChar:
 *
 * This is a basic byte in an UTF-8 encoded string.
 * It's unsigned allowing to pinpoint case where char * are assigned
 * to xmlChar * (possibly making serialization back impossible).
 */
    #[src_loc = "28:1"]
    pub type xmlChar = libc::c_uchar;
    /* __XML_STRING_H__ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/iconv.h:27"]
pub mod iconv_h {
    /* Copyright (C) 1999-2003, 2005-2006 Free Software Foundation, Inc.
   This file is part of the GNU LIBICONV Library.

   The GNU LIBICONV Library is free software; you can redistribute it
   and/or modify it under the terms of the GNU Library General Public
   License as published by the Free Software Foundation; either version 2
   of the License, or (at your option) any later version.

   The GNU LIBICONV Library is distributed in the hope that it will be
   useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.

   You should have received a copy of the GNU Library General Public
   License along with the GNU LIBICONV Library; see the file COPYING.LIB.
   If not, write to the Free Software Foundation, Inc., 51 Franklin Street,
   Fifth Floor, Boston, MA 02110-1301, USA.  */
    /* When installed, this file is called "iconv.h". */
    /* version number: (major<<8) + minor */
    /* Likewise */
    /* We would like to #include any system header file which could define
   iconv_t, 1. in order to eliminate the risk that the user gets compilation
   errors because some other system header file includes /usr/include/iconv.h
   which defines iconv_t or declares iconv after this file, 2. when compiling
   for LIBICONV_PLUG, we need the proper iconv_t type in order to produce
   binary compatible code.
   But gcc's #include_next is not portable. Thus, once libiconv's iconv.h
   has been installed in /usr/local/include, there is no way any more to
   include the original /usr/include/iconv.h. We simply have to get away
   without it.
   Ad 1. The risk that a system header file does
   #include "iconv.h"  or  #include_next "iconv.h"
   is small. They all do #include <iconv.h>.
   Ad 2. The iconv_t type is a pointer type in all cases I have seen. (It
   has to be a scalar type because (iconv_t)(-1) is a possible return value
   from iconv_open().) */
    /* Define iconv_t ourselves. */
    #[src_loc = "57:1"]
    pub type iconv_t = *mut libc::c_void;
    /* _LIBICONV_H */
    /* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h:20"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[src_loc = "180:6"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
    /* _STDIO_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:22"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[src_loc = "437:1"]
        pub fn close(_: libc::c_int) -> libc::c_int;
    }
    /* _UNISTD_H_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/string.h:23"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[src_loc = "72:7"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "81:7"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
    /* _STRING_H_ */
    /* _USE_EXTENDED_LOCALES_ */
    /* __DARWIN_C_LEVEL >= __DARWIN_C_FULL */
    /* Some functions historically defined in string.h were placed in strings.h
 * by SUS.  We are using "strings.h" instead of <strings.h> to avoid an issue
 * where /Developer/Headers/FlatCarbon/Strings.h could be included instead on
 * case-insensitive file systems.
 */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/libkern/i386/_OSByteOrder.h:24"]
pub mod _OSByteOrder_h {
    #[inline]
    #[src_loc = "53:1"]
    pub unsafe extern "C" fn _OSSwapInt32(mut _data: __uint32_t)
     -> __uint32_t {
        return _data.swap_bytes();
    }
    use super::_types_h::__uint32_t;
    /* ! _OS__OSBYTEORDERI386_H */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:24"]
pub mod _malloc_h {
    extern "C" {
        /*
 * Copyright (c) 2018 Apple Computer, Inc. All rights reserved.
 *
 * @APPLE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this
 * file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_LICENSE_HEADER_END@
 */
        /*
 * This header is included from <stdlib.h>, so the contents of this file have
 * broad source compatibility and POSIX conformance implications.
 * Be cautious about what is included and declared here.
 */
        #[no_mangle]
        #[src_loc = "40:7"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[src_loc = "42:1"]
        pub fn free(_: *mut libc::c_void);
    }
    /* _MALLOC_UNDERSCORE_MALLOC_H_ */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:25"]
pub mod errno_h {
    extern "C" {
        /*
 * Copyright (c) 2000-2012 Apple, Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* Copyright (c) 1995 NeXT Computer, Inc. All Rights Reserved */
/*
 * Copyright (c) 1982, 1986, 1989, 1993
 *	The Regents of the University of California.  All rights reserved.
 * (c) UNIX System Laboratories, Inc.
 * All or some portions of this file are derived from material licensed
 * to the University of California by American Telephone and Telegraph
 * Co. or Unix System Laboratories, Inc. and are reproduced herein with
 * the permission of UNIX System Laboratories, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)errno.h	8.5 (Berkeley) 1/21/94
 */
        #[no_mangle]
        #[src_loc = "80:1"]
        pub fn __error() -> *mut libc::c_int;
    }
    /* _SYS_ERRNO_H_ */
    /* Must be equal largest errno */
    /* Interface output queue is full */
    /* Previous owner died */
    /* State not recoverable */
    /* No such policy registered */
    /* __DARWIN_UNIX03 || KERNEL */
    /* Operation not supported on socket */
}
#[header_src =
  "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/time.h:27"]
pub mod time_h {
    use super::_time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[src_loc = "118:1"]
        pub fn time(_: *mut time_t) -> time_t;
    }
    /* !_TIME_H_ */
    /* _USE_EXTENDED_LOCALES_ */
}
#[header_src = "/usr/local/Cellar/gettext/0.20.1/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        /* Look up MSGID in the DOMAINNAME message catalog for the current
   LC_MESSAGES locale.  */
        #[no_mangle]
        #[src_loc = "156:1"]
        pub fn libintl_dgettext(__domainname: *const libc::c_char,
                                __msgid: *const libc::c_char)
         -> *mut libc::c_char;
    }
    /* libintl.h */
}
#[header_src = "/Users/cameron/github/openconnect/lzo.h:28"]
pub mod lzo_h {
    extern "C" {
        /*
 * LZO 1x decompression
 * copyright (c) 2006 Reimar Doeffinger
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
        /* *
 * @defgroup lavu_lzo LZO
 * @ingroup lavu_crypto
 *
 * @{
 */
        /* * @name Error flags returned by av_lzo1x_decode
 * @{ */
// / end of the input buffer reached before decoding finished
        // / decoded data did not fit into output buffer
        // / a reference to previously decoded data was wrong
        // / a non-specific error in the compressed bitstream
        /* * @} */
        /* *
 * @brief Decodes LZO 1x compressed data.
 * @param out output buffer
 * @param outlen size of output buffer, number of bytes left are returned here
 * @param in input buffer
 * @param inlen size of input buffer, number of bytes left are returned here
 * @return 0 on success, otherwise a combination of the error flags above
 *
 * Make sure all buffers are appropriately padded, in must provide
 * AV_LZO_INPUT_PADDING, out must provide AV_LZO_OUTPUT_PADDING additional bytes.
 */
        #[no_mangle]
        #[src_loc = "60:1"]
        pub fn av_lzo1x_decode(out: *mut libc::c_void,
                               outlen: *mut libc::c_int,
                               in_0: *const libc::c_void,
                               inlen: *mut libc::c_int) -> libc::c_int;
    }
    /* AVUTIL_LZO_H */
}
pub use self::_types_h::{__uint8_t, __int32_t, __uint32_t, __darwin_size_t,
                         __darwin_socklen_t, __darwin_ssize_t,
                         __darwin_time_t};
pub use self::sys__types_h::{__darwin_gid_t, __darwin_suseconds_t,
                             __darwin_uid_t};
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uid_t_h::uid_t;
pub use self::_gid_t_h::gid_t;
pub use self::_fd_def_h::fd_set;
pub use self::_timeval_h::timeval;
pub use self::_time_t_h::time_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::socket_h::{sockaddr, send, recv};
pub use self::netdb_h::addrinfo;
pub use self::openconnect_h::{oc_form_opt, oc_choice, oc_form_opt_select,
                              oc_auth_form, oc_split_include, oc_ip_info,
                              oc_vpn_option, oc_stats, openconnect_info,
                              openconnect_reconnected_vfn,
                              openconnect_setup_tun_vfn,
                              openconnect_getaddrinfo_vfn,
                              openconnect_protect_socket_vfn,
                              openconnect_progress_vfn,
                              openconnect_process_auth_form_vfn,
                              openconnect_write_new_config_vfn,
                              openconnect_validate_peer_cert_vfn,
                              openconnect_stats_vfn,
                              openconnect_unlock_token_vfn,
                              openconnect_lock_token_vfn};
pub use self::openconnect_internal_h::{pkt_q, pkt, C2RustUnnamed,
                                       C2RustUnnamed_0, C2RustUnnamed_1,
                                       C2RustUnnamed_2, C2RustUnnamed_3,
                                       C2RustUnnamed_4, keepalive_info,
                                       pin_cache, oc_text_buf,
                                       C2RustUnnamed_11, HOTP_SECRET_PSKC,
                                       HOTP_SECRET_HEX, HOTP_SECRET_RAW,
                                       HOTP_SECRET_BASE32, C2RustUnnamed_12,
                                       OATH_ALG_HMAC_SHA512,
                                       OATH_ALG_HMAC_SHA256,
                                       OATH_ALG_HMAC_SHA1, http_auth_state,
                                       C2RustUnnamed_13, C2RustUnnamed_14,
                                       C2RustUnnamed_15, esp, vpn_proto,
                                       oc_packed_uint32_t, dequeue_packet,
                                       queue_packet, store_be32, oc_pcsc_ctx,
                                       oncp_esp_send_probes,
                                       encrypt_esp_packet, keepalive_action,
                                       decrypt_esp_packet, ka_check_deadline,
                                       destroy_esp_ciphers, init_esp_ciphers,
                                       openconnect_random};
pub use self::ossl_typ_h::{SSL, ssl_st, BIGNUM, bignum_st, SSL_CTX,
                           ssl_ctx_st, EVP_MD_CTX, env_md_ctx_st,
                           EVP_PKEY_CTX, ENGINE, EVP_MD, env_md_st,
                           EVP_CIPHER_CTX, evp_cipher_ctx_st, EVP_CIPHER,
                           evp_cipher_st, ASN1_STRING, asn1_string_st,
                           ASN1_UTF8STRING, ASN1_VISIBLESTRING,
                           ASN1_GENERALIZEDTIME, ASN1_UTCTIME,
                           ASN1_UNIVERSALSTRING, ASN1_BMPSTRING,
                           ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_T61STRING,
                           ASN1_PRINTABLESTRING, ASN1_OCTET_STRING,
                           ASN1_BIT_STRING, ASN1_ENUMERATED, ASN1_INTEGER,
                           ASN1_OBJECT, asn1_object_st, ASN1_BOOLEAN,
                           X509_STORE_CTX, x509_store_ctx_st, CRYPTO_EX_DATA,
                           crypto_ex_data_st, X509_CRL, X509_crl_st,
                           X509_CRL_METHOD, ISSUING_DIST_POINT,
                           AUTHORITY_KEYID, X509_ALGOR, X509_algor_st,
                           ASN1_TIME, X509_NAME, X509_name_st, BUF_MEM,
                           buf_mem_st, X509, x509_st, NAME_CONSTRAINTS,
                           X509_POLICY_CACHE, X509_PUBKEY, X509_pubkey_st,
                           EVP_PKEY, evp_pkey_st, dh_st, DH_METHOD, dh_method,
                           BN_GENCB, bn_gencb_st, DH, BN_MONT_CTX,
                           bn_mont_ctx_st, BN_CTX, dsa_st, DSA_METHOD,
                           dsa_method, DSA, rsa_st, BN_BLINDING, RSA_METHOD,
                           rsa_meth_st, RSA, EVP_PKEY_ASN1_METHOD,
                           X509_POLICY_TREE, X509_STORE, x509_store_st,
                           COMP_METHOD, comp_method_st, evp_pkey_ctx_st,
                           engine_st, x509_crl_method_st,
                           ISSUING_DIST_POINT_st, AUTHORITY_KEYID_st,
                           NAME_CONSTRAINTS_st, X509_POLICY_CACHE_st,
                           bignum_ctx, bn_blinding_st,
                           evp_pkey_asn1_method_st, X509_POLICY_TREE_st};
pub use self::ssl_h::{SRP_CTX, srp_ctx_st, SRTP_PROTECTION_PROFILE,
                      srtp_protection_profile_st,
                      stack_st_SRTP_PROTECTION_PROFILE, GEN_SESSION_CB,
                      stack_st_SSL_COMP, C2RustUnnamed_8, SSL_SESSION,
                      ssl_session_st, stack_st_SSL_CIPHER, SSL_CIPHER,
                      ssl_cipher_st, lhash_st_SSL_SESSION, SSL_METHOD,
                      ssl_method_st, tls_session_secret_cb_fn,
                      tls_session_ticket_ext_cb_fn, TLS_SESSION_TICKET_EXT,
                      tls_session_ticket_ext_st, dtls1_state_st,
                      ssl3_state_st, SSL_COMP, ssl_comp_st, ssl2_state_st,
                      ssl3_buf_freelist_st, cert_st, sess_cert_st,
                      ssl3_enc_method, stack_st_OCSP_RESPID};
pub use self::stack_h::{_STACK, stack_st};
pub use self::hmac_h::{HMAC_CTX, hmac_ctx_st};
pub use self::asn1_h::{ASN1_TYPE, asn1_type_st, C2RustUnnamed_5, ASN1_VALUE,
                       stack_st_ASN1_OBJECT, ASN1_ENCODING, ASN1_ENCODING_st,
                       stack_st_X509_ALGOR, ASN1_VALUE_st};
pub use self::x509_vfy_h::{X509_VERIFY_PARAM, X509_VERIFY_PARAM_st,
                           X509_VERIFY_PARAM_ID, stack_st_X509_LOOKUP,
                           stack_st_X509_OBJECT, X509_VERIFY_PARAM_ID_st};
pub use self::crypto_h::{stack_st_void, bio_st};
pub use self::x509_h::{X509_CRL_INFO, X509_crl_info_st,
                       stack_st_X509_EXTENSION, stack_st_X509_REVOKED,
                       stack_st_X509_NAME_ENTRY, X509_CERT_AUX,
                       x509_cert_aux_st, X509_CINF, x509_cinf_st, X509_VAL,
                       X509_val_st, stack_st_X509, stack_st_X509_CRL,
                       stack_st_X509_NAME, X509_EXTENSIONS,
                       stack_st_GENERAL_NAMES, stack_st_GENERAL_NAME,
                       stack_st_DIST_POINT};
pub use self::evp_h::{stack_st_X509_ATTRIBUTE, C2RustUnnamed_6, ec_key_st};
pub use self::bn_h::C2RustUnnamed_7;
pub use self::dsa_h::{DSA_SIG, DSA_SIG_st};
pub use self::pem_h::pem_password_cb;
pub use self::comp_h::{COMP_CTX, comp_ctx_st};
pub use self::dtls1_h::{dtls1_timeout_st, hm_header_st,
                        dtls1_retransmit_state, record_pqueue,
                        record_pqueue_st, DTLS1_BITMAP, dtls1_bitmap_st};
pub use self::pqueue_h::{pqueue, _pqueue};
pub use self::ssl3_h::{C2RustUnnamed_9, SSL3_RECORD, ssl3_record_st,
                       SSL3_BUFFER, ssl3_buffer_st};
pub use self::ec_h::EC_KEY;
pub use self::bio_h::{BIO, BIO_METHOD, bio_method_st, bio_info_cb};
pub use self::ssl2_h::C2RustUnnamed_10;
pub use self::zlib_h::{z_stream, z_stream_s, free_func, alloc_func,
                       internal_state};
pub use self::zconf_h::{uLong, voidpf, uInt, Bytef, Byte};
use self::stoken_h::stoken_ctx;
pub use self::gssapi_h::{gss_ctx_id_t, gss_name_t, gss_ctx_id_struct,
                         gss_name_struct};
pub use self::proxy_h::{pxProxyFactory, pxProxyFactory_};
pub use self::tree_h::{xmlNode, _xmlNode, xmlNs, _xmlNs, _xmlDoc, _xmlDtd,
                       xmlElementType, XML_DOCB_DOCUMENT_NODE,
                       XML_XINCLUDE_END, XML_XINCLUDE_START,
                       XML_NAMESPACE_DECL, XML_ENTITY_DECL,
                       XML_ATTRIBUTE_DECL, XML_ELEMENT_DECL, XML_DTD_NODE,
                       XML_HTML_DOCUMENT_NODE, XML_NOTATION_NODE,
                       XML_DOCUMENT_FRAG_NODE, XML_DOCUMENT_TYPE_NODE,
                       XML_DOCUMENT_NODE, XML_COMMENT_NODE, XML_PI_NODE,
                       XML_ENTITY_NODE, XML_ENTITY_REF_NODE,
                       XML_CDATA_SECTION_NODE, XML_TEXT_NODE,
                       XML_ATTRIBUTE_NODE, XML_ELEMENT_NODE, xmlNsType,
                       _xmlAttr, xmlAttributeType, XML_ATTRIBUTE_NOTATION,
                       XML_ATTRIBUTE_ENUMERATION, XML_ATTRIBUTE_NMTOKENS,
                       XML_ATTRIBUTE_NMTOKEN, XML_ATTRIBUTE_ENTITIES,
                       XML_ATTRIBUTE_ENTITY, XML_ATTRIBUTE_IDREFS,
                       XML_ATTRIBUTE_IDREF, XML_ATTRIBUTE_ID,
                       XML_ATTRIBUTE_CDATA};
use self::dict_h::_xmlDict;
pub use self::xmlstring_h::xmlChar;
pub use self::iconv_h::iconv_t;
use self::stdio_h::sprintf;
use self::unistd_h::close;
use self::string_h::{memcpy, strerror};
pub use self::_OSByteOrder_h::_OSSwapInt32;
use self::_malloc_h::{malloc, free};
use self::errno_h::__error;
use self::time_h::time;
use self::libintl_h::libintl_dgettext;
use self::lzo_h::av_lzo1x_decode;
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2015 Intel Corporation.
 *
 * Author: David Woodhouse <dwmw2@infradead.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * version 2.1, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 */
#[no_mangle]
#[src_loc = "30:1"]
pub unsafe extern "C" fn print_esp_keys(mut vpninfo: *mut openconnect_info,
                                        mut name: *const libc::c_char,
                                        mut esp: *mut esp) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut enctype: *const libc::c_char = 0 as *const libc::c_char;
    let mut mactype: *const libc::c_char = 0 as *const libc::c_char;
    let mut enckey: [libc::c_char; 256] = [0; 256];
    let mut mackey: [libc::c_char; 256] = [0; 256];
    match (*vpninfo).esp_enc as libc::c_int {
        2 => {
            enctype =
                b"AES-128-CBC (RFC3602)\x00" as *const u8 as
                    *const libc::c_char
        }
        5 => {
            enctype =
                b"AES-256-CBC (RFC3602)\x00" as *const u8 as
                    *const libc::c_char
        }
        _ => { return -22i32 }
    }
    match (*vpninfo).esp_hmac as libc::c_int {
        1 => {
            mactype =
                b"HMAC-MD5-96 (RFC2403)\x00" as *const u8 as
                    *const libc::c_char
        }
        2 => {
            mactype =
                b"HMAC-SHA-1-96 (RFC2404)\x00" as *const u8 as
                    *const libc::c_char
        }
        3 => {
            mactype =
                b"HMAC-SHA-256-128 (RFC4868)\x00" as *const u8 as
                    *const libc::c_char
        }
        _ => { return -22i32 }
    }
    i = 0i32;
    while i < (*vpninfo).enc_key_len {
        sprintf(enckey.as_mut_ptr().offset((2i32 * i) as isize),
                b"%02x\x00" as *const u8 as *const libc::c_char,
                (*esp).enc_key[i as usize] as libc::c_int);
        i += 1
    }
    i = 0i32;
    while i < (*vpninfo).hmac_key_len {
        sprintf(mackey.as_mut_ptr().offset((2i32 * i) as isize),
                b"%02x\x00" as *const u8 as *const libc::c_char,
                (*esp).hmac_key[i as usize] as libc::c_int);
        i += 1
    }
    if (*vpninfo).verbose >= 3i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                3i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Parameters for %s ESP: SPI 0x%08x\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                name,
                                                                if 0 != 0 {
                                                                    (((*esp).spi
                                                                          &
                                                                          0xff000000u32)
                                                                         >>
                                                                         24i32
                                                                         |
                                                                         ((*esp).spi
                                                                              &
                                                                              0xff0000i32
                                                                                  as
                                                                                  libc::c_uint)
                                                                             >>
                                                                             8i32
                                                                         |
                                                                         ((*esp).spi
                                                                              &
                                                                              0xff00i32
                                                                                  as
                                                                                  libc::c_uint)
                                                                             <<
                                                                             8i32)
                                                                        |
                                                                        ((*esp).spi
                                                                             &
                                                                             0xffi32
                                                                                 as
                                                                                 libc::c_uint)
                                                                            <<
                                                                            24i32
                                                                } else {
                                                                    _OSSwapInt32((*esp).spi)
                                                                });
    }
    if (*vpninfo).verbose >= 3i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                3i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"ESP encryption type %s key 0x%s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                enctype,
                                                                enckey.as_mut_ptr());
    }
    if (*vpninfo).verbose >= 3i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                3i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"ESP authentication type %s key 0x%s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                mactype,
                                                                mackey.as_mut_ptr());
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "77:1"]
pub unsafe extern "C" fn esp_setup(mut vpninfo: *mut openconnect_info,
                                   mut dtls_attempt_period: libc::c_int)
 -> libc::c_int {
    if (*vpninfo).dtls_state == 2i32 || (*vpninfo).dtls_state == 0i32 {
        return -22i32
    }
    if (*vpninfo).esp_ssl_fallback != 0 {
        (*vpninfo).dtls_times.dpd = (*vpninfo).esp_ssl_fallback as libc::c_int
    } else { (*vpninfo).dtls_times.dpd = dtls_attempt_period }
    (*vpninfo).dtls_attempt_period = dtls_attempt_period;
    print_esp_keys(vpninfo,
                   libintl_dgettext(b"openconnect\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"incoming\x00" as *const u8 as
                                        *const libc::c_char),
                   &mut *(*vpninfo).esp_in.as_mut_ptr().offset((*vpninfo).current_esp_in
                                                                   as isize));
    print_esp_keys(vpninfo,
                   libintl_dgettext(b"openconnect\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"outgoing\x00" as *const u8 as
                                        *const libc::c_char),
                   &mut (*vpninfo).esp_out);
    if (*vpninfo).verbose >= 2i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                2i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Send ESP probes\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
    }
    if (*(*vpninfo).proto).udp_send_probes.is_some() {
        (*(*vpninfo).proto).udp_send_probes.expect("non-null function pointer")(vpninfo);
    }
    return 0i32;
}
#[no_mangle]
#[src_loc = "100:1"]
pub unsafe extern "C" fn construct_esp_packet(mut vpninfo:
                                                  *mut openconnect_info,
                                              mut pkt: *mut pkt,
                                              mut next_hdr: uint8_t)
 -> libc::c_int {
    let blksize: libc::c_int = 16i32;
    let mut i: libc::c_int = 0;
    let mut padlen: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if next_hdr == 0 {
        if *(*pkt).data.as_mut_ptr().offset(0) as libc::c_int & 0xf0i32 ==
               0x60i32 {
            /* iph->ip_v */
            next_hdr = 41i32 as uint8_t
        } else { next_hdr = 4i32 as uint8_t }
    }
    /* This gets much more fun if the IV is variable-length */
    (*pkt).c2rust_unnamed.esp.spi = (*vpninfo).esp_out.spi;
    (*pkt).c2rust_unnamed.esp.seq =
        if 0 != 0 {
            let fresh0 = (*vpninfo).esp_out.seq;
            (*vpninfo).esp_out.seq = (*vpninfo).esp_out.seq.wrapping_add(1);
            let fresh1 = (*vpninfo).esp_out.seq;
            (*vpninfo).esp_out.seq = (*vpninfo).esp_out.seq.wrapping_add(1);
            let fresh2 = (*vpninfo).esp_out.seq;
            (*vpninfo).esp_out.seq = (*vpninfo).esp_out.seq.wrapping_add(1);
            let fresh3 = (*vpninfo).esp_out.seq;
            (*vpninfo).esp_out.seq = (*vpninfo).esp_out.seq.wrapping_add(1);
            ((fresh0 as __uint32_t & 0xff000000u32) >> 24i32 |
                 (fresh1 as __uint32_t & 0xff0000i32 as libc::c_uint) >> 8i32
                 | (fresh2 as __uint32_t & 0xff00i32 as libc::c_uint) << 8i32)
                | (fresh3 as __uint32_t & 0xffi32 as libc::c_uint) << 24i32
        } else {
            let fresh4 = (*vpninfo).esp_out.seq;
            (*vpninfo).esp_out.seq = (*vpninfo).esp_out.seq.wrapping_add(1);
            _OSSwapInt32(fresh4 as __uint32_t)
        };
    padlen = blksize - 1i32 - ((*pkt).len + 1i32) % blksize;
    i = 0i32;
    while i < padlen {
        *(*pkt).data.as_mut_ptr().offset(((*pkt).len + i) as isize) =
            (i + 1i32) as libc::c_uchar;
        i += 1
    }
    *(*pkt).data.as_mut_ptr().offset(((*pkt).len + padlen) as isize) =
        padlen as libc::c_uchar;
    *(*pkt).data.as_mut_ptr().offset(((*pkt).len + padlen + 1i32) as isize) =
        next_hdr;
    memcpy((*pkt).c2rust_unnamed.esp.iv.as_mut_ptr() as *mut libc::c_void,
           (*vpninfo).esp_out.iv.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong);
    ret = encrypt_esp_packet(vpninfo, pkt, (*pkt).len + padlen + 2i32);
    if ret != 0 { return ret }
    return (::std::mem::size_of::<C2RustUnnamed_4>() as
                libc::c_ulong).wrapping_add((*pkt).len as
                                                libc::c_ulong).wrapping_add(padlen
                                                                                as
                                                                                libc::c_ulong).wrapping_add(2i32
                                                                                                                as
                                                                                                                libc::c_ulong).wrapping_add((*vpninfo).hmac_out_len
                                                                                                                                                as
                                                                                                                                                libc::c_ulong)
               as libc::c_int;
}
#[no_mangle]
#[src_loc = "131:1"]
pub unsafe extern "C" fn esp_mainloop(mut vpninfo: *mut openconnect_info,
                                      mut timeout: *mut libc::c_int,
                                      mut readable: libc::c_int)
 -> libc::c_int {
    let mut esp: *mut esp =
        &mut *(*vpninfo).esp_in.as_mut_ptr().offset((*vpninfo).current_esp_in
                                                        as isize) as *mut esp;
    let mut old_esp: *mut esp =
        &mut *(*vpninfo).esp_in.as_mut_ptr().offset(((*vpninfo).current_esp_in
                                                         ^ 1i32) as isize) as
            *mut esp;
    let mut this: *mut pkt = 0 as *mut pkt;
    let mut work_done: libc::c_int = 0i32;
    let mut ret: libc::c_int = 0;
    /* Some servers send us packets that are larger than negotiated
	   MTU, or lack the ability to negotiate MTU (see gpst.c). We
	   reserve some extra space to handle that */
    let mut receive_mtu: libc::c_int =
        if 2048i32 > (*vpninfo).ip_info.mtu + 256i32 {
            2048i32
        } else { ((*vpninfo).ip_info.mtu) + 256i32 };
    if (*vpninfo).dtls_state == 3i32 {
        if ka_check_deadline(timeout, time(0 as *mut time_t),
                             (*vpninfo).new_dtls_started +
                                 (*vpninfo).dtls_attempt_period as
                                     libc::c_long) != 0 ||
               (*vpninfo).dtls_need_reconnect != 0 {
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Send ESP probes\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            if (*(*vpninfo).proto).udp_send_probes.is_some() {
                (*(*vpninfo).proto).udp_send_probes.expect("non-null function pointer")(vpninfo);
            }
        }
    }
    if (*vpninfo).dtls_fd == -1i32 { return 0i32 }
    while readable != 0 {
        let mut len: libc::c_int = receive_mtu + (*vpninfo).pkt_trailer;
        let mut i: libc::c_int = 0;
        let mut pkt: *mut pkt = 0 as *mut pkt;
        if (*vpninfo).dtls_pkt.is_null() {
            (*vpninfo).dtls_pkt =
                malloc((::std::mem::size_of::<pkt>() as
                            libc::c_ulong).wrapping_add(len as libc::c_ulong))
                    as *mut pkt;
            if (*vpninfo).dtls_pkt.is_null() {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Allocation failed\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                break ;
            }
        }
        pkt = (*vpninfo).dtls_pkt;
        len =
            recv((*vpninfo).dtls_fd,
                 &mut (*pkt).c2rust_unnamed.esp as *mut C2RustUnnamed_4 as
                     *mut libc::c_void,
                 (len as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<C2RustUnnamed_4>()
                                                      as libc::c_ulong), 0i32)
                as libc::c_int;
        if len <= 0i32 { break ; }
        if (*vpninfo).verbose >= 3i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    3i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Received ESP packet of %d bytes\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    len);
        }
        work_done = 1i32;
        /* both supported algos (SHA1 and MD5) have 12-byte MAC lengths (RFC2403 and RFC2404) */
        if len as libc::c_ulong <=
               (::std::mem::size_of::<C2RustUnnamed_4>() as
                    libc::c_ulong).wrapping_add((*vpninfo).hmac_out_len as
                                                    libc::c_ulong) {
            continue ;
        }
        len =
            (len as
                 libc::c_ulong).wrapping_sub((::std::mem::size_of::<C2RustUnnamed_4>()
                                                  as
                                                  libc::c_ulong).wrapping_add((*vpninfo).hmac_out_len
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int;
        (*pkt).len = len;
        if (*pkt).c2rust_unnamed.esp.spi == (*esp).spi {
            if decrypt_esp_packet(vpninfo, esp, pkt) != 0 { continue ; }
        } else if (*pkt).c2rust_unnamed.esp.spi == (*old_esp).spi &&
                      ((if 0 != 0 {
                            (((*pkt).c2rust_unnamed.esp.seq & 0xff000000u32)
                                 >> 24i32 |
                                 ((*pkt).c2rust_unnamed.esp.seq &
                                      0xff0000i32 as libc::c_uint) >> 8i32 |
                                 ((*pkt).c2rust_unnamed.esp.seq &
                                      0xff00i32 as libc::c_uint) << 8i32) |
                                ((*pkt).c2rust_unnamed.esp.seq &
                                     0xffi32 as libc::c_uint) << 24i32
                        } else {
                            _OSSwapInt32((*pkt).c2rust_unnamed.esp.seq)
                        }) as libc::c_ulonglong).wrapping_add((*esp).seq) <
                          (*vpninfo).old_esp_maxseq as libc::c_ulonglong {
            if (*vpninfo).verbose >= 3i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        3i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Received ESP packet from old SPI 0x%x, seq %u\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        if 0
                                                                               !=
                                                                               0
                                                                           {
                                                                            (((*old_esp).spi
                                                                                  &
                                                                                  0xff000000u32)
                                                                                 >>
                                                                                 24i32
                                                                                 |
                                                                                 ((*old_esp).spi
                                                                                      &
                                                                                      0xff0000i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                                     >>
                                                                                     8i32
                                                                                 |
                                                                                 ((*old_esp).spi
                                                                                      &
                                                                                      0xff00i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                                     <<
                                                                                     8i32)
                                                                                |
                                                                                ((*old_esp).spi
                                                                                     &
                                                                                     0xffi32
                                                                                         as
                                                                                         libc::c_uint)
                                                                                    <<
                                                                                    24i32
                                                                        } else {
                                                                            _OSSwapInt32((*old_esp).spi)
                                                                        },
                                                                        if 0
                                                                               !=
                                                                               0
                                                                           {
                                                                            (((*pkt).c2rust_unnamed.esp.seq
                                                                                  &
                                                                                  0xff000000u32)
                                                                                 >>
                                                                                 24i32
                                                                                 |
                                                                                 ((*pkt).c2rust_unnamed.esp.seq
                                                                                      &
                                                                                      0xff0000i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                                     >>
                                                                                     8i32
                                                                                 |
                                                                                 ((*pkt).c2rust_unnamed.esp.seq
                                                                                      &
                                                                                      0xff00i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                                     <<
                                                                                     8i32)
                                                                                |
                                                                                ((*pkt).c2rust_unnamed.esp.seq
                                                                                     &
                                                                                     0xffi32
                                                                                         as
                                                                                         libc::c_uint)
                                                                                    <<
                                                                                    24i32
                                                                        } else {
                                                                            _OSSwapInt32((*pkt).c2rust_unnamed.esp.seq)
                                                                        });
            }
            if decrypt_esp_packet(vpninfo, old_esp, pkt) != 0 { continue ; }
        } else {
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Received ESP packet with invalid SPI 0x%08x\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        if 0
                                                                               !=
                                                                               0
                                                                           {
                                                                            (((*pkt).c2rust_unnamed.esp.spi
                                                                                  &
                                                                                  0xff000000u32)
                                                                                 >>
                                                                                 24i32
                                                                                 |
                                                                                 ((*pkt).c2rust_unnamed.esp.spi
                                                                                      &
                                                                                      0xff0000i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                                     >>
                                                                                     8i32
                                                                                 |
                                                                                 ((*pkt).c2rust_unnamed.esp.spi
                                                                                      &
                                                                                      0xff00i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                                     <<
                                                                                     8i32)
                                                                                |
                                                                                ((*pkt).c2rust_unnamed.esp.spi
                                                                                     &
                                                                                     0xffi32
                                                                                         as
                                                                                         libc::c_uint)
                                                                                    <<
                                                                                    24i32
                                                                        } else {
                                                                            _OSSwapInt32((*pkt).c2rust_unnamed.esp.spi)
                                                                        });
            }
            continue ;
        }
        /* Possible values of the Next Header field are:
		   0x04: IP[v4]-in-IP
		   0x05: supposed to mean Internet Stream Protocol
		         (XXX: but used for LZO compressed packets by Juniper)
		   0x29: IPv6 encapsulation */
        if *(*pkt).data.as_mut_ptr().offset((len - 1i32) as isize) as
               libc::c_int != 0x4i32 &&
               *(*pkt).data.as_mut_ptr().offset((len - 1i32) as isize) as
                   libc::c_int != 0x29i32 &&
               *(*pkt).data.as_mut_ptr().offset((len - 1i32) as isize) as
                   libc::c_int != 0x5i32 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Received ESP packet with unrecognised payload type %02x\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        *(*pkt).data.as_mut_ptr().offset((len
                                                                                                              -
                                                                                                              1i32)
                                                                                                             as
                                                                                                             isize)
                                                                            as
                                                                            libc::c_int);
            }
        } else if len <=
                      2i32 +
                          *(*pkt).data.as_mut_ptr().offset((len - 2i32) as
                                                               isize) as
                              libc::c_int {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Invalid padding length %02x in ESP\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        *(*pkt).data.as_mut_ptr().offset((len
                                                                                                              -
                                                                                                              2i32)
                                                                                                             as
                                                                                                             isize)
                                                                            as
                                                                            libc::c_int);
            }
        } else {
            (*pkt).len =
                len - 2i32 -
                    *(*pkt).data.as_mut_ptr().offset((len - 2i32) as isize) as
                        libc::c_int;
            i = 0i32;
            while i <
                      *(*pkt).data.as_mut_ptr().offset((len - 2i32) as isize)
                          as libc::c_int {
                if *(*pkt).data.as_mut_ptr().offset(((*pkt).len + i) as isize)
                       as libc::c_int != i + 1i32 {
                    break ;
                    /* We can't just 'continue' here because it
					* would only break out of this 'for' loop */
                }
                i += 1
            }
            if i !=
                   *(*pkt).data.as_mut_ptr().offset((len - 2i32) as isize) as
                       libc::c_int {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Invalid padding bytes in ESP\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                /* We can here, though */
            } else {
                (*vpninfo).dtls_times.last_rx = time(0 as *mut time_t);
                if (*(*vpninfo).proto).udp_catch_probe.is_some() {
                    if (*(*vpninfo).proto).udp_catch_probe.expect("non-null function pointer")(vpninfo,
                                                                                               pkt)
                           != 0 {
                        if (*vpninfo).dtls_state == 3i32 {
                            if (*vpninfo).verbose >= 1i32 {
                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                        1i32,
                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char,
                                                                                                         b"ESP session established with server\n\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char));
                            }
                            (*vpninfo).dtls_state = 4i32
                        }
                        continue ;
                    }
                }
                if *(*pkt).data.as_mut_ptr().offset((len - 1i32) as isize) as
                       libc::c_int == 0x5i32 {
                    let mut newpkt: *mut pkt =
                        malloc((::std::mem::size_of::<pkt>() as
                                    libc::c_ulong).wrapping_add(receive_mtu as
                                                                    libc::c_ulong).wrapping_add((*vpninfo).pkt_trailer
                                                                                                    as
                                                                                                    libc::c_ulong))
                            as *mut pkt;
                    let mut newlen: libc::c_int = receive_mtu;
                    if newpkt.is_null() {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Failed to allocate memory to decrypt ESP packet\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                    } else if av_lzo1x_decode((*newpkt).data.as_mut_ptr() as
                                                  *mut libc::c_void,
                                              &mut newlen,
                                              (*pkt).data.as_mut_ptr() as
                                                  *const libc::c_void,
                                              &mut (*pkt).len) != 0 ||
                                  (*pkt).len != 0 {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"LZO decompression of ESP packet failed\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        free(newpkt as *mut libc::c_void);
                    } else {
                        (*newpkt).len = receive_mtu - newlen;
                        if (*vpninfo).verbose >= 3i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    3i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"LZO decompressed %d bytes into %d\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char),
                                                                                    len
                                                                                        -
                                                                                        2i32
                                                                                        -
                                                                                        *(*pkt).data.as_mut_ptr().offset((len
                                                                                                                              -
                                                                                                                              2i32)
                                                                                                                             as
                                                                                                                             isize)
                                                                                            as
                                                                                            libc::c_int,
                                                                                    (*newpkt).len);
                        }
                        queue_packet(&mut (*vpninfo).incoming_queue, newpkt);
                    }
                } else {
                    queue_packet(&mut (*vpninfo).incoming_queue, pkt);
                    (*vpninfo).dtls_pkt = 0 as *mut pkt
                }
            }
        }
    }
    if (*vpninfo).dtls_state != 5i32 { return 0i32 }
    match keepalive_action(&mut (*vpninfo).dtls_times, timeout) {
        4 => {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Rekey not implemented for ESP\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
        }
        2 => {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"ESP detected dead peer\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            if (*(*vpninfo).proto).udp_close.is_some() {
                (*(*vpninfo).proto).udp_close.expect("non-null function pointer")(vpninfo);
            }
            if (*(*vpninfo).proto).udp_send_probes.is_some() {
                (*(*vpninfo).proto).udp_send_probes.expect("non-null function pointer")(vpninfo);
            }
            return 1i32
        }
        1 => {
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Send ESP probes for DPD\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            if (*(*vpninfo).proto).udp_send_probes.is_some() {
                (*(*vpninfo).proto).udp_send_probes.expect("non-null function pointer")(vpninfo);
            }
            work_done = 1i32
        }
        3 => {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Keepalive not implemented for ESP\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
        }
        0 | _ => { }
    }
    loop  {
        let mut len_0: libc::c_int = 0;
        if !(*vpninfo).deflate_pkt.is_null() {
            this = (*vpninfo).deflate_pkt;
            len_0 = (*this).len
        } else {
            this = dequeue_packet(&mut (*vpninfo).outgoing_queue);
            if this.is_null() { break ; }
            if (*(*vpninfo).proto).udp_send_probes ==
                   Some(oncp_esp_send_probes as
                            unsafe extern "C" fn(_: *mut openconnect_info)
                                -> libc::c_int) {
                let mut dontsend: uint8_t = 0;
                /* Pulse/NC can only accept ESP of the same protocol as the one
				 * you connected to it with. The other has to go over IF-T/TLS. */
                if (*(*vpninfo).dtls_addr).sa_family as libc::c_int == 30i32 {
                    dontsend = 0x40i32 as uint8_t
                } else { dontsend = 0x60i32 as uint8_t }
                if *(*this).data.as_mut_ptr().offset(0) as libc::c_int &
                       0xf0i32 == dontsend as libc::c_int {
                    store_be32(&mut (*this).c2rust_unnamed.pulse.vendor as
                                   *mut uint32_t as *mut libc::c_void,
                               0xa4ci32 as uint32_t);
                    store_be32(&mut (*this).c2rust_unnamed.pulse.type_0 as
                                   *mut uint32_t as *mut libc::c_void,
                               4i32 as uint32_t);
                    store_be32(&mut (*this).c2rust_unnamed.pulse.len as
                                   *mut uint32_t as *mut libc::c_void,
                               ((*this).len + 16i32) as uint32_t);
                    queue_packet(&mut (*vpninfo).oncp_control_queue, this);
                    work_done = 1i32;
                    continue ;
                }
            }
            len_0 = construct_esp_packet(vpninfo, this, 0i32 as uint8_t);
            if len_0 < 0i32 {
                /* Should we disable ESP? */
                free(this as *mut libc::c_void);
                work_done = 1i32;
                continue ;
            }
        }
        ret =
            send((*vpninfo).dtls_fd,
                 &mut (*this).c2rust_unnamed.esp as *mut C2RustUnnamed_4 as
                     *mut libc::c_void, len_0 as size_t, 0i32) as libc::c_int;
        if ret < 0i32 {
            /* Not that this is likely to happen with UDP, but... */
            if *__error() == 55i32 || *__error() == 35i32 ||
                   *__error() == 35i32 {
                let mut err: libc::c_int = *__error();
                (*vpninfo).deflate_pkt = this;
                (*this).len = len_0;
                if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Requeueing failed ESP send: %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            strerror(err));
                }
                let mut __fd: libc::c_int = (*vpninfo).dtls_fd;
                (*vpninfo)._select_wfds.fds_bits[(__fd as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8i32
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] |=
                    ((1i32 as libc::c_ulong) <<
                         (__fd as
                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                               as
                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                               as
                                                                                               libc::c_ulong)))
                        as __int32_t;
                return work_done
            } else {
                /* A real error in sending. Fall back to TCP? */
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Failed to send ESP packet: %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            strerror(*__error()));
                }
            }
        } else {
            (*vpninfo).dtls_times.last_tx = time(0 as *mut time_t);
            if (*vpninfo).verbose >= 3i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        3i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Sent ESP packet of %d bytes\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        len_0);
            }
        }
        if this == (*vpninfo).deflate_pkt {
            let mut __fd_0: libc::c_int = (*vpninfo).dtls_fd;
            (*vpninfo)._select_wfds.fds_bits[(__fd_0 as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8i32
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &=
                !(((1i32 as libc::c_ulong) <<
                       (__fd_0 as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8i32
                                                                                             as
                                                                                             libc::c_ulong)))
                      as __int32_t);
            (*vpninfo).deflate_pkt = 0 as *mut pkt
        }
        free(this as *mut libc::c_void);
        work_done = 1i32
    }
    return work_done;
}
#[no_mangle]
#[src_loc = "373:1"]
pub unsafe extern "C" fn esp_close(mut vpninfo: *mut openconnect_info) {
    /* We close and reopen the socket in case we roamed and our
	   local IP address has changed. */
    if (*vpninfo).dtls_fd != -1i32 {
        close((*vpninfo).dtls_fd);
        let mut __fd: libc::c_int = (*vpninfo).dtls_fd;
        (*vpninfo)._select_rfds.fds_bits[(__fd as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &=
            !(((1i32 as libc::c_ulong) <<
                   (__fd as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8i32
                                                                                         as
                                                                                         libc::c_ulong)))
                  as __int32_t);
        let mut __fd_0: libc::c_int = (*vpninfo).dtls_fd;
        (*vpninfo)._select_wfds.fds_bits[(__fd_0 as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &=
            !(((1i32 as libc::c_ulong) <<
                   (__fd_0 as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8i32
                                                                                         as
                                                                                         libc::c_ulong)))
                  as __int32_t);
        let mut __fd_1: libc::c_int = (*vpninfo).dtls_fd;
        (*vpninfo)._select_efds.fds_bits[(__fd_1 as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8i32
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &=
            !(((1i32 as libc::c_ulong) <<
                   (__fd_1 as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8i32
                                                                                         as
                                                                                         libc::c_ulong)))
                  as __int32_t);
        (*vpninfo).dtls_fd = -1i32
    }
    if (*vpninfo).dtls_state > 2i32 { (*vpninfo).dtls_state = 3i32 }
    if !(*vpninfo).deflate_pkt.is_null() {
        free((*vpninfo).deflate_pkt as *mut libc::c_void);
        (*vpninfo).deflate_pkt = 0 as *mut pkt
    };
}
#[no_mangle]
#[src_loc = "392:1"]
pub unsafe extern "C" fn esp_shutdown(mut vpninfo: *mut openconnect_info) {
    destroy_esp_ciphers(&mut *(*vpninfo).esp_in.as_mut_ptr().offset(0));
    destroy_esp_ciphers(&mut *(*vpninfo).esp_in.as_mut_ptr().offset(1));
    destroy_esp_ciphers(&mut (*vpninfo).esp_out);
    if (*(*vpninfo).proto).udp_close.is_some() {
        (*(*vpninfo).proto).udp_close.expect("non-null function pointer")(vpninfo);
    }
    if (*vpninfo).dtls_state != 2i32 { (*vpninfo).dtls_state = 0i32 };
}
/* Key material for DTLS-PSK */
/* Packet types */
/* Uncompressed data */
/* Dead Peer Detection */
/* DPD response */
/* Client disconnection notice */
/* Keepalive */
/* Compressed data */
/* Server kick */
/* Encryption and HMAC algorithms (matching Juniper/Pulse binary encoding) */
/* SHA256 */
/* Including the next-header field */
/* ***************************************************************************/
/* Oh Solaris how we hate thee! */
/* For systems that don't support O_CLOEXEC, just don't bother.
   We don't keep files open for long anyway. */
/* I always coded as if it worked like this. Now it does. */
/* ***************************************************************************/
/* iconv.c */
/* script.c */
/* tun.c / tun-win32.c */
/* {gnutls,openssl}-dtls.c */
/* dtls.c */
/* cstp.c */
/* auth-juniper.c */
/* oncp.c */
/* pulse.c */
/* auth-globalprotect.c */
/* gpst.c */
/* lzs.c */
/* ssl.c */
/* openssl-pkcs11.c */
/* esp.c */
#[no_mangle]
#[src_loc = "403:1"]
pub unsafe extern "C" fn openconnect_setup_esp_keys(mut vpninfo:
                                                        *mut openconnect_info,
                                                    mut new_keys: libc::c_int)
 -> libc::c_int {
    let mut esp_in: *mut esp = 0 as *mut esp;
    let mut ret: libc::c_int = 0;
    if (*vpninfo).dtls_state == 2i32 { return -102i32 }
    if (*vpninfo).dtls_addr.is_null() { return -22i32 }
    if (*vpninfo).esp_hmac as libc::c_int == 3i32 {
        (*vpninfo).hmac_out_len = 16i32
    } else {
        /* MD5 and SHA1 */
        (*vpninfo).hmac_out_len = 12i32
    }
    if new_keys != 0 {
        (*vpninfo).old_esp_maxseq =
            (*vpninfo).esp_in[(*vpninfo).current_esp_in as
                                  usize].seq.wrapping_add(32i32 as
                                                              libc::c_ulonglong)
                as libc::c_int;
        (*vpninfo).current_esp_in ^= 1i32
    }
    esp_in =
        &mut *(*vpninfo).esp_in.as_mut_ptr().offset((*vpninfo).current_esp_in
                                                        as isize) as *mut esp;
    if new_keys != 0 {
        if openconnect_random(&mut (*esp_in).spi as *mut uint32_t as
                                  *mut libc::c_void,
                              ::std::mem::size_of::<uint32_t>() as
                                  libc::c_ulong as libc::c_int) != 0 ||
               openconnect_random(&mut (*esp_in).enc_key as
                                      *mut [libc::c_uchar; 64] as
                                      *mut libc::c_void,
                                  (*vpninfo).enc_key_len) != 0 ||
               openconnect_random(&mut (*esp_in).hmac_key as
                                      *mut [libc::c_uchar; 64] as
                                      *mut libc::c_void,
                                  (*vpninfo).hmac_key_len) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Failed to generate random keys for ESP\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            return -5i32
        }
    }
    if openconnect_random((*vpninfo).esp_out.iv.as_mut_ptr() as
                              *mut libc::c_void,
                          ::std::mem::size_of::<[libc::c_uchar; 16]>() as
                              libc::c_ulong as libc::c_int) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Failed to generate initial IV for ESP\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return -5i32
    }
    /* This is the minimum; some implementations may increase it */
    (*vpninfo).pkt_trailer = 17i32 + 16i32 + 32i32;
    (*vpninfo).esp_out.seq_backlog = 0i32 as uint64_t;
    (*vpninfo).esp_out.seq = (*vpninfo).esp_out.seq_backlog;
    (*esp_in).seq_backlog = 0i32 as uint64_t;
    (*esp_in).seq = (*esp_in).seq_backlog;
    ret = init_esp_ciphers(vpninfo, &mut (*vpninfo).esp_out, esp_in);
    if ret != 0 { return ret }
    if (*vpninfo).dtls_state == 0i32 { (*vpninfo).dtls_state = 1i32 }
    return 0i32;
}
