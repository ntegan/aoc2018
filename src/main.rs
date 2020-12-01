mod input;
mod myerror;

//// Day 4
// on the walls, someone has been writing down the ID of the one guard on duty
// every night at midnight for the past few months.
// elves decided one guard is enough for the night shift.
// they've also decided to write down when they fall asleep or wake up while
// at their post (puzzle input).
//
// something like
// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
// [1518-11-01 00:30] falls asleep
// [1518-11-01 00:55] wakes up
// [1518-11-01 23:58] Guard #99 begins shift
// [1518-11-02 00:40] falls asleep
// [1518-11-02 00:50] wakes up
// [1518-11-03 00:05] Guard #10 begins shift
// [1518-11-03 00:24] falls asleep
// [1518-11-03 00:29] wakes up
// [1518-11-04 00:02] Guard #99 begins shift
// [1518-11-04 00:36] falls asleep
// [1518-11-04 00:46] wakes up
// [1518-11-05 00:03] Guard #99 begins shift
// [1518-11-05 00:45] falls asleep
// [1518-11-05 00:55] wakes up
//
// timestamps year-month-day hour:minute.
// guard falling asleep/waking up is always the one whose shift
// most recently started.
// All times are during midnight hour (00:00-00:59) only minute (00-59) is important.
//
// Here is when guards are asleep, visually
// awaks `.` asleep `#`
// Date   ID   Minute
//             000000000011111111112222222222333333333344444444445555555555
//             012345678901234567890123456789012345678901234567890123456789
// 11-01  #10  .....####################.....#########################.....
// 11-02  #99  ........................................##########..........
// 11-03  #10  ........................#####...............................
// 11-04  #99  ....................................##########..............
// 11-05  #99  .............................................##########.....
//
// if can figure out which guard most likely to be asleep at a specifig time,
// could trick guard into working tonight so have best change of sneaking in.
// two strategies for choosing best guard/minute combination
//
// Strategy 1:
// find guard that has most minutesasleep. what minute does that guard
// spend asleep the most?
// above: guard 10 spent most minutes asleep. (20+25+5)
// guard 99 (10 + 10 + 10)
// guard 10 asleep most during minute 24
//
// the above example entries were in chronological order.
// my entries are in the order in which I found them.
// i.e. need t oorganize/sort before can be analyzed
//
// What is the ID of the guard you chose multiplied by the minute you chose?
// e.g. above would be 10 * 24 = 240

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let _entry_string_list: Vec<&str> = input.lines().collect();

    Ok(())
}
