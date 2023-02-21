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
                let test_command = Command::new("pytest")
                .arg("--cov")
                .current_dir(&args[1])
                .output()
                .expect("pytest-cov를 설치해주세요!");
            
            let cov_info = String::from_utf8(test_command.stdout).unwrap();
            let mut iter = 0;
            let mut metrics = Vec::with_capacity(8);
            for token in cov_info.split_ascii_whitespace()
            {                    
                if iter != 0
                {
                    metrics.push(String::from(token));
                    iter = (iter + 1) % 6;
                }

                if token== "TOTAL"
                {
                    iter = (iter + 1) % 6;
                }
            }

           for metric in metrics
           {
                if metric.contains("%")
                {
                    println!("{} : {}", oid.to_string(), metric);
                }
           }
                index = index + 1;
            }
 
        },
        _ => panic!("알 수 없습니다!")
    }

   
}