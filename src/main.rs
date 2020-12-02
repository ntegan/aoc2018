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

mod repose_record {
    use regex::Regex;
    const TIMESTAMP_REGEX: &'static str = "^\\[([0-9]*)-([0-9]*)-([0-9]*) ([0-9]*):([0-9]*)\\].*$";
    const EVENT_REGEX: &'static str = "^\\[.*\\] (.*)$";
    const GUARD_NUMBER_REGEX: &'static str = "^Guard #([0-9]*) begins shift$";

    #[derive(Debug, Clone, Copy)]
    pub struct GuardNumber(u64);
    impl PartialEq for GuardNumber {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    #[derive(Debug)]
    pub enum Event {
        BeginShift(GuardNumber),
        FallAsleep,
        WakeUp,
    }
    impl Event {
        pub fn from_observation_string(
            observation_string: &str,
        ) -> Result<Event, Box<dyn std::error::Error>> {
            let re = Regex::new(EVENT_REGEX)?;
            let caps = re
                .captures(observation_string)
                .ok_or("No regex capture matches!")?;
            let event_string = caps.get(1).ok_or("Couldn't get event string")?.as_str();

            if event_string == "wakes up" {
                return Ok(Event::WakeUp);
            } else if event_string == "falls asleep" {
                return Ok(Event::FallAsleep);
            } else {
                let re = Regex::new(GUARD_NUMBER_REGEX)?;
                let caps = re
                    .captures(event_string)
                    .ok_or("No regex capture matches!")?;
                let guard_number = caps
                    .get(1)
                    .ok_or("Couldn't get guard number")?
                    .as_str()
                    .parse::<u64>()?;
                return Ok(Event::BeginShift(GuardNumber(guard_number)));
            }
        }
    }
    #[derive(Debug, Clone, Copy)]
    pub struct Timestamp {
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
    }
    impl PartialOrd for Timestamp {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Timestamp {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.year.cmp(&other.year) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Equal => match self.month.cmp(&other.month) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => match self.day.cmp(&other.day) {
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Equal => match self.hour.cmp(&other.hour) {
                            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                            std::cmp::Ordering::Equal => match self.minute.cmp(&other.minute) {
                                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                            },
                        },
                    },
                },
            }
        }
    }
    impl PartialEq for Timestamp {
        fn eq(&self, other: &Self) -> bool {
            self.year == other.year
                && self.month == other.month
                && self.day == other.day
                && self.hour == other.hour
                && self.minute == other.minute
        }
    }
    impl Eq for Timestamp {}
    impl Timestamp {
        pub fn from_observation_string(
            observation_string: &str,
        ) -> Result<Timestamp, Box<dyn std::error::Error>> {
            let re = Regex::new(TIMESTAMP_REGEX)?;
            let caps = re
                .captures(observation_string)
                .ok_or("No regex capture matches!")?;
            let year = caps
                .get(1)
                .ok_or("Couldn't get year")?
                .as_str()
                .parse::<u64>()?;
            let month = caps
                .get(2)
                .ok_or("Couldn't get month")?
                .as_str()
                .parse::<u64>()?;
            let day = caps
                .get(3)
                .ok_or("Couldn't get day")?
                .as_str()
                .parse::<u64>()?;
            let hour = caps
                .get(4)
                .ok_or("Couldn't get hour")?
                .as_str()
                .parse::<u64>()?;
            let minute = caps
                .get(5)
                .ok_or("Couldn't get minute")?
                .as_str()
                .parse::<u64>()?;
            Ok(Timestamp {
                year,
                month,
                day,
                hour,
                minute,
            })
        }
    }

    #[derive(Debug)]
    pub struct Nap {
        pub start_time: Timestamp,
        pub end_time: Timestamp,
    }

    #[derive(Debug)]
    pub struct Record {
        pub event: Event,
        pub time: Timestamp,
    }

    #[derive(Debug)]
    pub struct Ledger {
        pub records: Vec<Record>,
        pub guards: Vec<GuardNumber>,
        pub guard_naps: Vec<Vec<Nap>>,
    }

    impl Ledger {
        fn fill_guard_naps(
            guard_naps: &mut Vec<Vec<Nap>>,
            guards: &Vec<GuardNumber>,
            records: &Vec<Record>,
        ) {
            for guard in guards {
                let mut naps = Vec::new();
                let mut is_current_guard = false;

                let mut time1;
                let mut time2;

                for record in records {
                    match record.event {
                        Event::BeginShift(guard_number) => {
                            if guard_number == *guard {
                                is_current_guard = true;
                            } else {
                                is_current_guard = false;
                            }
                        }
                        Event::FallAsleep => {
                            time1 = record.time;
                        }
                        Event::WakeUp => {
                            time2 = record.time;
                            naps.push(Nap {
                                start_time: time1,
                                end_time: time2,
                            });
                        }
                    }
                }

                println!("{:?}", guard);
                guard_naps.push(naps);
            }
            //
        }
        pub fn from_observation_string_list(
            observation_list: Vec<&str>,
        ) -> Result<Ledger, Box<dyn std::error::Error>> {
            let mut records = Vec::new();
            let mut guards = Vec::new();
            let mut guard_naps = Vec::new();
            for observation in observation_list {
                let time = Timestamp::from_observation_string(observation)?;
                let event = Event::from_observation_string(observation)?;
                records.push(Record { event, time });
            }
            for record in &records {
                match record.event {
                    Event::BeginShift(guard_number) => {
                        if !guards.contains(&guard_number) {
                            guards.push(guard_number);
                        }
                    }
                    Event::FallAsleep => {}
                    Event::WakeUp => {}
                }
            }
            Self::fill_guard_naps(&mut guard_naps, &guards, &records);
            Ok(Ledger {
                records,
                guards,
                guard_naps,
            })
        }

        pub fn sort(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.records
                .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //input::example();

    let input = input::read_until_eof()?;

    let observation_list: Vec<&str> = input.lines().collect();

    let mut ledger = repose_record::Ledger::from_observation_string_list(observation_list)?;

    ledger.sort()?;

    /*
    for i in &ledger.records {
        println!("{:?}", i);
    }
    */
    //println!("{:#?}", ledger);

    Ok(())
}
