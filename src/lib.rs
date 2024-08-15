
// https://github.com/kkkgg/chord_finder/blob/6d905887bb2cea69da847b65a8a14d9ce9bcb286/js/chord_finder.js#L289

pub fn chord_recognize(mut notes: Vec<u8>) -> String {
    let mut chord: String = String::from("");
    let mut intervals: Vec<u8> = vec![];

    notes.retain(|&note|  128 < note);

    if notes.len() < 2 {
        return String::from("")
    }

    notes.sort();

    chord.push_str(
        match notes[0] % 12 {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "D#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            _ => "Nan"
        }
    );

    for note in notes {
        if !intervals.contains(&(note % 12)) {
            intervals.push(note % 12)
        }
    }

    let mut interval_distances: Vec<u8> = Vec::new();
    for i in 1..intervals.len() {
        interval_distances.push((intervals[i] + 12 - intervals[i - 1]) % 12);
    }

    if !intervals.is_empty() {
        interval_distances.push((intervals[0] + 12 - intervals[intervals.len() - 1]) % 12);
    }

    chord.push_str(
        match interval_distances.as_slice() {
            //2 chord
            [0, 7] => "5",

            //3 chord
            [0, 4, 7] => "Major",
            [0, 3, 7] => "Minor",
            [0, 4, 6] => "(-5)",
            [0, 3, 6] => "m-5",
            [0, 4, 8] => "aug",
            [0, 3, 8] => "m+5",
            [0, 5, 7] => "sus4",
            [0, 2, 7] => "sus2",

            //4 chord
            [0, 4, 7, 9] => "6",
            [0, 3, 7, 9] => "m6",
            [0, 4, 7, 10] => "7",
            [0, 3, 7, 10] => "m7",
            [0, 4, 7, 11] => "Major 7",
            [0, 3, 7, 11] => "Minor Major 7",
            [0, 4, 6, 10] => "7-5",
            [0, 3, 6, 10] => "Minor 7-5",
            [0, 3, 6, 11] => "Minor Major 7-5",
            [0, 5, 7, 10] => "7sus4",
            [0, 3, 6, 9] => "Dim7",
            [0, 2, 4, 7] => "add9",
            [0, 2, 3, 7] => "Minor(add9)",
            [0, 4, 5, 7] => "add4",
            [0, 4, 8, 10] => "aug7",

            //5 tension chord
            [0, 2, 4, 7, 10] => "9",
            [0, 3, 4, 7, 10] => "+9",
            [0, 2, 3, 7, 10] => "Minor 9",
            [0, 2, 4, 7, 11] => "Major 9",
            [0, 2, 3, 7, 11] => "Minor Major 9",
            [0, 2, 4, 7, 9] => "69",
            [0, 2, 3, 7, 9] => "Minor 69",

            //6 tension chord
            [0, 2, 4, 5, 7, 10] => "11",
            [0, 2, 3, 6, 7, 10] => "+11",
            [0, 2, 3, 5, 7, 10] => "Minor 11",
            [0, 2, 3, 6, 7, 10] => "Minor + 11",
            [0, 2, 4, 5, 7, 11] => "Major 11",
            [0, 2, 3, 5, 7, 11] => "Minor Major 11",

            //7 tension chord
            [0, 2, 4, 5, 7, 9, 10] => "13",
            [0, 2, 3, 5, 7, 9, 10] => "Minor 13",
            [0, 2, 3, 5, 7, 8, 10] => "Minor -13",
            [0, 2, 4, 5, 7, 9, 11] => "Major 13",
            [0, 2, 3, 5, 7, 9, 11] => "Minor Major 13",

            _ => "",
        }
    );

    chord
}

#[cfg(test)]
mod test {
    use crate::chord_recognize;

    #[test]
    fn main() {
        print!("{}", chord_recognize(vec![60, 63, 67, 69]))
    }
}