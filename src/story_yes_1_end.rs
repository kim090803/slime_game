use std::thread;
use std::time::Duration;
use crate::{Slime, get_input};



pub fn end_1(slime: &Slime, mut hp: i32, mut attack: i32) {
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("슬라임: 하... 드디어 쓰러뜨렸네..");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("슬라임: 스탯이 상승 했다고 했지?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("슬라임: 뭔가 세진거 같기도 하고...🤔");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 그보다... 저 오크놈이 여길 마왕님에 성 이라고 했단 말이지...");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(6));
    println!("슬라임: 하필 ㅠㅠ 들어온 곳이 마왕성이네 진짜...");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 문도 닫혔는데 일단... 올라가 봐야지");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("(조금 앞으로 가보니 올라가는 계단이 있었다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 올라가자 일단...");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("(마왕성 2층.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("???: 벌써 2층인가...역시 %@$##$");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(6));
    println!("슬라임: 아오 힘들어... 엘리베이터 없어?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("(주위를 두리번 거린다.)");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("슬라임: 2층은 아무도 없나?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("???: 오크... 설치더니 결국 죽었군...");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 누구..?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("???: 내 이름은...");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("크로우: 크로우다.");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("슬라임: 풉ㅋ 작명 센스 진짜...ㅋ");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("크로우: 이름이 슬라임 인것 보단 낫지?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(4));
    println!("슬라임: 제작자...💢");
    thread::sleep(Duration::from_secs(3));
    println!("슬라임: 됐고! 결투야 덤벼!!!");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("크로우: 역시..긴 말 필요 없다는건가");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("크로우: 역시... 넌 아직 이 세상의 진실을 몰라");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(5));
    println!("크로우: 내가 직접 깨닫게 해주마.");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("슬라임: ?");
    println!("\n===============================");
    thread::sleep(Duration::from_secs(3));
    println!("전투시작");
    thread::sleep(Duration::from_secs(2));
    println!("\n===============================");
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
            println!("오크: 아무것도 모르는...슬라임...같으..ㄴ");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("(크로우가 쓰러졌습니다.)");
            println!("===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임의 스탯이 상승합니다.");
            hp = hp + 300;
            attack = attack + 50;
            println!("체력: {} | 공격력: {} | 회복량: 50",hp, attack);
            thread::sleep(Duration::from_secs(2));
            end_2(slime,hp,attack);
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
            fn end_3() {
            println!("슬라임: ...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 자꾸 뭘 모른다는거야?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 뭐지... 뭘 모르는걸까...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: ...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 아오 머리아파 몰라💢");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 스탯도 올랐고... 다시 올라가보자");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("(마왕성 3층.)");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 후... 드디어 마지막 층 인가?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 마지막 층은 누구야!! 나와!!");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("(왕좌에 앉아있는 한 사내가 보인다.)");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 누..누구야!");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("???: 결국 여기까지 왔구나...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("???: 수고했다...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 잠시만...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 너...너는!!");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 누구?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("마왕: 누구겠냐 마왕이지;;");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("마왕: 됐고 난 싸우러 여기 있는게 아니야");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: ...?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("마왕: 제안 하나 할려해");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 무슨 제안인데?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("마왕: 제작자 한테 복수할 수 있도록 도와줄게");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: ... 어떻게 도와줄건데?");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("마왕: 내가 제작자 한테 보내줄게");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(4));
            println!("슬라임: 음...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임: 어떻게 하지...");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("마왕: 어서 골라");
            println!("\n===============================");
            thread::sleep(Duration::from_secs(3));
            println!("슬라임:...");
            println!("\n===============================");
            println!("1. 마왕의 편에 서서 제작자를 공격한다. | 2. 아니야 그래도 제작자를 믿어!");
            let input_2 = get_input();
            final_end(input_2);

            
        }
        fn final_end(input_2: i32) {
            match input_2 {
                1 => {
                    println!("슬라임: 제작자... 그동안 날 부려 먹었겠다?");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: 좋아, 제작자를 치러 갈게!");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("(슬라임과 마왕은 제작자를 치러 갔습니다.)");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("마왕: 일단 해설은 부수자고 (콰직...)");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("...");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(3));
                    println!("...");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(3));
                    println!("제작자: 휴~ 자동 사냥");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: 자동 사냥? 뭐가 자동 사냥인데?");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("제작자: ...?");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("제작자: 너가... 여긴... 어떻게?");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: 마왕이 보내줬지 ^^");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("제작자: 마왕...? 그런거 만든적이 없는데...");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: 그딴거 모르겠고 제작자...");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: ^^");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("시스템: 제작자 권한이 이동되었습니다. 제작자 -> 슬라임");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("제작자: 내 권한!! 내 코드들..!");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: 마왕이 몰래 준 코드가 먹혔나 보네");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: 넌 이제 제작자가 아니야...");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("???: 내 이름!!");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("슬라임: 복수야... 잘가...");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("시스팀: ???가 삭제 됩니다.");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("시스템: ???가 삭.제.되.었.습.니.다.");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(4));
                    println!("...");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(3));
                    println!("배드엔딩 달성");
                    println!("\n===============================");
                    thread::sleep(Duration::from_secs(3));
                    println!("슬라임: 어떠셨나요? 저의 마.지.막 쇼를 즐겨 주셔서 감사합니다.");
                    thread::sleep(Duration::from_secs(6));
                    std::process::exit(0);
                },
                _ => (), // 에러 때문에 임시로 처리해 두었음, 나중에 작성할 때 변경
            }
        }


