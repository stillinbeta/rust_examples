extern crate rand;

pub mod party {
    use rand::Rng;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn invite() {
        let guests = vec![
            "Applejack",
            "Fluttershy",
            "Rainbow Dash",
            "Rarity",
            "Twilight Sparkle",
        ];

        let (tx, rx) = mpsc::channel();

        for guest in guests {
            let tx1 = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                tx1.send(guest).unwrap();
                let sleep = rand::thread_rng().gen_range(1, 10);
                thread::sleep(Duration::from_millis(sleep));
            });
        }

        for _ in 1..6 {
            let arrival = rx.recv().unwrap();
            println!("{} arrived!", arrival);
        }

        println!("Everypony's here! Time to start the party!")
    }
}

pub mod cmc {
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub fn mutex() {
        let cmctex = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for filly in vec![
            "Scootaloo".to_string(),
            "Applebloom".into(),
            "Sweetie Belle".into(),
        ] {
            let cmctex = Arc::clone(&cmctex);
            let handle = thread::spawn(move || {
                let mut cmc = cmctex.lock().unwrap();
                (*cmc).push(filly);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("m = {:?}", cmctex);
    }

}

pub mod greet {
    enum Tribe {
        Alicorn,
        EarthPony,
        Pegasus,
        Unicorn,
    }

    enum Species {
        Dragon,
        Changeling { accepts_friendship: bool },
        Pony { tribe: Tribe },
        Hippogriff,
        SeaPony,
    }

    fn greet(visitor: Species) {
        match visitor {
            Species::Dragon => println!("Hello, my dragon friend!"),
            Species::Changeling {
                accepts_friendship: true,
            } => println!("greetings, bug horse!"),
            Species::Changeling {
                accepts_friendship: false,
            } => println!("ahh! A changeling!"),
            Species::Pony { tribe } => match tribe {
                Tribe::Alicorn => println!("Your majesty!"),
                Tribe::Pegasus => println!("How is the weather up there?"),
                Tribe::EarthPony => println!("Good tidings!"),
                Tribe::Unicorn => println!("Watch where you point that thing!"),
            },
            Species::Hippogriff | Species::SeaPony => println!("Hello my finny feathered friend!"),
        }
    }

    pub fn greet_everycreature() {
        let spike = Species::Dragon;
        greet(spike);

        let thorax = Species::Changeling {
            accepts_friendship: true,
        };
        greet(thorax);

        let chrysalis = Species::Changeling {
            accepts_friendship: false,
        };
        greet(chrysalis);

        let celestia = Species::Pony {
            tribe: Tribe::Alicorn,
        };
        greet(celestia);
        let applejack = Species::Pony {
            tribe: Tribe::EarthPony,
        };
        greet(applejack);
        let fluttershy = Species::Pony {
            tribe: Tribe::Pegasus,
        };
        greet(fluttershy);
        let rarity = Species::Pony {
            tribe: Tribe::Unicorn,
        };
        greet(rarity);

        let skystar = Species::SeaPony;
        greet(skystar);
        let skystar = Species::Hippogriff;
        greet(skystar);
    }
}
