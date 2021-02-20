use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Speaker {
    id: u32,
    full_name: String,
    twitter: String,
    bio: String,
    //image
}

pub fn generate_examples() -> Vec<Speaker> {
    let speakers = vec![
        Speaker {
            id: 1,
            full_name: "Albert Einstein".into(),
            twitter: "@TheAlbertEinstein".into(),
            bio:"Albert is a German-born theoretical physicist universally 
            acknowledged to be one of the two greatest physicists of all time, 
            the other being Isaac Newton. Einstein developed the theory of relativity, 
            one of the two pillars of modern physics (alongside quantum mechanics). 
            His work is also known for its influence on the philosophy of science. 
            His mass–energy equivalence formula E = mc2 has been dubbed 'the world's most famous equation'. 
            He received the 1921 Nobel Prize in Physics 'for his services to theoretical physics, and 
            especially for his discovery of the law of the photoelectric effect', a pivotal step in the 
            development of quantum theory. His intellectual achievements and originality resulted 
            in 'Einstein' becoming synonymous with 'genius'".into()
        },
        
        
        ];

    speakers
}
