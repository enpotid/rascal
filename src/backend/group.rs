pub struct Group {
    name:String,
    parmissions:Vec<Permission>
}
enum Permission {
    EditDocument,
    EditRequest,
    Issue,
    ManageIssue,
}