fn main() {
    let nameofcommisioner =  vec! ["Aigbobun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etiye"];
    let ministry = vec! ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geopoloticalzone = vec! ["South West", "North East", "South South", "South West", "South East"];

    print! ("\nMinisters from different geopolitical zones in the country:\n");
    for i in 0..geopoloticalzone.len() {
        print!("{} in the {} ministry located in the {} geopolitical zone\n", nameofcommisioner[i], ministry[i], geopoloticalzone[i]);
    }
}