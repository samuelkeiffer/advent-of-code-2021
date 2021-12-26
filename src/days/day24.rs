use crate::*;

pub fn run() {
    part1();
    part2();
}

fn part2() {
    let D24Input(input) = get_input::<D24Input>("day-24");
    let use_code = true;
    let smallest = (11111111111111..=31521119151421)
        .into_par_iter()
        .find_first(|model_number| {
            if use_code {
                if validation(*model_number) {
                    dbg!(model_number);
                    true
                } else {
                    false
                }
            } else {
                let mut buffer_in = *model_number;
                let mut buffer = VecDeque::new();
                while buffer.len() < 14 {
                    buffer.push_front(buffer_in % 10);
                    buffer_in /= 10;
                }
                if buffer.contains(&0) {
                    return false;
                }
                let mut alu = Alu::new();
                for op in &input {
                    alu = alu.operation(*op, &mut buffer);
                }
                if alu.z == 0 {
                    dbg!(model_number);
                    true
                } else {
                    false
                }
            }
        })
        .unwrap();
    dbg!(smallest);
}

// Successes:
// [src\days\day24.rs:31] model_number = 31521119151421
// [src\days\day24.rs:31] model_number = 31821419151421
// [src\days\day24.rs:31] model_number = 32854419151431
// [src\days\day24.rs:31] model_number = 33554119151441
// [src\days\day24.rs:27] model_number = 43754319151442
// [src\days\day24.rs:27] model_number = 43754319152542
// [src\days\day24.rs:27] model_number = 43754319153642
// [src\days\day24.rs:27] model_number = 43754319154742
// [src\days\day24.rs:27] model_number = 43754319155842
// [src\days\day24.rs:27] model_number = 43754319156942
// [src\days\day24.rs:27] model_number = 43754319261442
// [src\days\day24.rs:27] model_number = 43754319262542
// [src\days\day24.rs:27] model_number = 43754319263642
// [src\days\day24.rs:27] model_number = 43754319264742
// [src\days\day24.rs:27] model_number = 43754319265842
// [src\days\day24.rs:27] model_number = 43754319266942
// [src\days\day24.rs:27] model_number = 43754319371442
// [src\days\day24.rs:27] model_number = 43754319372542
// [src\days\day24.rs:27] model_number = 43754319373642
// [src\days\day24.rs:27] model_number = 43754319374742
// [src\days\day24.rs:27] model_number = 43754319375842
// [src\days\day24.rs:27] model_number = 43754319376942
// [src\days\day24.rs:27] model_number = 43754319481442
// [src\days\day24.rs:27] model_number = 43754319482542
// [src\days\day24.rs:27] model_number = 43754319483642
// [src\days\day24.rs:27] model_number = 43754319484742
// [src\days\day24.rs:27] model_number = 43754319485842
// [src\days\day24.rs:27] model_number = 43754319486942
// [src\days\day24.rs:27] model_number = 43754319591442
// [src\days\day24.rs:27] model_number = 43754319592542
// [src\days\day24.rs:27] model_number = 43754319593642
// [src\days\day24.rs:27] model_number = 43754319594742
// [src\days\day24.rs:27] model_number = 43754319595842
// [src\days\day24.rs:27] model_number = 43754319596942
// [src\days\day24.rs:59] model_number = 85943519151466
// [src\days\day24.rs:59] model_number = 85943519152566
// [src\days\day24.rs:59] model_number = 85943519153666
// [src\days\day24.rs:59] model_number = 85943519154766
// [src\days\day24.rs:59] model_number = 85943519155866
// [src\days\day24.rs:59] model_number = 85943519156966
// [src\days\day24.rs:59] model_number = 85943519261466
// [src\days\day24.rs:59] model_number = 85943519262566
// [src\days\day24.rs:59] model_number = 85943519263666
// [src\days\day24.rs:59] model_number = 85943519264766
// [src\days\day24.rs:59] model_number = 85943519265866
// [src\days\day24.rs:59] model_number = 85943519266966
// [src\days\day24.rs:59] model_number = 85943519371466
// [src\days\day24.rs:59] model_number = 85943519372566
// [src\days\day24.rs:59] model_number = 85943519373666
// [src\days\day24.rs:59] model_number = 85943519374766
// [src\days\day24.rs:59] model_number = 85943519375866
// [src\days\day24.rs:59] model_number = 85943519376966
// [src\days\day24.rs:59] model_number = 85943519481466
// [src\days\day24.rs:59] model_number = 85943519482566
// [src\days\day24.rs:59] model_number = 85943519483666
// [src\days\day24.rs:59] model_number = 85943519484766
// [src\days\day24.rs:59] model_number = 85943519485866
// [src\days\day24.rs:59] model_number = 85943519486966
// [src\days\day24.rs:59] model_number = 85943519591466
// [src\days\day24.rs:59] model_number = 85943519592566
// [src\days\day24.rs:59] model_number = 85943519593666
// [src\days\day24.rs:59] model_number = 85943519594766
// [src\days\day24.rs:59] model_number = 85943519595866
// [src\days\day24.rs:59] model_number = 85943519596966
// [src\days\day24.rs:89] model_number = 87921519151486
// [src\days\day24.rs:89] model_number = 87921519152586
// [src\days\day24.rs:89] model_number = 87921519153686
// [src\days\day24.rs:89] model_number = 87921519154786
// [src\days\day24.rs:89] model_number = 87921519155886
// [src\days\day24.rs:89] model_number = 87921519156986
// [src\days\day24.rs:89] model_number = 87921519261486
// [src\days\day24.rs:89] model_number = 87921519262586
// [src\days\day24.rs:89] model_number = 87921519263686
// [src\days\day24.rs:89] model_number = 87921519264786
// [src\days\day24.rs:89] model_number = 87921519265886
// [src\days\day24.rs:89] model_number = 87921519266986
// [src\days\day24.rs:89] model_number = 87921519371486
// [src\days\day24.rs:89] model_number = 87921519372586
// [src\days\day24.rs:89] model_number = 87921519373686
// [src\days\day24.rs:89] model_number = 87921519374786
// [src\days\day24.rs:89] model_number = 87921519375886
// [src\days\day24.rs:89] model_number = 87921519376986
// [src\days\day24.rs:89] model_number = 87921519481486
// [src\days\day24.rs:89] model_number = 87921519482586
// [src\days\day24.rs:89] model_number = 87921519483686
// [src\days\day24.rs:89] model_number = 87921519484786
// [src\days\day24.rs:89] model_number = 87921519485886
// [src\days\day24.rs:89] model_number = 87921519486986
// [src\days\day24.rs:89] model_number = 87921519591486
// [src\days\day24.rs:89] model_number = 87921519592586
// [src\days\day24.rs:89] model_number = 87921519593686
// [src\days\day24.rs:89] model_number = 87921519594786
// [src\days\day24.rs:89] model_number = 87921519595886
// [src\days\day24.rs:89] model_number = 87921519596986
// [src\days\day24.rs:125] model_number = 91698219151427
// [src\days\day24.rs:125] model_number = 91698219152527
// [src\days\day24.rs:125] model_number = 91698219153627
// [src\days\day24.rs:125] model_number = 91698219154727
// [src\days\day24.rs:125] model_number = 91698219155827
// [src\days\day24.rs:125] model_number = 91698219156927
// [src\days\day24.rs:125] model_number = 91698219261427
// [src\days\day24.rs:125] model_number = 91698219262527
// [src\days\day24.rs:125] model_number = 91698219263627
// [src\days\day24.rs:125] model_number = 91698219264727
// [src\days\day24.rs:125] model_number = 91698219265827
// [src\days\day24.rs:125] model_number = 91698219266927
// [src\days\day24.rs:125] model_number = 91698219371427
// [src\days\day24.rs:125] model_number = 91698219372527
// [src\days\day24.rs:125] model_number = 91698219373627
// [src\days\day24.rs:125] model_number = 91698219374727
// [src\days\day24.rs:125] model_number = 91698219375827
// [src\days\day24.rs:125] model_number = 91698219376927
// [src\days\day24.rs:125] model_number = 91698219481427
// [src\days\day24.rs:125] model_number = 91698219482527
// [src\days\day24.rs:125] model_number = 91698219483627
// [src\days\day24.rs:125] model_number = 91698219484727
// [src\days\day24.rs:125] model_number = 91698219485827
// [src\days\day24.rs:125] model_number = 91698219486927
// [src\days\day24.rs:125] model_number = 91698219591427
// [src\days\day24.rs:125] model_number = 91698219592527
// [src\days\day24.rs:125] model_number = 91698219593627
// [src\days\day24.rs:125] model_number = 91698219594727
// [src\days\day24.rs:125] model_number = 91698219595827
// [src\days\day24.rs:125] model_number = 91698219596927
// [src\days\day24.rs:155] model_number = 96887419151477
// [src\days\day24.rs:155] model_number = 96887419152577
// [src\days\day24.rs:155] model_number = 96887419153677
// [src\days\day24.rs:155] model_number = 96887419154777
// [src\days\day24.rs:155] model_number = 96887419155877
// [src\days\day24.rs:155] model_number = 96887419156977
// [src\days\day24.rs:155] model_number = 96887419261477
// [src\days\day24.rs:155] model_number = 96887419262577
// [src\days\day24.rs:155] model_number = 96887419263677
// [src\days\day24.rs:155] model_number = 96887419264777
// [src\days\day24.rs:155] model_number = 96887419265877
// [src\days\day24.rs:155] model_number = 96887419266977
// [src\days\day24.rs:155] model_number = 96887419371477
// [src\days\day24.rs:155] model_number = 96887419372577
// [src\days\day24.rs:155] model_number = 96887419373677
// [src\days\day24.rs:155] model_number = 96887419374777
// [src\days\day24.rs:155] model_number = 96887419375877
// [src\days\day24.rs:155] model_number = 96887419376977
// [src\days\day24.rs:155] model_number = 96887419481477
// [src\days\day24.rs:155] model_number = 96887419482577
// [src\days\day24.rs:155] model_number = 96887419483677
// [src\days\day24.rs:155] model_number = 96887419484777
// [src\days\day24.rs:155] model_number = 96887419485877
// [src\days\day24.rs:155] model_number = 96887419486977
// [src\days\day24.rs:155] model_number = 96887419591477
// [src\days\day24.rs:155] model_number = 96887419592577
// [src\days\day24.rs:155] model_number = 96887419593677
// [src\days\day24.rs:155] model_number = 96887419594777
// [src\days\day24.rs:155] model_number = 96887419595877
// [src\days\day24.rs:155] model_number = 96887419596977
// [src\days\day24.rs:185] model_number = 96987519151477
// [src\days\day24.rs:185] model_number = 96987519152577
// [src\days\day24.rs:185] model_number = 96987519153677
// [src\days\day24.rs:185] model_number = 96987519154777
// [src\days\day24.rs:185] model_number = 96987519155877
// [src\days\day24.rs:185] model_number = 96987519156977
// [src\days\day24.rs:185] model_number = 96987519261477
// [src\days\day24.rs:185] model_number = 96987519262577
// [src\days\day24.rs:185] model_number = 96987519263677
// [src\days\day24.rs:185] model_number = 96987519264777
// [src\days\day24.rs:185] model_number = 96987519265877
// [src\days\day24.rs:185] model_number = 96987519266977
// [src\days\day24.rs:185] model_number = 96987519371477
// [src\days\day24.rs:185] model_number = 96987519372577
// [src\days\day24.rs:185] model_number = 96987519373677
// [src\days\day24.rs:185] model_number = 96987519374777
// [src\days\day24.rs:185] model_number = 96987519375877
// [src\days\day24.rs:185] model_number = 96987519376977
// [src\days\day24.rs:185] model_number = 96987519481477
// [src\days\day24.rs:185] model_number = 96987519482577
// [src\days\day24.rs:185] model_number = 96987519483677
// [src\days\day24.rs:185] model_number = 96987519484777
// [src\days\day24.rs:185] model_number = 96987519485877
// [src\days\day24.rs:185] model_number = 96987519486977
// [src\days\day24.rs:185] model_number = 96987519591477
// [src\days\day24.rs:185] model_number = 96987519592577
// [src\days\day24.rs:185] model_number = 96987519593677
// [src\days\day24.rs:185] model_number = 96987519594777
// [src\days\day24.rs:185] model_number = 96987519595877
// [src\days\day24.rs:185] model_number = 96987519596977
// [src\days\day24.rs:215] model_number = 97554119151487
// [src\days\day24.rs:215] model_number = 97554119152587
// [src\days\day24.rs:215] model_number = 97554119153687
// [src\days\day24.rs:215] model_number = 97554119154787
// [src\days\day24.rs:215] model_number = 97554119155887
// [src\days\day24.rs:215] model_number = 97554119156987
// [src\days\day24.rs:215] model_number = 97554119261487
// [src\days\day24.rs:215] model_number = 97554119262587
// [src\days\day24.rs:215] model_number = 97554119263687
// [src\days\day24.rs:215] model_number = 97554119264787
// [src\days\day24.rs:215] model_number = 97554119265887
// [src\days\day24.rs:215] model_number = 97554119266987
// [src\days\day24.rs:215] model_number = 97554119371487
// [src\days\day24.rs:215] model_number = 97554119372587
// [src\days\day24.rs:215] model_number = 97554119373687
// [src\days\day24.rs:215] model_number = 97554119374787
// [src\days\day24.rs:215] model_number = 97554119375887
// [src\days\day24.rs:215] model_number = 97554119376987
// [src\days\day24.rs:215] model_number = 97554119481487
// [src\days\day24.rs:215] model_number = 97554119482587
// [src\days\day24.rs:215] model_number = 97554119483687
// [src\days\day24.rs:215] model_number = 97554119484787
// [src\days\day24.rs:215] model_number = 97554119485887
// [src\days\day24.rs:215] model_number = 97554119486987
// [src\days\day24.rs:215] model_number = 97554119591487
// [src\days\day24.rs:215] model_number = 97554119592587
// [src\days\day24.rs:215] model_number = 97554119593687
// [src\days\day24.rs:215] model_number = 97554119594787
// [src\days\day24.rs:215] model_number = 97554119595887
// [src\days\day24.rs:215] model_number = 97554119596987
// [src\days\day24.rs:215] model_number = 97976519151487
// [src\days\day24.rs:215] model_number = 97976519152587
// [src\days\day24.rs:215] model_number = 97976519153687
// [src\days\day24.rs:215] model_number = 97976519154787
// [src\days\day24.rs:215] model_number = 97976519155887
// [src\days\day24.rs:215] model_number = 97976519156987
// [src\days\day24.rs:215] model_number = 97976519261487
// [src\days\day24.rs:215] model_number = 97976519262587
// [src\days\day24.rs:215] model_number = 97976519263687
// [src\days\day24.rs:215] model_number = 97976519264787
// [src\days\day24.rs:215] model_number = 97976519265887
// [src\days\day24.rs:215] model_number = 97976519266987
// [src\days\day24.rs:215] model_number = 97976519371487
// [src\days\day24.rs:215] model_number = 97976519372587
// [src\days\day24.rs:215] model_number = 97976519373687
// [src\days\day24.rs:215] model_number = 97976519374787
// [src\days\day24.rs:215] model_number = 97976519375887
// [src\days\day24.rs:215] model_number = 97976519376987
// [src\days\day24.rs:215] model_number = 97976519481487
// [src\days\day24.rs:215] model_number = 97976519482587
// [src\days\day24.rs:215] model_number = 97976519483687
// [src\days\day24.rs:215] model_number = 97976519484787
// [src\days\day24.rs:215] model_number = 97976519485887
// [src\days\day24.rs:215] model_number = 97976519486987
// [src\days\day24.rs:215] model_number = 97976519591487
// [src\days\day24.rs:215] model_number = 97976519592587
// [src\days\day24.rs:215] model_number = 97976519593687
// [src\days\day24.rs:215] model_number = 97976519594787
// [src\days\day24.rs:215] model_number = 97976519595887
// [src\days\day24.rs:215] model_number = 97976519596987
// [src\days\day24.rs:215] model_number = 98965519151497
// [src\days\day24.rs:215] model_number = 98965519152597
// [src\days\day24.rs:215] model_number = 98965519153697
// [src\days\day24.rs:215] model_number = 98965519154797
// [src\days\day24.rs:215] model_number = 98965519155897
// [src\days\day24.rs:215] model_number = 98965519156997
// [src\days\day24.rs:215] model_number = 98965519261497
// [src\days\day24.rs:215] model_number = 98965519262597
// [src\days\day24.rs:215] model_number = 98965519263697
// [src\days\day24.rs:215] model_number = 98965519264797
// [src\days\day24.rs:215] model_number = 98965519265897
// [src\days\day24.rs:215] model_number = 98965519266997
// [src\days\day24.rs:215] model_number = 98965519371497
// [src\days\day24.rs:215] model_number = 98965519372597
// [src\days\day24.rs:215] model_number = 98965519373697
// [src\days\day24.rs:215] model_number = 98965519374797
// [src\days\day24.rs:215] model_number = 98965519375897
// [src\days\day24.rs:215] model_number = 98965519376997
// [src\days\day24.rs:215] model_number = 98965519481497
// [src\days\day24.rs:215] model_number = 98965519482597
// [src\days\day24.rs:215] model_number = 98965519483697
// [src\days\day24.rs:215] model_number = 98965519484797
// [src\days\day24.rs:215] model_number = 98965519485897
// [src\days\day24.rs:215] model_number = 98965519486997
// [src\days\day24.rs:215] model_number = 98965519591497
// [src\days\day24.rs:215] model_number = 98965519592597
// [src\days\day24.rs:215] model_number = 98965519593697
// [src\days\day24.rs:215] model_number = 98965519594797
// [src\days\day24.rs:215] model_number = 98965519595897
// [src\days\day24.rs:215] model_number = 98965519596997
// [src\days\day24.rs:305] model_number = 98998519151497
// [src\days\day24.rs:305] model_number = 98998519152597
// [src\days\day24.rs:305] model_number = 98998519153697
// [src\days\day24.rs:305] model_number = 98998519154797
// [src\days\day24.rs:305] model_number = 98998519155897
// [src\days\day24.rs:305] model_number = 98998519156997
// [src\days\day24.rs:305] model_number = 98998519261497
// [src\days\day24.rs:305] model_number = 98998519262597
// [src\days\day24.rs:305] model_number = 98998519263697
// [src\days\day24.rs:305] model_number = 98998519264797
// [src\days\day24.rs:305] model_number = 98998519265897
// [src\days\day24.rs:305] model_number = 98998519266997
// [src\days\day24.rs:305] model_number = 98998519371497
// [src\days\day24.rs:305] model_number = 98998519372597
// [src\days\day24.rs:305] model_number = 98998519373697
// [src\days\day24.rs:305] model_number = 98998519374797
// [src\days\day24.rs:305] model_number = 98998519375897
// [src\days\day24.rs:305] model_number = 98998519376997
// [src\days\day24.rs:305] model_number = 98998519481497
// [src\days\day24.rs:305] model_number = 98998519482597
// [src\days\day24.rs:305] model_number = 98998519483697
// [src\days\day24.rs:305] model_number = 98998519484797
// [src\days\day24.rs:305] model_number = 98998519485897
// [src\days\day24.rs:305] model_number = 98998519486997
// [src\days\day24.rs:305] model_number = 98998519591497
// [src\days\day24.rs:305] model_number = 98998519592597
// [src\days\day24.rs:305] model_number = 98998519593697
// [src\days\day24.rs:305] model_number = 98998519594797
// [src\days\day24.rs:305] model_number = 98998519595897
// [src\days\day24.rs:305] model_number = 98998519596997

fn part1() {
    let D24Input(input) = get_input::<D24Input>("day-24");
    let use_code = true;
    let largest = (98998519596997..=99999999999999i64)
        .into_par_iter()
        .find_last(|model_number| {
            if use_code {
                if validation(*model_number) {
                    dbg!(model_number);
                    true
                } else {
                    false
                }
            } else {
                let mut buffer_in = *model_number;
                let mut buffer = VecDeque::new();
                while buffer.len() < 14 {
                    buffer.push_front(buffer_in % 10);
                    buffer_in /= 10;
                }
                if buffer.contains(&0) {
                    return false;
                }
                let mut alu = Alu::new();
                for op in &input {
                    alu = alu.operation(*op, &mut buffer);
                }
                if alu.z == 0 {
                    dbg!(model_number);
                    true
                } else {
                    false
                }
            }
        })
        .unwrap();
    dbg!(largest);
}

fn validation(serial_number: i64) -> bool {
    let mut buffer = serial_number;
    let mut digits = [0; 14];
    for i in (0..14).rev() {
        digits[i] = buffer % 10;
        buffer /= 10;
    }

    if digits.contains(&0) {
        return false;
    }

    let mut z = digits[0];
    z += 10;

    z *= 26;
    z += digits[1];
    z += 16;

    z *= 26;
    z += digits[2];

    z *= 26;
    z += digits[3];
    z += 13;

    let w = digits[4];
    let mut x = z;
    x %= 26;
    z /= 26;
    x -= 14;
    x = if x != w { 1 } else { 0 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    let mut y = w;
    y += 7;
    y *= x;
    z += y;

    let w = digits[5];
    let mut x = z;
    x %= 26;
    z /= 26;
    x -= 4;
    x = if x != w { 1 } else { 0 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    let mut y = w;
    y += 11;
    y *= x;
    z += y;

    z *= 26;
    z += digits[6];
    z += 11;

    let w = digits[7];
    let mut x = z;
    x %= 26;
    z /= 26;
    x -= 3;
    x = if x != w { 1 } else { 0 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    let mut y = w;
    y += 10;
    y *= x;
    z += y;

    z *= 26;
    z += digits[8];
    z += 16;

    let w = digits[9];
    let mut x = z;
    x %= 26;
    z /= 26;
    x -= 12;
    x = if x != w { 1 } else { 0 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    let mut y = w;
    y += 8;
    y *= x;
    z += y;

    z *= 26;
    z += digits[10];
    z += 15;

    let w = digits[11];
    let mut x = z;
    x %= 26;
    z /= 26;
    x -= 12;
    x = if x != w { 1 } else { 0 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    let mut y = w;
    y += 2;
    y *= x;
    z += y;

    let w = digits[12];
    let mut x = z;
    x %= 26;
    z /= 26;
    x -= 15;
    x = if x != w { 1 } else { 0 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    let mut y = w;
    y += 5;
    y *= x;
    z += y;

    let w = digits[13];
    let mut x = z;
    x %= 26;
    z /= 26;
    x -= 12;
    x = if x != w { 1 } else { 0 };
    let mut y = 25;
    y *= x;
    y += 1;
    z *= y;
    let mut y = w;
    y += 10;
    y *= x;
    z += y;

    z == 0
}

#[derive(Clone, Deserialize, Copy)]
struct Operation {
    kind: OperationKind,
    target: Field,
    input: Input,
}

#[derive(Clone, Deserialize, Copy)]
enum OperationKind {
    Input,
    Addition,
    Multiplication,
    Division,
    Modulus,
    Equality,
    Inequality,
}

#[derive(Clone, Deserialize, Copy)]
enum Input {
    Field(Field),
    Value(i64),
    BufferFront,
}

impl Alu {
    fn operation(mut self, operation: Operation, buffer: &mut VecDeque<i64>) -> Self {
        let input = self.input_value(operation.input, buffer);
        let compute = |field| match operation.kind {
            OperationKind::Input => input,
            OperationKind::Addition => field + input,
            OperationKind::Multiplication => field * input,
            OperationKind::Division => field / input,
            OperationKind::Modulus => field % input,
            OperationKind::Equality => (field == input) as i64,
            OperationKind::Inequality => (field != input) as i64,
        };

        match operation.target {
            Field::W => self.w = compute(self.w),
            Field::X => self.x = compute(self.x),
            Field::Y => self.y = compute(self.y),
            Field::Z => self.z = compute(self.z),
        }

        self
    }

    fn input_value(&self, input: Input, buffer: &mut VecDeque<i64>) -> i64 {
        match input {
            Input::Field(field) => match field {
                Field::W => self.w,
                Field::X => self.x,
                Field::Y => self.y,
                Field::Z => self.z,
            },
            Input::Value(value) => value,
            Input::BufferFront => buffer.pop_front().unwrap(),
        }
    }

    fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

#[derive(Debug)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Deserialize, Copy)]
enum Field {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Deserialize)]
struct D24Input(Vec<Operation>);

impl Asset for D24Input {
    const EXTENSION: &'static str = "ron";
    type Loader = RonLoader;
}
