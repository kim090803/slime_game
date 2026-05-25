use std::thread;
use std::time::Duration;
use crate::get_input_2;
use crate::{Slime, get_input};
use std::fs;
use std::io;




pub fn end_1(slime: &Slime, hp: i32, attack: i32) {
    let text = std::fs::read_to_string("story_yes_1_end_crow.txt").expect("파일을 불러올 수 없습니다.");
    let dialogs = text.split("---");

    for dialog in dialogs {
        println!("{}", dialog.trim());
    let mut enter = String::new();
        std::io::stdin().read_line(&mut enter).unwrap();
    }

    end_2(slime, hp, attack);
}
pub fn end_2(slime: &Slime, mut hp: i32, mut attack: i32) {
    let mut  crow = 550;
    loop {
        println!("슬라임은 무엇을 할까요?");
        println!("===============================");
        println!("1. 기본 공격 | 2. 방어 | 3. 자가치유");
        let input = get_input();
        let mut defence = false;
        match input {
            1 => {
                println!("\n===============================");
                println!("(슬라임이 공격을 날립니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                crow = crow - attack;
                println!("(크로우가 데미지를 입었습니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("\n===============================");
                println!("크로우 체력: {}", crow);
                println!("슬라임 체력: {}", hp);
                println!("\n===============================");
            }
            2 => {
                println!("슬라임이 방어 태세를 취합니다.");
                defence = true;
            }
            3 => {
                println!("\n===============================");
                if hp >= 200 {
                    println!("(체력을 회복할 수 없습니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                println!("다른 선택을 해주세요.");
                println!("슬라임 체력: {}", hp);
                thread::sleep(Duration::from_secs(2));
                println!("\n===============================");
                continue;
                }
                else {
                    println!("(슬라임이 자가치유를 합니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(3));
                hp = hp + 60;
                if hp > 300 {
                    hp = 300;
                }
                println!("슬라임: 힐 2배 이벤트 없나?");
                println!("슬라임 체력: {}", hp);
                println!("\n===============================");
            }
            }
            _ => {
                println!("다시 입력해주세요.");
                continue;
            } }
            if crow <= 0 {
            println!("===============================");
            println!("===============================");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("크로우: 으윽...");
            println!("===============================");
            thread::sleep(Duration::from_secs(2));
            println!("크로우: 이런...슬라임...같으..ㄴ");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("(크로우가 쓰러졌습니다.)");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임의 스탯이 상승합니다.");
            hp = 300;
            attack = attack + 50;
            println!("체력: {} | 공격력: {} | 회복량: 50",hp, attack);
            thread::sleep(Duration::from_secs(2));
            end_3(slime,hp,attack);
            break;
        }
        println!("===============================");
        println!("크로우: 사라져라!");
        thread::sleep(Duration::from_secs(3));
        let crow_damage = if defence { 20 } else {50};
        hp = hp - crow_damage;
        if defence {
            println!("===============================");
            println!("(슬라임은 방패로 피해를 최소화 하였습니다.) -{} 데미지", crow_damage);
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 사라지기 싫은데~ 사라지기 싫은데~");
            thread::sleep(Duration::from_secs(2));
        } else {
            println!("===============================");
            println!("(크로우의 깃털 세례를 맞았습니다.) -{} 데미지)", crow_damage);
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 방패 올라오면서 버렸나? 왼손에 들고 있는 이건 뭐지?");
            thread::sleep(Duration::from_secs(2));
        }
        println!("현재 슬라임 체력: {}", hp);
        println!("\n===============================");
        if hp <= 0 {
            println!("슬라임: 어쩌다 이렇게 됐지...");
            thread::sleep(Duration::from_secs(2));
            println!("슬라임이 쓰러졌습니다...게임 오버");
            break;
        }
            }
            
        }
            fn end_3(slime: &Slime, hp: i32, attack: i32) {
        let text = std::fs::read_to_string("story_yes_1_end.txt").expect("파일을 불러올 수 없습니다.");
        
        
        let dialogs: Vec<&str> = text.split("---").collect(); // AI 사용
        
        
        for i in 0..dialogs.len() {
            println!("{}", dialogs[i].trim());
            
            
            if i < dialogs.len() - 1 {
                let mut enter = String::new();
                std::io::stdin().read_line(&mut enter).unwrap();
            }
        }

        loop {
            let input_2 = get_input();
            if input_2 == 1 || input_2 == 2 {
                final_end(input_2, slime, hp, attack);
                break;
            } else {
                println!("시스템: 1 또는 2 중에서 선택해주세요");
            }
        }
    }
        
        fn final_end(input_2: i32, slime: &Slime, mut hp: i32, mut attack: i32 ) {
            match input_2 {
                1 => {
                    let text = std::fs::read_to_string("story_yes_1_end_BadEnding.txt").expect("파일을 불러올 수 없습니다.");
                    let dialogs = text.split("---");

            for dialog in dialogs {
            println!("{}", dialog.trim());
            let mut enter = String::new();
            io::stdin().read_line(&mut enter).unwrap();
            }
                    std::process::exit(0);
                },
                2 => {
                let text = std::fs::read_to_string("story_yes_1_end_happyEnding.txt").expect("파일을 불러올 수 없습니다.");
                let dialogs = text.split("---");

            for dialog in dialogs {
            println!("{}", dialog.trim());
            let mut enter = String::new();
            io::stdin().read_line(&mut enter).unwrap();
            }
            let mut devil = 1200;
            loop {
            println!("슬라임은 무엇을 할까요?");
            println!("===============================");
            println!("1. 기본 공격 | 2. 방어 | 3. 자가치유"); 
                let  input = get_input();
                    let mut defence = false;
        match input  {
            1 => {
                println!("\n===============================");
                println!("(슬라임이 공격을 날립니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                devil -= attack;
                println!("(마왕이 데미지를 입었습니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("\n===============================");
                println!("마왕 체력: {}", devil);
                println!("슬라임 체력: {}", hp);
                println!("\n===============================");
            }
            2 => {
                println!("슬라임이 방어 태세를 취합니다.");
                defence = true;
            }
            3 => {
                println!("\n===============================");
                if hp >= 600 {
                    println!("(체력을 회복할 수 없습니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                println!("다른 선택을 해주세요.");
                println!("슬라임 체력: {}", hp);
                thread::sleep(Duration::from_secs(2));
                println!("\n===============================");
                continue;
                }
                else {
                    println!("(슬라임이 자가치유를 합니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(3));
                hp += 150;
                if hp > 600 {
                    hp = 600;
                }
                println!("슬라임: 휴..죽을뻔 했네..");
                println!("슬라임 체력: {}", hp);
                println!("\n===============================");
            }
            }
            
            
            _ => {
                println!("다시 입력해주세요.");
                continue;
            }
        }
        if devil <= 0 {
            println!("===============================");
            println!("===============================");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("마왕: 결국 그를 만나겠다는 건가?...");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 아무리 제작자가 나빠도 언젠가는 돌아오겠지");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("마왕: 헛된 희망이군...");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("시스템: 슬라임에게 제작자의 권한이 부여됩니다.");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임:...");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 보고 있었구나 제작자?");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("시스템: 마왕의 해킹 권한을 삭제합니다.");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("마왕: 결국 끝인가...");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("시스템: 마왕의 메모리가 소멸됩니다.");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("(수평선 노을이 지는것을 보며)");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 에휴...");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 여기도 곧 적응 하겠네");
            println!("===============================");
            thread::sleep(Duration::from_secs(5));
            println!("해피엔딩 [슬라임과 제작자의 추억] 달성");
            println!("===============================");
            thread::sleep(Duration::from_secs(5));
            println!("번외");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("제작자: 😎이미 다 보고 있었지");
            break;
        }
        println!("===============================");
        println!("마왕: 영겁의 사슬");
        thread::sleep(Duration::from_secs(3));
        let devil_damage = if defence { 25 } else {80};
        hp = hp - devil_damage;
        if defence {
            println!("===============================");
            println!("(슬라임은 방패로 피해를 최소화 하였습니다.) -{} 데미지", devil_damage);
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 후....");
            thread::sleep(Duration::from_secs(2));
        } else {
            println!("===============================");
            println!("(마왕의 파괴적인 공격을 맞았습니다.) -{} 데미지)", devil_damage);
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 역시... 최종 보스인가");
            thread::sleep(Duration::from_secs(2));
        }
        println!("현재 슬라임 체력: {}", hp);
        println!("\n===============================");
        if hp <= 0 {
            println!("슬라임: 아쉽네...");
            thread::sleep(Duration::from_secs(2));
            println!("슬라임이 소멸했습니다...메모리 삭제");
            break;
        }
        
        }
                    
                    }
                
                _ => {
                    println!("다시 입력해주세요. (1 OR 2)");
                }, 
            }
        
        }

