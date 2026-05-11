use crate::story::story;
use crate::{Slime, get_input, story_yes_1, story_yes_1_end};
use std::thread;
use std::time::Duration;
use std::io;
use std::fs;
struct fight {
attack: i32,
defencd: i32,
heal: i32,
}

struct slime_status {
    hp: i32,
}

pub fn story_true_1(slime: &Slime)  {
    let text = std::fs::read_to_string("story_yes_1.txt").expect("파일을 불러올 수 없습니다.");
    let dialogs = text.split("---");

    for dialog in dialogs {
        println!("{}", dialog.trim());
        let mut enter = String::new();
        io::stdin().read_line(&mut enter).unwrap(); 
    }
}
pub fn story_battle(slime: &Slime) {
    story_true_1(slime);
    let mut war = fight {
    attack: 10000, // 실제: 30 (테스트를 위해 1000으로 설정)
    defencd: 30,
    heal: 30,
    };
    let mut oke = 350;
    let mut slime_hp = status_2(slime);
    loop {
        println!("슬라임은 무엇을 할까요?");
        println!("===============================");
        println!("1. 기본 공격 | 2. 방어 | 3. 자가치유"); 
        let choice = get_input();
        let mut defence = false;
        match choice  {
            1 => {
                println!("\n===============================");
                println!("(슬라임이 공격을 날립니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                oke = oke - war.attack;
                println!("(오크가 데미지를 입었습니다.)");
                thread::sleep(Duration::from_secs(3));
                println!("\n===============================");
                println!("오크 체력: {}", oke);
                println!("슬라임 체력: {}", slime_hp.hp);
                println!("\n===============================");
            }
            2 => {
                println!("슬라임이 방어 태세를 취합니다.");
                defence = true;
            }
            3 => {
                println!("\n===============================");
                if slime_hp.hp >= 200 {
                    println!("(체력을 회복할 수 없습니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(2));
                println!("다른 선택을 해주세요.");
                println!("슬라임 체력: {}", slime_hp.hp);
                thread::sleep(Duration::from_secs(2));
                println!("\n===============================");
                continue;
                }
                else {
                    println!("(슬라임이 자가치유를 합니다.)");
                println!("===============================");
                thread::sleep(Duration::from_secs(3));
                slime_hp.hp = slime_hp.hp + 35;
                if slime_hp.hp > 200 {
                    slime_hp.hp = 200;
                }
                println!("슬라임: 휴..죽을뻔 했네..");
                println!("슬라임 체력: {}", slime_hp.hp);
                println!("\n===============================");
            }
            }
            
            
            _ => {
                println!("다시 입력해주세요.");
                continue;
            }
        }
        if oke <= 0 {
            println!("===============================");
            println!("===============================");
            println!("===============================");
            thread::sleep(Duration::from_secs(4));
            println!("오크: 나를 쓰러뜨리다니..");
            println!("===============================");
            thread::sleep(Duration::from_secs(2));
            println!("오크: 이런...");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("(오크가 쓰러졌습니다.)");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임의 스탯이 상승합니다.");
            slime_hp.hp = slime_hp.hp + 100;
            war.attack = war.attack + 20;
            println!("체력: {} | 공격력: {} | 회복량: 50",slime_hp.hp, war.attack);
            thread::sleep(Duration::from_secs(2));
            story_yes_1_end::end_1(slime, slime_hp.hp, war.attack);
            break;
        }
        println!("===============================");
        println!("오크: 받아라!!");
        thread::sleep(Duration::from_secs(3));
        let oke_damage = if defence { 10 } else {30};
        slime_hp.hp = slime_hp.hp - oke_damage;
        if defence {
            println!("===============================");
            println!("(슬라임은 방패로 피해를 최소화 하였습니다.) -{} 데미지", oke_damage);
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 안 아프지롱~ 다시 덤벼라!");
            thread::sleep(Duration::from_secs(2));
        } else {
            println!("===============================");
            println!("(오크의 강력한 공격을 맞았습니다.) -{} 데미지)", oke_damage);
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 초반 몹 밸런스 실화냐!? 방패 줬다며!");
            thread::sleep(Duration::from_secs(2));
        }
        println!("현재 슬라임 체력: {}", slime_hp.hp);
        println!("\n===============================");
        if slime_hp.hp <= 0 {
            println!("슬라임: 제작자 밸런스 패치 안 하냐...");
            thread::sleep(Duration::from_secs(2));
            println!("슬라임이 쓰러졌습니다...게임 오버");
            break;
        }
        }
        }
    
pub fn status_2 (slime: &Slime) -> slime_status  {
    let slime_infor = slime_status {
        hp: 200,
    };
    println!("슬라임 상태창");
    println!("===============================");
    println!("체력: {}",slime_infor.hp);
    slime_infor
    }
