use uuid::Uuid;

trait Common{
    
    fn clone(&self)->Self;
    fn to_owned(self)->Self;
    fn drop(self)->();
    fn new()->Self;

}

enum Gender{
    Male,
    Female,
    Null
}

impl Common for Gender{
    
    fn clone(&self)->Self{
        match self {
            Self::Male=>{return Self::Male ;},
            Self::Female=>{return Self::Female;},
            _=>{return Self::Null;}
        }
    }

    fn drop(self){
        return;
    }

    fn to_owned(self)->Self {
         return  self;       
    }

    fn new()->Self {
        return Self::Null
    }


}

impl Gender{
    
    fn is_male(&self)->bool{
         match self {
            Self::Male=>{return true;},
            _=>{return false;}
        }
    }
    
    fn is_female(&self)->bool{
        match self {
          Self::Female=>{return true;},
           _=>{return false;}
        }
    }

}


enum  PersonRelationShip{
    Single,
    Married,
    Seperated
}

impl Common for PersonRelationShip{
    
    fn clone(&self)->Self{
        match self {
            Self::Married=>{return Self::Married ;},
            Self::Seperated=>{return Self::Seperated;},
            _=>{return Self::Single;}
        }
    }

    fn drop(self){
        return;
    }

    fn to_owned(self)->Self {
         return  self;       
    }

    fn new()->Self {
        return Self::Single
    }
    
}
impl PersonRelationShip{
   
    fn is_married(&self)->bool{
         match self {
            Self::Married=>{return true;},
            _=>{return false;}
        }
    }

    fn is_single(&self)->bool{
         match self {
            Self::Single=>{return true;},
            _=>{return false;}
        }
    }

    fn is_seperated(&self)->bool{
       match self {
            Self::Seperated=>{return true;},
            _=>{return false;}
        }
    }
    
    fn to_single(&mut self){
        *self=Self::Single
    }

    fn to_married(&mut self){
        *self=Self::Married
    }

    fn to_seperated( &mut self){
        *self=Self::Seperated
    }


    
}
       
struct Person{
    id:String,
    name:String,
    job: String ,
    age:u8,
    gender: Gender ,
    relation:PersonRelationShip,
    sons:Vec<Person>
}


impl Common for  Person{
    fn new()->Self{
       return  Self{
            id :Uuid::new_v4().to_string(),
            name:String::new(),
            job:String::new(),
            age:0,
            gender:Gender::Null,
            relation:PersonRelationShip::Single,
            sons:Vec::<Person>::new()
        };
    }
    
    fn drop(self)->() {
        return;
    }
    
    fn to_owned(self)->Self {
        return self;
    }

    fn clone(&self)->Self {
        return  Self{
            id:self.id.clone(),
            name:self.name.clone(),
            job:self.job.clone(),
            age:self.age,
            gender:self.gender.clone(),
            relation:self.relation.clone(),
            sons:Vec::<Person>::new()
        };
    }
}

impl Person{
    fn from<'person>(name:&'person str  , job:&'person str  , age:u8 , gender:&'person str , relation:&'person str  )->Result<Self,String>{
        
        let mut person:Self = Self::new();
        
        person.name=name.to_string();
        person.age=age;
        person.job=job.to_string();

        match gender.to_lowercase().as_str() {
           "male"=>{    person.gender=Gender::Male  },
           "female"=>{  person.gender=Gender::Female  },
           "null"=>{    person.gender=Gender::Null  },
           _=>{return Err( format!("can,t Use {} as gender ! only male,female,null are allowed ",gender));}
       };
        
        match relation.to_lowercase().as_str() {
           "single"=>{  person.relation=PersonRelationShip::Single  },
           "married"=>{ person.relation=PersonRelationShip::Married  },
           "seperated"=>{   person.relation=PersonRelationShip::Seperated   },
           _=>{return  Err(format!("can,t Use {} as relation ! only single,married,seperated are allowed ",relation));}
        };
        return Ok(person);

    }
    
    fn set_id(&mut self , id:&str){
        self.id=id.to_string();
    }

    fn add_son(&mut self,son:&Person){
        self.sons.push(son.clone());
    }

    fn set_job(&mut self,job:&str){
        self.job=job.to_string();
    }

    fn set_name(&mut self,name:&str){
        self.name=name.to_string();
    }

    fn describe(&self){
        
        let gender:&str = if self.gender.is_male(){
            "male"
        }
        else if self.gender.is_female(){
            "female"
        }
        else{
            "null"
        };

    let relation:&str = if self.relation.is_single(){
            "single"
        }
        else if self.relation.is_married(){
            "married"
        }
        else{
            "seperated"
        };


        if self.sons.len()>0{
           
            print!("Person {{ id: {} , name: {} , age: {} , job: {} , gender: {} , relation : {} , sons : [",self.id , self.name , self.age , self.job , gender ,relation );
            for i in &self.sons{
                i.describe();
                println!(" , ");
            }
            println!("]}}");
        }
        else{
            println!("Person {{ id: {} , name: {} , age: {} , job: {} , gender: {} , relation : {} , sons : [] }}",self.id , self.name , self.age , self.job , gender ,relation );

        }
    }
    
}




fn main(){
  
   let mut me : Person=match Person::from("eslam", "Software engineer", 22,"male","single") {
    Ok(person)=>person,
    Err(error)=>panic!("Error!  : {}",error)
   };

   let mut sam:Person=Person::new();
   sam.set_name("sam");
   
   me.add_son(&sam);
   me.describe();

}
