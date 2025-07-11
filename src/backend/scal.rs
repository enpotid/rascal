use super::group::Group;

//scal.rs : 문서들이 들어있을 폴더
struct Skal {
    name:String,
    owner:usize, //owner's id
    discription:String,
    groups:Vec<Group>
}
impl Skal {
    #[doc = "레포 초기화 할 때 쓰는 함수"]
    fn new(&self, scalname:String, ownerid:usize, discription:String) -> Skal {
        Skal {
            name:scalname,
            owner:ownerid,
            discription,
            groups:Vec::new()
        }
    }
    #[doc = "레포를 실제 파일 시스템에다가 삽입"]
    fn build_scal(path:String) {
        todo!()
    }
}