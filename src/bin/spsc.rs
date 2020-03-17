use std::sync::Arc;
use std::time::Duration;

use os_learning::sync::spsc::Queue;

fn main() {
    let queue: Arc<Queue<Option<String>>> = Arc::new(Queue::new(10));

    let producer = {
        let queue = queue.clone();
        std::thread::spawn(move || {
            let lines = r#"sreteP miT yb ,nohtyP fo neZ ehT
.ylgu naht retteb si lufituaeB
.ticilpmi naht retteb si ticilpxE
.xelpmoc naht retteb si elpmiS
.detacilpmoc naht retteb si xelpmoC
.detsen naht retteb si talF
.esned naht retteb si esrapS
.stnuoc ytilibadaeR
.selur eht kaerb ot hguone laiceps t'nera sesac laicepS
.ytirup staeb ytilacitcarp hguohtlA
.yltnelis ssap reven dluohs srorrE
.decnelis ylticilpxe sselnU
.sseug ot noitatpmet eht esufer ,ytiugibma fo ecaf eht nI
.ti od ot yaw suoivbo-- eno ylno ylbareferp dna --eno eb dluohs erehT
.hctuD er'uoy sselnu tsrif ta suoivbo eb ton yam yaw taht hguohtlA
.reven naht retteb si woN
.won *thgir* naht retteb netfo si reven hguohtlA
.aedi dab a s'ti ,nialpxe ot drah si noitatnemelpmi eht fI
.aedi doog a eb yam ti ,nialpxe ot ysae si noitatnemelpmi eht fI
!esoht fo erom od s'tel -- aedi taerg gniknoh eno era secapsemaN
                "#;
            for s in lines.lines() {
                queue.send(Some(s.into()));
            }
            queue.send(None);
        })
    };

    let consumer = {
        std::thread::spawn(move || {
            while let Some(s) = queue.recv() {
                println!("{}", s.chars().rev().collect::<String>());
                std::thread::sleep(Duration::from_millis(300));
            }
        })
    };
    producer.join().expect("Producer Failed");
    consumer.join().expect("Consumer Failed");
}
