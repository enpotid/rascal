use std::{fs::{self, File}, io::{Error, ErrorKind, Write}, pat};

use crate::backend::utils::is_valid_name;

use super::group::Group;

//scal.rs : 문서들이 들어있을 폴더
struct Scal {
    name: String,
    owner: usize, //owner's id
    discription: String,
    groups: Vec<Group>,
}
impl Scal {
    /**레포 초기화 할 때 쓰는 함수*/
    fn new(scalname: String, ownerid: usize, discription: String) -> Result<Scal, std::io::Error> {
        is_valid_name(scalname.to_owned())?;
        Ok(Scal {
            name: scalname,
            owner: ownerid,
            discription,
            groups: Vec::new(),
        })
    }
    /**레포를 실제 파일 시스템에다가 삽입 path 뒤에 / 붙어있어야함 main은 브랜치 이름 */
    fn build_scal(&self, path: String, main:String) -> std::io::Result<()> {
        let mut infopath:String = path.to_owned();
        infopath.push_str("INFO");

        let mut defaultbranchpath:String = path.to_owned();
        infopath.push_str("DEFAULT_BRANCH");

        //폴더 만들기
        fs::create_dir_all(&path)?;

        //INFO 쓰기
        let mut infofile = File::create(infopath)?;
        infofile.write_all(format!("name:{}\ndiscription:{}", self.name, self.discription).as_bytes())?;

        //DEFAULT_BRANCH
        let mut defaultbranchfile = File::create(defaultbranchpath)?;
        let branchname = is_valid_name(main.clone())?;
        defaultbranchfile.write_all(format!("{}", branchname).as_bytes())?;
        Ok(())
    }
}
