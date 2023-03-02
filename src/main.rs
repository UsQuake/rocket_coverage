#![feature(fs_try_exists)]
use git2::Repository;
use git2::build::CheckoutBuilder;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len()
    {
        1 => panic!("local repository의 path를 입력하세요!"),
        2 => {
            let repo = Repository::open(&args[1]).unwrap();
            let mut revwalk = repo.revwalk().unwrap();
            revwalk.push_head().expect("HEAD 브랜치를 찾을 수 없습니다!");
            revwalk.set_sorting(   git2::Sort::REVERSE | git2::Sort::TIME).expect("err");
            let mut index = 0;

            for i in revwalk
            {
                let oid = i.unwrap();
                let commit = repo.find_commit(oid).unwrap();
                repo.reset(&commit.into_object(), git2::ResetType::Hard, Some(
                                CheckoutBuilder::new()
                                .allow_conflicts(true)
                                .conflict_style_diff3(true)
                            ))
                    .expect("Repository가 잠금 되어있어 바꿀 수 없습니다.");
                //git reset --hard {oid} 와 같은 명령을 수행하는 코드

                 Command::new("git")
                    .arg("clean")
                    .arg("-xdf")
                    .current_dir(&args[1])
                    .output()
                    .unwrap();
                //git clean -xdf 로 테스트로 발생한 임시 파일 제거

                //pom.xml이 있는 프로젝트인지 확인
                let does_pom_xml_exist = std::fs::try_exists(format!("{}/pom.xml", &args[1])).unwrap();
                if does_pom_xml_exist
                {
                    println!("try[{}] : {}", index, oid.to_string());

                    //mvn test 명령어 시도
                    Command::new("mvn")
                    .arg("test")
                    .arg("jacoco:report")
                    .arg("--file")
                    .arg("pom.xml")   
                    .arg("--no-transfer-progress")
                    .current_dir(&args[1])
                    .output()
                    .unwrap();

                //테스트가 성공해서 커버리지 리포트가 발생하면 리포트 폴더 통째로 ../coverage_report 위치로 옮김.
                Command::new("mv")
                .arg("target/site/jacoco")
                .arg(format!("../coverage_reports/{}_{}", index, oid.to_string()))
                .current_dir(&args[1])
                .output();
                //이때, 리포트의 폴더 이름을 {인덱스}_{커밋 oid}로 변경하면서 ../coverage_report로 옮김. 
                }
                else {
                    println!("skip[{}] : {}", index, oid.to_string());
                }
            index = index + 1;
            }
 
        },
        _ => panic!("알 수 없습니다!")
    }

}