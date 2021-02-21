use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub id: u32,
    pub title: String,
    pub tag: String,
    pub description: String,
    pub speaker_id: u32,
}

pub fn generate_examples() -> Vec<Session> {
    let sessions = vec![
    Session {
        id: 1,
        title: "Relativity is King".into(),
        tag: "Keynote".into(),
        speaker_id: 1,
        description:"The theory of relativity transformed theoretical physics and astronomy during the 20th century, superseding a 200-year-old theory of mechanics created primarily by Isaac Newton.It introduced concepts including spacetime as a unified entity of space and time, relativity of simultaneity, kinematic and gravitational time dilation, and length contraction. In the field of physics, relativity improved the science of elementary particles and their fundamental interactions, along with ushering in the nuclear age. With relativity, cosmology and astrophysics predicted extraordinary astronomical phenomena such as neutron stars, black holes, and gravitational waves.".into()
    },

    Session {
        id: 2,
        title: "Uncertain about Uncertainty?".into(),
        tag: "".into(),
        speaker_id: 2,
        description:"The uncertainty principle states that the more precisely the position of some particle is determined, the less precisely its momentum can be predicted from initial conditions, and vice versa. The listener will also learn how to not confuse this with the observer problem.".into()
    },

    Session {
        id: 3,
        title: "Welcome to Copenhagen".into(),
        tag: "".into(),
        speaker_id: 3,
        description:"While Copenhagen is a beautiful city in Denmark and always worth a visit, the Copenhagen interpretation is a collection of views about the meaning of quantum mechanics principally attributed to Niels Bohr and Werner Heisenberg. It is one of the oldest of numerous proposed interpretations of quantum mechanics, as features of it date to the development of quantum mechanics during 1925–1927, and it remains one of the most commonly taught.".into()
    },

    Session {
        id: 4,
        title: "Everything you know about cats is wrong!".into(),
        tag: "".into(),
        speaker_id: 4,
        description:"This talk will present a thought experiment that illustrates an apparent paradox of quantum superposition. In the thought experiment, a hypothetical cat may be considered simultaneously both alive and dead as a result of being linked to a random subatomic event that may or may not occur. No actual cats will be hurt.".into()
    },
    ];

    sessions
}
