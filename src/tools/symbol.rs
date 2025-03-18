pub fn generate_all_shanghai_stock_symbol(append_sh: bool) -> Vec<String> {
    let mut codes = Vec::new();

    // 生成 A 股主板代码（600、601、603、605 开头）
    for prefix in ["600", "601", "603", "605"] {
        for i in 0..1000 {
            let code = format!("{}{:03}", prefix, i);
            if append_sh {
                codes.push(format!("SH{}", code));
            } else {
                codes.push(code);
            }
        }
    }

    // 生成科创板代码（688 开头）
    for i in 0..1000 {
        let code = format!("688{:03}", i);
        if append_sh {
            codes.push(format!("SH{}", code));
        } else {
            codes.push(code);
        }
    }

    // 生成 B 股代码（900 开头）
    for i in 0..1000 {
        let code = format!("900{:03}", i);
        if append_sh {
            codes.push(format!("SH{}", code));
        } else {
            codes.push(code);
        }
    }

    codes
}