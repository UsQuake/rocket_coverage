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
            let mut index = 3700;

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

                let is_exist = std::fs::try_exists(format!("{}/pom.xml", &args[1])).unwrap();
                if is_exist
                {
                    println!("try[{}] : {}", index, oid.to_string());
                    Command::new("mvn")
                    .arg("test")
                    .arg("jacoco:report")
                    .arg("--file")
                    .arg("pom.xml")
                    .arg("--no-transfer-progress")
                    .current_dir(&args[1])
                    .output()
                    .unwrap();
                    std::fs::rename(format!("{}/target/site/jacoco", &args[1]), format!("{}/target/site/jacoco{}", &args[1], index)).unwrap();
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