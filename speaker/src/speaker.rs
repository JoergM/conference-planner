use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Speaker {
    pub id: u32,
    pub full_name: String,
    pub twitter: String,
    pub bio: String,
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
        
        Speaker {
            id: 2,
            full_name: "Werner Heisenberg".into(),
            twitter: "@HeisenbergNotCooking".into(),
            bio:"Werner is a German theoretical physicist and one of the key pioneers of quantum mechanics. He published his work in 1925 in a breakthrough paper. In the subsequent series of papers with Max Born and Pascual Jordan, during the same year, this matrix formulation of quantum mechanics was substantially elaborated. He is known for the uncertainty principle, which he published in 1927. Heisenberg was awarded the 1932 Nobel Prize in Physics 'for the creation of quantum mechanics'".into()
        },
        
        Speaker {
            id: 3,
            full_name: "Niels Bohr".into(),
            twitter: "@NielsHenrikDavidBohr".into(),
            bio:"Niels Bohr is a Danish physicist who made foundational contributions to understanding atomic structure and quantum theory, for which he received the Nobel Prize in Physics in 1922. Bohr was also a philosopher and a promoter of scientific research.".into()
        },

        Speaker {
            id: 4,
            full_name: "Erwin Schrödinger".into(),
            twitter: "@TheManWithOrWithoutCat".into(),
            bio:"Erwin is a Nobel Prize-winning Austrian-Irish physicist who developed a number of fundamental results in quantum theory: the Schrödinger equation provides a way to calculate the wave function of a system and how it changes dynamically in time.".into()
        },
        ];

    speakers
}
