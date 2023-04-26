use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brainfuck")?;

    cmd.arg("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    cmd.assert().success().stdout("Hello World!\n");
    Ok(())
}

#[test]
#[ignore = "file input not supported yet"]
fn input_from_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.")?;

    let mut cmd = Command::cargo_bin("brainfuck")?;

    cmd.arg("|").arg(file.path());

    cmd.assert().success().stdout("Hello World!\n");

    Ok(())
}

// Tests based on http://www.brainfuck.org/tests.b
#[test]
#[ignore = "reason"]
fn io() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brainfuck")?;

    cmd.arg(">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.");
    cmd.assert().success().stdout("LA\n");

    Ok(())
}

#[test]
fn big_enough() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brainfuck")?;

    cmd.arg("++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]+++++[>+++++++<<++>-]>.<<.");
    cmd.assert().success().stdout("#\n");

    Ok(())
}

#[test]
fn several_obscure() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brainfuck")?;

    cmd.arg(
        "[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]\"A*$\";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>.",
    );
    cmd.assert().success().stdout("H\n");

    Ok(())
}

#[test]
fn unmatched_lhs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brainfuck")?;

    cmd.arg("+++++[>+++++++>++<<-]>.>.[");
    cmd.assert().failure().stderr(predicate::str::contains("["));

    Ok(())
}

#[test]
fn unmatched_rhs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brainfuck")?;

    cmd.arg("+++++[>+++++++>++<<-]>.>.][");
    cmd.assert().failure().stderr(predicate::str::contains("]"));

    Ok(())
}

#[test]
#[ignore = "figuring out how to test this"]
fn rot_13() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("brainfuck")?;

    let inputs = ["~", "m", "l", "k", " ", "x", "y", "z", "\n"];
    let rot_13 = "
-,+[                         Read first character and start outer character reading loop
    -[                       Skip forward if character is 0
        >>++++[>++++++++<-]  Set up divisor (32) for division loop
                               (MEMORY LAYOUT: dividend copy remainder divisor quotient zero zero)
        <+<-[                Set up dividend (x minus 1) and enter division loop
            >+>+>-[>>>]      Increase copy and remainder / reduce divisor / Normal case: skip forward
            <[[>+<-]>>+>]    Special case: move remainder back to divisor and increase quotient
            <<<<<-           Decrement dividend
        ]                    End division loop
    ]>>>[-]+                 End skip loop; zero former divisor and reuse space for a flag
    >--[-[<->+++[-]]]<[         Zero that flag unless quotient was 2 or 3; zero quotient; check flag
        ++++++++++++<[       If flag then set up divisor (13) for second division loop
                               (MEMORY LAYOUT: zero copy dividend divisor remainder quotient zero zero)
            >-[>+>>]         Reduce divisor; Normal case: increase remainder
            >[+[<+>-]>+>>]   Special case: increase remainder / move it back to divisor / increase quotient
            <<<<<-           Decrease dividend
        ]                    End division loop
        >>[<+>-]             Add remainder back to divisor to get a useful 13
        >[                   Skip forward if quotient was 0
            -[               Decrement quotient and skip forward if quotient was 1
                -<<[-]>>     Zero quotient and divisor if quotient was 2
            ]<<[<<->>-]>>    Zero divisor and subtract 13 from copy if quotient was 1
        ]<<[<<+>>-]          Zero divisor and add 13 to copy if quotient was 0
    ]                        End outer skip loop (jump to here if ((character minus 1)/32) was not 2 or 3)
    <[-]                     Clear remainder from first division if second division was skipped
    <.[-]                    Output ROT13ed character from copy and clear it
    <-,+                     Read next character
]                            End character reading loop
";
    cmd.arg(rot_13)
        .write_stdin(inputs[0])
        .write_stdin(inputs[1])
        .write_stdin(inputs[2])
        .write_stdin(inputs[3])
        .write_stdin(inputs[4])
        .write_stdin(inputs[5])
        .write_stdin(inputs[6])
        .write_stdin(inputs[7])
        .write_stdin(inputs[8]);

    cmd.assert().success().stdout("~zyx mlk\n");

    Ok(())
}
