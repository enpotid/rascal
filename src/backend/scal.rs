use std::{fmt::format, fs::{self, File}, io::{Error, ErrorKind, Write}, pat};

use crate::backend::utils::is_valid_name;

use super::group::Group;

//scal.rs : 문서들이 들어있을 폴더
pub struct Scal {
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
    /// 레포 파일시스템에 초기화 함수 무조건 실행이 필요
    /// * `path` - 어디다 쓸지. /home/asdf/Desktop/ 까지. 레포 이름까진 안넣어도 됨. 경로 뒤에 /무조건 필요
    /// * `defaultbranch` - 기본 브렌치 이름 이상한거 넣으면 에러남
    fn build_scal(&self, path: String, defaultbranch:String) -> std::io::Result<()> {
        let mut infopath:String = path.to_owned();
        infopath.push_str("INFO");

        let mut defaultbranchpath:String = path.to_owned();
        defaultbranchpath.push_str("DEFAULT_BRANCH");
        
        let mut branchfolderpath:String = path.to_owned();
        branchfolderpath.push_str(&format!("branchs/"));

        let mut branchfilepath:String = path.to_owned();
        branchfilepath.push_str(&format!("branchs/{}", defaultbranch));

        let mut groupsfolderpath:String = path.to_owned();
        groupsfolderpath.push_str(&format!("groups/"));

        let mut ownergroupfilepath:String = path.to_owned();
        ownergroupfilepath.push_str(&format!("groups/owner"));

        let mut lastissuenumpath:String = path.to_owned();
        lastissuenumpath.push_str(&format!("LASTISSUENUM"));

        let mut issuesfolderpath:String = path.to_owned();
        issuesfolderpath.push_str(&format!("issues/"));

        //폴더 만들기
        fs::create_dir_all(&path)?;
        fs::create_dir_all(&branchfolderpath)?;
        fs::create_dir_all(&groupsfolderpath)?;
        fs::create_dir_all(&issuesfolderpath)?;

        //INFO 쓰기
        let mut infofile = File::create(infopath)?;
        infofile.write_all(format!("name:{}\ndiscription:{}\nowner:{}", self.name, self.discription, self.owner).as_bytes())?;

        //DEFAULT_BRANCH
        let mut defaultbranchfile = File::create(defaultbranchpath)?;
        let branchname = is_valid_name(self.name.clone())?;
        defaultbranchfile.write_all(format!("{}", branchname).as_bytes())?;

        //branchs/{defaultbranch}
        let mut branchfile = File::create(branchfilepath)?;
        branchfile.write_all(format!("commit:none").as_bytes())?;

        //groups/owner
        let mut ownergroupfile = File::create(ownergroupfilepath)?;
        ownergroupfile.write_all(b"prepass")?;

        //LAST ISSUE NUM
        let mut lastissuenumfile = File::create(lastissuenumpath)?;
        lastissuenumfile.write_all(b"none");

        Ok(())
    }
}
