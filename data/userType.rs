#[derive(Default, Debug)]
#[repr(C)]
pub struct Age {
  z_skill_id: Cell<usize>,
  z_skill_type_id: usize,
  z_foreign_data: Vec<foreign::FieldData>,
  age: i64,
}
pub trait AgeObject: SkillObject {
  fn get_age(&self) -> i64;
  fn set_age(&mut self, age: i64);
}
impl Age {
  pub fn new(skill_id: usize, skill_type_id: usize) -> Age {
    Age {
      z_skill_id: Cell::new(skill_id),
      z_skill_type_id: skill_type_id,
      z_foreign_data: Vec::default(),
      age: 0,
    }
  }
}
impl AgeObject for Age {
  fn get_age(&self) -> i64 { self.age }
  fn set_age(&mut self, value: i64) { self.age = value; }
}
impl foreign::ForeignObject for Age {
  fn foreign_fields(&self) -> &Vec<foreign::FieldData> {
    &self.z_foreign_data
  }
  fn foreign_fields_mut(&mut self) -> &mut Vec<foreign::FieldData> {
    &mut self.z_foreign_data
  }
}
impl SkillObject for Age {
  fn skill_type_id(&self) -> usize { self.z_skill_type_id }
  fn get_skill_id(&self) -> usize { self.z_skill_id.get() }
  fn set_skill_id(&self, skill_id: usize) -> Result<(), SkillFail> {
    if skill_id == skill_object::DELETE {
      return Err(SkillFail::user(UserFail::ReservedID { id: skill_id }));
    }
    self.z_skill_id.set(skill_id);
    Ok(())
  }
}
impl Deletable for Age {
  fn mark_for_deletion(&mut self) {
      self.z_skill_id.set(skill_object::DELETE);
  }
  fn to_delete(&self) -> bool {self.z_skill_id.get()==skill_object::DELETE}
}
