use crate::Slime;
use std::thread;          // 나중에 시간 추가를 위해 선언만 함
use std::time::Duration; // 나중에 시간 추가를 위해 선언만 함
use std::fs;            // fs 라이브러리를 활용해서 대사 파일 불러오기


pub fn story(slime: &Slime)  {
    let text = fs::read_to_string("story.txt").expect("파일을 불러오는데 실패했습니다."); // fs 라이브러리로 파일 불러오기
    let dialogs = text.split("---"); // 구분선을 기준으로 문자열 슬라이스
    let mut enter = String::new();            // 엔터키 입력을 받기 위한 문자열 변수 선언
    for dialog in dialogs {                    // 반복문으로 대사를 순서대로 꺼내기
        println!("{}", dialog.trim());              // 불필요한 공백 제거
        enter.clear();                              // 다음 입력을 받기 위해 변수를 초기홪
        std::io::stdin().read_line(&mut enter).unwrap(); // 엔터키 입력을 받으면 다음 대사 출력
    }
    loop {
        let choice = enter.trim().to_lowercase(); 

    match choice.as_str() {
        "yes" | "y" | ""=> {      // main.rs 스토리 스킵 부분과 동일하게 설계
            crate:: story_yes_1::story_battle(slime);
            break;
        }
        "no"| "n" => {
            crate:: story_no_1::story_no_1(slime);
            break;
        }
    _ => {
        println!("잘못 입력하셨습니다. yes OR no");
        enter.clear(); 
                std::io::stdin().read_line(&mut enter).unwrap();
    } 
    }
    }
}
