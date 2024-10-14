use evolut::creature::{brain::Brain, genome::Gene};

fn main() {
    let genes = vec![
        Gene::new(0, 0, 0.1),
        Gene::new(0, 128, 0.2),
        Gene::new(129, 130, 0.3),
        Gene::new(131, 130, 0.4),
        Gene::new(130, 1, 0.5),
        Gene::new(1, 131, 0.6),
        Gene::new(2, 131, 0.7),
        Gene::new(3, 132, 0.8),
        Gene::new(132, 131, 0.9),
        Gene::new(130, 132, 1.0),
        Gene::new(131, 131, 1.1),
    ];

    let brain = Brain::new(&genes);

    for neuron in brain.neurons() {
        println!("{:#?}", neuron);
    }
}
