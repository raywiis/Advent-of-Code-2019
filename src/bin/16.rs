fn main () {
    // let input_string = String::from("12345678");
    // let input_string = String::from("80871224585914546619083218645595");
    // let mut input_string = String::from("19617804207202209144916044189917");
    // let mut input_string = String::from("69317163492948606335995924319873");
    let mut input_string = String::from("59791875142707344554745984624833270124746225787022156176259864082972613206097260696475359886661459314067969858521185244807128606896674972341093111690401527976891268108040443281821862422244152800144859031661510297789792278726877676645835805097902853584093615895099152578276185267316851163313487136731134073054989870018294373731775466754420075119913101001966739563592696702233028356328979384389178001923889641041703308599918672055860556825287836987992883550004999016194930620165247185883506733712391462975446192414198344745434022955974228926237100271949068464343172968939069550036969073411905889066207300644632441054836725463178144030305115977951503567");
    let start = 5979187;
    // let mut input_string = String::from("03036732577212944063491565474664");
    // let start = 303673;
    // let mut input_string = String::from("02935109699940807407585447034323");
    // let start = 293510;
    // let mut input_string = String::from("03081770884921959731165446850517");
    // let start = 308177;
    let mut input_signal: Vec<i32> = input_string.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    println!("parse complete");

    let phase_count = 100;
    let mut result = fft(input_signal.clone(), vec!(0, 1, 0, -1), phase_count);
    result.truncate(8);
    println!("Part 1 {:?}", result);

    input_string = input_string.repeat(10000);
    input_signal = input_string.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut part_2 = reverse_fft(input_signal, start);
    part_2.truncate(8);
    println!("Part 2 {:?}", part_2);
}

fn reverse_fft(
    initial_list: Vec<i32>,
    start: i32
) -> Vec<i32> {
    let mut last_signal = initial_list.clone();
    last_signal.reverse();
    let list_cap = (last_signal.len() as i32) - start;

    for phase in 0..100 {
        let mut signal = vec!();
        let mut sum = 0;
        for i in 0..list_cap {
            sum += last_signal[i as usize];
            signal.push(sum % 10);
        }
        last_signal = signal.clone();
        signal = vec!();
    }

    last_signal.reverse();
    last_signal
}

fn fft(
    initial_list: Vec<i32>,
    pattern: Vec<i32>,
    phase_count: i32,
    // len_mult: i32,
) -> Vec<i32> {
    let mut last_signal = initial_list;
    let mut signal = vec!();

    for phase in 0..phase_count {
        for i in 0..last_signal.len() {
            let mut value = 0;
            for j in i..last_signal.len() {
                let pattern_idx = ((j + 1) / (i + 1)) % pattern.len();
                value += (last_signal[j] * pattern[pattern_idx]) % 10;
                // print!("{:2}", pattern[pattern_idx]);
                // println!("+ {}", last_signal[j] * pattern[pattern_idx]);
            };
            // println!("{}", i);
            signal.push(value.abs() % 10);
            // println!("= {}, {}", value.abs() % 10, value);
        };
        last_signal = signal.clone();
        signal = vec!();
    }

    last_signal
}
