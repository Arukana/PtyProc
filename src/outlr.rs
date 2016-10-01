use std::str::FromStr;
use outp;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__CursorUse {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use outp;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_227_22(&'input str),
        Term_228_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3b0c_22(&'input str),
        Term_22_3bc_22(&'input str),
        Term_22A_22(&'input str),
        Term_22B_22(&'input str),
        Term_22C_22(&'input str),
        Term_22D_22(&'input str),
        Term_22H_22(&'input str),
        Term_22M_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5b1J_22(&'input str),
        Term_22_5b1K_22(&'input str),
        Term_22_5b2J_22(&'input str),
        Term_22_5b2K_22(&'input str),
        Term_22_5b7h_22(&'input str),
        Term_22_5b7l_22(&'input str),
        Term_22_5b_3bH_22(&'input str),
        Term_22_5b_3bf_22(&'input str),
        Term_22_5b_3e_22(&'input str),
        Term_22_5b_3e0c_22(&'input str),
        Term_22_5bA_22(&'input str),
        Term_22_5bB_22(&'input str),
        Term_22_5bC_22(&'input str),
        Term_22_5bD_22(&'input str),
        Term_22_5bH_22(&'input str),
        Term_22_5bJ_22(&'input str),
        Term_22_5bK_22(&'input str),
        Term_22_5bf_22(&'input str),
        Term_22_5br_22(&'input str),
        Term_22_5bs_22(&'input str),
        Term_22_5bu_22(&'input str),
        Term_22c_22(&'input str),
        Term_22f_22(&'input str),
        Term_22m_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5bA_2dZ_5d_22_23(&'input str),
        Termr_23_22_5b_5c_5cx1B_2d_5c_5cx1B_5d_22_23(&'input str),
        Termr_23_22_5ba_2dz_5d_22_23(&'input str),
        NtCursorUse((outp::Cursor)),
        NtEsc(u16),
        NtMaj(char),
        NtMin(char),
        NtNum(u16),
        Nt____CursorUse((outp::Cursor)),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        5, // on r#"[0-9]+"#, goto 4
        0, // on r#"[A-Z]"#, error
        6, // on r#"[\\x1B-\\x1B]"#, goto 5
        0, // on r#"[a-z]"#, error
        // State 1
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 2
        7, // on "7", goto 6
        8, // on "8", goto 7
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        9, // on "D", goto 8
        0, // on "H", error
        10, // on "M", goto 9
        11, // on "[", goto 10
        12, // on "[1J", goto 11
        13, // on "[1K", goto 12
        14, // on "[2J", goto 13
        15, // on "[2K", goto 14
        16, // on "[7h", goto 15
        17, // on "[7l", goto 16
        18, // on "[;H", goto 17
        19, // on "[;f", goto 18
        20, // on "[>", goto 19
        21, // on "[>0c", goto 20
        22, // on "[A", goto 21
        23, // on "[B", goto 22
        24, // on "[C", goto 23
        25, // on "[D", goto 24
        26, // on "[H", goto 25
        27, // on "[J", goto 26
        28, // on "[K", goto 27
        29, // on "[f", goto 28
        30, // on "[r", goto 29
        31, // on "[s", goto 30
        32, // on "[u", goto 31
        33, // on "c", goto 32
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 3
        0, // on "7", error
        0, // on "8", error
        34, // on ";", goto 33
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 4
        0, // on "7", error
        0, // on "8", error
        -44, // on ";", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 5
        -41, // on "7", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "8", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        -41, // on "D", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        0, // on "H", error
        -41, // on "M", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[1J", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[1K", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[2J", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[2K", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[7h", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[7l", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[;H", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[;f", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[>", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[>0c", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[A", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[B", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[C", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[D", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[H", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[J", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[K", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[f", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[r", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[s", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "[u", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        -41, // on "c", reduce `Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);`
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 6
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 7
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 8
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 9
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 10
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        38, // on r#"[0-9]+"#, goto 37
        39, // on r#"[A-Z]"#, goto 38
        0, // on r#"[\\x1B-\\x1B]"#, error
        40, // on r#"[a-z]"#, goto 39
        // State 11
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 12
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 13
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 14
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 15
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 16
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 17
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 18
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 19
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        5, // on r#"[0-9]+"#, goto 4
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 20
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 21
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 22
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 23
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 24
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 25
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 26
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 27
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 28
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 29
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 30
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 31
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 32
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 33
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        43, // on r#"[0-9]+"#, goto 42
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 34
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 35
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 36
        0, // on "7", error
        0, // on "8", error
        46, // on ";", goto 45
        0, // on ";0c", error
        0, // on ";c", error
        47, // on "A", goto 46
        48, // on "B", goto 47
        49, // on "C", goto 48
        50, // on "D", goto 49
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        51, // on "m", goto 50
        0, // on r#"[0-9]+"#, error
        39, // on r#"[A-Z]"#, goto 38
        0, // on r#"[\\x1B-\\x1B]"#, error
        40, // on r#"[a-z]"#, goto 39
        // State 37
        0, // on "7", error
        0, // on "8", error
        -44, // on ";", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on ";0c", error
        0, // on ";c", error
        -44, // on "A", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        -44, // on "B", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        -44, // on "C", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        -44, // on "D", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        -44, // on "m", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on r#"[0-9]+"#, error
        -44, // on r#"[A-Z]"#, reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on r#"[\\x1B-\\x1B]"#, error
        -44, // on r#"[a-z]"#, reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        // State 38
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 39
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 40
        0, // on "7", error
        0, // on "8", error
        52, // on ";", goto 51
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 41
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        53, // on ";0c", goto 52
        54, // on ";c", goto 53
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 42
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        -44, // on ";0c", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        -44, // on ";c", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 43
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 44
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 45
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        56, // on r#"[0-9]+"#, goto 55
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 46
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 47
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 48
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 49
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 50
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 51
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        43, // on r#"[0-9]+"#, goto 42
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 52
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 53
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 54
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        58, // on "H", goto 57
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        59, // on "f", goto 58
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 55
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        -44, // on "H", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        -44, // on "f", reduce `Num = r#"[0-9]+"# => ActionFn(1);`
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 56
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        60, // on ";0c", goto 59
        61, // on ";c", goto 60
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 57
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 58
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 59
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
        // State 60
        0, // on "7", error
        0, // on "8", error
        0, // on ";", error
        0, // on ";0c", error
        0, // on ";c", error
        0, // on "A", error
        0, // on "B", error
        0, // on "C", error
        0, // on "D", error
        0, // on "H", error
        0, // on "M", error
        0, // on "[", error
        0, // on "[1J", error
        0, // on "[1K", error
        0, // on "[2J", error
        0, // on "[2K", error
        0, // on "[7h", error
        0, // on "[7l", error
        0, // on "[;H", error
        0, // on "[;f", error
        0, // on "[>", error
        0, // on "[>0c", error
        0, // on "[A", error
        0, // on "[B", error
        0, // on "[C", error
        0, // on "[D", error
        0, // on "[H", error
        0, // on "[J", error
        0, // on "[K", error
        0, // on "[f", error
        0, // on "[r", error
        0, // on "[s", error
        0, // on "[u", error
        0, // on "c", error
        0, // on "f", error
        0, // on "m", error
        0, // on r#"[0-9]+"#, error
        0, // on r#"[A-Z]"#, error
        0, // on r#"[\\x1B-\\x1B]"#, error
        0, // on r#"[a-z]"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -45, // on EOF, reduce `__CursorUse = CursorUse => ActionFn(0);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -25, // on EOF, reduce `CursorUse = Esc, "7" => ActionFn(29);`
        -27, // on EOF, reduce `CursorUse = Esc, "8" => ActionFn(31);`
        -28, // on EOF, reduce `CursorUse = Esc, "D" => ActionFn(32);`
        -29, // on EOF, reduce `CursorUse = Esc, "M" => ActionFn(33);`
        0, // on EOF, error
        -34, // on EOF, reduce `CursorUse = Esc, "[1J" => ActionFn(38);`
        -31, // on EOF, reduce `CursorUse = Esc, "[1K" => ActionFn(35);`
        -35, // on EOF, reduce `CursorUse = Esc, "[2J" => ActionFn(39);`
        -32, // on EOF, reduce `CursorUse = Esc, "[2K" => ActionFn(36);`
        -1, // on EOF, reduce `CursorUse = Esc, "[7h" => ActionFn(5);`
        -2, // on EOF, reduce `CursorUse = Esc, "[7l" => ActionFn(6);`
        -14, // on EOF, reduce `CursorUse = Esc, "[;H" => ActionFn(18);`
        -15, // on EOF, reduce `CursorUse = Esc, "[;f" => ActionFn(19);`
        0, // on EOF, error
        -4, // on EOF, reduce `CursorUse = Esc, "[>0c" => ActionFn(8);`
        -20, // on EOF, reduce `CursorUse = Esc, "[A" => ActionFn(24);`
        -21, // on EOF, reduce `CursorUse = Esc, "[B" => ActionFn(25);`
        -22, // on EOF, reduce `CursorUse = Esc, "[C" => ActionFn(26);`
        -23, // on EOF, reduce `CursorUse = Esc, "[D" => ActionFn(27);`
        -12, // on EOF, reduce `CursorUse = Esc, "[H" => ActionFn(16);`
        -33, // on EOF, reduce `CursorUse = Esc, "[J" => ActionFn(37);`
        -30, // on EOF, reduce `CursorUse = Esc, "[K" => ActionFn(34);`
        -13, // on EOF, reduce `CursorUse = Esc, "[f" => ActionFn(17);`
        -3, // on EOF, reduce `CursorUse = Esc, "[r" => ActionFn(7);`
        -24, // on EOF, reduce `CursorUse = Esc, "[s" => ActionFn(28);`
        -26, // on EOF, reduce `CursorUse = Esc, "[u" => ActionFn(30);`
        -9, // on EOF, reduce `CursorUse = Esc, "c" => ActionFn(13);`
        0, // on EOF, error
        -39, // on EOF, reduce `CursorUse = Esc, "[", Maj => ActionFn(43);`
        -40, // on EOF, reduce `CursorUse = Esc, "[", Min => ActionFn(44);`
        0, // on EOF, error
        0, // on EOF, error
        -42, // on EOF, reduce `Maj = r#"[A-Z]"# => ActionFn(3);`
        -43, // on EOF, reduce `Min = r#"[a-z]"# => ActionFn(4);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -37, // on EOF, reduce `CursorUse = Esc, "[", Num, Maj => ActionFn(41);`
        -38, // on EOF, reduce `CursorUse = Esc, "[", Num, Min => ActionFn(42);`
        0, // on EOF, error
        -16, // on EOF, reduce `CursorUse = Esc, "[", Num, "A" => ActionFn(20);`
        -17, // on EOF, reduce `CursorUse = Esc, "[", Num, "B" => ActionFn(21);`
        -18, // on EOF, reduce `CursorUse = Esc, "[", Num, "C" => ActionFn(22);`
        -19, // on EOF, reduce `CursorUse = Esc, "[", Num, "D" => ActionFn(23);`
        -36, // on EOF, reduce `CursorUse = Esc, "[", Num, "m" => ActionFn(40);`
        0, // on EOF, error
        -6, // on EOF, reduce `CursorUse = Num, ";", Num, ";0c" => ActionFn(10);`
        -5, // on EOF, reduce `CursorUse = Num, ";", Num, ";c" => ActionFn(9);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -10, // on EOF, reduce `CursorUse = Esc, "[", Num, ";", Num, "H" => ActionFn(14);`
        -11, // on EOF, reduce `CursorUse = Esc, "[", Num, ";", Num, "f" => ActionFn(15);`
        -8, // on EOF, reduce `CursorUse = Esc, "[>", Num, ";", Num, ";0c" => ActionFn(12);`
        -7, // on EOF, reduce `CursorUse = Esc, "[>", Num, ";", Num, ";c" => ActionFn(11);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on CursorUse, goto 1
        3, // on Esc, goto 2
        0, // on Maj, error
        0, // on Min, error
        4, // on Num, goto 3
        0, // on __CursorUse, error
        // State 1
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 2
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 3
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 4
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 5
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 6
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 7
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 8
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 9
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 10
        0, // on CursorUse, error
        0, // on Esc, error
        35, // on Maj, goto 34
        36, // on Min, goto 35
        37, // on Num, goto 36
        0, // on __CursorUse, error
        // State 11
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 12
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 13
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 14
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 15
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 16
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 17
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 18
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 19
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        41, // on Num, goto 40
        0, // on __CursorUse, error
        // State 20
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 21
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 22
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 23
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 24
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 25
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 26
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 27
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 28
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 29
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 30
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 31
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 32
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 33
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        42, // on Num, goto 41
        0, // on __CursorUse, error
        // State 34
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 35
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 36
        0, // on CursorUse, error
        0, // on Esc, error
        44, // on Maj, goto 43
        45, // on Min, goto 44
        0, // on Num, error
        0, // on __CursorUse, error
        // State 37
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 38
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 39
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 40
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 41
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 42
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 43
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 44
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 45
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        55, // on Num, goto 54
        0, // on __CursorUse, error
        // State 46
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 47
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 48
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 49
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 50
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 51
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        57, // on Num, goto 56
        0, // on __CursorUse, error
        // State 52
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 53
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 54
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 55
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 56
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 57
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 58
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 59
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
        // State 60
        0, // on CursorUse, error
        0, // on Esc, error
        0, // on Maj, error
        0, // on Min, error
        0, // on Num, error
        0, // on __CursorUse, error
    ];
    pub fn parse_CursorUse<
        'input,
    >(
        input: &'input str,
    ) -> Result<(outp::Cursor), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                (_, (3, _), _) if true => 3,
                (_, (4, _), _) if true => 4,
                (_, (5, _), _) if true => 5,
                (_, (6, _), _) if true => 6,
                (_, (7, _), _) if true => 7,
                (_, (8, _), _) if true => 8,
                (_, (9, _), _) if true => 9,
                (_, (10, _), _) if true => 10,
                (_, (11, _), _) if true => 11,
                (_, (12, _), _) if true => 12,
                (_, (13, _), _) if true => 13,
                (_, (14, _), _) if true => 14,
                (_, (15, _), _) if true => 15,
                (_, (16, _), _) if true => 16,
                (_, (17, _), _) if true => 17,
                (_, (18, _), _) if true => 18,
                (_, (19, _), _) if true => 19,
                (_, (20, _), _) if true => 20,
                (_, (21, _), _) if true => 21,
                (_, (22, _), _) if true => 22,
                (_, (23, _), _) if true => 23,
                (_, (24, _), _) if true => 24,
                (_, (25, _), _) if true => 25,
                (_, (26, _), _) if true => 26,
                (_, (27, _), _) if true => 27,
                (_, (28, _), _) if true => 28,
                (_, (29, _), _) if true => 29,
                (_, (30, _), _) if true => 30,
                (_, (31, _), _) if true => 31,
                (_, (32, _), _) if true => 32,
                (_, (33, _), _) if true => 33,
                (_, (34, _), _) if true => 34,
                (_, (35, _), _) if true => 35,
                (_, (36, _), _) if true => 36,
                (_, (37, _), _) if true => 37,
                (_, (38, _), _) if true => 38,
                (_, (39, _), _) if true => 39,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 40 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_227_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_228_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_3b_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b0c_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3bc_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22A_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22B_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22C_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22D_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22H_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22M_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_5b1J_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_5b1K_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_5b2J_22(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_5b2K_22(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_5b7h_22(__tok0),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22_5b7l_22(__tok0),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_5b_3bH_22(__tok0),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22_5b_3bf_22(__tok0),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22_5b_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22_5b_3e0c_22(__tok0),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22_5bA_22(__tok0),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22_5bB_22(__tok0),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22_5bC_22(__tok0),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22_5bD_22(__tok0),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            (26, __tok0) => __Symbol::Term_22_5bH_22(__tok0),
                            _ => unreachable!(),
                        },
                        27 => match __lookahead.1 {
                            (27, __tok0) => __Symbol::Term_22_5bJ_22(__tok0),
                            _ => unreachable!(),
                        },
                        28 => match __lookahead.1 {
                            (28, __tok0) => __Symbol::Term_22_5bK_22(__tok0),
                            _ => unreachable!(),
                        },
                        29 => match __lookahead.1 {
                            (29, __tok0) => __Symbol::Term_22_5bf_22(__tok0),
                            _ => unreachable!(),
                        },
                        30 => match __lookahead.1 {
                            (30, __tok0) => __Symbol::Term_22_5br_22(__tok0),
                            _ => unreachable!(),
                        },
                        31 => match __lookahead.1 {
                            (31, __tok0) => __Symbol::Term_22_5bs_22(__tok0),
                            _ => unreachable!(),
                        },
                        32 => match __lookahead.1 {
                            (32, __tok0) => __Symbol::Term_22_5bu_22(__tok0),
                            _ => unreachable!(),
                        },
                        33 => match __lookahead.1 {
                            (33, __tok0) => __Symbol::Term_22c_22(__tok0),
                            _ => unreachable!(),
                        },
                        34 => match __lookahead.1 {
                            (34, __tok0) => __Symbol::Term_22f_22(__tok0),
                            _ => unreachable!(),
                        },
                        35 => match __lookahead.1 {
                            (35, __tok0) => __Symbol::Term_22m_22(__tok0),
                            _ => unreachable!(),
                        },
                        36 => match __lookahead.1 {
                            (36, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        37 => match __lookahead.1 {
                            (37, __tok0) => __Symbol::Termr_23_22_5bA_2dZ_5d_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        38 => match __lookahead.1 {
                            (38, __tok0) => __Symbol::Termr_23_22_5b_5c_5cx1B_2d_5c_5cx1B_5d_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        39 => match __lookahead.1 {
                            (39, __tok0) => __Symbol::Termr_23_22_5ba_2dz_5d_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<(outp::Cursor),__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // CursorUse = Esc, "[7h" => ActionFn(5);
                let __sym1 = __pop_Term_22_5b7h_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            2 => {
                // CursorUse = Esc, "[7l" => ActionFn(6);
                let __sym1 = __pop_Term_22_5b7l_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            3 => {
                // CursorUse = Esc, "[r" => ActionFn(7);
                let __sym1 = __pop_Term_22_5br_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            4 => {
                // CursorUse = Esc, "[>0c" => ActionFn(8);
                let __sym1 = __pop_Term_22_5b_3e0c_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            5 => {
                // CursorUse = Num, ";", Num, ";c" => ActionFn(9);
                let __sym3 = __pop_Term_22_3bc_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            6 => {
                // CursorUse = Num, ";", Num, ";0c" => ActionFn(10);
                let __sym3 = __pop_Term_22_3b0c_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            7 => {
                // CursorUse = Esc, "[>", Num, ";", Num, ";c" => ActionFn(11);
                let __sym5 = __pop_Term_22_3bc_22(__symbols);
                let __sym4 = __pop_NtNum(__symbols);
                let __sym3 = __pop_Term_22_3b_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_3e_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            8 => {
                // CursorUse = Esc, "[>", Num, ";", Num, ";0c" => ActionFn(12);
                let __sym5 = __pop_Term_22_3b0c_22(__symbols);
                let __sym4 = __pop_NtNum(__symbols);
                let __sym3 = __pop_Term_22_3b_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_3e_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            9 => {
                // CursorUse = Esc, "c" => ActionFn(13);
                let __sym1 = __pop_Term_22c_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            10 => {
                // CursorUse = Esc, "[", Num, ";", Num, "H" => ActionFn(14);
                let __sym5 = __pop_Term_22H_22(__symbols);
                let __sym4 = __pop_NtNum(__symbols);
                let __sym3 = __pop_Term_22_3b_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            11 => {
                // CursorUse = Esc, "[", Num, ";", Num, "f" => ActionFn(15);
                let __sym5 = __pop_Term_22f_22(__symbols);
                let __sym4 = __pop_NtNum(__symbols);
                let __sym3 = __pop_Term_22_3b_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            12 => {
                // CursorUse = Esc, "[H" => ActionFn(16);
                let __sym1 = __pop_Term_22_5bH_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            13 => {
                // CursorUse = Esc, "[f" => ActionFn(17);
                let __sym1 = __pop_Term_22_5bf_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            14 => {
                // CursorUse = Esc, "[;H" => ActionFn(18);
                let __sym1 = __pop_Term_22_5b_3bH_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            15 => {
                // CursorUse = Esc, "[;f" => ActionFn(19);
                let __sym1 = __pop_Term_22_5b_3bf_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            16 => {
                // CursorUse = Esc, "[", Num, "A" => ActionFn(20);
                let __sym3 = __pop_Term_22A_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            17 => {
                // CursorUse = Esc, "[", Num, "B" => ActionFn(21);
                let __sym3 = __pop_Term_22B_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            18 => {
                // CursorUse = Esc, "[", Num, "C" => ActionFn(22);
                let __sym3 = __pop_Term_22C_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            19 => {
                // CursorUse = Esc, "[", Num, "D" => ActionFn(23);
                let __sym3 = __pop_Term_22D_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action23::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            20 => {
                // CursorUse = Esc, "[A" => ActionFn(24);
                let __sym1 = __pop_Term_22_5bA_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            21 => {
                // CursorUse = Esc, "[B" => ActionFn(25);
                let __sym1 = __pop_Term_22_5bB_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action25::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            22 => {
                // CursorUse = Esc, "[C" => ActionFn(26);
                let __sym1 = __pop_Term_22_5bC_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            23 => {
                // CursorUse = Esc, "[D" => ActionFn(27);
                let __sym1 = __pop_Term_22_5bD_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            24 => {
                // CursorUse = Esc, "[s" => ActionFn(28);
                let __sym1 = __pop_Term_22_5bs_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            25 => {
                // CursorUse = Esc, "7" => ActionFn(29);
                let __sym1 = __pop_Term_227_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            26 => {
                // CursorUse = Esc, "[u" => ActionFn(30);
                let __sym1 = __pop_Term_22_5bu_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            27 => {
                // CursorUse = Esc, "8" => ActionFn(31);
                let __sym1 = __pop_Term_228_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            28 => {
                // CursorUse = Esc, "D" => ActionFn(32);
                let __sym1 = __pop_Term_22D_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action32::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            29 => {
                // CursorUse = Esc, "M" => ActionFn(33);
                let __sym1 = __pop_Term_22M_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            30 => {
                // CursorUse = Esc, "[K" => ActionFn(34);
                let __sym1 = __pop_Term_22_5bK_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            31 => {
                // CursorUse = Esc, "[1K" => ActionFn(35);
                let __sym1 = __pop_Term_22_5b1K_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            32 => {
                // CursorUse = Esc, "[2K" => ActionFn(36);
                let __sym1 = __pop_Term_22_5b2K_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            33 => {
                // CursorUse = Esc, "[J" => ActionFn(37);
                let __sym1 = __pop_Term_22_5bJ_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            34 => {
                // CursorUse = Esc, "[1J" => ActionFn(38);
                let __sym1 = __pop_Term_22_5b1J_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            35 => {
                // CursorUse = Esc, "[2J" => ActionFn(39);
                let __sym1 = __pop_Term_22_5b2J_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            36 => {
                // CursorUse = Esc, "[", Num, "m" => ActionFn(40);
                let __sym3 = __pop_Term_22m_22(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            37 => {
                // CursorUse = Esc, "[", Num, Maj => ActionFn(41);
                let __sym3 = __pop_NtMaj(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            38 => {
                // CursorUse = Esc, "[", Num, Min => ActionFn(42);
                let __sym3 = __pop_NtMin(__symbols);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            39 => {
                // CursorUse = Esc, "[", Maj => ActionFn(43);
                let __sym2 = __pop_NtMaj(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            40 => {
                // CursorUse = Esc, "[", Min => ActionFn(44);
                let __sym2 = __pop_NtMin(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtEsc(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action44::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtCursorUse(__nt), __end));
                0
            }
            41 => {
                // Esc = r#"[\\x1B-\\x1B]"# => ActionFn(2);
                let __sym0 = __pop_Termr_23_22_5b_5c_5cx1B_2d_5c_5cx1B_5d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtEsc(__nt), __end));
                1
            }
            42 => {
                // Maj = r#"[A-Z]"# => ActionFn(3);
                let __sym0 = __pop_Termr_23_22_5bA_2dZ_5d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMaj(__nt), __end));
                2
            }
            43 => {
                // Min = r#"[a-z]"# => ActionFn(4);
                let __sym0 = __pop_Termr_23_22_5ba_2dz_5d_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMin(__nt), __end));
                3
            }
            44 => {
                // Num = r#"[0-9]+"# => ActionFn(1);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                4
            }
            45 => {
                // __CursorUse = CursorUse => ActionFn(0);
                let __sym0 = __pop_NtCursorUse(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_227_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_227_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_228_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_228_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b0c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b0c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3bc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3bc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22A_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22A_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22B_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22B_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22C_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22C_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22D_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22D_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22H_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22H_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22M_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22M_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b1J_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b1J_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b1K_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b1K_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b2J_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b2J_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b2K_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b2K_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b7h_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b7h_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b7l_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b7l_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_3bH_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_3bH_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_3bf_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_3bf_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_3e0c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_3e0c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bA_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bA_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bB_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bB_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bC_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bC_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bH_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bH_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bJ_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bJ_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bK_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bK_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bf_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bf_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5br_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5br_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bs_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bs_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5bu_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5bu_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22m_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22m_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5bA_2dZ_5d_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5bA_2dZ_5d_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_5c_5cx1B_2d_5c_5cx1B_5d_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5c_5cx1B_2d_5c_5cx1B_5d_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dz_5d_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dz_5d_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCursorUse<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (outp::Cursor), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCursorUse(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtEsc<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtEsc(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMaj<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, char, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMaj(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMin<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, char, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMin(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____CursorUse<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (outp::Cursor), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____CursorUse(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__CursorUse::parse_CursorUse;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        27 => /* '\u{1b}' */ {
                            __current_match = Some((38, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        48 ... 54 => {
                            __current_match = Some((36, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        55 => /* '7' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        56 => /* '8' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        57 => /* '9' */ {
                            __current_match = Some((36, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        65 => /* 'A' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        66 => /* 'B' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        67 => /* 'C' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        68 => /* 'D' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        69 ... 71 => {
                            __current_match = Some((37, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        72 => /* 'H' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        73 ... 76 => {
                            __current_match = Some((37, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        77 => /* 'M' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        78 ... 90 => {
                            __current_match = Some((37, __index + __ch.len_utf8()));
                            __current_state = 10;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((39, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((33, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        100 ... 101 => {
                            __current_match = Some((39, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((34, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        103 ... 108 => {
                            __current_match = Some((39, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((35, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((39, __index + __ch.len_utf8()));
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((36, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((36, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((36, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 => /* '0' */ {
                            __current_state = 20;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        49 => /* '1' */ {
                            __current_state = 22;
                            continue;
                        }
                        50 => /* '2' */ {
                            __current_state = 23;
                            continue;
                        }
                        55 => /* '7' */ {
                            __current_state = 24;
                            continue;
                        }
                        59 => /* ';' */ {
                            __current_state = 25;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((20, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        65 => /* 'A' */ {
                            __current_match = Some((22, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        66 => /* 'B' */ {
                            __current_match = Some((23, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        67 => /* 'C' */ {
                            __current_match = Some((24, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        68 => /* 'D' */ {
                            __current_match = Some((25, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        72 => /* 'H' */ {
                            __current_match = Some((26, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        74 => /* 'J' */ {
                            __current_match = Some((27, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        75 => /* 'K' */ {
                            __current_match = Some((28, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((29, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((30, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((31, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((32, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((36, __index + __ch.len_utf8()));
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        99 => /* 'c' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        74 => /* 'J' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        75 => /* 'K' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        74 => /* 'J' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 41;
                            continue;
                        }
                        75 => /* 'K' */ {
                            __current_match = Some((15, __index + 1));
                            __current_state = 42;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        104 => /* 'h' */ {
                            __current_match = Some((16, __index + 1));
                            __current_state = 43;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((17, __index + 1));
                            __current_state = 44;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        72 => /* 'H' */ {
                            __current_match = Some((18, __index + 1));
                            __current_state = 45;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((19, __index + 1));
                            __current_state = 46;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 => /* '0' */ {
                            __current_state = 47;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        99 => /* 'c' */ {
                            __current_match = Some((21, __index + 1));
                            __current_state = 48;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (outp::Cursor), usize),
) -> (outp::Cursor)
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> u16
{
    u16::from_str(s).unwrap()
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> u16
{
    27
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> char
{
    (s.as_bytes()[0]) as char
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> char
{
    (s.as_bytes()[0]) as char
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::LineWrap(true)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::LineWrap(false)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::ScrollEnable
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::TermVersionIn
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::TermVersionOut(v, w)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::TermVersionOut(v, w)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::TermVersionOut(v, w)
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::TermVersionOut(v, w)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::TermReset
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, x, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorGoto(x, y)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, x, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorGoto(x, y)
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorGoto(1, 1)
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorGoto(1, 1)
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorGoto(1, 1)
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorGoto(1, 1)
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, k, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorUp(k)
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, k, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorDown(k)
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, k, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorRight(k)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, k, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorLeft(k)
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorUp(1)
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorDown(1)
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorRight(1)
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::CursorLeft(1)
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::SaveCursor
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::SaveCursor
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::RestoreCursor
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::RestoreCursor
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::ScrollUp
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::ScrollDown
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::EraseRightLine
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::EraseLeftLine
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::EraseLine
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::EraseDown
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::EraseUp
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::Clear
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, attr, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> (outp::Cursor)
{
    outp::Cursor::Attribute(attr)
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, u16, usize),
    (_, c, _): (usize, char, usize),
) -> (outp::Cursor)
{
    outp::Cursor::Unimplemented(r, c)
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, u16, usize),
    (_, c, _): (usize, char, usize),
) -> (outp::Cursor)
{
    outp::Cursor::Unimplemented(r, c)
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, c, _): (usize, char, usize),
) -> (outp::Cursor)
{
    outp::Cursor::Unimplemented(0, c)
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, c, _): (usize, char, usize),
) -> (outp::Cursor)
{
    outp::Cursor::Unimplemented(0, c)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
